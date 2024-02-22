// Copyright 2023 Raven Industries inc.
use crate::j1939::Frame;

#[derive(Debug)]
#[non_exhaustive]
pub enum DriverOpenError {
    /// The j1939 failed to open with filesystem semantics
    IoError(std::io::Error),
}

impl std::fmt::Display for DriverOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to open j1939: {:?}", self)
    }
}
impl std::error::Error for DriverOpenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            DriverOpenError::IoError(e) => Some(e),
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
    /// The j1939 has been closed
    DriverClosed,
    /// The j1939 received an error frame
    ErrorFrame(),
    /// The j1939 failed to read with filesystem semantics
    IoError(std::io::Error),
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
    /// The j1939's internal buffer is full, or the j1939 is otherwise busy
    NotReady,
    /// The j1939 has been closed
    DriverClosed,
    /// Some fault with the CAN bus
    BusError(),
    /// Some fault with filesystem semantics
    IoError(std::io::Error),
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
/// This trait does _not_ define how to construct and configure a j1939, as the details are likely
/// to differ from j1939 to j1939.
pub trait Driver {
    /// Determine whether the j1939 is connected and healthy
    fn is_valid(&self) -> bool;

    /// Open the j1939
    ///
    /// It is expected you must open the j1939 after creating it
    fn open(&mut self) -> Result<(), DriverOpenError>;

    /// Close the j1939
    ///
    /// It is not necessary to close the j1939 before dropping it
    fn close(&mut self) -> Result<(), DriverCloseError>;

    /// Read a [Frame] from the j1939, if possible
    ///
    /// This is a non-blocking read. If there is no frame ready to read, this function will return
    /// [DriverReadError::NoFrameReady].
    ///
    /// An out-parameter is used, so that the user can choose whether to construct a new frame for
    /// each call, or to re-use memory.
    fn read_nonblocking(&mut self, frame: &mut Frame) -> Result<(), DriverReadError>;

    /// Write a [Frame] to the j1939, if possible
    ///
    /// This is a non-blocking write. If the frame cannot be written because the j1939's
    /// queue/buffer is full (for drivers like `socketcan` that do internal buffering), or if
    /// it's otherwise busy, this function will return [DriverWriteError::NotReady].
    ///
    /// For drivers that defer to some other implementation (Peak, Socketcan), it's likely that the
    /// given `frame` is copied before being written.
    fn write_nonblocking(&mut self, frame: &Frame) -> Result<(), DriverWriteError>;
}
