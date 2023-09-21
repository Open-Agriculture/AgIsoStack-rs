// Copyright 2023 Raven Industries inc.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Pgn(u32);

impl Pgn {
    /// A fake PGN used to denote a PGN that does not exist
    pub const NULL: Pgn = Pgn(0xFFFFFFFF);

    pub fn from_id(can_id: u32) -> Self {
        const PDU2_FORMAT_MASK: u32 = 0x00F00000;
        let raw_pgn = if (can_id & PDU2_FORMAT_MASK) < PDU2_FORMAT_MASK {
            // point-to-point
            (can_id >> 8) & 0x03FF00
        } else {
            // broadcast
            (can_id >> 8) & 0x03FFFF
        };
        Pgn(raw_pgn)
    }

    pub fn from_raw(pgn: u32) -> Self {
        Pgn(pgn)
    }

    #[inline]
    pub fn is_broadcast(&self) -> bool {
        !self.is_destination_specific()
    }

    #[inline]
    pub fn is_destination_specific(&self) -> bool {
        // PDU1 / destination specific PGNs have a PDU Format 0x00 - 0xEF
        // PDU2 / broadcast PGNs have a PDU Format 0xF0 - 0xFF
        self.pdu_format() <= 0xEF
    }

    #[inline]
    pub fn is_proprietary(&self) -> bool {
        self.pdu_format() == 0xEF
    }

    #[inline]
    pub fn raw(&self) -> u32 {
        self.0
    }

    #[inline]
    pub fn pdu_specific(&self) -> u8 {
        (self.raw() & 0x00FF) as u8
    }

    #[inline]
    pub fn pdu_format(&self) -> u8 {
        ((self.raw() & 0xFF00) >> 8) as u8
    }

    #[inline]
    pub fn data_page(&self) -> u8 {
        ((self.raw() & 0x10000) >> 16) as u8
    }

    #[inline]
    pub fn extended_data_page(&self) -> u8 {
        ((self.raw() & 0x20000) >> 17) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_id() {
        let pgn = Pgn::from_id(0x18EF1CF5);
        let expected = Pgn::from_raw(0x0EF00);
        assert_eq!(pgn, expected);

        let pgn = Pgn::from_id(0x18FF3F13);
        let expected = Pgn::from_raw(0x0FF3F);
        assert_eq!(pgn, expected);
    }

    #[test]
    fn test_bitmath() {
        let pgn = Pgn::from_raw(0x30000);
        assert_eq!(pgn.data_page(), 0x01);
        assert_eq!(pgn.extended_data_page(), 0x01);

        let pgn = Pgn::from_raw(0x0FF00);
        assert_eq!(pgn.pdu_format(), 0xFF);
        assert_eq!(pgn.pdu_specific(), 0x00);

        let pgn = Pgn::from_raw(0x000FF);
        assert_eq!(pgn.pdu_format(), 0x00);
        assert_eq!(pgn.pdu_specific(), 0xFF);
    }

    #[test]
    fn test_p2p() {
        let pgn = Pgn::from_raw(0x0EE00);
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::from_raw(0x0EF00);
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::from_raw(0x0F000);
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::from_raw(0x0FEFF);
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::from_raw(0x0FF00);
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::from_raw(0x0FFFF);
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::from_raw(0x10000);
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::from_raw(0x1EE00);
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::from_raw(0x1EF00);
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::from_raw(0x1F000);
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::from_raw(0x1FEFF);
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::from_raw(0x1FF00);
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::from_raw(0x1FFFF);
        assert_eq!(pgn.is_destination_specific(), false);
    }
}
