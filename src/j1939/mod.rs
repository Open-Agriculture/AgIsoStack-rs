// Copyright 2023 Raven Industries inc.

//! CAN Driver layer
//!
//! This module defines:
//! 1. An abstract `Driver` trait for different CAN drivers to implement
//! 2. `Frame`, `Pgn`, `Address`, et al types

mod address;
mod driver;
mod extended_id;
mod frame;
mod page;
mod pgn;
mod priority;
mod standard_id;
mod id;

pub use address::Address;
pub use driver::{Driver, DriverCloseError, DriverOpenError, DriverReadError, DriverWriteError};
pub use extended_id::ExtendedId;
pub use frame::{Channel, Frame};
pub use pgn::Pgn;
pub use priority::Priority;
