// Copyright 2023 Raven Industries inc.

use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
use std::io::Read;

pub enum ParsePgnError {
    InvalidPgnLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Pgn {
    reserved: bool,
    data_page: bool,
    pdu_format: u8,
    pdu_specific: u8,
}

impl Pgn {
    pub const MAX_PGN_VALUE: u32 = 0x3FFFF;

    pub fn new(reserved: bool, data_page: bool, pdu_format: u8, pdu_specific: u8) -> Self {
        Self {
            reserved,
            data_page,
            pdu_format,
            pdu_specific,
        }
    }
}

impl TryFrom<u32> for Pgn {
    type Error = ParsePgnError;

    fn try_from(raw_pgn: u32) -> Result<Self, Self::Error> {
        if (raw_pgn > Self::MAX_PGN_VALUE) {
            // raw value is too large to fit in PGN with 18 bits
            Err(ParsePgnError::InvalidPgnLength)
        }

        let mut bit_data = raw_pgn.view_bits::<Msb0>().to_bitvec();
        Ok(Self {
            reserved: bit_data.pop().unwrap(),
            data_page: bit_data.pop().unwrap(),
            pdu_format: bit_data.load::<u8>().unwrap(),
            pdu_specific: bit_data.load::<u8>().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::j1939::Pgn;

    #[test]
    fn test_from_raw() {
        let pgn_parsed = Pgn::from_raw(0x00F04);
        assert_eq!(pgn_parsed.is_ok(), true);

        let pgn = Pgn::new(false, false, 0xF0, 0x04);
        assert_eq!(pgn, pgn_parsed.unwrap());
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
