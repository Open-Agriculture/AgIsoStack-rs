/*
Copyright 2023 Raven Industries inc.

@author Jannes Brands
@date 2024-02-22
*/

use crate::j1939::id::Id;
use embedded_can::{Frame as EmbeddedFrame, Id as EmbeddedId};

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Channel(u8);

/// J1939 frame
#[derive(Debug, Clone, Default)]
pub struct Frame {
    id: Id,
    data: Vec<u8>,
}

impl Frame {
    /// Creates a new J1939 data frame
    pub fn new(id: impl Into<EmbeddedId>, data: Vec<u8>) -> Option<Self> {
        Some(Self {
            id: Id::try_from(id.into()).expect("Invalid J1939 ID"),
            data,
        })
    }

    /// Identifier of the frame
    #[inline]
    pub fn id(&self) -> Id {
        self.id
    }

    /// Data of the frame
    #[inline]
    pub fn data(self) -> Vec<u8> {
        self.data
    }
}

impl EmbeddedFrame for Frame {
    /// Creates a new J1939 data frame from a standard CAN data frame
    fn new(id: impl Into<EmbeddedId>, data: &[u8]) -> Option<Self> {
        Frame::new(id, data.to_vec())
    }

    /// Creates a new remote frame (only to satisfy the trait)
    /// <div class="warning">
    /// This will always return `None` as J1939 does not support remote frames
    /// </div>
    fn new_remote(_id: impl Into<EmbeddedId>, _dlc: usize) -> Option<Self> {
        None
    }

    /// Returns `true` if the frame is an extended frame
    fn is_extended(&self) -> bool {
        match self.id {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    /// Returns `true` if the frame is a standard frame
    fn is_standard(&self) -> bool {
        match self.id {
            Id::Standard(_) => true,
            Id::Extended(_) => false,
        }
    }

    /// returns always `false` as J1939 does not support remote frames
    fn is_remote_frame(&self) -> bool {
        false
    }

    /// returns always `true` as J1939 only supports data frames
    fn is_data_frame(&self) -> bool {
        true
    }

    /// Identifier of the frame
    fn id(&self) -> EmbeddedId {
        self.id.into()
    }

    /// Data length code of the frame
    fn dlc(&self) -> usize {
        self.data.len()
    }

    /// Data of the frame
    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}
