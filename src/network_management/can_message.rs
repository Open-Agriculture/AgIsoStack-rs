// Copyright 2023 Raven Industries inc.
use super::name::DEFAULT_NAME;
use super::name::NAME;
use crate::driver::CanId;

pub struct CANMessage {
    data: Vec<u8>,
    identifier: CanId,
    source_name: NAME,
    destination_name: NAME,
}

impl CANMessage {
    pub(super) fn new(data: Vec<u8>, identifier: CanId) -> CANMessage {
        CANMessage {
            data,
            identifier,
            source_name: NAME {
                raw_name: DEFAULT_NAME,
            },
            destination_name: NAME {
                raw_name: DEFAULT_NAME,
            },
        }
    }
}
