// Copyright 2023 Raven Industries inc.
use crate::j1939::byte_field::ByteField;
use crate::j1939::page::Page;
use crate::j1939::pdu_format::PduFormat;
use crate::j1939::pdu_specific::PduSpecific;
use crate::j1939::Address;
use bitvec::field::BitField;
use bitvec::order::Lsb0;
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

/// Parameter Group Number (PGN)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pgn {
    extended_data_page: bool,
    data_page: bool,
    pdu_format: PduFormat,
    pdu_specific: PduSpecific,
}

impl Pgn {
    /// The number of bits used to represent the PGN
    pub const BIT_LENGTH: usize = 18;
    /// The maximum value of the PGN
    pub const MAX_VALUE: u32 = 0x3FFFF;
    const EDP_START: u8 = 0;
    const DP_START: u8 = 1;
    const PDU_FORMAT_START: usize = 2;
    const PDU_FORMAT_END: usize = 10;
    const PDU_SPECIFIC_START: usize = 10;
    pub const PDU_SPECIFIC_END: usize = 18;
    /// PDU 1 proprietary A parameter group
    /// format: destination specific (not global)
    /// data length: 0 - 1785 bytes (multi-packet support)
    /// default priority: [`Priority::DEFAULT`](`crate::j1939::priority::Priority::DEFAULT)
    pub const PDU_1_FORMAT_PROPRIETARY_A: u32 = 0xEF00;
    /// PDU 1 proprietary A2 parameter group
    /// format: destination specific (not global)
    /// data length: 0 - 1785 bytes (multi-packet support)
    /// default priority: [`DEFAULT`](`crate::j1939::priority::Priority::DEFAULT)
    pub const PDU_1_FORMAT_PROPRIETARY_A2: u32 = 0x1EF00;
    /// PDU 2 proprietary B parameter group
    /// format: group extension
    /// data length: 0 - 1785 bytes (multi-packet support)
    /// default priority: [`Priority::DEFAULT`](`crate::j1939::priority::Priority::DEFAULT)
    pub const PDU_2_FORMAT_PROPRIETARY_B: (std::ops::Range<u32>, std::ops::Range<u32>) =
        (0x00FF00..0x00FFFF, 0x01FF00..0x01FFFF);

    /// Create a new [Pgn] instance
    pub fn new(
        extended_data_page: bool,
        data_page: bool,
        pdu_format: PduFormat,
        pdu_specific: PduSpecific,
    ) -> Self {
        Self {
            extended_data_page,
            data_page,
            pdu_format,
            pdu_specific,
        }
    }

    /// Get the raw bits of the [Pgn]
    pub fn raw_bits(self) -> [bool; 18] {
        let mut raw_pgn: BitVec<u8> = BitVec::new();
        raw_pgn.push(self.extended_data_page);
        raw_pgn.push(self.data_page);
        raw_pgn.extend(self.pdu_format.raw_bits());
        raw_pgn.extend(self.pdu_specific.raw_bits());
        [
            raw_pgn[0],
            raw_pgn[1],
            raw_pgn[2],
            raw_pgn[3],
            raw_pgn[4],
            raw_pgn[5],
            raw_pgn[6],
            raw_pgn[7],
            raw_pgn[8],
            raw_pgn[9],
            raw_pgn[10],
            raw_pgn[11],
            raw_pgn[12],
            raw_pgn[13],
            raw_pgn[14],
            raw_pgn[15],
            raw_pgn[16],
            raw_pgn[17],
        ]
    }

    /// Get the raw value of the [Pgn] as a 32-bit integer
    pub fn raw(self) -> u32 {
        let mut raw_pgn: BitVec<u32> = BitVec::new();
        raw_pgn.extend(self.raw_bits());
        raw_pgn.reverse();
        raw_pgn.load()
    }

    /// Get the destination address of the [Pgn] if it is destination specific
    pub fn get_destination_address(&self) -> Option<Address> {
        if self.pdu_format().is_destination_specific() {
            Some(Address::new(self.pdu_specific.raw()))
        } else {
            None
        }
    }

    /// Set the destination [Address] of the [Pgn] if it is destination specific
    pub fn set_destination_address(&mut self, address: Address) {
        if self.pdu_format.is_destination_specific() {
            self.pdu_specific = PduSpecific::new(address.raw());
        }
    }

    /// Get the group extension of the [Pgn] if it is a group extension
    pub fn get_group_extension(&self) -> Option<u8> {
        if self.pdu_format().is_group_extension() {
            Some(self.pdu_specific.raw())
        } else {
            None
        }
    }

    /// Set the group extension of the [Pgn] if it is a group extension
    pub fn set_group_extension(&mut self, group_extension: u8) {
        if self.pdu_format.is_group_extension() {
            self.pdu_specific = PduSpecific::new(group_extension);
        }
    }

    /// Returns if the [Pgn] is proprietary
    pub fn is_proprietary(&self) -> bool {
        self.raw() == Self::PDU_1_FORMAT_PROPRIETARY_A
            || Self::PDU_2_FORMAT_PROPRIETARY_B.0.contains(&self.raw())
            || Self::PDU_2_FORMAT_PROPRIETARY_B.1.contains(&self.raw())
            || self.raw() == Self::PDU_1_FORMAT_PROPRIETARY_A2
    }

    /// Get the extended data page of the [Pgn]
    #[inline]
    pub fn extended_data_page(&self) -> bool {
        self.extended_data_page
    }

    /// Get the data page of the [Pgn]
    #[inline]
    pub fn data_page(&self) -> bool {
        self.data_page
    }

    /// Get the [Page] of the [Pgn] resulting from the extended data page and data page
    #[inline]
    pub fn page(&self) -> Page {
        Page::from([self.extended_data_page, self.data_page])
    }

    /// Get the [PduFormat] of the [Pgn]
    #[inline]
    pub fn pdu_format(&self) -> PduFormat {
        self.pdu_format
    }

    /// Get the [PduSpecific] of the [Pgn]
    #[inline]
    pub fn pdu_specific(&self) -> PduSpecific {
        self.pdu_specific
    }
}

impl TryFrom<u32> for Pgn {
    type Error = ParsePgnError;

    fn try_from(raw_pgn: u32) -> Result<Self, Self::Error> {
        if raw_pgn > Self::MAX_VALUE {
            // raw value is too large to fit in PGN with 18 bits
            return Err(ParsePgnError::InvalidPgnLength(raw_pgn));
        }

        let raw_pgn_be_bytes = raw_pgn.to_le_bytes();
        let mut bit_data = raw_pgn_be_bytes.view_bits::<Lsb0>().to_bitvec();
        bit_data.truncate(Pgn::BIT_LENGTH);
        bit_data.reverse();

        let mut pdu_format = bit_data[Self::PDU_FORMAT_START..Self::PDU_FORMAT_END].to_bitvec();
        pdu_format.reverse();

        let mut pdu_specific =
            bit_data[Self::PDU_SPECIFIC_START..Self::PDU_SPECIFIC_END].to_bitvec();
        pdu_specific.reverse();

        let pgn = Self {
            extended_data_page: bit_data[Self::EDP_START as usize],
            data_page: bit_data[Self::DP_START as usize],
            pdu_format: PduFormat::new(pdu_format.load()),
            pdu_specific: PduSpecific::new(pdu_specific.load()),
        };

        Ok(pgn)
    }
}

#[cfg(test)]
mod tests {
    use crate::j1939::pdu_format::PduFormat;
    use crate::j1939::pdu_specific::PduSpecific;
    use crate::j1939::{Address, Page, Pgn};

    #[test]
    fn test_raw_bits() {
        let pgn = Pgn::new(true, false, PduFormat::new(0xE6), PduSpecific::new(0xBA));
        let raw_bits = pgn.raw_bits();
        assert_eq!(
            raw_bits,
            [
                true, false, true, true, true, false, false, true, true, false, true, false, true,
                true, true, false, true, false
            ]
        );

        let pgn = Pgn::new(false, true, PduFormat::new(0x00), PduSpecific::new(0xFF));
        let raw_bits = pgn.raw_bits();
        assert_eq!(
            raw_bits,
            [
                false, true, false, false, false, false, false, false, false, false, true, true,
                true, true, true, true, true, true
            ]
        );

        let pgn = Pgn::new(false, true, PduFormat::new(0x12), PduSpecific::new(0x34));
        let raw_bits = pgn.raw_bits();
        assert_eq!(
            raw_bits,
            [
                false, true, false, false, false, true, false, false, true, false, false, false,
                true, true, false, true, false, false
            ]
        );
    }

    #[test]
    fn test_raw() {
        let pgn = Pgn::new(true, false, PduFormat::new(0xE6), PduSpecific::new(0xBA));
        assert_eq!(pgn.raw(), 0x2E6BA);

        let pgn = Pgn::new(false, true, PduFormat::new(0x00), PduSpecific::new(0xFF));
        assert_eq!(pgn.raw(), 0x100FF);

        let pgn = Pgn::new(false, true, PduFormat::new(0x12), PduSpecific::new(0x34));
        assert_eq!(pgn.raw(), 0x11234);
    }

    #[test]
    fn test_get_destination_address() {
        let pgn = Pgn::new(true, false, PduFormat::new(0xF3), PduSpecific::new(0xBA));
        assert_eq!(pgn.get_destination_address(), None);

        let pgn = Pgn::new(false, true, PduFormat::new(0x00), PduSpecific::new(0xFF));
        assert_eq!(pgn.get_destination_address(), Some(Address::BROADCAST));

        let pgn = Pgn::new(false, true, PduFormat::new(0x12), PduSpecific::new(0x34));
        assert_eq!(pgn.get_destination_address(), Some(Address::new(0x34)));
    }

    #[test]
    fn test_set_destination_address() {
        let mut pgn = Pgn::new(true, false, PduFormat::new(0xF3), PduSpecific::new(0xBA));
        pgn.set_destination_address(Address::new(0x34));
        assert_eq!(pgn.get_destination_address(), None);

        let mut pgn = Pgn::new(false, true, PduFormat::new(0x00), PduSpecific::new(0xFF));
        pgn.set_destination_address(Address::new(0x34));
        assert_eq!(pgn.get_destination_address(), Some(Address::new(0x34)));

        let mut pgn = Pgn::new(false, true, PduFormat::new(0x12), PduSpecific::new(0x34));
        pgn.set_destination_address(Address::new(0x56));
        assert_eq!(pgn.get_destination_address(), Some(Address::new(0x56)));
    }

    #[test]
    fn test_get_group_extension() {
        let pgn = Pgn::new(true, false, PduFormat::new(0xF3), PduSpecific::new(0xBA));
        assert_eq!(pgn.get_group_extension(), Some(0xBA));

        let pgn = Pgn::new(false, true, PduFormat::new(0x00), PduSpecific::new(0xFF));
        assert_eq!(pgn.get_group_extension(), None);

        let pgn = Pgn::new(false, true, PduFormat::new(0x12), PduSpecific::new(0x34));
        assert_eq!(pgn.get_group_extension(), None);
    }

    #[test]
    fn test_set_group_extension() {
        let mut pgn = Pgn::new(true, false, PduFormat::new(0xF3), PduSpecific::new(0xBA));
        pgn.set_group_extension(0x34);
        assert_eq!(pgn.get_group_extension(), Some(0x34));

        let mut pgn = Pgn::new(false, true, PduFormat::new(0x00), PduSpecific::new(0xFF));
        pgn.set_group_extension(0x34);
        assert_eq!(pgn.get_group_extension(), None);

        let mut pgn = Pgn::new(false, true, PduFormat::new(0x12), PduSpecific::new(0x34));
        pgn.set_group_extension(0x56);
        assert_eq!(pgn.get_group_extension(), None);
    }

    #[test]
    fn test_is_proprietary() {
        let pgn = Pgn::new(true, false, PduFormat::new(0xF3), PduSpecific::new(0xBA));
        assert!(!pgn.is_proprietary());

        let pgn = Pgn::new(false, true, PduFormat::new(0x00), PduSpecific::new(0xFF));
        assert!(!pgn.is_proprietary());

        let pgn = Pgn::new(false, true, PduFormat::new(0x12), PduSpecific::new(0x34));
        assert!(!pgn.is_proprietary());

        let pgn = Pgn::new(false, true, PduFormat::new(0xEF), PduSpecific::new(0x00));
        assert!(pgn.is_proprietary());

        let pgn = Pgn::new(false, true, PduFormat::new(0xFF), PduSpecific::new(0x00));
        assert!(pgn.is_proprietary());

        let pgn = Pgn::new(false, true, PduFormat::new(0x1E), PduSpecific::new(0x00));
        assert!(!pgn.is_proprietary());

        let pgn = Pgn::new(false, true, PduFormat::new(0x1F), PduSpecific::new(0x00));
        assert!(!pgn.is_proprietary());
    }

    #[test]
    fn test_page() {
        let pgn = Pgn::new(false, false, PduFormat::new(0xF3), PduSpecific::new(0xBA));
        assert_eq!(pgn.page(), Page::J1939Page0);

        let pgn = Pgn::new(false, true, PduFormat::new(0x00), PduSpecific::new(0xFF));
        assert_eq!(pgn.page(), Page::J1939Page1);

        let pgn = Pgn::new(true, false, PduFormat::new(0x12), PduSpecific::new(0x34));
        assert_eq!(pgn.page(), Page::J1939PageReserved);

        let pgn = Pgn::new(true, true, PduFormat::new(0x12), PduSpecific::new(0x34));
        assert_eq!(pgn.page(), Page::ISO11992_4Defined);
    }

    #[test]
    fn test_try_from() {
        let pgn_parsed = Pgn::try_from(0x2E6BA).expect("Failed to parse PGN");

        let pgn = Pgn::new(true, false, PduFormat::new(0xE6), PduSpecific::new(0xBA));
        assert_eq!(pgn, pgn_parsed);

        let pgn_parsed = Pgn::try_from(0x100FF).expect("Failed to parse PGN");

        let pgn = Pgn::new(false, true, PduFormat::new(0x00), PduSpecific::new(0xFF));
        assert_eq!(pgn, pgn_parsed);

        let pgn_parsed = Pgn::try_from(0x11234).expect("Failed to parse PGN");

        let pgn = Pgn::new(false, true, PduFormat::new(0x12), PduSpecific::new(0x34));
        assert_eq!(pgn, pgn_parsed);
    }
}
