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

impl CanId {
    pub fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Get the raw value of the CAN ID
    #[inline]
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Get the type of the ID (standard or extended)
    #[inline]
    pub fn type_(&self) -> Type {
        if self.raw() <= 0x7FF {
            Type::Standard
        } else {
            Type::Extended
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
    fn test_standard_extended() {
        let can_id = CanId::new(0x07FF);
        assert_eq!(can_id.type_(), Type::Standard);

        let can_id = CanId::new(0x08FF);
        assert_eq!(can_id.type_(), Type::Extended);
    }

    #[test]
    fn test_priority() {
        let can_id = CanId::new(0x18EF1CF5);
        assert_eq!(can_id.priority(), Priority::Default);
    }

    #[test]
    fn test_source_address() {
        let can_id = CanId::new(0x0705);
        assert_eq!(can_id.type_(), Type::Standard);
        // TODO: Is this right? Do 11-bit IDs always have a global address?
        assert_eq!(can_id.source_address(), Address::GLOBAL);

        let can_id = CanId::new(0x18EF1CF5);
        assert_eq!(can_id.source_address(), Address(0xF5));
    }

    #[test]
    fn test_destination_address() {
        let can_id = CanId::new(0x0705);
        assert_eq!(can_id.destination_address(), Address::GLOBAL);

        let can_id = CanId::new(0x18EEFF1C);
        assert_eq!(can_id.destination_address(), Address::GLOBAL);

        let can_id = CanId::new(0x09F8031C);
        assert_eq!(can_id.destination_address(), Address::GLOBAL);

        let can_id = CanId::new(0x0CAC1C13);
        assert_eq!(can_id.destination_address(), Address(0x1C));
    }

    #[test]
    fn test_pgn() {
        let can_id = CanId::new(0x07FF);
        assert_eq!(can_id.pgn(), Pgn::NULL);

        let can_id = CanId::new(0x0CAC1C13);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0AC00));

        let can_id = CanId::new(0x18FF3F13);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0FF3F));

        let can_id = CanId::new(0x18EF1CF5);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0EF00));

        let can_id = CanId::new(0x18EEFF1C);
        assert_eq!(can_id.pgn(), Pgn::from_raw(0x0EE00));
    }
}
