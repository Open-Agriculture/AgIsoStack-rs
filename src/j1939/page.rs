use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;

#[derive(Debug, PartialEq)]
pub enum ParsePageError {
    InvalidPage(u8),
}

/// Page definition (EDP & DP combination)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Page {
    /// J1939 page 0
    J1939Page0 = 0,
    /// J1939 page 1
    J1939Page1 = 1,
    /// J1939 page reserved
    J1939PageReserved = 2,
    /// ISO 11992-4 defined
    /// <div class="warning">
    /// In this case the rest of the identifier is not defined by J1939!
    /// Please refer to the ISO 11992-4 standard for more information.
    /// </div>
    ISO11992_4Defined = 3,
}

impl Page {
    pub const BIT_LENGTH: usize = 2;

    pub fn try_from_raw(raw_page: u8) -> Result<Self, ParsePageError> {
        match raw_page {
            0x0 => Ok(Page::J1939Page0),
            0x1 => Ok(Page::J1939Page1),
            0x2 => Ok(Page::J1939PageReserved),
            0x3 => Ok(Page::ISO11992_4Defined),
            _ => Err(ParsePageError::InvalidPage(raw_page)),
        }
    }

    pub fn from_raw_bits(raw_page: [bool; 2]) -> Self {
        let mut page_vec: BitVec<u8, Msb0> = BitVec::new();
        page_vec.extend(raw_page.iter());
        Page::try_from_raw(page_vec.load::<u8>()).unwrap()
    }
}

impl TryFrom<u8> for Page {
    type Error = ParsePageError;

    fn try_from(raw_page: u8) -> Result<Self, Self::Error> {
        Page::try_from_raw(raw_page)
    }
}

impl From<[bool; 2]> for Page {
    fn from(value: [bool; 2]) -> Self {
        Page::from_raw_bits(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::j1939::{Page, ParsePageError};

    #[test]
    fn test_try_from_u8_for_page() {
        assert_eq!(Page::try_from_raw(0x0).unwrap(), Page::J1939Page0);
        assert_eq!(Page::try_from_raw(0x1).unwrap(), Page::J1939Page1);
        assert_eq!(Page::try_from_raw(0x2).unwrap(), Page::J1939PageReserved);
        assert_eq!(Page::try_from_raw(0x3).unwrap(), Page::ISO11992_4Defined);
        assert_eq!(
            Page::try_from_raw(0x4).unwrap_err(),
            ParsePageError::InvalidPage(4)
        );
    }

    #[test]
    fn test_from_bool_array_for_page() {
        assert_eq!(Page::from_raw_bits([false, false]), Page::J1939Page0);
        assert_eq!(Page::from_raw_bits([false, true]), Page::J1939Page1);
        assert_eq!(Page::from_raw_bits([true, false]), Page::J1939PageReserved);
        assert_eq!(Page::from_raw_bits([true, true]), Page::ISO11992_4Defined);
    }
}
