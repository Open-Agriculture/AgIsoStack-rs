use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
// Copyright 2023 Raven Industries inc.
use crate::j1939::priority::Priority;
use crate::j1939::{Address, Pgn};
use embedded_can::{ExtendedId, Id as EmbeddedId};

pub enum ParseIdError {
    priority_parse_error,
    pgn_parse_error,
    source_address_parse_error,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Id {
    priority: Priority,
    pgn: Pgn,
    source_address: Address,
}

impl Id {
    pub fn new(priority: Priority, pgn: Pgn, source_address: Address) -> Self {
        Self {
            priority,
            pgn,
            source_address,
        }
    }

    /// Get the raw value of the CAN ID
    #[inline]
    pub fn raw(&self) -> u32 {
        let mut raw_id: BitVec<u32> = BitVec::new();
        raw_id.extend(self.priority);
        raw_id.extend(self.pgn);
        raw_id.extend(self.source_address);
        raw_id.load::<u32>().unwrap()
    }

    /// Get the priority of the ID
    #[inline]
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Get the source address of the ID
    #[inline]
    pub fn source_address(&self) -> Address {
        self.source_address
    }
}

impl From<Id> for EmbeddedId {
    fn from(id: Id) -> Self {
        EmbeddedId::Extended(ExtendedId(id.raw()))
    }
}

impl TryFrom<u32> for Id {
    type Error = ParseIdError;

    fn try_from(raw_id: u32) -> Result<Self, Self::Error> {
        let bit_data = raw_id.view_bits::<Msb0>().to_bitvec();
        let priority_parse = Priority::try_from(bit_data.load::<u8>());
        let pgn_parse = Pgn::try_from(bit_data.load::<u32>());
    }
}

//TODO: tests -> especially for 'bit_data.load::<u32>()'
