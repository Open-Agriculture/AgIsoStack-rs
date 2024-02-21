// Copyright 2023 Raven Industries inc.
use crate::j1939::byte_field::ByteField;
use crate::j1939::id::{Id, ParseIdError};
use crate::j1939::priority::Priority;
use crate::j1939::standard_id::StandardId;
use crate::j1939::{Address, Pgn};
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
use embedded_can::{ExtendedId as EmbeddedExtendedId, Id as EmbeddedId};

/// Extended 29-bit J1939 identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ExtendedId {
    standard_id: StandardId,
    pgn: Pgn,
}

impl ExtendedId {
    pub const BIT_LENGTH: u8 = 29;

    pub fn new(standard_id: StandardId, pgn: Pgn) -> Self {
        Self { standard_id, pgn }
    }

    /// Get the raw value of the extended identifier
    pub fn raw(&self) -> u32 {
        let mut raw_id: BitVec<u32> = BitVec::new();
        raw_id.extend(self.standard_id.priority().raw_bits());
        raw_id.extend(self.pgn.raw_bits());
        raw_id.extend(self.standard_id.source_address().raw_bits());
        raw_id.reverse();
        raw_id.load::<u32>()
    }

    /// Get the PGN of the identifier
    #[inline]
    pub fn pgn(&self) -> Pgn {
        self.pgn
    }
}

impl From<ExtendedId> for EmbeddedId {
    fn from(id: ExtendedId) -> Self {
        EmbeddedId::Extended(EmbeddedExtendedId::new(id.raw()).unwrap())
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

#[cfg(test)]
mod tests {
    use crate::j1939::{Address, ExtendedId, PduFormat, PduSpecific, Pgn, Priority, StandardId};

    #[test]
    fn test_raw() {
        let id = ExtendedId::new(
            StandardId::new(Priority::Zero, Address::new(0x25)),
            Pgn::new(false, true, PduFormat::new(0x8A), PduSpecific::new(0x0F)),
        );
        assert_eq!(id.raw(), 0x18A0F25);
    }
}
