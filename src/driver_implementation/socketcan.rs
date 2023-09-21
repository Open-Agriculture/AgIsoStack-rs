// Copyright 2023 Raven Industries inc.
use std::time::Instant;

use socketcan::frame::{CanDataFrame, CanFrame, Frame};
use socketcan::{CanSocket, EmbeddedFrame, ExtendedId, Id, Socket, StandardId};

use crate::driver_implementation::{
    Driver,
    DriverCloseError,
    DriverOpenError,
    DriverReadError,
    DriverWriteError
};

use crate::datalink_network_network_management_3_4_5::{
    CanId,
    Channel,
    Frame as InternalFrame, Type,
};

impl From<socketcan::Error> for DriverReadError {
    fn from(e: socketcan::Error) -> DriverReadError {
        match e {
            socketcan::Error::Can(_) => DriverReadError::ErrorFrame(),
            socketcan::Error::Io(e) => DriverReadError::IoError(e),
        }
    }
}

impl From<socketcan::Error> for DriverWriteError {
    fn from(e: socketcan::Error) -> DriverWriteError {
        match e {
            socketcan::Error::Can(_) => DriverWriteError::BusError(),
            socketcan::Error::Io(e) => DriverWriteError::IoError(e),
        }
    }
}

impl From<&InternalFrame> for socketcan::frame::CanDataFrame {
    fn from(f: &InternalFrame) -> socketcan::frame::CanDataFrame {
        let id = match f.id.type_() {
            Type::Standard => Id::Standard(unsafe { StandardId::new_unchecked(f.id.raw() as u16) }),
            Type::Extended => Id::Extended(unsafe { ExtendedId::new_unchecked(f.id.raw()) }),
        };
        CanDataFrame::new(id, &f.data[..f.data_length.min(8) as usize])
            // guaranteed to not crash, because `f.data` is an [u8; 8]
            .expect("Can frame had too much data")
    }
}

enum SocketcanIface {
    Name(String),
    Index(u32),
}

/// A Linux socketcan [Driver]
///
/// Enabled with the optional `socketcan` feature
pub struct SocketcanDriver {
    iface: SocketcanIface,
    sock: Option<CanSocket>,
    opened_timestamp: Instant,
}

impl SocketcanDriver {
    /// Create a socketcan driver with the given interface name. E.g., `can0`, or `vcan1`
    pub fn new_by_name(if_name: &str) -> Self {
        Self {
            iface: SocketcanIface::Name(if_name.to_string()),
            sock: None,
            opened_timestamp: Instant::now(),
        }
    }

    /// Create a socketcan driver with the given interface index
    pub fn new_by_index(if_index: u32) -> Self {
        Self {
            iface: SocketcanIface::Index(if_index),
            sock: None,
            opened_timestamp: Instant::now(),
        }
    }

    fn to_frame(&self, f: CanFrame) -> InternalFrame {
        match f {
            CanFrame::Remote(_r) => todo!("Remote frames unsupported yet"),
            CanFrame::Error(_e) => todo!("Error frames unsupported yet"),
            CanFrame::Data(f) => {
                let timestamp = self.opened_timestamp.elapsed();
                let raw_id = f.raw_id();
                let extended = f.is_extended();
                let frame_type = if extended {
                    Type::Extended
                } else {
                    Type::Standard
                };

                let id = CanId::new(raw_id, frame_type);
                // TODO: The Driver trait doesn't know anything about Channels yet.
                //
                // The channel exists so that we can tie Frames and CANMessages back to the network
                // manager they originated from. This channel value should be passed to the Driver
                // when it's created (or opened?)
                let channel = Channel::default();
                let mut data = [0; 8];
                let data_length = f.dlc().min(8);
                data[..data_length].copy_from_slice(f.data());
                let data_length = data_length as u8;

                InternalFrame {
                    timestamp,
                    id,
                    channel,
                    data,
                    data_length,
                    extended,
                }
            }
        }
    }
}

impl Driver for SocketcanDriver {
    fn is_valid(&self) -> bool {
        self.sock.is_some()
    }
    fn open(&mut self) -> Result<(), DriverOpenError> {
        match &self.iface {
            SocketcanIface::Name(s) => self.sock = Some(CanSocket::open(s)?),
            SocketcanIface::Index(i) => self.sock = Some(CanSocket::open_iface(*i)?),
        }
        self.opened_timestamp = Instant::now();

        // NOTE: unwrap() is safe, because we return a DriverOpenError if we fail to create it.
        self.sock.as_ref().unwrap().set_nonblocking(true)?;
        Ok(())
    }
    fn close(&mut self) -> Result<(), DriverCloseError> {
        self.sock = None;
        Ok(())
    }

    /// Read a frame from the driver, if possible
    ///
    /// The timestamp on the frame is the duration since [`open`](Self::open) was last called.
    fn read_nonblocking(&mut self, frame: &mut InternalFrame) -> Result<(), DriverReadError> {
        let Some(sock) = self.sock.as_mut() else {
            return Err(DriverReadError::DriverClosed);
        };
        let socketcan_frame = sock.read_frame()?;
        *frame = self.to_frame(socketcan_frame);
        Ok(())
    }
    fn write_nonblocking(&mut self, frame: &InternalFrame) -> Result<(), DriverWriteError> {
        let Some(sock) = self.sock.as_mut() else {
            return Err(DriverWriteError::DriverClosed);
        };
        let socketcan_frame: socketcan::frame::CanDataFrame = frame.into();
        sock.write_frame(&socketcan_frame)?;
        Ok(())
    }
}
