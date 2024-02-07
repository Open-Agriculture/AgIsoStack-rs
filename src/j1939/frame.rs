// Copyright 2023 Raven Industries inc.
use crate::j1939::Id;
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
        let frame_id = match id.into() {
            EmbeddedId::Standard(_) => None,
            EmbeddedId::Extended(id) => Some(id),
        };

        let parsed_id = Id::try_from(EmbeddedId::Extended(frame_id.unwrap()));

        if frame_id.is_none() || parsed_id.is_err() {
            return None;
        }

        Some(Self {
            id: parsed_id.unwrap(),
            data,
        })
    }

    #[inline]
    pub fn id(&self) -> Id {
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
