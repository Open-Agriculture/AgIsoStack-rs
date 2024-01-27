use socketcan::{embedded_can, Id as EmbeddedId};
// Copyright 2023 Raven Industries inc.
use crate::j1939::Id;
use embedded_can::Frame as EmbeddedFrame;

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Channel(u8);

#[derive(Debug, Default)]
pub struct Frame {
    pub timestamp: std::time::Duration,
    pub id: Id,
    pub data: Vec<u8>,
}

impl Frame {
    pub fn new(id: Id, data: Vec<u8>) -> Self {
        Self {
            timestamp: todo!(),
            id,
            data,
        }
    }
}

impl EmbeddedFrame for Frame {
    fn new(id: impl Into<EmbeddedId>, data: &[u8]) -> Option<Self> {
        Self::new(id.into(), data.to_vec())
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        //J1939 does not support remote frame
        None
    }

    fn is_extended(&self) -> bool {
        // J1939 only supports extended frames
        true
    }

    fn is_standard(&self) -> bool {
        // J1939 only supports extended frames
        false
    }

    fn is_remote_frame(&self) -> bool {
        // J1939 does not support remote frames
        false
    }

    fn is_data_frame(&self) -> bool {
        // J1939 only supports data frames
        true
    }

    fn id(&self) -> EmbeddedId {
        todo!()
    }

    fn dlc(&self) -> usize {
        self.data.len()
    }

    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}
