use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
// Copyright 2023 Raven Industries inc.
use crate::j1939::priority::Priority;
use crate::j1939::{Address, Pgn};

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

    pub fn try_from_raw(raw_id: u32) -> Result<Self, ParseIdError> {
        let bit_data = raw_id.view_bits::<Msb0>().to_bitvec();
        let priority_parse = Priority::try_from(bit_data.load::<u8>());
        let pgn_parse = Pgn::try_from();
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

    /// Get the type of the ID (standard or extended)
    #[inline]
    pub fn type_(&self) -> Type {
        if self.0 & CAN_EFF_FLAG != 0 {
            Type::Extended
        } else {
            Type::Standard
        }
    }

    /// Get the priority of the ID
    #[inline]
    pub fn priority(&self) -> Priority {
        match self.type_() {
            Type::Standard => Priority::Highest,
            Type::Extended => {
                let raw = ((self.raw() & 0x1C000000) >> 26) as u8;
                raw.into()
            }
        }
    }

    /// Get the source address of the ID
    #[inline]
    pub fn source_address(&self) -> Address {
        match self.type_() {
            Type::Standard => Address::GLOBAL,
            Type::Extended => Address((self.raw() & 0xFF) as u8),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        let can_id = Id::new(0x18EF1CF5, Type::Extended);
        assert_eq!(can_id.priority(), Priority::Default);
    }

    #[test]
    fn test_source_address() {
        let can_id = Id::new(0x0705, Type::Standard);
        assert_eq!(can_id.type_(), Type::Standard);
        // TODO: Is this right? Do 11-bit IDs always have a global address?
        assert_eq!(can_id.source_address(), Address::GLOBAL);

        let can_id = Id::new(0x18EF1CF5, Type::Extended);
        assert_eq!(can_id.source_address(), Address(0xF5));
    }

    #[test]
    fn test_destination_address() {
        let can_id = Id::new(0x0705, Type::Standard);
        assert_eq!(can_id.destination_address(), Address::GLOBAL);

        let can_id = Id::new(0x18EEFF1C, Type::Extended);
        assert_eq!(can_id.destination_address(), Address::GLOBAL);

        let can_id = Id::new(0x09F8031C, Type::Extended);
        assert_eq!(can_id.destination_address(), Address::GLOBAL);

        let can_id = Id::new(0x0CAC1C13, Type::Extended);
        assert_eq!(can_id.destination_address(), Address(0x1C));
    }

    #[test]
    fn test_pgn() {
        let can_id = Id::new(0x07FF, Type::Standard);
        assert_eq!(can_id.pgn(), Pgn::NULL);

        let can_id = Id::new(0x0CAC1C13, Type::Extended);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0AC00));

        let can_id = Id::new(0x18FF3F13, Type::Extended);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0FF3F));

        let can_id = Id::new(0x18EF1CF5, Type::Extended);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0EF00));

        let can_id = Id::new(0x18EEFF1C, Type::Extended);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0EE00));
    }

    #[test]
    fn test_encode() {
        let encode_result = Id::try_encode(
            Pgn::from_raw(0x00EF00),
            Address(0x81),
            Address(0xF9),
            Priority::Six,
        );
        let can_id = encode_result.expect("EF00 Message was not encodable");
        assert_eq!(can_id.pgn(), Pgn::from_raw(0xEF00));
        assert_eq!(can_id.destination_address(), Address(0xF9));
        assert_eq!(can_id.source_address(), Address(0x81));
        assert_eq!(can_id.priority(), Priority::Six);

        let encode_result = Id::try_encode(
            Pgn::from_raw(0x00FF40),
            Address(0x81),
            Address(0xFF),
            Priority::Six,
        );
        let can_id = encode_result.expect("FF40 Message was not encodable");
        assert_eq!(can_id.pgn(), Pgn::from_raw(0xFF40));
        assert_eq!(can_id.destination_address(), Address(0xFF));
        assert_eq!(can_id.source_address(), Address(0x81));
        assert_eq!(can_id.priority(), Priority::Six);

        let encode_result = Id::try_encode(
            Pgn::from_raw(0x00FF40),
            Address(0x81),
            Address(0x0F),
            Priority::Six,
        );
        assert!(encode_result.is_err());

        let error_contents: EncodingError = encode_result.unwrap_err();
        assert_eq!(error_contents.priority, Priority::Six);
        assert_eq!(error_contents.source_address, Address(0x81));
        assert_eq!(error_contents.destination_address, Address(0x0F));
        assert_eq!(error_contents.parameter_group_number, Pgn::from_raw(0xFF40));
    }
}
