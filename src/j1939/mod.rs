// Copyright 2023 Raven Industries inc.

//! CAN Driver layer
//!
//! This module defines:
//! 1. An abstract `Driver` trait for different CAN drivers to implement
//! 2. `Frame`, `Pgn`, `Address`, et al types

mod address;
mod driver;
mod frame;
mod id;
mod pgn;
mod priority;

pub use address::Address;
pub use driver::{Driver, DriverCloseError, DriverOpenError, DriverReadError, DriverWriteError};
pub use frame::{Channel, Frame};
pub use id::Id;
pub use pgn::Pgn;
pub use priority::Priority;
