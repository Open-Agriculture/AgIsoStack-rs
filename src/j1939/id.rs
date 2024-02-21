use crate::j1939::standard_id::StandardId;
use crate::j1939::{Address, ExtendedId, Pgn, Priority};
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::view::BitView;
use embedded_can::Id as EmbeddedId;

#[derive(Debug)]
pub enum ParseIdError {
    Priority,
    Pgn,
    SourceAddress,
    StandardId,
    ExtendedId,
}

/// Identifier for a J1939 message (standard or extended)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    Standard(StandardId),
    Extended(ExtendedId),
}

impl Default for Id {
    fn default() -> Self {
        Id::Extended(ExtendedId::default())
    }
}

impl From<Id> for EmbeddedId {
    fn from(id: Id) -> Self {
        match id {
            Id::Standard(id) => id.into(),
            Id::Extended(id) => id.into(),
        }
    }
}

impl TryFrom<EmbeddedId> for Id {
    type Error = ParseIdError;

    fn try_from(value: EmbeddedId) -> Result<Self, Self::Error> {
        match value {
            EmbeddedId::Standard(id) => {
                let bit_data = id.as_raw().view_bits::<Msb0>().to_bitvec();
                let priority_bits =
                    bit_data[StandardId::PRIORITY_START..StandardId::PRIORITY_END].to_bitvec();
                let source_address_bits = bit_data
                    [StandardId::SOURCE_ADDRESS_START..StandardId::SOURCE_ADDRESS_END]
                    .to_bitvec();
                let priority =
                    Priority::from([priority_bits[0], priority_bits[1], priority_bits[2]]);
                let source_address = Address::new(source_address_bits.load());

                Ok(Id::Standard(StandardId::new(priority, source_address)))
            }
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

                Ok(Id::Extended(ExtendedId::new(
                    StandardId::new(priority.unwrap(), source_address),
                    pgn.unwrap(),
                )))
            }
        }
    }
}
