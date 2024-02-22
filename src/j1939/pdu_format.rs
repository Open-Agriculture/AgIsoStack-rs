/*
Copyright 2023 Raven Industries inc.

@author Jannes Brands
@date 2024-02-22
*/
use crate::j1939::byte_field::ByteField;

/// PDU format field defined in the PGN
#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, Hash)]
#[repr(transparent)]
pub struct PduFormat(u8);

impl PduFormat {
    pub const PDU_1_START: u8 = 0x00;
    pub const PDU_1_END: u8 = 0xEF;
    pub const PDU_2_START: u8 = 0xF0;
    pub const PDU_2_END: u8 = 0xFF;

    pub fn new(raw_pdu_format: u8) -> Self {
        Self(raw_pdu_format)
    }

    #[inline]
    pub fn is_destination_specific(&self) -> bool {
        self.0 <= Self::PDU_1_END
    }

    #[inline]
    pub fn is_group_extension(&self) -> bool {
        self.0 >= Self::PDU_2_START
    }
}

impl ByteField for PduFormat {
    fn raw(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::j1939::pdu_format::PduFormat;

    #[test]
    fn test_pdu_format() {
        let pdu_format = PduFormat::new(0x00);
        assert_eq!(pdu_format.is_destination_specific(), true);
        assert_eq!(pdu_format.is_group_extension(), false);

        let pdu_format = PduFormat::new(0xEF);
        assert_eq!(pdu_format.is_destination_specific(), true);
        assert_eq!(pdu_format.is_group_extension(), false);

        let pdu_format = PduFormat::new(0xF0);
        assert_eq!(pdu_format.is_destination_specific(), false);
        assert_eq!(pdu_format.is_group_extension(), true);

        let pdu_format = PduFormat::new(0xFF);
        assert_eq!(pdu_format.is_destination_specific(), false);
    }
}
