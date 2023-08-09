// Copyright 2023 Raven Industries inc.
use crate::driver::{Address, Pgn};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Priority {
    /// You may also use [`Priority::Highest`] as an alias
    Zero = 0x0,
    One = 0x1,
    Two = 0x2,
    Three = 0x3,
    Four = 0x4,
    Five = 0x5,
    /// You may also use [`Priority::Default`] as an alias
    Six = 0x6,
    /// You may also use [`Priority::Lowest`] as an alias
    Seven = 0x7,
}

#[allow(non_upper_case_globals)]
impl Priority {
    pub const Highest: Priority = Priority::Zero;
    pub const Default: Priority = Priority::Six;
    pub const Lowest: Priority = Priority::Seven;
}

impl From<u8> for Priority {
    fn from(value: u8) -> Priority {
        match value {
            0x0 => Priority::Zero,
            0x1 => Priority::One,
            0x2 => Priority::Two,
            0x3 => Priority::Three,
            0x4 => Priority::Four,
            0x5 => Priority::Five,
            0x6 => Priority::Six,
            0x7 => Priority::Seven,
            _ => unreachable!(
                "Internal error converting a value larger than 3 bits to a CAN ID priority"
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    /// 11-bit CAN ID
    Standard = 0x0,
    /// 29-bit CAN ID
    Extended = 0x1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CanId(u32);

// Linux uses the top three unused bits to indicate whether the frame is standard/extended, remote,
// or an error frame. We do the same, because it's convenient.
const CAN_EFF_FLAG: u32 = 0x80000000;
// const CAN_RTR_FLAG: u32 = 0x40000000;
// const CAN_ERR_FLAG: u32 = 0x20000000;

const CAN_EFF_MASK: u32 = 0x1FFFFFFF;
const CAN_SFF_MASK: u32 = 0x000007FF;

impl CanId {
    pub fn new(raw: u32, type_: Type) -> Self {
        let raw = match type_ {
            Type::Extended => (raw & CAN_EFF_MASK) | CAN_EFF_FLAG,
            Type::Standard => raw & CAN_SFF_MASK,
        };
        Self(raw)
    }

    /// Encodes a new extended ID using the discrete parts of an identifier
    pub fn encode(
        parameter_group_number: Pgn,
        source_address: Address,
        destination_address: Address,
        priority: Priority,
    ) -> Result<CanId, &'static str> {
        let mut raw_id: u32 = 0;

        raw_id |= (priority as u32 & 0x07) << 26;
        raw_id |= source_address.0 as u32;

        if Address::GLOBAL == destination_address {
            if (parameter_group_number.raw() & 0xF000) >= 0xF000 {
                raw_id |= (parameter_group_number.raw() & 0x3FFFF) << 8;
            } else {
                raw_id |= (destination_address.0 as u32) << 8;
                raw_id |= (parameter_group_number.raw() & 0x3FF00) << 8;
            }
        } else if (parameter_group_number.raw() & 0xF000) < 0xF000 {
            raw_id |= (destination_address.0 as u32) << 8;
            raw_id |= (parameter_group_number.raw() & 0x3FF00) << 8;
        } else {
            return Err("Cannot encode destination specific message with broadcast PGN.");
        }
        Ok(CanId::new(raw_id & CAN_EFF_MASK, Type::Extended))
    }

    /// Get the raw value of the CAN ID
    #[inline]
    pub fn raw(&self) -> u32 {
        match self.type_() {
            Type::Extended => self.0 & CAN_EFF_MASK,
            Type::Standard => self.0 & CAN_SFF_MASK,
        }
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

    /// Get the ID's PGN
    ///
    /// In the case the the ID is a standard 11-bit ID, a NULL PGN will be returned.
    #[inline]
    pub fn pgn(&self) -> Pgn {
        match self.type_() {
            Type::Standard => Pgn::NULL,
            Type::Extended => Pgn::from_id(self.raw()),
        }
    }

    /// Get the destination address for this CAN ID, if it's a destination-specific PGN
    #[inline]
    pub fn destination_address(&self) -> Address {
        let pgn = self.pgn();
        if pgn == Pgn::NULL || pgn.is_broadcast() {
            return Address::GLOBAL;
        }

        let raw_pdu_s = ((self.raw() & 0xFF00) >> 8) as u8;
        Address(raw_pdu_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        let can_id = CanId::new(0x18EF1CF5, Type::Extended);
        assert_eq!(can_id.priority(), Priority::Default);
    }

    #[test]
    fn test_source_address() {
        let can_id = CanId::new(0x0705, Type::Standard);
        assert_eq!(can_id.type_(), Type::Standard);
        // TODO: Is this right? Do 11-bit IDs always have a global address?
        assert_eq!(can_id.source_address(), Address::GLOBAL);

        let can_id = CanId::new(0x18EF1CF5, Type::Extended);
        assert_eq!(can_id.source_address(), Address(0xF5));
    }

    #[test]
    fn test_destination_address() {
        let can_id = CanId::new(0x0705, Type::Standard);
        assert_eq!(can_id.destination_address(), Address::GLOBAL);

        let can_id = CanId::new(0x18EEFF1C, Type::Extended);
        assert_eq!(can_id.destination_address(), Address::GLOBAL);

        let can_id = CanId::new(0x09F8031C, Type::Extended);
        assert_eq!(can_id.destination_address(), Address::GLOBAL);

        let can_id = CanId::new(0x0CAC1C13, Type::Extended);
        assert_eq!(can_id.destination_address(), Address(0x1C));
    }

    #[test]
    fn test_pgn() {
        let can_id = CanId::new(0x07FF, Type::Standard);
        assert_eq!(can_id.pgn(), Pgn::NULL);

        let can_id = CanId::new(0x0CAC1C13, Type::Extended);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0AC00));

        let can_id = CanId::new(0x18FF3F13, Type::Extended);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0FF3F));

        let can_id = CanId::new(0x18EF1CF5, Type::Extended);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0EF00));

        let can_id = CanId::new(0x18EEFF1C, Type::Extended);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0EE00));
    }

    #[test]
    fn test_encode() {
        let encode_result = CanId::encode(
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

        let encode_result = CanId::encode(
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

        let encode_result = CanId::encode(
            Pgn::from_raw(0x00FF40),
            Address(0x81),
            Address(0x0F),
            Priority::Six,
        );
        assert!(matches!(encode_result, Err(_)));
    }
}
