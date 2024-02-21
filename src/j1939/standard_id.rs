use crate::j1939::byte_field::ByteField;
use crate::j1939::id::Id;
use crate::j1939::{Address, Priority};
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use embedded_can::{Id as EmbeddedId, StandardId as EmbeddedStandardId};

/// Standard 11-bit J1939 identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StandardId {
    priority: Priority,
    source_address: Address,
}

impl StandardId {
    pub const SOURCE_ADDRESS_START: usize = 0;
    pub const SOURCE_ADDRESS_END: usize = 8;
    pub const PRIORITY_START: usize = 8;
    pub const PRIORITY_END: usize = 11;

    /// Creates a new standard identifier out of a priority and source address
    pub fn new(priority: Priority, source_address: Address) -> Self {
        Self {
            priority,
            source_address,
        }
    }

    /// Get the priority of the identifier
    #[inline]
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Get the source address of the identifier
    #[inline]
    pub fn source_address(&self) -> Address {
        self.source_address
    }

    /// Get the raw value of the standard identifier
    pub fn raw(&self) -> u16 {
        let priority_bits: [bool; 3] = self.priority.into();
        let raw_source_address: [bool; 8] = self.source_address.raw_bits();
        let mut raw_id = BitVec::<u16, Msb0>::new();
        raw_id.extend(priority_bits);
        raw_id.extend(raw_source_address);
        raw_id.load()
    }

    /// Get the raw bits of the standard identifier
    pub fn raw_bits(&self) -> [bool; 11] {
        let priority_bits: [bool; 3] = self.priority.into();
        let raw_source_address: [bool; 8] = self.source_address.raw_bits();
        [
            priority_bits[0],
            priority_bits[1],
            priority_bits[2],
            raw_source_address[0],
            raw_source_address[1],
            raw_source_address[2],
            raw_source_address[3],
            raw_source_address[4],
            raw_source_address[5],
            raw_source_address[6],
            raw_source_address[7],
        ]
    }
}

impl From<StandardId> for Id {
    fn from(id: StandardId) -> Self {
        Id::Standard(id)
    }
}

impl From<StandardId> for EmbeddedId {
    fn from(id: StandardId) -> Self {
        EmbeddedId::Standard(EmbeddedStandardId::new(id.raw()).unwrap())
    }
}

impl From<StandardId> for [bool; 11] {
    fn from(id: StandardId) -> Self {
        id.raw_bits()
    }
}

#[cfg(test)]
mod tests {
    use crate::j1939::{Address, Priority, StandardId};

    #[test]
    fn test_raw() {
        let id = StandardId::new(Priority::Three, Address::new(0x0A));
        assert_eq!(id.raw(), 0x30A);

        let id = StandardId::new(Priority::Seven, Address::new(0x0F));
        assert_eq!(id.raw(), 0x70F);

        let id = StandardId::new(Priority::Zero, Address::new(0x00));
        assert_eq!(id.raw(), 0x000);
    }
}
