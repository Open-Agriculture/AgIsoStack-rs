pub use pcan_basic::bus::UsbBus;
// NOTE: PcanError is undocumented in the Rust source - you should go to
// https://github.com/tsabelmann/pcan-basic-sys/blob/master/header/PCANBasic.h for error
// documentation.
//
// PcanError::XmtFull - controller transmit buffer full
// PcanError::Overrun - controller was read too late
// PcanError::BusLight - error counter reached the light limit
// PcanError::BusHeavy - error counter reached the heavy limit
// PcanError::BusPassive
// PcanError::BusOff
// PcanError::AnyBusErr
// PcanError::QrcvEmpty - receive queue empty
// PcanError::QOverrun - receive queue overrun
// PcanError::QxmtFull - transmit queue full
// PcanError::RegTest - controller hardware registers test failed
// PcanError::NoDriver - driver not loaded
// PcanError::HwInUse - Hardware already in use by a network
// PcanError::NetInUse - A client is already connected to the Net
// PcanError::IllHw - Hardware handle is invalid
// PcanError::IllNet - Net handle is invalid
// PcanError::IllClient - Client handle is invalid
// PcanError::Resource - Resource (FIFO, Client, timeout) cannot be created
// PcanError::IllParamType - Invalid parameter
// PcanError::IllParamVal - Invalid parameter value
// PcanError::Unknown
// PcanError::IllData - Invalid data, function, or action
// PcanError::IllMode - Driver object state is wrong for the attempted operation
// PcanError::Caution - An opperation was successfully carried out, but irregularities were found
// PcanError::Initialize - Channel not initialized
// PcanError::IllOperation - Invalid operation
pub use pcan_basic::error::PcanError;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::{Baudrate as PcanBaudrate, RecvCan, SendCan};

use crate::driver::{
    Driver, DriverCloseError, DriverOpenError, DriverReadError, DriverWriteError, Frame,
};
use crate::tracing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Baudrate {
    Baud1M,
    Baud800K,
    Baud500K,
    Baud250K,
    Baud125K,
    Baud100K,
    Baud95K,
    Baud83K,
    Baud50K,
    Baud47K,
    Baud33K,
    Baud20K,
    Baud10K,
    Baud5K,
}

// It's unfortunate that we have to duplicate pcan's baudrate enum, but it doesn't implement Clone
// or Debug, so we have to.
impl From<Baudrate> for PcanBaudrate {
    fn from(b: Baudrate) -> PcanBaudrate {
        match b {
            Baudrate::Baud1M => PcanBaudrate::Baud1M,
            Baudrate::Baud800K => PcanBaudrate::Baud800K,
            Baudrate::Baud500K => PcanBaudrate::Baud500K,
            Baudrate::Baud250K => PcanBaudrate::Baud250K,
            Baudrate::Baud125K => PcanBaudrate::Baud125K,
            Baudrate::Baud100K => PcanBaudrate::Baud100K,
            Baudrate::Baud95K => PcanBaudrate::Baud95K,
            Baudrate::Baud83K => PcanBaudrate::Baud83,
            Baudrate::Baud50K => PcanBaudrate::Baud50K,
            Baudrate::Baud47K => PcanBaudrate::Baud47K,
            Baudrate::Baud33K => PcanBaudrate::Baud33K,
            Baudrate::Baud20K => PcanBaudrate::Baud20K,
            Baudrate::Baud10K => PcanBaudrate::Baud10K,
            Baudrate::Baud5K => PcanBaudrate::Baud5K,
        }
    }
}

impl From<PcanError> for DriverOpenError {
    fn from(e: PcanError) -> DriverOpenError {
        DriverOpenError::PeakError(e)
    }
}
impl From<PcanError> for DriverReadError {
    fn from(e: PcanError) -> DriverReadError {
        match e {
            PcanError::QrcvEmpty => DriverReadError::NoFrameReady,
            _ => DriverReadError::PeakError(e),
        }
    }
}
impl From<PcanError> for DriverWriteError {
    fn from(e: PcanError) -> DriverWriteError {
        match e {
            PcanError::QxmtFull | PcanError::XmtFull | PcanError::Overrun => {
                DriverWriteError::NotReady
            }
            _ => DriverWriteError::PeakError(e),
        }
    }
}

pub struct PeakDriver {
    sock: Option<UsbCanSocket>,
    bus: UsbBus,
    baud: Baudrate,
}

impl PeakDriver {
    pub fn new(bus: UsbBus, baud: Baudrate) -> Self {
        Self {
            sock: None,
            bus,
            baud,
        }
    }
}

impl Driver for PeakDriver {
    fn is_valid(&self) -> bool {
        self.sock.is_some()
    }
    fn open(&mut self) -> Result<(), DriverOpenError> {
        // pcan's Baudrate type doesn't implement Clone, so you can't pass it to open() behind the
        // &mut self mutable reference.
        match UsbCanSocket::open(self.bus, self.baud.clone().into()) {
            Ok(sock) => {
                tracing::info!("Opened {:?} @ {:?}", self.bus, self.baud);
                self.sock = Some(sock);
                Ok(())
            }
            Err(e) => {
                let e = e.into();
                tracing::error!("Failed to open {:?} @ {:?}: {e:?}", self.bus, self.baud);
                Err(e)
            }
        }
    }
    fn close(&mut self) -> Result<(), DriverCloseError> {
        self.sock = None;
        Ok(())
    }

    fn read_nonblocking(&mut self, _frame: &mut Frame) -> Result<(), DriverReadError> {
        let Some(sock) = self.sock.as_mut() else {
            tracing::warn!("Tried to read from closed interface {:?}", self.bus);
            return Err(DriverReadError::DriverClosed);
        };

        match sock.recv() {
            Ok((_frame, _timestamp)) => {
                tracing::trace!("Received {_frame:?} @ {_timestamp:?}");
                Ok(())
            }
            Err(PcanError::QrcvEmpty) => Err(DriverReadError::NoFrameReady),
            Err(e) => {
                tracing::error!("Error receiving frame: {e:?}");
                Err(e.into())
            }
        }
    }

    fn write_nonblocking(&mut self, _frame: &Frame) -> Result<(), DriverWriteError> {
        let Some(sock) = self.sock.as_mut() else {
            tracing::warn!("Tried to write to closed interface {:?}", self.bus);
            return Err(DriverWriteError::DriverClosed);
        };

        let pcan_frame = pcan_basic::socket::CanFrame::default();
        match sock.send(pcan_frame) {
            Ok(_) => {
                tracing::trace!("Wrote frame: {_frame:?}");
                Ok(())
            }
            Err(e @ (PcanError::QxmtFull | PcanError::XmtFull | PcanError::Overrun)) => {
                Err(e.into())
            }
            Err(e) => {
                tracing::error!("Error: {e:?} writing frame: {_frame:?}");
                Err(e.into())
            }
        }
    }
}
