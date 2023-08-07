// Copyright 2023 Raven Industries inc.
use crate::driver::CanId;

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Channel(u8);

#[derive(Debug, Default)]
pub struct Frame {
    // TODO: Is a Duration too large (64 + 32 bits) for an object that will be created so often?
    // Would it be better to use a u64 for microseconds?
    // TODO: Is this just a monotonically increasing number, or is it a unix timestamp?
    pub timestamp: std::time::Duration,
    pub id: CanId,
    pub channel: Channel,
    pub data: [u8; 8],
    pub data_length: u8,
    pub extended: bool,
}
