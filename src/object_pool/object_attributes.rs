use crate::object_pool::object_id::ObjectId;
use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
use strum_macros::FromRepr;

#[derive(FromRepr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum WindowType {
    FreeForm = 0,
    NumericOutputValueWithUnits1x1 = 1,
    NumericOutputValueNoUnits1x1 = 2,
    StringOutputValue1x1 = 3,
    NumericInputValueWithUnits1x1 = 4,
    NumericInputValueNoUnits1x1 = 5,
    StringInputValue1x1 = 6,
    HorizontalLinearBarGraph1x1 = 7,
    SingleButton1x1 = 8,
    DoubleButton1x1 = 9,
    NumericOutputValueWithUnits2x1 = 10,
    NumericOutputValueNoUnits2x1 = 11,
    StringOutputValue2x1 = 12,
    NumericInputValueWithUnits2x1 = 13,
    NumericInputValueNoUnits2x1 = 14,
    StringInputValue2x1 = 15,
    HorizontalLinearBarGraph2x1 = 16,
    SingleButton2x1 = 17,
    DoubleButton2x1 = 18,
}

impl From<u8> for WindowType {
    fn from(value: u8) -> Self {
        WindowType::from_repr(value).unwrap()
    }
}

impl From<WindowType> for u8 {
    fn from(value: WindowType) -> Self {
        value.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowMaskCellFormat {
    CF1x1,
    CF1x2,
    CF1x3,
    CF1x4,
    CF1x5,
    CF1x6,
    CF2x1,
    CF2x2,
    CF2x3,
    CF2x4,
    CF2x5,
    CF2x6,
}

impl WindowMaskCellFormat {
    const fn from_size(x: u8, y: u8) -> WindowMaskCellFormat {
        let size = Point { x, y };
        match size {
            Point { x: 1, y: 1 } => WindowMaskCellFormat::CF1x1,
            Point { x: 1, y: 2 } => WindowMaskCellFormat::CF1x2,
            Point { x: 1, y: 3 } => WindowMaskCellFormat::CF1x3,
            Point { x: 1, y: 4 } => WindowMaskCellFormat::CF1x4,
            Point { x: 1, y: 5 } => WindowMaskCellFormat::CF1x5,
            Point { x: 1, y: 6 } => WindowMaskCellFormat::CF1x6,
            Point { x: 2, y: 1 } => WindowMaskCellFormat::CF2x1,
            Point { x: 2, y: 2 } => WindowMaskCellFormat::CF2x2,
            Point { x: 2, y: 3 } => WindowMaskCellFormat::CF2x3,
            Point { x: 2, y: 4 } => WindowMaskCellFormat::CF2x4,
            Point { x: 2, y: 5 } => WindowMaskCellFormat::CF2x5,
            Point { x: 2, y: 6 } => WindowMaskCellFormat::CF2x6,
            _ => WindowMaskCellFormat::CF1x1,
        }
    }

    pub const fn size(self) -> Point<u8> {
        match self {
            WindowMaskCellFormat::CF1x1 => Point { x: 1, y: 1 },
            WindowMaskCellFormat::CF1x2 => Point { x: 1, y: 2 },
            WindowMaskCellFormat::CF1x3 => Point { x: 1, y: 3 },
            WindowMaskCellFormat::CF1x4 => Point { x: 1, y: 4 },
            WindowMaskCellFormat::CF1x5 => Point { x: 1, y: 5 },
            WindowMaskCellFormat::CF1x6 => Point { x: 1, y: 6 },
            WindowMaskCellFormat::CF2x1 => Point { x: 2, y: 1 },
            WindowMaskCellFormat::CF2x2 => Point { x: 2, y: 2 },
            WindowMaskCellFormat::CF2x3 => Point { x: 2, y: 3 },
            WindowMaskCellFormat::CF2x4 => Point { x: 2, y: 4 },
            WindowMaskCellFormat::CF2x5 => Point { x: 2, y: 5 },
            WindowMaskCellFormat::CF2x6 => Point { x: 2, y: 6 },
        }
    }
}

impl From<u16> for WindowMaskCellFormat {
    fn from(value: u16) -> Self {
        WindowMaskCellFormat::from_size((value << 8) as u8, value as u8)
    }
}

impl From<WindowMaskCellFormat> for u16 {
    fn from(value: WindowMaskCellFormat) -> Self {
        let size = value.size();
        ((size.x as u16) << 8) | size.y as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowMaskOptions {
    pub available: bool,
    pub transparent: bool,
}

impl From<u8> for WindowMaskOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        WindowMaskOptions {
            available: bit_data.pop().unwrap(),
            transparent: bit_data.pop().unwrap(),
        }
    }
}

impl From<WindowMaskOptions> for u8 {
    fn from(value: WindowMaskOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.available);
        bit_data.push(value.transparent);
        bit_data.extend([0; 6]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectRef {
    pub id: ObjectId,
    pub offset: Point<i16>,
    // pub x: i16,
    // pub y: i16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MacroRef {
    pub macro_id: u8,
    pub event_id: u8,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl core::ops::Add<Point<i16>> for Point<u16> {
    type Output = Point<u16>;

    fn add(self, rhs: Point<i16>) -> Self::Output {
        Point {
            x: (self.x as i16 + rhs.x) as u16,
            y: (self.y as i16 + rhs.y) as u16,
        }
    }
}

#[derive(Debug)]
pub struct ObjectLabel {
    pub id: ObjectId,
    pub string_variable_reference: ObjectId,
    pub font_type: u8,
    pub graphic_representation: ObjectId,
}
