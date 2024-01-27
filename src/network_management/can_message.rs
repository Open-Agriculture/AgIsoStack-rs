// Copyright 2023 Raven Industries inc.
use super::name::NAME;
use crate::j1939::Id;

pub struct CANMessage {
    data: Vec<u8>,
    identifier: Id,
    source_name: NAME,
    destination_name: NAME,
}

impl CANMessage {
    pub(super) fn new(data: Vec<u8>, identifier: Id) -> CANMessage {
        CANMessage {
            data,
            identifier,
            source_name: NAME::default(),
            destination_name: NAME::default(),
        }
    }

    pub fn get_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn get_identifier(&self) -> Id {
        self.identifier
    }

    pub fn get_source_name(&self) -> NAME {
        self.source_name
    }

    pub fn get_destination_name(&self) -> NAME {
        self.destination_name
    }
}
