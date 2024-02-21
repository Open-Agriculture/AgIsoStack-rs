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
    /// The number of bits in the extended identifier
    pub const BIT_LENGTH: u8 = 29;
    const PRIORITY_START: usize = 0;
    const PRIORITY_END: usize = 3;
    const PGN_START: usize = 3;
    const PGN_END: usize = 21;
    const SOURCE_ADDRESS_START: usize = 21;
    const SOURCE_ADDRESS_END: usize = 29;

    pub fn new(standard_id: StandardId, pgn: Pgn) -> Self {
        Self { standard_id, pgn }
    }

    /// Raw value of the extended identifier
    pub fn raw(&self) -> u32 {
        let mut raw_id: BitVec<u32> = BitVec::new();
        raw_id.extend(self.standard_id.priority().raw_bits());
        raw_id.extend(self.pgn.raw_bits());
        raw_id.extend(self.standard_id.source_address().raw_bits());
        raw_id.reverse();
        raw_id.load::<u32>()
    }

    /// Raw bits of the extended identifier
    pub fn raw_bits(&self) -> [bool; 29] {
        let mut raw_id: BitVec<u32> = BitVec::new();
        raw_id.extend(self.standard_id.priority().raw_bits());
        raw_id.extend(self.pgn.raw_bits());
        raw_id.extend(self.standard_id.source_address().raw_bits());
        [
            raw_id[0], raw_id[1], raw_id[2], raw_id[3], raw_id[4], raw_id[5], raw_id[6], raw_id[7],
            raw_id[8], raw_id[9], raw_id[10], raw_id[11], raw_id[12], raw_id[13], raw_id[14],
            raw_id[15], raw_id[16], raw_id[17], raw_id[18], raw_id[19], raw_id[20], raw_id[21],
            raw_id[22], raw_id[23], raw_id[24], raw_id[25], raw_id[26], raw_id[27], raw_id[28],
        ]
    }

    /// PGN of the identifier
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
        let mut priority_bits =
            bit_data[ExtendedId::PRIORITY_START..ExtendedId::PRIORITY_END].to_bitvec();
        let mut pgn_bits = bit_data[ExtendedId::PGN_START..ExtendedId::PGN_END].to_bitvec();
        let mut source_address_bits =
            bit_data[ExtendedId::SOURCE_ADDRESS_START..ExtendedId::SOURCE_ADDRESS_END].to_bitvec();

        priority_bits.reverse();
        pgn_bits.reverse();
        source_address_bits.reverse();

        let priority = Priority::try_from(priority_bits.load::<u8>());
        let pgn = Pgn::try_from(pgn_bits.load::<u32>());
        let source_address = Address::new(source_address_bits.load::<u8>());

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

        let id = ExtendedId::new(
            StandardId::new(Priority::Seven, Address::new(0xAF)),
            Pgn::new(false, true, PduFormat::new(0x8A), PduSpecific::new(0x2F)),
        );
        assert_eq!(id.raw(), 0x1D8A2FAF);

        let id = ExtendedId::new(
            StandardId::new(Priority::Zero, Address::new(0x00)),
            Pgn::new(true, false, PduFormat::new(0x4C), PduSpecific::new(0x12)),
        );
        assert_eq!(id.raw(), 0x24C1200);
    }

    #[test]
    fn test_raw_bits() {
        let id = ExtendedId::new(
            StandardId::new(Priority::Zero, Address::new(0x25)),
            Pgn::new(false, true, PduFormat::new(0x8A), PduSpecific::new(0x0F)),
        );
        assert_eq!(
            id.raw_bits(),
            [
                false, false, false, false, true, true, false, false, false, true, false, true,
                false, false, false, false, false, true, true, true, true, false, false, true,
                false, false, true, false, true
            ]
        );

        let id = ExtendedId::new(
            StandardId::new(Priority::Seven, Address::new(0xAF)),
            Pgn::new(false, true, PduFormat::new(0x8A), PduSpecific::new(0x2F)),
        );

        assert_eq!(
            id.raw_bits(),
            [
                true, true, true, false, true, true, false, false, false, true, false, true, false,
                false, false, true, false, true, true, true, true, true, false, true, false, true,
                true, true, true
            ]
        );
    }

    #[test]
    fn test_from_extended_id_for_embedded_id() {
        let id = ExtendedId::new(
            StandardId::new(Priority::Zero, Address::new(0x25)),
            Pgn::new(false, true, PduFormat::new(0x8A), PduSpecific::new(0x0F)),
        );
        let embedded_id: embedded_can::Id = id.into();
        assert_eq!(
            embedded_id,
            embedded_can::Id::Extended(embedded_can::ExtendedId::new(0x18A0F25).unwrap())
        );
    }

    // not finished yet TODO!
    //#[test]
    fn test_try_from_u32_for_extended_id() {
        let id = ExtendedId::try_from(0x18A0F25).unwrap();
        assert_eq!(
            id,
            ExtendedId::new(
                StandardId::new(Priority::Zero, Address::new(0x25)),
                Pgn::new(false, true, PduFormat::new(0x8A), PduSpecific::new(0x0F)),
            )
        );

        let id = ExtendedId::try_from(0x1D8A2FAF).unwrap();
        assert_eq!(
            id,
            ExtendedId::new(
                StandardId::new(Priority::Seven, Address::new(0xAF)),
                Pgn::new(false, true, PduFormat::new(0x8A), PduSpecific::new(0x2F)),
            )
        );

        let id = ExtendedId::try_from(0x24C1200).unwrap();
        assert_eq!(
            id,
            ExtendedId::new(
                StandardId::new(Priority::Zero, Address::new(0x00)),
                Pgn::new(true, false, PduFormat::new(0x4C), PduSpecific::new(0x12)),
            )
        );
    }
}
