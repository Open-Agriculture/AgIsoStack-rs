// Copyright 2023 Raven Industries inc.
use crate::driver::Frame;

#[derive(Debug)]
#[non_exhaustive]
pub enum DriverOpenError {
    /// The driver failed to open with filesystem semantics
    IoError(std::io::Error),
    // TODO: Here and throughout. I don't love the pcan errors. They're not real std::error::Error
    // types, and it's not obvious what they mean. Maybe we should re-think this error design?
    #[cfg(feature = "peak")]
    PeakError(pcan_basic::error::PcanError),
}

impl std::fmt::Display for DriverOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to open driver: {:?}", self)
    }
}
impl std::error::Error for DriverOpenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            DriverOpenError::IoError(e) => Some(e),
            // PcanError doesn't implement the Error trait
            // DriverOpenError::PeakError(e) => Some(e),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}

impl From<std::io::Error> for DriverOpenError {
    fn from(e: std::io::Error) -> DriverOpenError {
        DriverOpenError::IoError(e)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DriverCloseError {}

impl std::fmt::Display for DriverCloseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for DriverCloseError {}

#[derive(Debug)]
#[non_exhaustive]
pub enum DriverReadError {
    /// There is no frame ready to be read
    NoFrameReady,
    /// The driver has been closed
    DriverClosed,
    /// The driver received an error frame
    ErrorFrame(),
    /// The driver failed to read with filesystem semantics
    IoError(std::io::Error),
    #[cfg(feature = "peak")]
    PeakError(pcan_basic::error::PcanError),
}

impl std::fmt::Display for DriverReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for DriverReadError {}

impl From<std::io::Error> for DriverReadError {
    fn from(e: std::io::Error) -> DriverReadError {
        if matches!(e.kind(), std::io::ErrorKind::WouldBlock) {
            DriverReadError::NoFrameReady
        } else {
            DriverReadError::IoError(e)
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DriverWriteError {
    /// The driver's internal buffer is full, or the driver is otherwise busy
    NotReady,
    /// The driver has been closed
    DriverClosed,
    /// Some fault with the CAN bus
    BusError(),
    /// Some fault with filesystem semantics
    IoError(std::io::Error),
    #[cfg(feature = "peak")]
    PeakError(pcan_basic::error::PcanError),
}

impl std::fmt::Display for DriverWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for DriverWriteError {}

impl From<std::io::Error> for DriverWriteError {
    fn from(e: std::io::Error) -> DriverWriteError {
        if matches!(e.kind(), std::io::ErrorKind::WouldBlock) {
            DriverWriteError::NotReady
        } else {
            DriverWriteError::IoError(e)
        }
    }
}

/// Generic interface for CAN drivers
///
/// This layer is meant to abstract the hardware, and should not do its own queuing/buffering.
///
/// This trait does _not_ define how to construct and configure a driver, as the details are likely
/// to differ from driver to driver.
pub trait Driver {
    /// Determine whether the driver is connected and healthy
    fn is_valid(&self) -> bool;

    /// Open the driver
    ///
    /// It is expected you must open the driver after creating it
    fn open(&mut self) -> Result<(), DriverOpenError>;

    /// Close the driver
    ///
    /// It is not necessary to close the driver before dropping it
    fn close(&mut self) -> Result<(), DriverCloseError>;

    /// Read a [Frame] from the driver, if possible
    ///
    /// This is a non-blocking read. If there is no frame ready to read, this function will return
    /// [DriverReadError::NoFrameReady].
    ///
    /// An out-parameter is used, so that the user can choose whether to construct a new frame for
    /// each call, or to re-use memory.
    fn read_nonblocking(&mut self, frame: &mut Frame) -> Result<(), DriverReadError>;

    /// Write a [Frame] to the driver, if possible
    ///
    /// This is a non-blocking write. If the frame cannot be written because the driver's
    /// queue/buffer is full (for drivers like `socketcan` that do internal buffering), or if
    /// it's otherwise busy, this function will return [DriverWriteError::NotReady].
    ///
    /// For drivers that defer to some other implementation (Peak, Socketcan), it's likely that the
    /// given `frame` is copied before being written.
    fn write_nonblocking(&mut self, frame: &Frame) -> Result<(), DriverWriteError>;
}
