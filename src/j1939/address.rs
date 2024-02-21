// Copyright 2023 Raven Industries inc.

use crate::j1939::byte_field::ByteField;

/// J1939 address (8-bits) used to identify ECUs on the network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(u8);

impl Address {
    /// The number of bits in the address
    pub const BIT_LENGTH: u8 = 8;
    /// Address representing broadcasts for destination specific PGNs
    pub const GLOBAL: Address = Self::BROADCAST;
    /// Alias for the global address
    pub const BROADCAST: Address = Address(0xFF);
    /// The null address is used by ECUs without an address such as during address claiming
    pub const NULL: Address = Address(0xFE);

    /// Create a new address
    pub fn new(raw_address: u8) -> Self {
        Self(raw_address)
    }

    /// Returns if the address is the [Address::GLOBAL] (same as [Address::is_broadcast])
    #[inline]
    pub fn is_global(&self) -> bool {
        self.is_broadcast()
    }

    /// Returns if the address is the [Address::BROADCAST] (same as [Address::is_global])
    #[inline]
    pub fn is_broadcast(self) -> bool {
        self == Self::BROADCAST
    }

    /// Returns if the address is the [Address::NULL]
    #[inline]
    pub fn is_null(self) -> bool {
        self == Self::NULL
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::NULL
    }
}

impl ByteField for Address {
    fn raw(self) -> u8 {
        self.0
    }
}

mod tests {
    use crate::j1939::Address;

    #[test]
    fn test_address() {
        let address = Address::new(0b1010_1010);
        assert!(!address.is_global());
        assert!(!address.is_broadcast());
        assert!(!address.is_null());

        let address = Address::BROADCAST;
        assert!(address.is_global());

        let address = Address::NULL;
        assert!(address.is_null());
    }
}
