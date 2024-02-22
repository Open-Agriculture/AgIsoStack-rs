/*
Copyright 2023 Raven Industries inc.

@author Jannes Brands
@date 2024-02-22
*/

use crate::j1939::byte_field::ByteField;

/// J1939 address (8-bits) used to identify control applications on the network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(u8);

impl Address {
    /// The number of bits in the address
    pub const BIT_LENGTH: u8 = 8;

    /// Global Preferred Addresses
    ///
    /// only to be used by control applications that handles the given function and
    /// function instance, if applicable, that is assigned to that address by SAE J1939.
    ///
    /// For more information see SAE J1939 4.6.1
    pub const GLOBAL_PREFERRED_ADDRESSES: (std::ops::Range<u8>, std::ops::Range<u8>) =
        (0x00..0x7F, 0xF8..0xFD);

    /// Dynamic addresses
    ///
    /// any control application executing any system function can claim and use it.
    /// The supplier of a control application can employ any strategy
    /// to select the initial address within the range of 128 to 247.
    ///
    /// For more information see SAE J1939 4.6.2
    pub const DYNAMIC_ADDRESSES: std::ops::Range<u8> = 0x80..0xF7;

    /// Global Address
    ///
    /// The SAE J1939 source address 255 serves as the global destination address.
    /// This global destination address is exclusively utilized as the destination
    /// address in a D_PDU1 data frame to signify that the SAE J1939 data frame is
    /// intended for all Control Applications (CAs) on the network.
    ///
    /// For more information see SAE J1939 4.6.3
    pub const GLOBAL: Address = Self::BROADCAST;
    /// Alias for the [Address::GLOBAL]
    pub const BROADCAST: Address = Address(0xFF);

    /// Null Address
    ///
    /// The SAE J1939 source address 254 is designated as the Null address.
    /// This Null address is specifically employed as the source (transmitter) address
    /// within a D_PDU1 or D_PDU2 data frame.
    ///
    /// There are only two approved applications for the Null address:
    ///
    /// 1. The Null address can be utilized with an Address Claimed Parameter Group (PG)
    /// when a Control Application (CA) reports its inability to claim an SAE J1939 Address.
    ///
    /// 2. the Null address can be employed with a Request PG soliciting
    /// the Address Claimed PG when the Request PG is transmitted by a CA
    /// prior to claiming a source address.
    ///
    /// For more information see SAE J1939 4.6.4
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

#[cfg(test)]
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

        let address = Address::default();
        assert!(address.is_null());
    }
}
