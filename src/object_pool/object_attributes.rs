use crate::object_pool::object_id::ObjectId;
use bitvec::field::BitField;
use bitvec::order::{Lsb0, Msb0};
use bitvec::vec::BitVec;
use bitvec::view::BitView;
use strum_macros::FromRepr;

use super::object_id::NullableObjectId;

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
        value as u8
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
        bit_data.extend([false; 6]);
        bit_data.load::<u8>()
    }
}

#[derive(FromRepr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Event {
    Reserved = 0,
    OnActivate = 1,
    OnDeactivate = 2,
    OnShow = 3,
    OnHide = 4,
    // OnRefresh = N/A
    OnEnable = 5,
    OnDisable = 6,
    OnChangeActiveMask = 7,
    OnChangeSoftKeyMask = 8,
    OnChangeAttribute = 9,
    OnChangeBackgroundColour = 10,
    OnChangeFontAttributes = 11,
    OnChangeLineAttributes = 12,
    OnChangeFillAttributes = 13,
    OnChangeChildLocation = 14,
    OnChangeSize = 15,
    OnChangeValue = 16,
    OnChangePriority = 17,
    OnChangeEndPoint = 18,
    OnInputFieldSelection = 19,
    OnInputFieldDeselection = 20,
    OnESC = 21,
    OnEntryOfValue = 22,
    OnEntryOfNewValue = 23,
    OnKeyPress = 24,
    OnKeyRelease = 25,
    OnChangeChildPosition = 26,
    OnPointingEventPress = 27,
    OnPointingEventRelease = 28,
    // Reserved 29-239
    ProprietaryEvent1 = 240,
    ProprietaryEvent2 = 241,
    ProprietaryEvent3 = 242,
    ProprietaryEvent4 = 243,
    ProprietaryEvent5 = 244,
    ProprietaryEvent6 = 245,
    ProprietaryEvent7 = 246,
    ProprietaryEvent8 = 247,
    ProprietaryEvent9 = 248,
    ProprietaryEvent10 = 249,
    ProprietaryEvent11 = 250,
    ProprietaryEvent12 = 251,
    ProprietaryEvent13 = 252,
    ProprietaryEvent14 = 253,
    ProprietaryEvent15 = 254,
    UseExtendedMacro = 255,
}

impl Event {
    pub fn iter() -> impl Iterator<Item = Event> {
        (0..=255).map(Event::from).filter(|e| *e != Event::Reserved)
    }
}

impl From<u8> for Event {
    fn from(value: u8) -> Self {
        Event::from_repr(value).unwrap_or(Event::Reserved)
    }
}

impl From<Event> for u8 {
    fn from(value: Event) -> Self {
        value as u8
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ObjectRef {
    pub id: ObjectId,
    pub offset: Point<i16>,
    // pub x: i16,
    // pub y: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MacroRef {
    pub macro_id: u8,
    pub event_id: Event,
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

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectLabel {
    pub id: ObjectId,
    pub string_variable_reference: NullableObjectId,
    pub font_type: u8,
    pub graphic_representation: NullableObjectId,
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
        bit_data.extend([false; 2]);
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
        bit_data.extend([false; 5]);
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
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
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
                false => HorizontalAlignment::Right,
                true => HorizontalAlignment::Reserved,
            },
        }
    }
}

impl From<HorizontalAlignment> for [bool; 2] {
    fn from(value: HorizontalAlignment) -> Self {
        match value {
            HorizontalAlignment::Left => [false, false],
            HorizontalAlignment::Middle => [true, false],
            HorizontalAlignment::Right => [false, true],
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
            VerticalAlignment::Middle => [true, false],
            VerticalAlignment::Bottom => [false, true],
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
        bit_data.extend([false; 6]);
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
        bit_data.extend([false; 6]);
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
        bit_data.extend([false; 4]);
        bit_data.load::<u8>()
    }
}

impl From<OutputStringOptions> for u8 {
    fn from(value: OutputStringOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.transparent);
        bit_data.push(value.auto_wrap);
        bit_data.push(value.wrap_on_hyphen);
        bit_data.extend([false; 5]);
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
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
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
        bit_data.extend([false; 6]);
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
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
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
        bit_data.extend([false; 6]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeflectionDirection {
    AntiClockwise,
    Clockwise,
}

impl From<bool> for DeflectionDirection {
    fn from(value: bool) -> Self {
        match value {
            false => DeflectionDirection::AntiClockwise,
            true => DeflectionDirection::Clockwise,
        }
    }
}

impl From<DeflectionDirection> for bool {
    fn from(value: DeflectionDirection) -> Self {
        match value {
            DeflectionDirection::AntiClockwise => false,
            DeflectionDirection::Clockwise => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputMeterOptions {
    pub draw_arc: bool,
    pub draw_border: bool,
    pub draw_ticks: bool,
    pub deflection_direction: DeflectionDirection,
}

impl From<u8> for OutputMeterOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        OutputMeterOptions {
            draw_arc: bit_data.pop().unwrap(),
            draw_border: bit_data.pop().unwrap(),
            draw_ticks: bit_data.pop().unwrap(),
            deflection_direction: bit_data.pop().unwrap().into(),
        }
    }
}

impl From<OutputMeterOptions> for u8 {
    fn from(value: OutputMeterOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.draw_arc);
        bit_data.push(value.draw_border);
        bit_data.push(value.draw_ticks);
        bit_data.push(value.deflection_direction.into());
        bit_data.extend([false; 4]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarGraphType {
    Filled,
    NotFilled,
}

impl From<bool> for BarGraphType {
    fn from(value: bool) -> Self {
        match value {
            false => BarGraphType::Filled,
            true => BarGraphType::NotFilled,
        }
    }
}

impl From<BarGraphType> for bool {
    fn from(value: BarGraphType) -> Self {
        match value {
            BarGraphType::Filled => false,
            BarGraphType::NotFilled => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisOrientation {
    Vertical,
    Horizontal,
}

impl From<bool> for AxisOrientation {
    fn from(value: bool) -> Self {
        match value {
            false => AxisOrientation::Vertical,
            true => AxisOrientation::Horizontal,
        }
    }
}

impl From<AxisOrientation> for bool {
    fn from(value: AxisOrientation) -> Self {
        match value {
            AxisOrientation::Vertical => false,
            AxisOrientation::Horizontal => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrowDirection {
    GrowLeftDown,
    GrowRightUp,
}

impl From<bool> for GrowDirection {
    fn from(value: bool) -> Self {
        match value {
            false => GrowDirection::GrowLeftDown,
            true => GrowDirection::GrowRightUp,
        }
    }
}

impl From<GrowDirection> for bool {
    fn from(value: GrowDirection) -> Self {
        match value {
            GrowDirection::GrowLeftDown => false,
            GrowDirection::GrowRightUp => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputLinearBarGraphOptions {
    pub draw_border: bool,
    pub draw_target_line: bool,
    pub draw_ticks: bool,
    pub bar_graph_type: BarGraphType,
    pub axis_orientation: AxisOrientation,
    pub grow_direction: GrowDirection,
}

impl From<u8> for OutputLinearBarGraphOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        OutputLinearBarGraphOptions {
            draw_border: bit_data.pop().unwrap(),
            draw_target_line: bit_data.pop().unwrap(),
            draw_ticks: bit_data.pop().unwrap(),
            bar_graph_type: bit_data.pop().unwrap().into(),
            axis_orientation: bit_data.pop().unwrap().into(),
            grow_direction: bit_data.pop().unwrap().into(),
        }
    }
}

impl From<OutputLinearBarGraphOptions> for u8 {
    fn from(value: OutputLinearBarGraphOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.draw_border);
        bit_data.push(value.draw_target_line);
        bit_data.push(value.draw_ticks);
        bit_data.push(value.bar_graph_type.into());
        bit_data.push(value.axis_orientation.into());
        bit_data.push(value.grow_direction.into());
        bit_data.extend([false; 2]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputArchedBarGraphOptions {
    pub draw_border: bool,
    pub draw_target_line: bool,
    pub bar_graph_type: BarGraphType,
    pub axis_orientation: AxisOrientation,
    pub grow_direction: GrowDirection,
    pub deflection_direction: DeflectionDirection,
}

impl From<u8> for OutputArchedBarGraphOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        let draw_border = bit_data.pop().unwrap();
        let draw_target_line = bit_data.pop().unwrap();
        bit_data.pop(); //undefined bit

        OutputArchedBarGraphOptions {
            draw_border,
            draw_target_line,
            bar_graph_type: bit_data.pop().unwrap().into(),
            axis_orientation: bit_data.pop().unwrap().into(),
            grow_direction: bit_data.pop().unwrap().into(),
            deflection_direction: bit_data.pop().unwrap().into(),
        }
    }
}

impl From<OutputArchedBarGraphOptions> for u8 {
    fn from(value: OutputArchedBarGraphOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.draw_border);
        bit_data.push(value.draw_target_line);
        bit_data.push(false); //undefined bit
        bit_data.push(value.bar_graph_type.into());
        bit_data.push(value.axis_orientation.into());
        bit_data.push(value.grow_direction.into());
        bit_data.push(value.deflection_direction.into());
        bit_data.extend([false; 1]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataCodeType {
    Raw,
    RunLength,
}

impl From<bool> for DataCodeType {
    fn from(value: bool) -> Self {
        match value {
            false => DataCodeType::Raw,
            true => DataCodeType::RunLength,
        }
    }
}

impl From<DataCodeType> for bool {
    fn from(value: DataCodeType) -> Self {
        match value {
            DataCodeType::Raw => false,
            DataCodeType::RunLength => true,
        }
    }
}

#[derive(FromRepr, Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PictureGraphicFormat {
    Monochrome = 0,
    FourBit = 1,
    EightBit = 2,
}

impl From<u8> for PictureGraphicFormat {
    fn from(value: u8) -> Self {
        PictureGraphicFormat::from_repr(value).unwrap()
    }
}

impl From<PictureGraphicFormat> for u8 {
    fn from(value: PictureGraphicFormat) -> Self {
        value as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PictureGraphicOptions {
    pub transparent: bool,
    pub flashing: bool,
    pub data_code_type: DataCodeType,
}

impl From<u8> for PictureGraphicOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        PictureGraphicOptions {
            transparent: bit_data.pop().unwrap(),
            flashing: bit_data.pop().unwrap(),
            data_code_type: bit_data.pop().unwrap().into(),
        }
    }
}

impl From<PictureGraphicOptions> for u8 {
    fn from(value: PictureGraphicOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.transparent);
        bit_data.push(value.flashing);
        bit_data.push(value.data_code_type.into());
        bit_data.extend([false; 5]);
        bit_data.load::<u8>()
    }
}

/// Represents the non-proportional font sizes as per the standard.
/// Each variant corresponds to a pixel width × height combination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NonProportionalFontSize {
    Px6x8 = 0,
    Px8x8 = 1,
    Px8x12 = 2,
    Px12x16 = 3,
    Px16x16 = 4,
    Px16x24 = 5,
    Px24x32 = 6,
    Px32x32 = 7,
    Px32x48 = 8,
    Px48x64 = 9,
    Px64x64 = 10,
    Px64x96 = 11,
    Px96x128 = 12,
    Px128x128 = 13,
    Px128x192 = 14,
}

impl NonProportionalFontSize {
    pub fn height(&self) -> u8 {
        match self {
            NonProportionalFontSize::Px6x8 => 8,
            NonProportionalFontSize::Px8x8 => 8,
            NonProportionalFontSize::Px8x12 => 12,
            NonProportionalFontSize::Px12x16 => 16,
            NonProportionalFontSize::Px16x16 => 16,
            NonProportionalFontSize::Px16x24 => 24,
            NonProportionalFontSize::Px24x32 => 32,
            NonProportionalFontSize::Px32x32 => 32,
            NonProportionalFontSize::Px32x48 => 48,
            NonProportionalFontSize::Px48x64 => 64,
            NonProportionalFontSize::Px64x64 => 64,
            NonProportionalFontSize::Px64x96 => 96,
            NonProportionalFontSize::Px96x128 => 128,
            NonProportionalFontSize::Px128x128 => 128,
            NonProportionalFontSize::Px128x192 => 192,
        }
    }

    pub fn width(&self) -> u8 {
        match self {
            NonProportionalFontSize::Px6x8 => 6,
            NonProportionalFontSize::Px8x8 => 8,
            NonProportionalFontSize::Px8x12 => 8,
            NonProportionalFontSize::Px12x16 => 12,
            NonProportionalFontSize::Px16x16 => 16,
            NonProportionalFontSize::Px16x24 => 16,
            NonProportionalFontSize::Px24x32 => 24,
            NonProportionalFontSize::Px32x32 => 32,
            NonProportionalFontSize::Px32x48 => 32,
            NonProportionalFontSize::Px48x64 => 48,
            NonProportionalFontSize::Px64x64 => 64,
            NonProportionalFontSize::Px64x96 => 64,
            NonProportionalFontSize::Px96x128 => 96,
            NonProportionalFontSize::Px128x128 => 128,
            NonProportionalFontSize::Px128x192 => 128,
        }
    }
}

impl From<u8> for NonProportionalFontSize {
    fn from(value: u8) -> Self {
        match value {
            0 => NonProportionalFontSize::Px6x8,
            1 => NonProportionalFontSize::Px8x8,
            2 => NonProportionalFontSize::Px8x12,
            3 => NonProportionalFontSize::Px12x16,
            4 => NonProportionalFontSize::Px16x16,
            5 => NonProportionalFontSize::Px16x24,
            6 => NonProportionalFontSize::Px24x32,
            7 => NonProportionalFontSize::Px32x32,
            8 => NonProportionalFontSize::Px32x48,
            9 => NonProportionalFontSize::Px48x64,
            10 => NonProportionalFontSize::Px64x64,
            11 => NonProportionalFontSize::Px64x96,
            12 => NonProportionalFontSize::Px96x128,
            13 => NonProportionalFontSize::Px128x128,
            14 => NonProportionalFontSize::Px128x192,
            _ => panic!("Invalid non-proportional font size"),
        }
    }
}

impl From<NonProportionalFontSize> for u8 {
    fn from(value: NonProportionalFontSize) -> u8 {
        value as u8
    }
}

/// Represents the font size attribute. If proportional font rendering is enabled
/// (bit 7 in font_style), the font size is a pixel height (8 to N). If not proportional,
/// it is one of the predefined non-proportional sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSize {
    NonProportional(NonProportionalFontSize),
    Proportional(u8), // 8 to N pixel height
}

impl From<u8> for FontSize {
    fn from(value: u8) -> Self {
        if value & 0b1000_0000 == 0 {
            FontSize::NonProportional(NonProportionalFontSize::from(value))
        } else {
            FontSize::Proportional(value & 0b0111_1111)
        }
    }
}

impl From<FontSize> for u8 {
    fn from(value: FontSize) -> u8 {
        match value {
            FontSize::NonProportional(size) => size as u8,
            FontSize::Proportional(size) => size | 0b1000_0000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontType {
    /// 0: ISO 8859-1 (ISO Latin 1)
    Latin1,
    /// 1: ISO 8859-15 (ISO Latin 9)
    Latin9,
    /// 2: ISO 8859-2 (ISO Latin 2)
    Latin2,
    /// 4: ISO 8859-4 (ISO Latin 4)
    Latin4,
    /// 5: ISO 8859-5 (Cyrillic)
    Cyrillic,
    /// 7: ISO 8859-7 (Greek)
    Greek,

    /// Reserved values: 3, 6, and 8–239 fall into this category.
    /// We store the raw value to keep track of exactly which reserved value it is.
    Reserved(u8),

    /// Proprietary values: 240–255.
    /// These may require special handling as defined by the system (see 4.6.24).
    /// We store the raw value to maintain the exact proprietary code.
    Proprietary(u8),
}

impl From<u8> for FontType {
    fn from(value: u8) -> Self {
        match value {
            0 => FontType::Latin1,
            1 => FontType::Latin9,
            2 => FontType::Latin2,
            4 => FontType::Latin4,
            5 => FontType::Cyrillic,
            7 => FontType::Greek,

            // Proprietary range: 240–255
            240..=255 => FontType::Proprietary(value),

            // Reserved ranges:
            // 3, 6, and 8–239 fall here.
            _ => FontType::Reserved(value),
        }
    }
}

impl From<FontType> for u8 {
    fn from(value: FontType) -> u8 {
        match value {
            FontType::Latin1 => 0,
            FontType::Latin9 => 1,
            FontType::Latin2 => 2,
            FontType::Latin4 => 4,
            FontType::Cyrillic => 5,
            FontType::Greek => 7,
            FontType::Reserved(v) => v,
            FontType::Proprietary(v) => v,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontStyle {
    pub bold: bool,
    pub crossed_out: bool,
    pub underlined: bool,
    pub italic: bool,
    pub inverted: bool,
    pub flashing_inverted: bool,
    pub flashing_hidden: bool,
    pub proportional: bool,
}

impl From<u8> for FontStyle {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        FontStyle {
            bold: bit_data.pop().unwrap(),
            crossed_out: bit_data.pop().unwrap(),
            underlined: bit_data.pop().unwrap(),
            italic: bit_data.pop().unwrap(),
            inverted: bit_data.pop().unwrap(),
            flashing_inverted: bit_data.pop().unwrap(),
            flashing_hidden: bit_data.pop().unwrap(),
            proportional: bit_data.pop().unwrap(),
        }
    }
}

impl From<FontStyle> for u8 {
    fn from(value: FontStyle) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.bold);
        bit_data.push(value.crossed_out);
        bit_data.push(value.underlined);
        bit_data.push(value.italic);
        bit_data.push(value.inverted);
        bit_data.push(value.flashing_inverted);
        bit_data.push(value.flashing_hidden);
        bit_data.push(value.proportional);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalObjectDefinitionOptions {
    pub enabled: bool,
}

impl From<u8> for ExternalObjectDefinitionOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        ExternalObjectDefinitionOptions {
            enabled: bit_data.pop().unwrap(),
        }
    }
}

impl From<ExternalObjectDefinitionOptions> for u8 {
    fn from(value: ExternalObjectDefinitionOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.enabled);
        bit_data.extend([false; 7]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalReferenceNameOptions {
    pub enabled: bool,
}

impl From<u8> for ExternalReferenceNameOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Msb0>().to_bitvec();
        ExternalReferenceNameOptions {
            enabled: bit_data.pop().unwrap(),
        }
    }
}

impl From<ExternalReferenceNameOptions> for u8 {
    fn from(value: ExternalReferenceNameOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.enabled);
        bit_data.extend([false; 7]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationSequence {
    SingleShot,
    Loop,
}

impl From<bool> for AnimationSequence {
    fn from(value: bool) -> Self {
        match value {
            false => AnimationSequence::SingleShot,
            true => AnimationSequence::Loop,
        }
    }
}

impl From<AnimationSequence> for bool {
    fn from(value: AnimationSequence) -> Self {
        match value {
            AnimationSequence::SingleShot => false,
            AnimationSequence::Loop => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisabledBehaviour {
    Pause,
    ResetToFirst,
    DefaultObject,
    Blank,
}

impl From<[bool; 2]> for DisabledBehaviour {
    fn from(value: [bool; 2]) -> Self {
        match value {
            [false, false] => DisabledBehaviour::Pause,
            [false, true] => DisabledBehaviour::ResetToFirst,
            [true, false] => DisabledBehaviour::DefaultObject,
            [true, true] => DisabledBehaviour::Blank,
        }
    }
}

impl From<DisabledBehaviour> for [bool; 2] {
    fn from(value: DisabledBehaviour) -> Self {
        match value {
            DisabledBehaviour::Pause => [false, false],
            DisabledBehaviour::ResetToFirst => [false, true],
            DisabledBehaviour::DefaultObject => [true, false],
            DisabledBehaviour::Blank => [true, true],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationOptions {
    pub animation_sequence: AnimationSequence,
    pub disabled_behaviour: DisabledBehaviour,
}

impl From<u8> for AnimationOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Lsb0>().to_bitvec();
        AnimationOptions {
            animation_sequence: bit_data.pop().unwrap().into(),
            disabled_behaviour: DisabledBehaviour::from([
                bit_data.pop().unwrap(),
                bit_data.pop().unwrap(),
            ]),
        }
    }
}

impl From<AnimationOptions> for u8 {
    fn from(value: AnimationOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.animation_sequence.into());
        let disabled_behaviour: [bool; 2] = value.disabled_behaviour.into();
        bit_data.push(disabled_behaviour[0]);
        bit_data.push(disabled_behaviour[1]);
        bit_data.extend([false; 5]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColourPaletteOptions {}

impl From<u8> for ColourPaletteOptions {
    fn from(value: u8) -> Self {
        let mut _bit_data = value.view_bits::<Lsb0>().to_bitvec();
        ColourPaletteOptions {}
    }
}

impl From<ColourPaletteOptions> for u8 {
    fn from(_value: ColourPaletteOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.extend([false; 8]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScaledGraphicOptions {
    pub flashing: bool,
}

impl From<u8> for ScaledGraphicOptions {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Lsb0>().to_bitvec();
        ScaledGraphicOptions {
            flashing: bit_data.pop().unwrap(),
        }
    }
}

impl From<ScaledGraphicOptions> for u8 {
    fn from(value: ScaledGraphicOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.flashing);
        bit_data.extend([false; 7]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AuxiliaryFunctionType {
    /// 0 = Boolean (Latching), two-position switch (maintains position)
    /// On/Off with values 0,1
    BooleanLatching = 0,

    /// 1 = Analogue (maintains position setting)
    /// Value 1: 0–100% of position, Value 2: FFFF16 = reserved
    AnalogueMaintains = 1,

    /// 2 = Boolean (Non-Latching), two-position switch (return to off)
    /// Momentary On/Off with values 0,1,2 (0=Off, 1=On, 2=Held)
    BooleanNonLatching = 2,

    /// 3 = Analogue (return to 50%), two-way analogue return to center
    /// 0% to 100%, centered at 50%
    AnalogueReturnToCenter = 3,

    /// 4 = Analogue (return to 0%), one-way analogue return to zero
    /// 0% to 100%, returning to 0% position
    AnalogueReturnToZero = 4,

    /// 5 = Dual Boolean (Latching), three-position switch (latching)
    /// Maintains position with values 0,1,4
    DualBooleanLatching = 5,

    /// 6 = Dual Boolean (Non-Latching), three-position switch returning to center
    /// Values may include momentary and held states
    DualBooleanNonLatching = 6,

    /// 7 = Dual Boolean (Latching Up, Momentary Down), three-position switch
    /// Latching in one direction, momentary in the other
    DualBooleanLatchingUp = 7,

    /// 8 = Dual Boolean (Latching Down, Momentary Up), three-position switch
    /// Latching in one direction, momentary in the other
    DualBooleanLatchingDown = 8,

    /// 9 = Combined Analogue (return to 50%) with Dual Boolean Latching at endpoints
    /// Analogue axis returning to center with latchable endpoints
    CombinedAnalogueReturnWithLatch = 9,

    /// 10 = Combined Analogue (maintains position) with Dual Boolean Latching at endpoints
    /// Analogue axis maintains position with latchable endpoints
    CombinedAnalogueMaintainsWithLatch = 10,

    /// 11 = Quadrature Boolean (Non-Latching)
    /// Two axes of non-latching boolean states, represented as bit pairs
    QuadratureBooleanNonLatching = 11,

    /// 12 = Quadrature Analogue (maintains position)
    /// Two axes of analogue with full maintainable positions
    QuadratureAnalogueMaintains = 12,

    /// 13 = Quadrature Analogue (return to 50%)
    /// Two axes of analogue returning to center
    QuadratureAnalogueReturnToCenter = 13,

    /// 14 = Bidirectional Encoder
    /// Value increments/decrements as an encoder is turned
    BidirectionalEncoder = 14,

    /// 15–30 = Reserved
    Reserved(u8),

    /// 31 = Reserved for remove assignment command
    RemoveAssignment = 31,
}

impl From<u8> for AuxiliaryFunctionType {
    fn from(value: u8) -> Self {
        match value {
            0 => AuxiliaryFunctionType::BooleanLatching,
            1 => AuxiliaryFunctionType::AnalogueMaintains,
            2 => AuxiliaryFunctionType::BooleanNonLatching,
            3 => AuxiliaryFunctionType::AnalogueReturnToCenter,
            4 => AuxiliaryFunctionType::AnalogueReturnToZero,
            5 => AuxiliaryFunctionType::DualBooleanLatching,
            6 => AuxiliaryFunctionType::DualBooleanNonLatching,
            7 => AuxiliaryFunctionType::DualBooleanLatchingUp,
            8 => AuxiliaryFunctionType::DualBooleanLatchingDown,
            9 => AuxiliaryFunctionType::CombinedAnalogueReturnWithLatch,
            10 => AuxiliaryFunctionType::CombinedAnalogueMaintainsWithLatch,
            11 => AuxiliaryFunctionType::QuadratureBooleanNonLatching,
            12 => AuxiliaryFunctionType::QuadratureAnalogueMaintains,
            13 => AuxiliaryFunctionType::QuadratureAnalogueReturnToCenter,
            14 => AuxiliaryFunctionType::BidirectionalEncoder,
            31 => AuxiliaryFunctionType::RemoveAssignment,
            v if v > 14 && v < 31 => AuxiliaryFunctionType::Reserved(v),
            _ => panic!("Invalid function type"),
        }
    }
}

impl From<AuxiliaryFunctionType> for u8 {
    fn from(value: AuxiliaryFunctionType) -> Self {
        match value {
            AuxiliaryFunctionType::BooleanLatching => 0,
            AuxiliaryFunctionType::AnalogueMaintains => 1,
            AuxiliaryFunctionType::BooleanNonLatching => 2,
            AuxiliaryFunctionType::AnalogueReturnToCenter => 3,
            AuxiliaryFunctionType::AnalogueReturnToZero => 4,
            AuxiliaryFunctionType::DualBooleanLatching => 5,
            AuxiliaryFunctionType::DualBooleanNonLatching => 6,
            AuxiliaryFunctionType::DualBooleanLatchingUp => 7,
            AuxiliaryFunctionType::DualBooleanLatchingDown => 8,
            AuxiliaryFunctionType::CombinedAnalogueReturnWithLatch => 9,
            AuxiliaryFunctionType::CombinedAnalogueMaintainsWithLatch => 10,
            AuxiliaryFunctionType::QuadratureBooleanNonLatching => 11,
            AuxiliaryFunctionType::QuadratureAnalogueMaintains => 12,
            AuxiliaryFunctionType::QuadratureAnalogueReturnToCenter => 13,
            AuxiliaryFunctionType::BidirectionalEncoder => 14,
            AuxiliaryFunctionType::RemoveAssignment => 31,
            AuxiliaryFunctionType::Reserved(v) => v,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionAttributes {
    pub function_type: AuxiliaryFunctionType, // Bit 0-4
    pub critical: bool,                       // Bit 5
    pub restricted: bool, // Bit 6 for AuxiliaryFunctionType2, reserved for AuxiliaryInputType2
    pub single_assignment: bool, // Bit 7
}

impl From<u8> for FunctionAttributes {
    fn from(value: u8) -> Self {
        let mut bit_data = value.view_bits::<Lsb0>().to_bitvec();
        FunctionAttributes {
            function_type: bit_data[0..5].load::<u8>().into(),
            critical: bit_data.pop().unwrap(),
            restricted: bit_data.pop().unwrap(),
            single_assignment: bit_data.pop().unwrap(),
        }
    }
}

impl From<FunctionAttributes> for u8 {
    fn from(value: FunctionAttributes) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        let ft_val: u8 = value.function_type.into();
        let ft_bits = ft_val.view_bits::<Lsb0>()[0..5].iter().copied();
        bit_data.extend(ft_bits);
        bit_data.push(value.critical);
        bit_data.push(value.restricted);
        bit_data.push(value.single_assignment);
        bit_data.load::<u8>()
    }
}
