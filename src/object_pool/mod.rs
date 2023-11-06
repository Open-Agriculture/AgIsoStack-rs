pub mod reader;
pub mod writer;

mod object_pool;

use crate::network_management::name::NAME;
use bitvec::field::BitField;
use bitvec::order::{Lsb0, Msb0};
use bitvec::vec::BitVec;
use bitvec::view::BitView;
pub use object_pool::ObjectPool;
use strum_macros::FromRepr;

pub enum ParseError {
    DataEmpty,
    UnknownObjectType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    WorkingSet = 0,
    DataMask = 1,
    AlarmMask = 2,
    Container = 3,
    SoftKeyMask = 4,
    Key = 5,
    Button = 6,
    InputBoolean = 7,
    InputString = 8,
    InputNumber = 9,
    InputList = 10,
    OutputString = 11,
    OutputNumber = 12,
    OutputLine = 13,
    OutputRectangle = 14,
    OutputEllipse = 15,
    OutputPolygon = 16,
    OutputMeter = 17,
    OutputLinearBarGraph = 18,
    OutputArchedBarGraph = 19,
    PictureGraphic = 20,
    NumberVariable = 21,
    StringVariable = 22,
    FontAttributes = 23,
    LineAttributes = 24,
    FillAttributes = 25,
    InputAttributes = 26,
    ObjectPointer = 27,
    Macro = 28,
    AuxiliaryFunctionType1 = 29,
    AuxiliaryInputType1 = 30,
    AuxiliaryFunctionType2 = 31,
    AuxiliaryInputType2 = 32,
    AuxiliaryControlDesignatorType2 = 33,
    WindowMask = 34,
    KeyGroup = 35,
    GraphicsContext = 36,
    OutputList = 37,
    ExtendedInputAttributes = 38,
    ColourMap = 39,
    ObjectLabelReferenceList = 40,
    ExternalObjectDefinition = 41,
    ExternalReferenceName = 42,
    ExternalObjectPointer = 43,
    Animation = 44,
    ColourPalette = 45,
    GraphicData = 46,
    WorkingSetSpecialControls = 47,
    ScaledGraphic = 48,
}

impl TryFrom<u8> for ObjectType {
    type Error = ParseError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::WorkingSet),
            1 => Ok(Self::DataMask),
            2 => Ok(Self::AlarmMask),
            3 => Ok(Self::Container),
            4 => Ok(Self::SoftKeyMask),
            5 => Ok(Self::Key),
            6 => Ok(Self::Button),
            7 => Ok(Self::InputBoolean),
            8 => Ok(Self::InputString),
            9 => Ok(Self::InputNumber),
            10 => Ok(Self::InputList),
            11 => Ok(Self::OutputString),
            12 => Ok(Self::OutputNumber),
            13 => Ok(Self::OutputLine),
            14 => Ok(Self::OutputRectangle),
            15 => Ok(Self::OutputEllipse),
            16 => Ok(Self::OutputPolygon),
            17 => Ok(Self::OutputMeter),
            18 => Ok(Self::OutputLinearBarGraph),
            19 => Ok(Self::OutputArchedBarGraph),
            20 => Ok(Self::PictureGraphic),
            21 => Ok(Self::NumberVariable),
            22 => Ok(Self::StringVariable),
            23 => Ok(Self::FontAttributes),
            24 => Ok(Self::LineAttributes),
            25 => Ok(Self::FillAttributes),
            26 => Ok(Self::InputAttributes),
            27 => Ok(Self::ObjectPointer),
            28 => Ok(Self::Macro),
            29 => Ok(Self::AuxiliaryFunctionType1),
            30 => Ok(Self::AuxiliaryInputType1),
            31 => Ok(Self::AuxiliaryFunctionType2),
            32 => Ok(Self::AuxiliaryInputType2),
            33 => Ok(Self::AuxiliaryControlDesignatorType2),
            34 => Ok(Self::WindowMask),
            35 => Ok(Self::KeyGroup),
            36 => Ok(Self::GraphicsContext),
            37 => Ok(Self::OutputList),
            38 => Ok(Self::ExtendedInputAttributes),
            39 => Ok(Self::ColourMap),
            40 => Ok(Self::ObjectLabelReferenceList),
            41 => Ok(Self::ExternalObjectDefinition),
            42 => Ok(Self::ExternalReferenceName),
            43 => Ok(Self::ExternalObjectPointer),
            44 => Ok(Self::Animation),
            45 => Ok(Self::ColourPalette),
            46 => Ok(Self::GraphicData),
            47 => Ok(Self::WorkingSetSpecialControls),
            48 => Ok(Self::ScaledGraphic),
            _ => Err(ParseError::UnknownObjectType),
        }
    }
}

impl From<ObjectType> for u8 {
    fn from(val: ObjectType) -> Self {
        match val {
            ObjectType::WorkingSet => 0,
            ObjectType::DataMask => 1,
            ObjectType::AlarmMask => 2,
            ObjectType::Container => 3,
            ObjectType::SoftKeyMask => 4,
            ObjectType::Key => 5,
            ObjectType::Button => 6,
            ObjectType::InputBoolean => 7,
            ObjectType::InputString => 8,
            ObjectType::InputNumber => 9,
            ObjectType::InputList => 10,
            ObjectType::OutputString => 11,
            ObjectType::OutputNumber => 12,
            ObjectType::OutputLine => 13,
            ObjectType::OutputRectangle => 14,
            ObjectType::OutputEllipse => 15,
            ObjectType::OutputPolygon => 16,
            ObjectType::OutputMeter => 17,
            ObjectType::OutputLinearBarGraph => 18,
            ObjectType::OutputArchedBarGraph => 19,
            ObjectType::PictureGraphic => 20,
            ObjectType::NumberVariable => 21,
            ObjectType::StringVariable => 22,
            ObjectType::FontAttributes => 23,
            ObjectType::LineAttributes => 24,
            ObjectType::FillAttributes => 25,
            ObjectType::InputAttributes => 26,
            ObjectType::ObjectPointer => 27,
            ObjectType::Macro => 28,
            ObjectType::AuxiliaryFunctionType1 => 29,
            ObjectType::AuxiliaryInputType1 => 30,
            ObjectType::AuxiliaryFunctionType2 => 31,
            ObjectType::AuxiliaryInputType2 => 32,
            ObjectType::AuxiliaryControlDesignatorType2 => 33,
            ObjectType::WindowMask => 34,
            ObjectType::KeyGroup => 35,
            ObjectType::GraphicsContext => 36,
            ObjectType::OutputList => 37,
            ObjectType::ExtendedInputAttributes => 38,
            ObjectType::ColourMap => 39,
            ObjectType::ObjectLabelReferenceList => 40,
            ObjectType::ExternalObjectDefinition => 41,
            ObjectType::ExternalReferenceName => 42,
            ObjectType::ExternalObjectPointer => 43,
            ObjectType::Animation => 44,
            ObjectType::ColourPalette => 45,
            ObjectType::GraphicData => 46,
            ObjectType::WorkingSetSpecialControls => 47,
            ObjectType::ScaledGraphic => 48,
        }
    }
}

#[derive(Debug, Default)]
pub enum VtVersion {
    Version0,
    Version1,
    Version2,
    #[default]
    Version3,
    Version4,
    Version5,
    Version6,
}

impl From<VtVersion> for u8 {
    fn from(vt_version: VtVersion) -> Self {
        match vt_version {
            VtVersion::Version0 => 0,
            VtVersion::Version1 => 1,
            VtVersion::Version2 => 2,
            VtVersion::Version3 => 3,
            VtVersion::Version4 => 4,
            VtVersion::Version5 => 5,
            VtVersion::Version6 => 6,
        }
    }
}

impl TryFrom<u8> for VtVersion {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VtVersion::Version0),
            1 => Ok(VtVersion::Version1),
            2 => Ok(VtVersion::Version2),
            3 => Ok(VtVersion::Version3),
            4 => Ok(VtVersion::Version4),
            5 => Ok(VtVersion::Version5),
            6 => Ok(VtVersion::Version6),
            _ => Err(ParseError::UnknownObjectType),
        }
    }
}

#[derive(Debug)]
pub enum Object {
    WorkingSet(WorkingSet),
    DataMask(DataMask),
    AlarmMask(AlarmMask),
    Container(Container),
    SoftKeyMask(SoftKeyMask),
    Key(Key),
    Button(Button),
    InputBoolean(InputBoolean),
    InputString(InputString),
    InputNumber(InputNumber),
    InputList(InputList),
    OutputString(OutputString),
    OutputNumber(OutputNumber),
    OutputLine(OutputLine),
    OutputRectangle(OutputRectangle),
    OutputEllipse(OutputEllipse),
    OutputPolygon(OutputPolygon),
    OutputMeter(OutputMeter),
    OutputLinearBarGraph(OutputLinearBarGraph),
    OutputArchedBarGraph(OutputArchedBarGraph),
    PictureGraphic(PictureGraphic),
    NumberVariable(NumberVariable),
    StringVariable(StringVariable),
    FontAttributes(FontAttributes),
    LineAttributes(LineAttributes),
    FillAttributes(FillAttributes),
    InputAttributes(InputAttributes),
    ObjectPointer(ObjectPointer),
    Macro(Macro),
    AuxiliaryFunctionType1(AuxiliaryFunctionType1),
    AuxiliaryInputType1(AuxiliaryInputType1),
    AuxiliaryFunctionType2(AuxiliaryFunctionType2),
    AuxiliaryInputType2(AuxiliaryInputType2),
    AuxiliaryControlDesignatorType2(AuxiliaryControlDesignatorType2),
    WindowMask(WindowMask),
    KeyGroup(KeyGroup),
    GraphicsContext(GraphicsContext),
    OutputList(OutputList),
    ExtendedInputAttributes(ExtendedInputAttributes),
    ColourMap(ColourMap),
    ObjectLabelReferenceList(ObjectLabelReferenceList),
    ExternalObjectDefinition(ExternalObjectDefinition),
    ExternalReferenceName(ExternalReferenceName),
    ExternalObjectPointer(ExternalObjectPointer),
    Animation(Animation),
    ColourPalette(ColourPalette),
    GraphicData(GraphicData),
    WorkingSetSpecialControls(WorkingSetSpecialControls),
    ScaledGraphic(ScaledGraphic),
}

impl Object {
    pub fn id(&self) -> ObjectId {
        match self {
            Object::WorkingSet(o) => o.id,
            Object::DataMask(o) => o.id,
            Object::AlarmMask(o) => o.id,
            Object::Container(o) => o.id,
            Object::SoftKeyMask(o) => o.id,
            Object::Key(o) => o.id,
            Object::Button(o) => o.id,
            Object::InputBoolean(o) => o.id,
            Object::InputString(o) => o.id,
            Object::InputNumber(o) => o.id,
            Object::InputList(o) => o.id,
            Object::OutputString(o) => o.id,
            Object::OutputNumber(o) => o.id,
            Object::OutputLine(o) => o.id,
            Object::OutputRectangle(o) => o.id,
            Object::OutputEllipse(o) => o.id,
            Object::OutputPolygon(o) => o.id,
            Object::OutputMeter(o) => o.id,
            Object::OutputLinearBarGraph(o) => o.id,
            Object::OutputArchedBarGraph(o) => o.id,
            Object::PictureGraphic(o) => o.id,
            Object::NumberVariable(o) => o.id,
            Object::StringVariable(o) => o.id,
            Object::FontAttributes(o) => o.id,
            Object::LineAttributes(o) => o.id,
            Object::FillAttributes(o) => o.id,
            Object::InputAttributes(o) => o.id,
            Object::ObjectPointer(o) => o.id,
            Object::Macro(o) => o.id,
            Object::AuxiliaryFunctionType1(o) => o.id,
            Object::AuxiliaryInputType1(o) => o.id,
            Object::AuxiliaryFunctionType2(o) => o.id,
            Object::AuxiliaryInputType2(o) => o.id,
            Object::AuxiliaryControlDesignatorType2(o) => o.id,
            Object::WindowMask(o) => o.id,
            Object::KeyGroup(o) => o.id,
            Object::GraphicsContext(o) => o.id,
            Object::OutputList(o) => o.id,
            Object::ExtendedInputAttributes(o) => o.id,
            Object::ColourMap(o) => o.id,
            Object::ObjectLabelReferenceList(o) => o.id,
            Object::ExternalObjectDefinition(o) => o.id,
            Object::ExternalReferenceName(o) => o.id,
            Object::ExternalObjectPointer(o) => o.id,
            Object::Animation(o) => o.id,
            Object::ColourPalette(o) => o.id,
            Object::GraphicData(o) => o.id,
            Object::WorkingSetSpecialControls(o) => o.id,
            Object::ScaledGraphic(o) => o.id,
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::WorkingSet(_) => ObjectType::WorkingSet,
            Object::DataMask(_) => ObjectType::DataMask,
            Object::AlarmMask(_) => ObjectType::AlarmMask,
            Object::Container(_) => ObjectType::Container,
            Object::SoftKeyMask(_) => ObjectType::SoftKeyMask,
            Object::Key(_) => ObjectType::Key,
            Object::Button(_) => ObjectType::Button,
            Object::InputBoolean(_) => ObjectType::InputBoolean,
            Object::InputString(_) => ObjectType::InputString,
            Object::InputNumber(_) => ObjectType::InputNumber,
            Object::InputList(_) => ObjectType::InputList,
            Object::OutputString(_) => ObjectType::OutputString,
            Object::OutputNumber(_) => ObjectType::OutputNumber,
            Object::OutputLine(_) => ObjectType::OutputLine,
            Object::OutputRectangle(_) => ObjectType::OutputRectangle,
            Object::OutputEllipse(_) => ObjectType::OutputEllipse,
            Object::OutputPolygon(_) => ObjectType::OutputPolygon,
            Object::OutputMeter(_) => ObjectType::OutputMeter,
            Object::OutputLinearBarGraph(_) => ObjectType::OutputLinearBarGraph,
            Object::OutputArchedBarGraph(_) => ObjectType::OutputArchedBarGraph,
            Object::PictureGraphic(_) => ObjectType::PictureGraphic,
            Object::NumberVariable(_) => ObjectType::NumberVariable,
            Object::StringVariable(_) => ObjectType::StringVariable,
            Object::FontAttributes(_) => ObjectType::FontAttributes,
            Object::LineAttributes(_) => ObjectType::LineAttributes,
            Object::FillAttributes(_) => ObjectType::FillAttributes,
            Object::InputAttributes(_) => ObjectType::InputAttributes,
            Object::ObjectPointer(_) => ObjectType::ObjectPointer,
            Object::Macro(_) => ObjectType::Macro,
            Object::AuxiliaryFunctionType1(_) => ObjectType::AuxiliaryFunctionType1,
            Object::AuxiliaryInputType1(_) => ObjectType::AuxiliaryInputType1,
            Object::AuxiliaryFunctionType2(_) => ObjectType::AuxiliaryFunctionType2,
            Object::AuxiliaryInputType2(_) => ObjectType::AuxiliaryInputType2,
            Object::AuxiliaryControlDesignatorType2(_) => {
                ObjectType::AuxiliaryControlDesignatorType2
            }
            Object::WindowMask(_) => ObjectType::WindowMask,
            Object::KeyGroup(_) => ObjectType::KeyGroup,
            Object::GraphicsContext(_) => ObjectType::GraphicsContext,
            Object::OutputList(_) => ObjectType::OutputList,
            Object::ExtendedInputAttributes(_) => ObjectType::ExtendedInputAttributes,
            Object::ColourMap(_) => ObjectType::ColourMap,
            Object::ObjectLabelReferenceList(_) => ObjectType::ObjectLabelReferenceList,
            Object::ExternalObjectDefinition(_) => ObjectType::ExternalObjectDefinition,
            Object::ExternalReferenceName(_) => ObjectType::ExternalReferenceName,
            Object::ExternalObjectPointer(_) => ObjectType::ExternalObjectPointer,
            Object::Animation(_) => ObjectType::Animation,
            Object::ColourPalette(_) => ObjectType::ColourPalette,
            Object::GraphicData(_) => ObjectType::GraphicData,
            Object::WorkingSetSpecialControls(_) => ObjectType::WorkingSetSpecialControls,
            Object::ScaledGraphic(_) => ObjectType::ScaledGraphic,
        }
    }
}

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
    HorizontalLinearBargraph1x1 = 7,
    SingleButton1x1 = 8,
    DoubleButton1x1 = 9,
    NumericOutputValueWithUnits2x1 = 10,
    NumericOutputValueNoUnits2x1 = 11,
    StringOutputValue2x1 = 12,
    NumericInputValueWithUnits2x1 = 13,
    NumericInputValueNoUnits2x1 = 14,
    StringInputValue2x1 = 15,
    HorizontalLinearBargraph2x1 = 16,
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

    const fn size(self) -> Point<u8> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectId(u16);
impl ObjectId {
    pub const NULL: ObjectId = ObjectId(0xFFFF);
}
impl Default for ObjectId {
    fn default() -> Self {
        Self::NULL
    }
}
impl From<u16> for ObjectId {
    fn from(val: u16) -> Self {
        ObjectId(val)
    }
}
impl From<ObjectId> for u16 {
    fn from(val: ObjectId) -> Self {
        val.0
    }
}
impl From<[u8; 2]> for ObjectId {
    fn from(val: [u8; 2]) -> Self {
        ObjectId(u16::from_le_bytes(val))
    }
}
impl From<ObjectId> for [u8; 2] {
    fn from(val: ObjectId) -> Self {
        val.0.to_le_bytes()
    }
}
// impl From<Vec<u8>> for ObjectId {
//     fn from(val: Vec<u8>) -> Self {
//         let val: ObjectId = val.as_slice().into();
//         val
//     }
// }
// impl From<ObjectId> for Vec<u8> {
//     fn from(val: ObjectId) -> Self {
//         let val: [u8;2] = val.into();
//         val.to_vec()
//     }
// }
impl From<&[u8]> for ObjectId {
    fn from(val: &[u8]) -> Self {
        match val.len() {
            2.. => ObjectId(u16::from_le_bytes([val[0], val[1]])),
            _ => ObjectId::NULL,
        }
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

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn as_rgb(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    pub fn as_rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub const BLACK: Colour = Colour::COLOUR_PALETTE[0];
    pub const WHITE: Colour = Colour::COLOUR_PALETTE[1];
    pub const GREEN: Colour = Colour::COLOUR_PALETTE[2];
    pub const TEAL: Colour = Colour::COLOUR_PALETTE[3];
    pub const MAROON: Colour = Colour::COLOUR_PALETTE[4];
    pub const PURPLE: Colour = Colour::COLOUR_PALETTE[5];
    pub const OLIVE: Colour = Colour::COLOUR_PALETTE[6];
    pub const SILVER: Colour = Colour::COLOUR_PALETTE[7];
    pub const GREY: Colour = Colour::COLOUR_PALETTE[8];
    pub const BLUE: Colour = Colour::COLOUR_PALETTE[9];
    pub const LIME: Colour = Colour::COLOUR_PALETTE[10];
    pub const CYAN: Colour = Colour::COLOUR_PALETTE[11];
    pub const RED: Colour = Colour::COLOUR_PALETTE[12];
    pub const MAGENTA: Colour = Colour::COLOUR_PALETTE[13];
    pub const YELLOW: Colour = Colour::COLOUR_PALETTE[14];
    pub const NAVY: Colour = Colour::COLOUR_PALETTE[15];

    #[rustfmt::skip] // Skip formatting the lines
    pub const COLOUR_PALETTE: [Colour; 256] = [
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0x00, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0x33, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0x66, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0x99, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0xCC, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0x00, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0x33, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0x66, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0x99, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0xCC, b: 0xFF, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x00, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x33, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x66, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0x99, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0xCC, a: 0xFF },
        Colour { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
        Colour { r: 0x00, g: 0x00, b: 0x00, a: 0xFF },
    ];
}

impl Default for Colour {
    fn default() -> Self {
        Colour::GREY
    }
}

impl From<u32> for Colour {
    fn from(val: u32) -> Self {
        let b = val.to_le_bytes();
        Colour {
            r: b[0],
            g: b[1],
            b: b[2],
            a: b[3],
        }
    }
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

#[derive(Debug, PartialEq)]
pub struct WorkingSet {
    pub id: ObjectId,
    pub background_colour: u8,
    pub selectable: bool,
    pub active_mask: ObjectId,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
    pub language_codes: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct DataMask {
    pub id: ObjectId,
    pub background_colour: u8,
    pub soft_key_mask: ObjectId,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq)]
pub struct AlarmMask {
    pub id: ObjectId,
    pub background_colour: u8,
    pub soft_key_mask: ObjectId,
    pub priority: u8,
    pub acoustic_signal: u8,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq)]
pub struct Container {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub hidden: bool,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq)]
pub struct SoftKeyMask {
    pub id: ObjectId,
    pub background_colour: u8,
    pub objects: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq)]
pub struct Key {
    pub id: ObjectId,
    pub background_colour: u8,
    pub key_code: u8,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq)]
pub struct Button {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub border_colour: u8,
    pub key_code: u8,
    pub options: ButtonOptions,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ButtonState {
    RELEASED,
    LATCHED,
}

impl From<ButtonState> for bool {
    fn from(value: ButtonState) -> Self {
        match value {
            ButtonState::RELEASED => false,
            ButtonState::LATCHED => true,
        }
    }
}

impl From<bool> for ButtonState {
    fn from(value: bool) -> Self {
        match value {
            false => ButtonState::RELEASED,
            true => ButtonState::LATCHED,
        }
    }
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

#[derive(Debug, PartialEq)]
pub struct InputBoolean {
    pub id: ObjectId,
    pub background_colour: u8,
    pub width: u16,
    pub foreground_colour: ObjectId,
    pub variable_reference: ObjectId,
    pub value: bool,
    pub enabled: bool,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq)]
pub struct InputString {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: ObjectId,
    pub input_attributes: ObjectId,
    pub options: InputStringOptions,
    pub variable_reference: ObjectId,
    pub justification: Alignment,
    pub value: String,
    pub enabled: bool,
    pub macro_refs: Vec<MacroRef>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct InputNumber {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: ObjectId,
    pub options: NumberOptions,
    pub variable_reference: ObjectId,
    pub value: u32,
    pub min_value: u32,
    pub max_value: u32,
    pub offset: i32,
    pub scale: f32,
    pub nr_of_decimals: u8,
    pub format: FormatType,
    pub justification: Alignment,
    pub options2: InputNumberOptions,
    pub macro_refs: Vec<MacroRef>,
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

impl Into<u8> for InputListOptions {
    fn into(self) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(self.enabled);
        bit_data.push(self.real_time_editing);
        bit_data.extend([0; 6]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputList {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub variable_reference: ObjectId,
    pub value: u8,
    pub options: InputListOptions,
    pub list_items: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
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

impl Into<u8> for OutputStringOptions {
    fn into(self) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(self.transparent);
        bit_data.push(self.auto_wrap);
        bit_data.push(self.wrap_on_hyphen);
        bit_data.extend([0; 5]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutputString {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: ObjectId,
    pub options: OutputStringOptions,
    pub variable_reference: ObjectId,
    pub justification: Alignment,
    pub value: String,
    pub macro_refs: Vec<MacroRef>,
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

impl Into<u8> for NumberOptions {
    fn into(self) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(self.transparent);
        bit_data.push(self.display_leading_zeros);
        bit_data.push(self.display_zero_as_blank);
        bit_data.push(self.truncate);
        bit_data.extend([0; 5]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, PartialEq)]
pub struct OutputNumber {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: ObjectId,
    pub options: NumberOptions,
    pub variable_reference: ObjectId,
    pub value: u32,
    pub offset: i32,
    pub scale: f32,
    pub nr_of_decimals: u8,
    pub format: FormatType,
    pub justification: Alignment,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutputList {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub variable_reference: ObjectId,
    pub value: u8,
    pub list_items: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct OutputLine {
    pub id: ObjectId,
    pub line_attributes: ObjectId,
    pub width: u16,
    pub height: u16,
    pub line_direction: LineDirection,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct OutputRectangle {
    pub id: ObjectId,
    pub line_attributes: ObjectId,
    pub width: u16,
    pub height: u16,
    pub line_suppression: u8,
    pub fill_attributes: ObjectId,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct OutputEllipse {
    pub id: ObjectId,
    pub line_attributes: ObjectId,
    pub width: u16,
    pub height: u16,
    pub ellipse_type: u8,
    pub start_angle: u8,
    pub end_angle: u8,
    pub fill_attributes: ObjectId,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct OutputPolygon {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub line_attributes: ObjectId,
    pub fill_attributes: ObjectId,
    pub polygon_type: u8,
    pub points: Vec<Point<u16>>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct OutputMeter {
    pub id: ObjectId,
    pub width: u16,
    pub needle_colour: u8,
    pub border_colour: u8,
    pub arc_and_tick_colour: u8,
    pub options: u8,
    pub nr_of_ticks: u8,
    pub start_angle: u8,
    pub end_angle: u8,
    pub min_value: u16,
    pub max_value: u16,
    pub variable_reference: ObjectId,
    pub value: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct OutputLinearBarGraph {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub colour: u8,
    pub target_line_colour: u8,
    pub options: u8,
    pub nr_of_ticks: u8,
    pub min_value: u16,
    pub max_value: u16,
    pub variable_reference: ObjectId,
    pub value: u16,
    pub target_value_variable_reference: ObjectId,
    pub target_value: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct OutputArchedBarGraph {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub colour: u8,
    pub target_line_colour: u8,
    pub options: u8,
    pub start_angle: u8,
    pub end_angle: u8,
    pub bar_graph_width: u16,
    pub min_value: u16,
    pub max_value: u16,
    pub variable_reference: ObjectId,
    pub value: u16,
    pub target_value_variable_reference: ObjectId,
    pub target_value: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct PictureGraphic {
    pub id: ObjectId,
    pub width: u16,
    pub actual_width: u16,
    pub actual_height: u16,
    pub format: u8,
    pub options: u8,
    pub transparency_colour: u8,
    pub data: Vec<u8>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct NumberVariable {
    pub id: ObjectId,
    pub value: u32,
}

#[derive(Debug)]
pub struct StringVariable {
    pub id: ObjectId,
    pub value: String,
}

#[derive(Debug)]
pub struct FontAttributes {
    pub id: ObjectId,
    pub font_colour: u8,
    pub font_size: u8,
    pub font_type: u8,
    pub font_style: u8,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct LineAttributes {
    pub id: ObjectId,
    pub line_colour: u8,
    pub line_width: u8,
    pub line_art: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct FillAttributes {
    pub id: ObjectId,
    pub fill_type: u8,
    pub fill_colour: u8,
    pub fill_pattern: ObjectId,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct InputAttributes {
    pub id: ObjectId,
    pub validation_type: u8,
    pub validation_string: String,
    pub macro_refs: Vec<MacroRef>,
}

// TODO; Implement code planes
#[derive(Debug)]
pub struct ExtendedInputAttributes {
    pub id: ObjectId,
    pub validation_type: u8,
    pub nr_of_code_planes: u8,
}

#[derive(Debug)]
pub struct ObjectPointer {
    pub id: ObjectId,
    pub value: ObjectId,
}

#[derive(Debug)]
pub struct Macro {
    pub id: ObjectId,
    pub commands: Vec<u8>,
}

#[derive(Debug)]
pub struct AuxiliaryFunctionType1 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_type: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Debug)]
pub struct AuxiliaryInputType1 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_type: u8,
    pub input_id: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Debug)]
pub struct AuxiliaryFunctionType2 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_attributes: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Debug)]
pub struct AuxiliaryInputType2 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_attributes: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Debug)]
pub struct AuxiliaryControlDesignatorType2 {
    pub id: ObjectId,
    pub pointer_type: u8,
    pub auxiliary_object_id: ObjectId,
}

#[derive(Debug)]
pub struct ColourMap {
    pub id: ObjectId,
    pub colour_map: Vec<u8>,
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

impl Into<u8> for GraphicsContextOptions {
    fn into(self) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(self.transparent);
        bit_data.push(self.color.into());
        bit_data.extend([0; 6]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GraphicsContext {
    pub id: ObjectId,
    pub viewport_width: u16,
    pub viewport_height: u16,
    pub viewport_x: i16,
    pub viewport_y: i16,
    pub canvas_width: u16,
    pub canvas_height: u16,
    pub viewport_zoom: f32,
    pub graphics_cursor_x: i16,
    pub graphics_cursor_y: i16,
    pub foreground_colour: u8,
    pub background_colour: u8,
    pub font_attributes_object: ObjectId,
    pub line_attributes_object: ObjectId,
    pub fill_attributes_object: ObjectId,
    pub format: ColorFormat,
    pub options: GraphicsContextOptions,
    pub transparency_colour: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowMask {
    pub id: ObjectId,
    pub cell_format: WindowMaskCellFormat,
    pub window_type: WindowType,
    pub background_colour: u8,
    pub options: WindowMaskOptions,
    pub name: ObjectId,
    pub window_title: ObjectId,
    pub window_icon: ObjectId,
    pub objects: Vec<ObjectId>,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
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

impl Into<u8> for KeyGroupOptions {
    fn into(self) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(self.available);
        bit_data.push(self.transparent);
        bit_data.extend([0; 6]);
        bit_data.load::<u8>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyGroup {
    pub id: ObjectId,
    pub options: KeyGroupOptions,
    pub name: ObjectId,
    pub key_group_icon: ObjectId,
    pub objects: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct ObjectLabelReferenceList {
    pub id: ObjectId,
    pub object_labels: Vec<ObjectLabel>,
}

#[derive(Debug)]
pub struct ExternalObjectDefinition {
    pub id: ObjectId,
    pub options: u8,
    pub name: NAME,
    pub objects: Vec<ObjectId>,
}

#[derive(Debug)]
pub struct ExternalReferenceName {
    pub id: ObjectId,
    pub options: u8,
    pub name: NAME,
}

#[derive(Debug)]
pub struct ExternalObjectPointer {
    pub id: ObjectId,
    pub default_object_id: ObjectId,
    pub external_reference_name_id: ObjectId,
    pub external_object_id: ObjectId,
}

#[derive(Debug)]
pub struct Animation {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub refresh_interval: u16,
    pub value: u8,
    pub enabled: bool,
    pub first_child_index: u8,
    pub last_child_index: u8,
    pub default_child_index: u8,
    pub options: u8,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct ColourPalette {
    pub id: ObjectId,
    pub options: u16,
    pub colours: Vec<Colour>,
}

#[derive(Debug)]
pub struct GraphicData {
    pub id: ObjectId,
    pub format: u8,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct ScaledGraphic {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub scale_type: u8,
    pub options: u8,
    pub value: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug)]
pub struct WorkingSetSpecialControls {
    pub id: ObjectId,
    pub id_of_colour_map: ObjectId,
    pub id_of_colour_palette: ObjectId,
    pub language_pairs: Vec<(String, String)>,
}
