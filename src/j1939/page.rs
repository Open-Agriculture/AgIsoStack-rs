use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;

#[derive(Debug)]
enum ParsePageError {
    InvalidPage(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Page {
    J1939Page0 = 0,
    J1939Page1 = 1,
    J1939PageReserved = 2,
    ISO11992_4 = 3,
}

impl TryFrom<u8> for Page {
    type Error = ParsePageError;

    fn try_from(raw_page: u8) -> Result<Self, Self::Error> {
        match raw_page {
            0x0 => Ok(Page::J1939Page0),
            0x1 => Ok(Page::J1939Page1),
            0x2 => Ok(Page::J1939PageReserved),
            0x3 => Ok(Page::ISO11992_4),
            _ => Err(ParsePageError::InvalidPage(raw_page)),
        }
    }
}

impl From<[bool; 2]> for Page {
    fn from(value: [bool; 2]) -> Self {
        let mut page_vec: BitVec<u8, Msb0> = BitVec::new();
        page_vec.resize(8, false);
        page_vec[0] = value[0];
        page_vec[1] = value[1];
        Page::try_from(page_vec.load::<u8>()).unwrap()
    }
}
