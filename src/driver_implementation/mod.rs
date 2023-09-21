// Copyright 2023 Raven Industries inc.


//! Driver implementation for different interfaces
//! This module defines:
//! 1. An abstract `Driver` trait for different CAN drivers to implement


mod driver;

#[cfg(feature = "socketcan")]
mod socketcan;

pub use driver::{Driver, DriverCloseError, DriverOpenError, DriverReadError, DriverWriteError};

#[cfg(feature = "socketcan")]
pub use self::socketcan::SocketcanDriver;
