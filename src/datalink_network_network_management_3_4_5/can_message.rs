// Copyright 2023 Raven Industries inc.
use super::name::{DEFAULT_NAME, NAME};
use crate::datalink_network_network_management_3_4_5::CanId;

pub struct CANMessage {
    data: Vec<u8>,
    identifier: CanId,
    source_name: NAME,
    destination_name: NAME,
}

impl CANMessage {
    pub(crate) fn new(data: Vec<u8>, identifier: CanId) -> CANMessage {
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

    pub fn get_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn get_identifier(&self) -> CanId {
        self.identifier
    }

    pub fn get_source_name(&self) -> NAME {
        self.source_name
    }

    pub fn get_destination_name(&self) -> NAME {
        self.destination_name
    }
}
