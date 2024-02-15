// Copyright 2023 Raven Industries inc.
use crate::j1939::id::Id;
use crate::j1939::standard_id::StandardId;
use crate::j1939::ExtendedId;
use embedded_can::{Frame as EmbeddedFrame, Id as EmbeddedId};

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Channel(u8);

#[derive(Debug, Clone, Default)]
pub struct Frame {
    id: Id,
    data: Vec<u8>,
}

impl Frame {
    pub fn new(id: impl Into<EmbeddedId>, data: Vec<u8>) -> Option<Self> {
        let frame_id: Id = match id.into() {
            EmbeddedId::Standard(id) => StandardId::try_from(EmbeddedId::Standard(id))
                .expect("Invalid standard ID")
                .into(),
            EmbeddedId::Extended(id) => ExtendedId::try_from(EmbeddedId::Extended(id))
                .expect("Invalid extended ID")
                .into(),
        };

        if frame_id {
            return None;
        }

        Some(Self {
            id: parsed_id.unwrap(),
            data,
        })
    }

    #[inline]
    pub fn id(&self) -> ExtendedId {
        self.id
    }

    #[inline]
    pub fn data(self) -> Vec<u8> {
        self.data
    }
}

impl EmbeddedFrame for Frame {
    fn new(id: impl Into<EmbeddedId>, data: &[u8]) -> Option<Self> {
        Frame::new(id, data.to_vec())
    }

    fn new_remote(_id: impl Into<EmbeddedId>, _dlc: usize) -> Option<Self> {
        //J1939 does not support remote frames
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
        EmbeddedId::from(self.id)
    }

    fn dlc(&self) -> usize {
        self.data.len()
    }

    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}
