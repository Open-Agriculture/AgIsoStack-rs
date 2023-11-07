// Copyright 2023 Raven Industries inc.
use std::io::ErrorKind;
use std::time::Instant;

use pcan_basic::bus::{DngBus, IsaBus, LanBus, PccBus, PciBus, UsbBus};
use pcan_basic::error::PcanError;
use pcan_basic::socket::dng::DngCanSocket;
use pcan_basic::socket::isa::IsaCanSocket;
use pcan_basic::socket::lan::LanCanSocket;
use pcan_basic::socket::pcc::PccCanSocket;
use pcan_basic::socket::pci::PciCanSocket;
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::{Baudrate as PeakBaudrate, CanFrame, MessageType, RecvCan, SendCan};

use crate::driver::{
    Baudrate, CanId, Channel, Driver, DriverCloseError, DriverOpenError, DriverReadError,
    DriverWriteError, Frame as InternalFrame, Type,
};

impl From<PcanError> for DriverOpenError {
    fn from(e: PcanError) -> DriverOpenError {
        match e {
            PcanError::RegTest => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Test of the CAN controller hardware registers failed (no hardware found)",
            )),
            PcanError::NoDriver => {
                DriverOpenError::IoError(std::io::Error::new(ErrorKind::Other, "Driver not loaded"))
            }
            PcanError::HwInUse => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::AddrInUse,
                "Hardware already in use by a Net",
            )),
            PcanError::NetInUse => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::AddrInUse,
                "A Client is already connected to the Net",
            )),
            PcanError::IllHw => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Hardware handle is invalid",
            )),
            PcanError::IllNet => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Net handle is invalid",
            )),
            PcanError::IllClient => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Client handle is invalid",
            )),
            PcanError::Resource => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Resource (FIFO, Client, timeout) cannot be created",
            )),
            PcanError::IllParamType => {
                DriverOpenError::IoError(std::io::Error::new(ErrorKind::Other, "Invalid parameter"))
            }
            PcanError::IllParamVal => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Invalid parameter value",
            )),
            PcanError::Unknown => {
                DriverOpenError::IoError(std::io::Error::new(ErrorKind::Other, "Unknown error"))
            }
            PcanError::IllData => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::InvalidData,
                "Invalid data, function, or action",
            )),
            PcanError::IllMode => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Driver object state is wrong for the attempted operation",
            )),
            PcanError::Initialize => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Channel is not initialized [Value was changed from 0x40000 to 0x4000000]",
            )),
            PcanError::IllOperation => DriverOpenError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Invalid operation [Value was changed from 0x80000 to 0x8000000]",
            )),
            _ => DriverOpenError::IoError(std::io::Error::new(ErrorKind::Other, "Unknown")),
        }
    }
}

impl From<PcanError> for DriverReadError {
    fn from(e: PcanError) -> DriverReadError {
        match e {
            PcanError::QrcvEmpty | PcanError::QOverrun => DriverReadError::NoFrameReady,
            PcanError::Resource => DriverReadError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Resource (FIFO, Client, timeout) cannot be created",
            )),
            PcanError::IllParamType => {
                DriverReadError::IoError(std::io::Error::new(ErrorKind::Other, "Invalid parameter"))
            }
            PcanError::IllParamVal => DriverReadError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Invalid parameter value",
            )),
            PcanError::Unknown => {
                DriverReadError::IoError(std::io::Error::new(ErrorKind::Other, "Unknown error"))
            }
            PcanError::IllData => DriverReadError::IoError(std::io::Error::new(
                ErrorKind::InvalidData,
                "Invalid data, function, or action",
            )),
            _ => DriverReadError::IoError(std::io::Error::new(ErrorKind::Other, "Unknown")),
        }
    }
}

impl From<PcanError> for DriverWriteError {
    fn from(e: PcanError) -> DriverWriteError {
        match e {
            PcanError::AnyBusErr => DriverWriteError::BusError(),
            PcanError::XmtFull | PcanError::QxmtFull => DriverWriteError::NotReady,
            PcanError::IllOperation => DriverWriteError::IoError(std::io::Error::new(
                ErrorKind::Other,
                "Invalid operation [Value was changed from 0x80000 to 0x8000000]",
            )),
            _ => DriverWriteError::IoError(std::io::Error::new(ErrorKind::Other, "Unknown")),
        }
    }
}

impl From<&InternalFrame> for CanFrame {
    fn from(f: &InternalFrame) -> CanFrame {
        let msg_type = match f.id.type_() {
            Type::Standard => MessageType::Standard,
            Type::Extended => MessageType::Extended,
        };
        CanFrame::new(
            f.id.raw(),
            msg_type,
            &f.data[..f.data_length.min(8) as usize],
        )
        // guaranteed to not crash, because `f.data` is an [u8; 8]
        .expect("Can frame had too much data")
    }
}

// It's unfortunate that we have to duplicate pcan's baudrate enum, but it doesn't implement Clone
// or Debug, so we have to.
impl From<Baudrate> for PeakBaudrate {
    fn from(b: Baudrate) -> PeakBaudrate {
        match b {
            Baudrate::Baud1M => PeakBaudrate::Baud1M,
            Baudrate::Baud800K => PeakBaudrate::Baud800K,
            Baudrate::Baud500K => PeakBaudrate::Baud500K,
            Baudrate::Baud250K => PeakBaudrate::Baud250K,
            Baudrate::Baud125K => PeakBaudrate::Baud125K,
            Baudrate::Baud100K => PeakBaudrate::Baud100K,
            Baudrate::Baud95K => PeakBaudrate::Baud95K,
            Baudrate::Baud83K => PeakBaudrate::Baud83,
            Baudrate::Baud50K => PeakBaudrate::Baud50K,
            Baudrate::Baud47K => PeakBaudrate::Baud47K,
            Baudrate::Baud33K => PeakBaudrate::Baud33K,
            Baudrate::Baud20K => PeakBaudrate::Baud20K,
            Baudrate::Baud10K => PeakBaudrate::Baud10K,
            Baudrate::Baud5K => PeakBaudrate::Baud5K,
        }
    }
}

enum PeakIface {
    Dng(DngBus),
    Isa(IsaBus),
    Lan(LanBus),
    Pcc(PccBus),
    Pci(PciBus),
    Usb(UsbBus),
}

enum PeakSocket {
    Dng(DngCanSocket),
    Isa(IsaCanSocket),
    Lan(LanCanSocket),
    Pcc(PccCanSocket),
    Pci(PciCanSocket),
    Usb(UsbCanSocket),
}

/// PCan Basic Driver [Driver]
///
/// Enabled with the optional `pcan-basic` feature
pub struct PeakDriver {
    iface: PeakIface,
    baudrate: Baudrate,
    socket: Option<PeakSocket>,
    opened_timestamp: Instant,
}

impl PeakDriver {
    pub fn new_dng(if_bus: DngBus, baudrate: Baudrate) -> Self {
        Self {
            iface: PeakIface::Dng(if_bus),
            baudrate,
            socket: None,
            opened_timestamp: Instant::now(),
        }
    }

    pub fn new_isa(if_bus: IsaBus, baudrate: Baudrate) -> Self {
        Self {
            iface: PeakIface::Isa(if_bus),
            baudrate,
            socket: None,
            opened_timestamp: Instant::now(),
        }
    }

    pub fn new_lan(if_bus: LanBus, baudrate: Baudrate) -> Self {
        Self {
            iface: PeakIface::Lan(if_bus),
            baudrate,
            socket: None,
            opened_timestamp: Instant::now(),
        }
    }

    pub fn new_pcc(if_bus: PccBus, baudrate: Baudrate) -> Self {
        Self {
            iface: PeakIface::Pcc(if_bus),
            socket: None,
            baudrate,
            opened_timestamp: Instant::now(),
        }
    }

    pub fn new_pci(if_bus: PciBus, baudrate: Baudrate) -> Self {
        Self {
            iface: PeakIface::Pci(if_bus),
            baudrate,
            socket: None,
            opened_timestamp: Instant::now(),
        }
    }

    pub fn new_usb(if_bus: UsbBus, baudrate: Baudrate) -> Self {
        Self {
            iface: PeakIface::Usb(if_bus),
            baudrate,
            socket: None,
            opened_timestamp: Instant::now(),
        }
    }

    fn to_frame(&self, frame: CanFrame) -> InternalFrame {
        let timestamp = self.opened_timestamp.elapsed();
        let raw_id = frame.can_id();
        let extended = frame.is_extended_frame();
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
        let data_length = usize::from(frame.dlc().min(8));
        data[..data_length].copy_from_slice(frame.data());
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

impl Driver for PeakDriver {
    fn is_valid(&self) -> bool {
        self.socket.is_some()
    }
    fn open(&mut self) -> Result<(), DriverOpenError> {
        self.socket = match &self.iface {
            PeakIface::Dng(dng) => Some(PeakSocket::Dng(DngCanSocket::open(
                *dng,
                PeakBaudrate::from(self.baudrate),
            )?)),
            PeakIface::Isa(isa) => Some(PeakSocket::Isa(IsaCanSocket::open(
                *isa,
                PeakBaudrate::from(self.baudrate),
            )?)),
            PeakIface::Lan(lan) => Some(PeakSocket::Lan(LanCanSocket::open(
                *lan,
                PeakBaudrate::from(self.baudrate),
            )?)),
            PeakIface::Pcc(pcc) => Some(PeakSocket::Pcc(PccCanSocket::open(
                *pcc,
                PeakBaudrate::from(self.baudrate),
            )?)),
            PeakIface::Pci(pci) => Some(PeakSocket::Pci(PciCanSocket::open(
                *pci,
                PeakBaudrate::from(self.baudrate),
            )?)),
            PeakIface::Usb(usb) => Some(PeakSocket::Usb(UsbCanSocket::open(
                *usb,
                PeakBaudrate::from(self.baudrate),
            )?)),
        };
        self.opened_timestamp = Instant::now();

        Ok(())
    }
    fn close(&mut self) -> Result<(), DriverCloseError> {
        self.socket = None;
        Ok(())
    }

    /// Read a frame from the driver, if possible
    ///
    /// The timestamp on the frame is the duration since [`open`](Self::open) was last called.
    fn read_nonblocking(&mut self, frame: &mut InternalFrame) -> Result<(), DriverReadError> {
        let Some(sock) = self.socket.as_mut() else {
            return Err(DriverReadError::DriverClosed);
        };
        let peak_frame = match sock {
            PeakSocket::Dng(dng) => dng.recv()?,
            PeakSocket::Isa(isa) => isa.recv()?,
            PeakSocket::Lan(lan) => lan.recv()?,
            PeakSocket::Pcc(pcc) => pcc.recv()?,
            PeakSocket::Pci(pci) => pci.recv()?,
            PeakSocket::Usb(usb) => usb.recv()?,
        };

        *frame = self.to_frame(peak_frame.0);
        Ok(())
    }
    fn write_nonblocking(&mut self, frame: &InternalFrame) -> Result<(), DriverWriteError> {
        let Some(sock) = self.socket.as_mut() else {
            return Err(DriverWriteError::DriverClosed);
        };
        let peak_frame: CanFrame = frame.into();

        match sock {
            PeakSocket::Dng(dng) => dng.send(peak_frame)?,
            PeakSocket::Isa(isa) => isa.send(peak_frame)?,
            PeakSocket::Lan(lan) => lan.send(peak_frame)?,
            PeakSocket::Pcc(pcc) => pcc.send(peak_frame)?,
            PeakSocket::Pci(pci) => pci.send(peak_frame)?,
            PeakSocket::Usb(usb) => usb.send(peak_frame)?,
        };

        Ok(())
    }
}
