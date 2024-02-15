use crate::j1939::id::Id;
use crate::j1939::id::ParseIdError;
use crate::j1939::{Address, Priority};
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::view::BitView;
use embedded_can::Id as EmbeddedId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StandardId {
    priority: Priority,
    source_address: Address,
}

impl StandardId {
    pub fn new(priority: Priority, source_address: Address) -> Self {
        Self {
            priority,
            source_address,
        }
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
}

impl TryFrom<EmbeddedId> for StandardId {
    type Error = ParseIdError;

    fn try_from(id: EmbeddedId) -> Result<Self, Self::Error> {
        match id {
            EmbeddedId::Standard(id) => {
                let bit_data = id.as_raw().view_bits::<Msb0>().to_bitvec();
                let priority = Priority::try_from(bit_data.load::<u8>());
                let source_address = Address::new(bit_data.load::<u8>());

                if priority.is_err() {
                    return Err(ParseIdError::Priority);
                }

                Ok(Self::new(priority.unwrap(), source_address))
            }
            EmbeddedId::Extended(_) => Err(ParseIdError::ExtendedId),
        }
    }
}

impl From<StandardId> for Id {
    fn from(id: StandardId) -> Self {
        Id::Standard(id)
    }
}
