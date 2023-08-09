// Copyright 2023 Raven Industries inc.
#![allow(dead_code)]

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
            source_name: NAME::default(),
            destination_name: NAME::default(),
        }
    }
}
