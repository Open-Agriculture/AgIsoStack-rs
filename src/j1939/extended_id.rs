// Copyright 2023 Raven Industries inc.
use crate::j1939::id::{Id, ParseIdError};
use crate::j1939::priority::Priority;
use crate::j1939::standard_id::StandardId;
use crate::j1939::{Address, Pgn};
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
use embedded_can::{ExtendedId as EmbeddedExtendedId, Id as EmbeddedId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ExtendedId {
    standard_id: StandardId,
    pgn: Pgn,
}

impl ExtendedId {
    pub fn new(standard_id: StandardId, pgn: Pgn) -> Self {
        Self { standard_id, pgn }
    }

    /// Get the raw value of the CAN ID
    #[inline]
    pub fn raw(&self) -> u32 {
        let mut raw_id: BitVec<u32> = BitVec::new();
        raw_id.append(
            &mut (self.standard_id.priority() as u8)
                .view_bits_mut::<Msb0>()
                .to_bitvec(),
        );
        raw_id.append(&mut self.pgn.raw());
        raw_id.append(
            &mut self
                .standard_id
                .source_address()
                .raw()
                .view_bits::<Msb0>()
                .to_bitvec(),
        );
        raw_id.load::<u32>()
    }

    /// Get the PGN of the ID
    #[inline]
    pub fn pgn(&self) -> Pgn {
        self.pgn
    }
}

impl From<ExtendedId> for EmbeddedId {
    fn from(id: ExtendedId) -> Self {
        EmbeddedId::Extended(EmbeddedExtendedId::new(id.raw()).unwrap_or(EmbeddedExtendedId::ZERO))
    }
}

impl TryFrom<EmbeddedId> for ExtendedId {
    type Error = ParseIdError;

    fn try_from(value: EmbeddedId) -> Result<Self, Self::Error> {
        match value {
            EmbeddedId::Standard(_) => Err(ParseIdError::StandardId),
            EmbeddedId::Extended(id) => {
                let bit_data = id.as_raw().view_bits::<Msb0>().to_bitvec();
                let priority = Priority::try_from(bit_data.load::<u8>());
                let pgn = Pgn::try_from(bit_data.load::<u32>());
                let source_address = Address::new(bit_data.load::<u8>());

                if priority.is_err() {
                    return Err(ParseIdError::Priority);
                }

                if pgn.is_err() {
                    return Err(ParseIdError::Pgn);
                }

                Ok(ExtendedId::new(
                    StandardId::new(priority.unwrap(), source_address),
                    pgn.unwrap(),
                ))
            }
        }
    }
}

impl TryFrom<u32> for ExtendedId {
    type Error = ParseIdError;

    fn try_from(raw_id: u32) -> Result<Self, Self::Error> {
        let bit_data = raw_id.view_bits::<Msb0>().to_bitvec();
        let priority = Priority::try_from(bit_data.load::<u8>());
        let pgn = Pgn::try_from(bit_data.load::<u32>());
        let source_address = Address::new(bit_data.load::<u8>());

        if priority.is_err() || pgn.is_err() {
            return Err(ParseIdError::Priority);
        }

        Ok(ExtendedId::new(
            StandardId::new(priority.unwrap(), source_address),
            pgn.unwrap(),
        ))
    }
}

impl From<ExtendedId> for Id {
    fn from(id: ExtendedId) -> Self {
        Id::Extended(id)
    }
}

//TODO: tests -> especially for 'bit_data.load::<u32>()'
