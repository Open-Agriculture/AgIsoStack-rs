// Copyright 2023 Raven Industries inc.

//! CAN Driver layer
//!
//! This module defines:
//! 1. An abstract `Driver` trait for different CAN drivers to implement
//! 2. `Frame`, `Pgn`, `Address`, et al types

mod address;
mod can_id;
mod driver;
mod frame;
mod pgn;

#[cfg(feature = "socketcan")]
mod socketcan;

#[cfg(feature = "peak")]
mod peak;

pub use address::Address;
pub use can_id::{CanId, Priority, Type};
pub use driver::{Driver, DriverCloseError, DriverOpenError, DriverReadError, DriverWriteError};
pub use frame::{Channel, Frame};
pub use pgn::Pgn;

#[cfg(feature = "socketcan")]
pub use self::socketcan::SocketcanDriver;

#[cfg(feature = "peak")]
pub use self::peak::PeakDriver;
#[cfg(feature = "peak")]
pub use self::peak::Baudrate;

