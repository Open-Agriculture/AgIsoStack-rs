// Copyright 2023 Raven Industries inc.
use crate::j1939::id::Id;
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
        Some(Self {
            id: Id::try_from(id.into()).expect("Invalid J1939 ID"),
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

    /// create a new remote frame
    /// <div class="warning">
    /// J1939 does not support remote frames (see J1939-21 5.4) so this is always [None]
    /// </div>
    fn new_remote(_id: impl Into<EmbeddedId>, _dlc: usize) -> Option<Self> {
        None
    }

    fn is_extended(&self) -> bool {
        match self.id {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_standard(&self) -> bool {
        match self.id {
            Id::Standard(_) => true,
            Id::Extended(_) => false,
        }
    }

    fn is_remote_frame(&self) -> bool {
        // J1939 does not support remote frames (see J1939-21 5.4)
        false
    }

    fn is_data_frame(&self) -> bool {
        // J1939 only supports data frames
        true
    }

    fn id(&self) -> EmbeddedId {
        self.id.into()
    }

    fn dlc(&self) -> usize {
        self.data.len()
    }

    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}
