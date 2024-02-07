use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
// Copyright 2023 Raven Industries inc.
use crate::j1939::priority::Priority;
use crate::j1939::{Address, Pgn};
use embedded_can::{ExtendedId, Id as EmbeddedId};

#[derive(Debug)]
pub enum ParseIdError {
    PriorityParseError,
    PgnParseError,
    SourceAddressParseError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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
        raw_id.append(&mut (self.priority as u8).view_bits_mut::<Msb0>().to_bitvec());
        raw_id.append(&mut self.pgn.raw());
        raw_id.append(&mut self.source_address.raw().view_bits::<Msb0>().to_bitvec());
        raw_id.load::<u32>()
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

    /// Get the PGN of the ID
    #[inline]
    pub fn pgn(&self) -> Pgn {
        self.pgn
    }
}

impl From<Id> for EmbeddedId {
    fn from(id: Id) -> Self {
        EmbeddedId::Extended(ExtendedId::new(id.raw()).unwrap_or(ExtendedId::ZERO))
    }
}

impl TryFrom<EmbeddedId> for Id {
    type Error = ParseIdError;

    fn try_from(value: EmbeddedId) -> Result<Self, Self::Error> {
        match value {
            EmbeddedId::Standard(_) => Err(ParseIdError::PgnParseError),
            EmbeddedId::Extended(id) => {
                let bit_data = id.as_raw().view_bits::<Msb0>().to_bitvec();
                let priority = Priority::try_from(bit_data.load::<u8>());
                let pgn = Pgn::try_from(bit_data.load::<u32>());
                let source_address = Address::new(bit_data.load::<u8>());

                if priority.is_err() {
                    return Err(ParseIdError::PriorityParseError);
                }

                if pgn.is_err() {
                    return Err(ParseIdError::PgnParseError);
                }

                Ok(Id::new(priority.unwrap(), pgn.unwrap(), source_address))
            }
        }
    }
}

impl TryFrom<u32> for Id {
    type Error = ParseIdError;

    fn try_from(raw_id: u32) -> Result<Self, Self::Error> {
        let bit_data = raw_id.view_bits::<Msb0>().to_bitvec();
        let priority = Priority::try_from(bit_data.load::<u8>());
        let pgn = Pgn::try_from(bit_data.load::<u32>());
        let source_address = Address::new(bit_data.load::<u8>());

        if priority.is_err() || pgn.is_err() {
            return Err(ParseIdError::PriorityParseError);
        }

        Ok(Id::new(priority.unwrap(), pgn.unwrap(), source_address))
    }
}

//TODO: tests -> especially for 'bit_data.load::<u32>()'
