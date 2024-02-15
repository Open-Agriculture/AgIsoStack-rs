// Copyright 2023 Raven Industries inc.
use crate::j1939::page::Page;
use crate::j1939::Address;
use bitvec::field::BitField;
use bitvec::order::{Lsb0, Msb0};
use bitvec::vec::BitVec;
use bitvec::view::BitView;

#[derive(Debug)]
pub enum ParsePgnError {
    InvalidPgnLength(u32),
}

impl std::fmt::Display for ParsePgnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePgnError::InvalidPgnLength(value) => write!(
                f,
                "Parse '{:?}' failed because the permitted PGN value is between 0 and 0x3FFFF!",
                value
            ),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pgn {
    extended_data_page: bool,
    data_page: bool,
    pdu_format: u8,
    pdu_specific: u8,
}

impl Pgn {
    pub const PGN_LENGTH: usize = 18;
    pub const MAX_PGN_VALUE: u32 = 0x3FFFF;
    pub const PDU_1: u8 = 0xEF;
    pub const PDU_2: u8 = 0xFF;
    pub const EDP_START: u8 = 0;
    pub const DP_START: u8 = 1;
    pub const PDU_FORMAT_START: usize = 2;
    pub const PDU_FORMAT_END: usize = 10;
    pub const PDU_SPECIFIC_START: usize = 10;
    pub const PDU_SPECIFIC_END: usize = 18;

    pub fn new(
        extended_data_page: bool,
        data_page: bool,
        pdu_format: u8,
        pdu_specific: u8,
    ) -> Self {
        Self {
            extended_data_page,
            data_page,
            pdu_format,
            pdu_specific,
        }
    }

    pub fn raw(self) -> BitVec<u8> {
        let mut raw_pgn: BitVec<u8> = BitVec::new();
        raw_pgn.append(
            &mut (self.extended_data_page as u8)
                .view_bits_mut::<Msb0>()
                .to_bitvec(),
        );
        raw_pgn.append(&mut (self.data_page as u8).view_bits_mut::<Lsb0>().to_bitvec());
        raw_pgn.extend(self.pdu_format.view_bits::<Lsb0>());
        raw_pgn.extend(self.pdu_specific.view_bits::<Lsb0>());
        raw_pgn
    }

    pub fn is_destination_specific(&self) -> bool {
        self.pdu_format <= Self::PDU_1
    }

    pub fn get_destination_address(&self) -> Option<Address> {
        if self.is_destination_specific() {
            Some(Address::new(self.pdu_specific))
        } else {
            None
        }
    }

    pub fn set_destination_address(&mut self, address: Address) {
        if self.is_destination_specific() {
            self.pdu_specific = address.raw();
        }
    }

    pub fn is_group_extension(&self) -> bool {
        self.pdu_specific > Self::PDU_1
    }

    pub fn get_group_extension(&self) -> Option<u8> {
        if self.is_group_extension() {
            Some(self.pdu_specific)
        } else {
            None
        }
    }

    #[inline]
    pub fn extended_data_page(&self) -> bool {
        self.extended_data_page
    }

    #[inline]
    pub fn data_page(&self) -> bool {
        self.data_page
    }

    pub fn page(&self) -> Page {
        Page::from([self.extended_data_page, self.data_page])
    }

    #[inline]
    pub fn pdu_format(&self) -> u8 {
        self.pdu_format
    }

    #[inline]
    pub fn pdu_specific(&self) -> u8 {
        self.pdu_specific
    }
}

impl TryFrom<u32> for Pgn {
    type Error = ParsePgnError;

    fn try_from(raw_pgn: u32) -> Result<Self, Self::Error> {
        if raw_pgn > Self::MAX_PGN_VALUE {
            // raw value is too large to fit in PGN with 18 bits
            return Err(ParsePgnError::InvalidPgnLength(raw_pgn));
        }

        let raw_pgn_be_bytes = raw_pgn.to_le_bytes();
        let mut bit_data = raw_pgn_be_bytes.view_bits::<Lsb0>().to_bitvec();
        bit_data.truncate(Pgn::PGN_LENGTH);
        bit_data.reverse();

        let mut pdu_format = bit_data[Self::PDU_FORMAT_START..Self::PDU_FORMAT_END].to_bitvec();
        pdu_format.reverse();

        let mut pdu_specific =
            bit_data[Self::PDU_SPECIFIC_START..Self::PDU_SPECIFIC_END].to_bitvec();
        pdu_specific.reverse();

        let pgn = Self {
            extended_data_page: bit_data[Self::EDP_START as usize],
            data_page: bit_data[Self::DP_START as usize],
            pdu_format: pdu_format.load(),
            pdu_specific: pdu_specific.load(),
        };

        Ok(pgn)
    }
}

#[cfg(test)]
mod tests {
    use crate::j1939::Pgn;

    #[test]
    fn test_try_from() {
        let pgn_parsed = Pgn::try_from(0x2E6BA).expect("Failed to parse PGN");

        let pgn = Pgn::new(true, false, 0xE6, 0xBA);
        assert_eq!(pgn, pgn_parsed);
    }

    #[test]
    fn test_bitmath() {
        let pgn = Pgn::try_from(0x30000).unwrap();
        assert_eq!(pgn.data_page, true);
        assert_eq!(pgn.extended_data_page, true);

        let pgn = Pgn::try_from(0x0FF00).unwrap();
        assert_eq!(pgn.pdu_format, 0xFF);
        assert_eq!(pgn.pdu_specific, 0x00);

        let pgn = Pgn::try_from(0x000FF).unwrap();
        assert_eq!(pgn.pdu_format, 0x00);
        assert_eq!(pgn.pdu_specific, 0xFF);
    }

    #[test]
    fn test_p2p() {
        let pgn = Pgn::try_from(0x0EE00).unwrap();
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::try_from(0x0EF00).unwrap();
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::try_from(0x0F000).unwrap();
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::try_from(0x0FEFF).unwrap();
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::try_from(0x0FF00).unwrap();
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::try_from(0x0FFFF).unwrap();
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::try_from(0x10000).unwrap();
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::try_from(0x1EE00).unwrap();
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::try_from(0x1EF00).unwrap();
        assert_eq!(pgn.is_destination_specific(), true);
        let pgn = Pgn::try_from(0x1F000).unwrap();
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::try_from(0x1FEFF).unwrap();
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::try_from(0x1FF00).unwrap();
        assert_eq!(pgn.is_destination_specific(), false);
        let pgn = Pgn::try_from(0x1FFFF).unwrap();
        assert_eq!(pgn.is_destination_specific(), false);
    }
}
