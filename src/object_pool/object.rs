use crate::network_management::name::NAME;
use crate::object_pool::object_attributes::{
    MacroRef, ObjectLabel, ObjectRef, Point, WindowMaskCellFormat, WindowMaskOptions, WindowType,
};
use crate::object_pool::object_id::ObjectId;
use crate::object_pool::{Colour, ObjectType};
use bitvec::field::BitField;
use bitvec::order::{Lsb0, Msb0};
use bitvec::vec::BitVec;
use bitvec::view::BitView;

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

#[derive(Debug, PartialEq)]
pub struct WorkingSet {
    pub id: ObjectId,
    pub background_colour: Colour,
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

impl From<InputListOptions> for u8 {
    fn from(value: InputListOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.enabled);
        bit_data.push(value.real_time_editing);
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

#[derive(Debug, Copy, Clone)]
pub enum ValidationType {
    ValidCharacters,
    InvalidCharacters,
}

impl From<ValidationType> for u8 {
    fn from(value: ValidationType) -> Self {
        match value {
            ValidationType::ValidCharacters => 0,
            ValidationType::InvalidCharacters => 1,
        }
    }
}

impl From<u8> for ValidationType {
    fn from(value: u8) -> Self {
        match value {
            0 => ValidationType::ValidCharacters,
            1 => ValidationType::InvalidCharacters,
            _ => panic!("Invalid validation type"),
        }
    }
}

#[derive(Debug)]
pub struct CharacterRange {
    pub first_character: u16,
    pub last_character: u16,
}

#[derive(Debug)]
pub struct CodePlane {
    pub number: u8,
    pub character_ranges: Vec<CharacterRange>,
}

#[derive(Debug)]
pub struct ExtendedInputAttributes {
    pub id: ObjectId,
    pub validation_type: ValidationType,
    pub code_planes: Vec<CodePlane>,
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

impl From<GraphicsContextOptions> for u8 {
    fn from(value: GraphicsContextOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.transparent);
        bit_data.push(value.color.into());
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

impl From<KeyGroupOptions> for u8 {
    fn from(value: KeyGroupOptions) -> u8 {
        let mut bit_data: BitVec<u8> = BitVec::new();
        bit_data.push(value.available);
        bit_data.push(value.transparent);
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