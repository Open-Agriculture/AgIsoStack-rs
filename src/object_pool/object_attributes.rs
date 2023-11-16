use crate::object_pool::object_id::ObjectId;
use bitvec::field::BitField;
use bitvec::order::{Lsb0, Msb0};
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ButtonOptions {
    pub latchable: bool,
    pub state: ButtonState,
    pub suppress_border: bool,
    pub transparent_background: bool,
    pub disabled: bool,
    pub no_border: bool,
}

impl From<u8> for ButtonOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        ButtonOptions {
            latchable: bit_data.pop().unwrap(),
            state: bit_data.pop().unwrap().into(),
            suppress_border: bit_data.pop().unwrap(),
            transparent_background: bit_data.pop().unwrap(),
            disabled: bit_data.pop().unwrap(),
            no_border: bit_data.pop().unwrap(),
        }
    }
}

impl From<ButtonOptions> for u8 {
    fn from(value: ButtonOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.latchable);
        bit_data.push(value.state.into());
        bit_data.push(value.suppress_border);
        bit_data.push(value.transparent_background);
        bit_data.push(value.disabled);
        bit_data.push(value.no_border);
        bit_data.extend([0; 3]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ButtonState {
    Released,
    Latched,
}

impl From<ButtonState> for bool {
    fn from(value: ButtonState) -> Self {
        match value {
            ButtonState::Released => false,
            ButtonState::Latched => true,
        }
    }
}

impl From<bool> for ButtonState {
    fn from(value: bool) -> Self {
        match value {
            false => ButtonState::Released,
            true => ButtonState::Latched,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct InputStringOptions {
    pub transparent: bool,
    pub auto_wrap: bool,
    pub wrap_on_hyphen: bool,
}

impl From<u8> for InputStringOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        InputStringOptions {
            transparent: bit_data.pop().unwrap(),
            auto_wrap: bit_data.pop().unwrap(),
            wrap_on_hyphen: bit_data.pop().unwrap(),
        }
    }
}

impl From<InputStringOptions> for u8 {
    fn from(value: InputStringOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.transparent);
        bit_data.push(value.auto_wrap);
        bit_data.push(value.wrap_on_hyphen);
        bit_data.extend([0; 5]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alignment {
    pub horizontal: HorizontalAlignment,
    pub vertical: VerticalAlignment,
}

impl From<u8> for Alignment {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Lsb0>().to_bitvec();
        Alignment {
            horizontal: HorizontalAlignment::from([
                bit_data.pop().unwrap(),
                bit_data.pop().unwrap(),
            ]),
            vertical: VerticalAlignment::from([bit_data.pop().unwrap(), bit_data.pop().unwrap()]),
        }
    }
}

impl From<Alignment> for u8 {
    fn from(value: Alignment) -> Self {
        let mut bit_data: BitVec<u8> = BitVec::new();
        let horizontal_align: [bool; 2] = value.horizontal.into();
        let vertical_align: [bool; 2] = value.vertical.into();

        bit_data.push(horizontal_align[0]);
        bit_data.push(horizontal_align[1]);

        bit_data.push(vertical_align[0]);
        bit_data.push(vertical_align[1]);

        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HorizontalAlignment {
    Left = 0,
    Middle = 1,
    Right = 2,
    Reserved = 3,
}

impl From<[bool; 2]> for HorizontalAlignment {
    fn from(value: [bool; 2]) -> Self {
        match value[0] {
            false => match value[1] {
                false => HorizontalAlignment::Left,
                true => HorizontalAlignment::Middle,
            },
            true => match value[1] {
                false => HorizontalAlignment::Middle,
                true => HorizontalAlignment::Reserved,
            },
        }
    }
}

impl From<HorizontalAlignment> for [bool; 2] {
    fn from(value: HorizontalAlignment) -> Self {
        match value {
            HorizontalAlignment::Left => [false, false],
            HorizontalAlignment::Middle => [false, true],
            HorizontalAlignment::Right => [true, false],
            HorizontalAlignment::Reserved => [true, true],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerticalAlignment {
    Top = 0,
    Middle = 1,
    Bottom = 2,
    Reserved = 3,
}

impl From<[bool; 2]> for VerticalAlignment {
    fn from(value: [bool; 2]) -> Self {
        match value[0] {
            false => match value[1] {
                false => VerticalAlignment::Top,
                true => VerticalAlignment::Middle,
            },
            true => match value[1] {
                false => VerticalAlignment::Bottom,
                true => VerticalAlignment::Reserved,
            },
        }
    }
}

impl From<VerticalAlignment> for [bool; 2] {
    fn from(value: VerticalAlignment) -> Self {
        match value {
            VerticalAlignment::Top => [false, false],
            VerticalAlignment::Middle => [false, true],
            VerticalAlignment::Bottom => [true, false],
            VerticalAlignment::Reserved => [true, true],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InputNumberOptions {
    pub enabled: bool,
    pub real_time_editing: bool,
}

impl From<u8> for InputNumberOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        InputNumberOptions {
            enabled: bit_data.pop().unwrap(),
            real_time_editing: bit_data.pop().unwrap(),
        }
    }
}

impl From<InputNumberOptions> for u8 {
    fn from(value: InputNumberOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.enabled);
        bit_data.push(value.real_time_editing);
        bit_data.extend([0; 6]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormatType {
    Decimal,
    Exponential,
}

impl From<bool> for FormatType {
    fn from(value: bool) -> Self {
        match value {
            false => FormatType::Decimal,
            true => FormatType::Exponential,
        }
    }
}

impl From<FormatType> for bool {
    fn from(value: FormatType) -> Self {
        match value {
            FormatType::Decimal => false,
            FormatType::Exponential => true,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct InputListOptions {
    pub enabled: bool,
    pub real_time_editing: bool,
}

impl From<u8> for InputListOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        InputListOptions {
            enabled: bit_data.pop().unwrap(),
            real_time_editing: bit_data.pop().unwrap(),
        }
    }
}

impl From<InputListOptions> for u8 {
    fn from(value: InputListOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.enabled);
        bit_data.push(value.real_time_editing);
        bit_data.extend([0; 6]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OutputStringOptions {
    pub transparent: bool,
    pub auto_wrap: bool,
    pub wrap_on_hyphen: bool,
}

impl From<u8> for OutputStringOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        OutputStringOptions {
            transparent: bit_data.pop().unwrap(),
            auto_wrap: bit_data.pop().unwrap(),
            wrap_on_hyphen: bit_data.pop().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NumberOptions {
    pub transparent: bool,
    pub display_leading_zeros: bool,
    pub display_zero_as_blank: bool,
    pub truncate: bool,
}

impl From<u8> for NumberOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        NumberOptions {
            transparent: bit_data.pop().unwrap(),
            display_leading_zeros: bit_data.pop().unwrap(),
            display_zero_as_blank: bit_data.pop().unwrap(),
            truncate: bit_data.pop().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LineDirection {
    TopLeftToBottomRight,
    BottomLeftToTopRight,
}

impl From<u8> for LineDirection {
    fn from(value: u8) -> Self {
        match value {
            0 => LineDirection::TopLeftToBottomRight,
            1 => LineDirection::BottomLeftToTopRight,
            _ => panic!("Invalid line direction"),
        }
    }
}

impl From<LineDirection> for u8 {
    fn from(value: LineDirection) -> Self {
        match value {
            LineDirection::TopLeftToBottomRight => 0,
            LineDirection::BottomLeftToTopRight => 1,
        }
    }
}

impl From<NumberOptions> for u8 {
    fn from(value: NumberOptions) -> Self {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.transparent);
        bit_data.push(value.display_leading_zeros);
        bit_data.push(value.display_zero_as_blank);
        bit_data.push(value.truncate);
        bit_data.extend([0; 4]);
        bit_data.load::<u8>()
    }
}

impl From<OutputStringOptions> for u8 {
    fn from(value: OutputStringOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.transparent);
        bit_data.push(value.auto_wrap);
        bit_data.push(value.wrap_on_hyphen);
        bit_data.extend([0; 5]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ColorFormat {
    ColorMonochrome,
    Color4Bit,
    Color8Bit,
}

impl From<ColorFormat> for u8 {
    fn from(value: ColorFormat) -> Self {
        match value {
            ColorFormat::ColorMonochrome => 0,
            ColorFormat::Color4Bit => 1,
            ColorFormat::Color8Bit => 2,
        }
    }
}

impl From<u8> for ColorFormat {
    fn from(value: u8) -> Self {
        match value {
            0 => ColorFormat::ColorMonochrome,
            1 => ColorFormat::Color4Bit,
            2 => ColorFormat::Color8Bit,
            _ => panic!("Invalid color format: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ColorOption {
    ForegroundBackground,
    LineFontFill,
}

impl From<bool> for ColorOption {
    fn from(value: bool) -> Self {
        match value {
            false => ColorOption::ForegroundBackground,
            true => ColorOption::LineFontFill,
        }
    }
}

impl From<ColorOption> for bool {
    fn from(value: ColorOption) -> Self {
        match value {
            ColorOption::ForegroundBackground => false,
            ColorOption::LineFontFill => true,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GraphicsContextOptions {
    pub transparent: bool,
    pub color: ColorOption,
}

impl From<u8> for GraphicsContextOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Lsb0>().to_bitvec();
        GraphicsContextOptions {
            transparent: bit_data.pop().unwrap(),
            color: bit_data.pop().unwrap().into(),
        }
    }
}

impl From<GraphicsContextOptions> for u8 {
    fn from(value: GraphicsContextOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.transparent);
        bit_data.push(value.color.into());
        bit_data.extend([0; 6]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyGroupOptions {
    pub available: bool,
    pub transparent: bool,
}

impl From<u8> for KeyGroupOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Lsb0>().to_bitvec();
        KeyGroupOptions {
            available: bit_data.pop().unwrap(),
            transparent: bit_data.pop().unwrap(),
        }
    }
}

impl From<KeyGroupOptions> for u8 {
    fn from(value: KeyGroupOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.available);
        bit_data.push(value.transparent);
        bit_data.extend([0; 6]);
        bit_data.load::<u8>()
    }
}
