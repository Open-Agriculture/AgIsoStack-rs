// Copyright 2023 Raven Industries inc.

use socketcan::{CanSocket, Socket};

use crate::driver::{
    Driver, DriverCloseError, DriverOpenError, DriverReadError, DriverWriteError, Frame,
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
        match &self.iface {
            SocketcanIface::Name(s) => self.sock = Some(CanSocket::open(s)?),
            SocketcanIface::Index(i) => self.sock = Some(CanSocket::open_iface(*i)?),
        }
        // NOTE: To get any kind of non-blocking behavior, EVEN if using NonBlockingCan::receive()
        // you MUST set this flag. But setting this flag causes even BlockingCan::receive() to
        // return immediately with an error if there is no frame ready.
        // NOTE: unwrap() is safe, because we return a DriverOpenError if we fail to create it.
        self.sock.as_ref().unwrap().set_nonblocking(true)?;
        Ok(())
    }
    fn close(&mut self) -> Result<(), DriverCloseError> {
        self.sock = None;
        Ok(())
    }

    fn read_nonblocking(&mut self, _frame: &mut Frame) -> Result<(), DriverReadError> {
        let Some(sock) = self.sock.as_mut() else {
            return Err(DriverReadError::DriverClosed);
        };
        let _frame = sock.read_frame()?;
        // TODO: Convert socketcan CanFrame to Frame.
        Ok(())
    }
    fn write_nonblocking(&mut self, _frame: &Frame) -> Result<(), DriverWriteError> {
        let Some(sock) = self.sock.as_mut() else {
            return Err(DriverWriteError::DriverClosed);
        };
        // TODO: Convert Frame to socketcan CanFrame
        let socketcan_frame = socketcan::CanFrame::default();
        sock.write_frame(&socketcan_frame)?;
        Ok(())
    }
}
