// Copyright 2023 Raven Industries inc.

use socketcan::frame::{CanDataFrame, CanFrame, Frame};
use socketcan::{CanSocket, EmbeddedFrame, ExtendedId, Id, Socket, StandardId};

use crate::driver::{
    CanId, Channel, Driver, DriverCloseError, DriverOpenError, DriverReadError, DriverWriteError,
    Frame as InternalFrame, Type,
};
use crate::tracing;

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

impl From<CanFrame> for InternalFrame {
    fn from(f: CanFrame) -> InternalFrame {
        match f {
            CanFrame::Remote(_r) => todo!("Remote frames unsupported yet"),
            CanFrame::Error(_e) => todo!("Error frames unsupported yet"),
            CanFrame::Data(f) => {
                // TODO: Keep track of the std::time::Instant when open() was called, and use
                // duration_since()?
                let timestamp = std::time::Duration::default();
                let raw_id = f.raw_id();
                let extended = f.is_extended();
                let frame_type = if extended {
                    Type::Extended
                } else {
                    Type::Standard
                };

                let id = CanId::new(raw_id, frame_type);
                // TODO: The Driver trait doesn't know anything about Channels. We'll want to make
                // it behave consistently with the pcan_basic UsbBus, and the socketcan interface
                // name and index.
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

impl From<&InternalFrame> for socketcan::frame::CanDataFrame {
    fn from(f: &InternalFrame) -> socketcan::frame::CanDataFrame {
        let id = match f.id.type_() {
            Type::Standard => Id::Standard(unsafe { StandardId::new_unchecked(f.id.raw() as u16) }),
            Type::Extended => Id::Extended(unsafe { ExtendedId::new_unchecked(f.id.raw()) }),
        };
        CanDataFrame::new(id, &f.data[..f.data_length.min(8) as usize])
            .expect("Can frame had too much data")
    }
}

#[derive(Debug)]
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
}

impl SocketcanDriver {
    /// Create a socketcan driver with the given interface name. E.g., `can0`, or `vcan1`
    pub fn new_by_name(if_name: &str) -> Self {
        Self {
            iface: SocketcanIface::Name(if_name.to_string()),
            sock: None,
        }
    }

    /// Create a socketcan driver with the given interface index
    pub fn new_by_index(if_index: u32) -> Self {
        Self {
            iface: SocketcanIface::Index(if_index),
            sock: None,
        }
    }
}

impl Driver for SocketcanDriver {
    fn is_valid(&self) -> bool {
        self.sock.is_some()
    }
    fn open(&mut self) -> Result<(), DriverOpenError> {
        tracing::info!("Opening interface {:?}", self.iface);
        let result = match &self.iface {
            SocketcanIface::Name(s) => CanSocket::open(s),
            SocketcanIface::Index(i) => CanSocket::open_iface(*i),
        };
        match result {
            Ok(sock) => self.sock = Some(sock),
            Err(e) => {
                tracing::error!("Error '{e:?}' opening interface {:?}", self.iface);
                return Err(e.into());
            }
        }
        // NOTE: To get any kind of non-blocking behavior, EVEN if using NonBlockingCan::receive()
        // you MUST set this flag. But setting this flag causes even BlockingCan::receive() to
        // return immediately with an error if there is no frame ready.
        // NOTE: unwrap() is safe, because we return a DriverOpenError if we fail to create it.
        self.sock.as_ref().unwrap().set_nonblocking(true)?;
        Ok(())
    }
    fn close(&mut self) -> Result<(), DriverCloseError> {
        tracing::info!("Closing interface {:?}", self.iface);
        self.sock = None;
        Ok(())
    }

    fn read_nonblocking(&mut self, frame: &mut InternalFrame) -> Result<(), DriverReadError> {
        let Some(sock) = self.sock.as_mut() else {
            tracing::warn!("Failed to read from closed interface {:?}", self.iface);
            return Err(DriverReadError::DriverClosed);
        };
        let socketcan_frame = match sock.read_frame() {
            Ok(frame) => frame,
            Err(e) => {
                if e.kind() != std::io::ErrorKind::WouldBlock {
                    tracing::error!(
                        "Error '{e:?}' receiving frame from interface {:?}",
                        self.iface
                    );
                }
                return Err(e.into());
            }
        };
        *frame = socketcan_frame.into();
        tracing::trace!("Read frame {frame:?} from interface {:?}", self.iface);
        Ok(())
    }
    fn write_nonblocking(&mut self, frame: &InternalFrame) -> Result<(), DriverWriteError> {
        let Some(sock) = self.sock.as_mut() else {
            tracing::warn!("Tried to write to closed interface {:?}", self.iface);
            return Err(DriverWriteError::DriverClosed);
        };
        let socketcan_frame: socketcan::frame::CanDataFrame = frame.into();
        match sock.write_frame(&socketcan_frame) {
            Ok(_) => tracing::trace!("Wrote frame {frame:?} to interface {:?}", self.iface),
            Err(_e) => {
                if _e.kind() != std::io::ErrorKind::WouldBlock {
                    tracing::error!(
                        "Error '{_e:?}' writing frame {frame:?} to interface {:?}",
                        self.iface
                    );
                }
            }
        }
        Ok(())
    }
}
