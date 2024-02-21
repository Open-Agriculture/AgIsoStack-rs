use crate::j1939::byte_field::ByteField;

/// PDU specific field defined in the PGN
#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, Hash)]
#[repr(transparent)]
pub struct PduSpecific(u8);

impl PduSpecific {
    pub fn new(raw_pdu_specific: u8) -> Self {
        Self(raw_pdu_specific)
    }
}

impl ByteField for PduSpecific {
    fn raw(self) -> u8 {
        self.0
    }
}
