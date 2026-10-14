use crate::network_management::name::NAME;
use crate::object_pool::object_attributes::{
    Alignment, AnimationOptions, ButtonOptions, ColorFormat, ColourPaletteOptions,
    ExternalObjectDefinitionOptions, ExternalReferenceNameOptions, FormatType,
    GraphicsContextOptions, InputListOptions, InputNumberOptions, InputStringOptions,
    KeyGroupOptions, LineDirection, MacroRef, NumberOptions, ObjectLabel, ObjectRef,
    OutputArchedBarGraphOptions, OutputLinearBarGraphOptions, OutputMeterOptions,
    OutputStringOptions, PictureGraphicOptions, Point, ScaledGraphicOptions, WindowMaskCellFormat,
    WindowMaskOptions, WindowType,
};
use crate::object_pool::object_id::ObjectId;
use crate::object_pool::{Colour, ObjectType};
use serde::{Deserialize, Serialize};

use super::object_attributes::{
    DataCodeType, FontSize, FontStyle, FontType, FunctionAttributes, PictureGraphicFormat,
};
use super::object_id::NullableObjectId;

/// A string carried by an ISO 11783-6 object.
///
/// Imported object-pool strings retain their exact wire bytes in `Encoded` so
/// that reading and writing an object pool does not change its character
/// encoding. `Text` is used for strings created or edited by applications and
/// is exported as a BOM-prefixed UTF-16LE wide string.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VtString {
    /// Unicode text supplied by an editor or legacy JSON project.
    Text(String),
    /// Exact bytes read from an ISO 11783-6 object pool.
    Encoded { encoded_bytes: Vec<u8> },
}

impl Default for VtString {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl From<String> for VtString {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for VtString {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl VtString {
    /// Decode this string for presentation using its Font Attributes character set.
    /// BOM-prefixed UTF-16LE strings ignore `font_type`, as required by ISO 11783-6.
    /// Reserved and proprietary character sets return an explicit error.
    pub fn decode(
        &self,
        font_type: super::object_attributes::FontType,
    ) -> Result<String, &'static str> {
        match self {
            Self::Text(text) => Ok(text.clone()),
            Self::Encoded { encoded_bytes } => {
                if encoded_bytes.starts_with(&[0xFF, 0xFE]) {
                    let end = encoded_bytes.len() - (encoded_bytes.len() % 2);
                    let words = encoded_bytes[2..end]
                        .chunks_exact(2)
                        .map(|b| u16::from_le_bytes([b[0], b[1]]));
                    return Ok(char::decode_utf16(words)
                        .map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER))
                        .collect());
                }
                let map = match font_type {
                    super::object_attributes::FontType::Latin1 => &ISO_8859_1,
                    super::object_attributes::FontType::Latin9 => &ISO_8859_15,
                    super::object_attributes::FontType::Latin2 => &ISO_8859_2,
                    super::object_attributes::FontType::Latin4 => &ISO_8859_4,
                    super::object_attributes::FontType::Cyrillic => &ISO_8859_5,
                    super::object_attributes::FontType::Greek => &ISO_8859_7,
                    super::object_attributes::FontType::Reserved(_) => {
                        return Err("reserved ISO 11783-6 character set")
                    }
                    super::object_attributes::FontType::Proprietary(_) => {
                        return Err("proprietary ISO 11783-6 character set is unsupported")
                    }
                };
                Ok(encoded_bytes
                    .iter()
                    .map(|byte| {
                        let cp = if *byte < 0x80 {
                            *byte as u16
                        } else {
                            map[(*byte - 0x80) as usize]
                        };
                        char::from_u32(cp as u32).unwrap_or(char::REPLACEMENT_CHARACTER)
                    })
                    .collect())
            }
        }
    }

    /// Return the bytes to serialize on the wire. Imported values are returned
    /// verbatim; edited Unicode values become BOM-prefixed UTF-16LE.
    pub fn wire_bytes(&self) -> Vec<u8> {
        match self {
            Self::Encoded { encoded_bytes } => encoded_bytes.clone(),
            Self::Text(text) => {
                let mut bytes = vec![0xFF, 0xFE];
                bytes.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
                bytes
            }
        }
    }
}

// Unicode mappings for bytes 80..FF. Undefined code points use FFFD.
const ISO_8859_1: [u16; 128] = [
    0x0080, 0x0081, 0x0082, 0x0083, 0x0084, 0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008A, 0x008B,
    0x008C, 0x008D, 0x008E, 0x008F, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094, 0x0095, 0x0096, 0x0097,
    0x0098, 0x0099, 0x009A, 0x009B, 0x009C, 0x009D, 0x009E, 0x009F, 0x00A0, 0x00A1, 0x00A2, 0x00A3,
    0x00A4, 0x00A5, 0x00A6, 0x00A7, 0x00A8, 0x00A9, 0x00AA, 0x00AB, 0x00AC, 0x00AD, 0x00AE, 0x00AF,
    0x00B0, 0x00B1, 0x00B2, 0x00B3, 0x00B4, 0x00B5, 0x00B6, 0x00B7, 0x00B8, 0x00B9, 0x00BA, 0x00BB,
    0x00BC, 0x00BD, 0x00BE, 0x00BF, 0x00C0, 0x00C1, 0x00C2, 0x00C3, 0x00C4, 0x00C5, 0x00C6, 0x00C7,
    0x00C8, 0x00C9, 0x00CA, 0x00CB, 0x00CC, 0x00CD, 0x00CE, 0x00CF, 0x00D0, 0x00D1, 0x00D2, 0x00D3,
    0x00D4, 0x00D5, 0x00D6, 0x00D7, 0x00D8, 0x00D9, 0x00DA, 0x00DB, 0x00DC, 0x00DD, 0x00DE, 0x00DF,
    0x00E0, 0x00E1, 0x00E2, 0x00E3, 0x00E4, 0x00E5, 0x00E6, 0x00E7, 0x00E8, 0x00E9, 0x00EA, 0x00EB,
    0x00EC, 0x00ED, 0x00EE, 0x00EF, 0x00F0, 0x00F1, 0x00F2, 0x00F3, 0x00F4, 0x00F5, 0x00F6, 0x00F7,
    0x00F8, 0x00F9, 0x00FA, 0x00FB, 0x00FC, 0x00FD, 0x00FE, 0x00FF,
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "properties")]
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
    OutputList(OutputList),
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
            Object::OutputList(o) => o.id,
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

    pub fn mut_id(&mut self) -> &mut ObjectId {
        match self {
            Object::WorkingSet(o) => &mut o.id,
            Object::DataMask(o) => &mut o.id,
            Object::AlarmMask(o) => &mut o.id,
            Object::Container(o) => &mut o.id,
            Object::SoftKeyMask(o) => &mut o.id,
            Object::Key(o) => &mut o.id,
            Object::Button(o) => &mut o.id,
            Object::InputBoolean(o) => &mut o.id,
            Object::InputString(o) => &mut o.id,
            Object::InputNumber(o) => &mut o.id,
            Object::InputList(o) => &mut o.id,
            Object::OutputString(o) => &mut o.id,
            Object::OutputNumber(o) => &mut o.id,
            Object::OutputList(o) => &mut o.id,
            Object::OutputLine(o) => &mut o.id,
            Object::OutputRectangle(o) => &mut o.id,
            Object::OutputEllipse(o) => &mut o.id,
            Object::OutputPolygon(o) => &mut o.id,
            Object::OutputMeter(o) => &mut o.id,
            Object::OutputLinearBarGraph(o) => &mut o.id,
            Object::OutputArchedBarGraph(o) => &mut o.id,
            Object::PictureGraphic(o) => &mut o.id,
            Object::NumberVariable(o) => &mut o.id,
            Object::StringVariable(o) => &mut o.id,
            Object::FontAttributes(o) => &mut o.id,
            Object::LineAttributes(o) => &mut o.id,
            Object::FillAttributes(o) => &mut o.id,
            Object::InputAttributes(o) => &mut o.id,
            Object::ObjectPointer(o) => &mut o.id,
            Object::Macro(o) => &mut o.id,
            Object::AuxiliaryFunctionType1(o) => &mut o.id,
            Object::AuxiliaryInputType1(o) => &mut o.id,
            Object::AuxiliaryFunctionType2(o) => &mut o.id,
            Object::AuxiliaryInputType2(o) => &mut o.id,
            Object::AuxiliaryControlDesignatorType2(o) => &mut o.id,
            Object::WindowMask(o) => &mut o.id,
            Object::KeyGroup(o) => &mut o.id,
            Object::GraphicsContext(o) => &mut o.id,
            Object::ExtendedInputAttributes(o) => &mut o.id,
            Object::ColourMap(o) => &mut o.id,
            Object::ObjectLabelReferenceList(o) => &mut o.id,
            Object::ExternalObjectDefinition(o) => &mut o.id,
            Object::ExternalReferenceName(o) => &mut o.id,
            Object::ExternalObjectPointer(o) => &mut o.id,
            Object::Animation(o) => &mut o.id,
            Object::ColourPalette(o) => &mut o.id,
            Object::GraphicData(o) => &mut o.id,
            Object::WorkingSetSpecialControls(o) => &mut o.id,
            Object::ScaledGraphic(o) => &mut o.id,
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
            Object::OutputList(_) => ObjectType::OutputList,
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

    ///
    /// Returns a list of object IDs referenced by this object in the order they are listed
    /// as attributes in the object specification.
    ///
    /// # Examples
    /// We can use this method recursively to process objects in a "depth-first" manner
    /// as required by the ISO 11783-6 (ch. 4.6.1.3) standard:
    /// ```
    /// use ag_iso_stack::object_pool::ObjectId;
    /// use ag_iso_stack::object_pool::ObjectPool;
    ///
    /// fn process_object(object_pool: &ObjectPool, object_id: ObjectId) {
    ///     let object = object_pool.object_by_id(object_id).unwrap();
    ///     for referenced_object_id in object.referenced_objects() {
    ///         process_object(object_pool, referenced_object_id);
    ///     }
    ///     // Process the object here
    /// }
    ///
    /// let working_set = object_pool.working_set_object().unwrap();
    /// process_object(&object_pool, working_set.id);
    ///
    pub fn referenced_objects(&self) -> Vec<ObjectId> {
        let mut refs: Vec<ObjectId> = vec![];

        fn extend_object_refs(refs: &mut Vec<ObjectId>, object_refs: &[ObjectRef]) {
            refs.extend(object_refs.iter().map(|x| x.id));
        }

        fn push_nullable_id(refs: &mut Vec<ObjectId>, nullable_id: &NullableObjectId) {
            if let Some(id) = nullable_id.0 {
                refs.push(id);
            }
        }

        fn extend_nullable_ids(refs: &mut Vec<ObjectId>, nullable_ids: &[NullableObjectId]) {
            refs.extend(nullable_ids.iter().filter_map(|x| x.0));
        }

        match self {
            Object::WorkingSet(o) => {
                refs.push(o.active_mask);
                extend_object_refs(&mut refs, &o.object_refs);
            }
            Object::DataMask(o) => {
                push_nullable_id(&mut refs, &o.soft_key_mask);
                extend_object_refs(&mut refs, &o.object_refs);
            }
            Object::AlarmMask(o) => {
                push_nullable_id(&mut refs, &o.soft_key_mask);
                extend_object_refs(&mut refs, &o.object_refs);
            }
            Object::Container(o) => extend_object_refs(&mut refs, &o.object_refs),
            Object::SoftKeyMask(o) => refs.extend(&o.objects),
            Object::Key(o) => extend_object_refs(&mut refs, &o.object_refs),
            Object::Button(o) => extend_object_refs(&mut refs, &o.object_refs),
            Object::InputBoolean(o) => {
                push_nullable_id(&mut refs, &o.foreground_colour);
                push_nullable_id(&mut refs, &o.variable_reference);
            }
            Object::InputString(o) => {
                push_nullable_id(&mut refs, &o.font_attributes);
                push_nullable_id(&mut refs, &o.input_attributes);
                push_nullable_id(&mut refs, &o.variable_reference);
            }
            Object::InputNumber(o) => {
                push_nullable_id(&mut refs, &o.font_attributes);
                push_nullable_id(&mut refs, &o.variable_reference);
            }
            Object::InputList(o) => {
                push_nullable_id(&mut refs, &o.variable_reference);
                extend_nullable_ids(&mut refs, &o.list_items);
            }
            Object::OutputString(o) => {
                push_nullable_id(&mut refs, &o.font_attributes);
                push_nullable_id(&mut refs, &o.variable_reference);
            }
            Object::OutputNumber(o) => {
                push_nullable_id(&mut refs, &o.font_attributes);
                push_nullable_id(&mut refs, &o.variable_reference);
            }
            Object::OutputList(o) => {
                push_nullable_id(&mut refs, &o.variable_reference);
                extend_nullable_ids(&mut refs, &o.list_items);
            }
            Object::OutputLine(o) => push_nullable_id(&mut refs, &o.line_attributes),
            Object::OutputRectangle(o) => {
                push_nullable_id(&mut refs, &o.line_attributes);
                push_nullable_id(&mut refs, &o.fill_attributes);
            }
            Object::OutputEllipse(o) => {
                push_nullable_id(&mut refs, &o.line_attributes);
                push_nullable_id(&mut refs, &o.fill_attributes);
            }
            Object::OutputPolygon(o) => {
                push_nullable_id(&mut refs, &o.line_attributes);
                push_nullable_id(&mut refs, &o.fill_attributes);
            }
            Object::OutputMeter(o) => push_nullable_id(&mut refs, &o.variable_reference),
            Object::OutputLinearBarGraph(o) => {
                push_nullable_id(&mut refs, &o.variable_reference);
                push_nullable_id(&mut refs, &o.target_value_variable_reference);
            }
            Object::OutputArchedBarGraph(o) => {
                push_nullable_id(&mut refs, &o.variable_reference);
                push_nullable_id(&mut refs, &o.target_value_variable_reference);
            }
            Object::PictureGraphic(_)
            | Object::NumberVariable(_)
            | Object::StringVariable(_)
            | Object::FontAttributes(_)
            | Object::LineAttributes(_) => (), // No references
            Object::FillAttributes(o) => push_nullable_id(&mut refs, &o.fill_pattern),
            Object::InputAttributes(_) => (), // No references
            Object::ObjectPointer(o) => push_nullable_id(&mut refs, &o.value),
            Object::Macro(_)
            | Object::AuxiliaryFunctionType1(_)
            | Object::AuxiliaryInputType1(_)
            | Object::AuxiliaryFunctionType2(_)
            | Object::AuxiliaryInputType2(_) => (), // No references
            Object::AuxiliaryControlDesignatorType2(o) => {
                push_nullable_id(&mut refs, &o.auxiliary_object_id)
            }
            Object::WindowMask(o) => {
                push_nullable_id(&mut refs, &o.window_title);
                push_nullable_id(&mut refs, &o.window_icon);
                extend_nullable_ids(&mut refs, &o.objects);
                extend_object_refs(&mut refs, &o.object_refs);
            }
            Object::KeyGroup(o) => {
                refs.push(o.name);
                push_nullable_id(&mut refs, &o.key_group_icon);
                refs.extend(&o.objects);
            }
            Object::GraphicsContext(o) => {
                push_nullable_id(&mut refs, &o.font_attributes_object);
                push_nullable_id(&mut refs, &o.line_attributes_object);
                push_nullable_id(&mut refs, &o.fill_attributes_object);
            }
            Object::ExtendedInputAttributes(_) | Object::ColourMap(_) => (), // No references
            Object::ObjectLabelReferenceList(o) => {
                for label in o.object_labels.as_slice() {
                    refs.push(label.id);
                    push_nullable_id(&mut refs, &label.string_variable_reference);
                    push_nullable_id(&mut refs, &label.graphic_representation);
                }
            }
            Object::ExternalObjectDefinition(o) => extend_nullable_ids(&mut refs, &o.objects),
            Object::ExternalReferenceName(_) => (), // No references
            Object::ExternalObjectPointer(o) => {
                push_nullable_id(&mut refs, &o.default_object_id);
                push_nullable_id(&mut refs, &o.external_reference_name_id);
                push_nullable_id(&mut refs, &o.external_object_id);
            }
            Object::Animation(o) => extend_object_refs(&mut refs, &o.object_refs),

            Object::ColourPalette(_) | Object::GraphicData(_) => (), // No references
            Object::WorkingSetSpecialControls(o) => {
                push_nullable_id(&mut refs, &o.id_of_colour_map);
                push_nullable_id(&mut refs, &o.id_of_colour_palette);
            }
            Object::ScaledGraphic(o) => push_nullable_id(&mut refs, &o.value),
        };

        refs
    }

    pub fn as_sized_object(&self) -> Option<&dyn SizedObject> {
        match self {
            Object::Container(o) => Some(o),
            Object::Button(o) => Some(o),
            Object::InputBoolean(o) => Some(o),
            Object::InputString(o) => Some(o),
            Object::InputNumber(o) => Some(o),
            Object::InputList(o) => Some(o),
            Object::OutputString(o) => Some(o),
            Object::OutputNumber(o) => Some(o),
            Object::OutputList(o) => Some(o),
            Object::OutputLine(o) => Some(o),
            Object::OutputRectangle(o) => Some(o),
            Object::OutputEllipse(o) => Some(o),
            Object::OutputPolygon(o) => Some(o),
            Object::OutputMeter(o) => Some(o),
            Object::OutputLinearBarGraph(o) => Some(o),
            Object::OutputArchedBarGraph(o) => Some(o),
            Object::PictureGraphic(o) => Some(o),
            Object::ScaledGraphic(o) => Some(o),
            _ => None,
        }
    }
}

pub trait SizedObject {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WorkingSet {
    pub id: ObjectId,
    pub background_colour: u8,
    pub selectable: bool,
    pub active_mask: ObjectId,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
    pub language_codes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DataMask {
    pub id: ObjectId,
    pub background_colour: u8,
    pub soft_key_mask: NullableObjectId,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AlarmMask {
    pub id: ObjectId,
    pub background_colour: u8,
    pub soft_key_mask: NullableObjectId,
    pub priority: u8,
    pub acoustic_signal: u8,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub hidden: bool,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for Container {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SoftKeyMask {
    pub id: ObjectId,
    pub background_colour: u8,
    pub objects: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Key {
    pub id: ObjectId,
    pub background_colour: u8,
    pub key_code: u8,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

impl SizedObject for Button {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InputBoolean {
    pub id: ObjectId,
    pub background_colour: u8,
    pub width: u16,
    pub foreground_colour: NullableObjectId,
    pub variable_reference: NullableObjectId,
    /// Raw value byte. VT4 and later allow the full 8-bit range.
    #[serde(deserialize_with = "deserialize_input_boolean_value")]
    pub value: u8,
    pub enabled: bool,
    pub macro_refs: Vec<MacroRef>,
}

fn deserialize_input_boolean_value<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum LegacyOrRawValue {
        LegacyBool(bool),
        RawByte(u8),
    }

    match LegacyOrRawValue::deserialize(deserializer)? {
        LegacyOrRawValue::LegacyBool(value) => Ok(u8::from(value)),
        LegacyOrRawValue::RawByte(value) => Ok(value),
    }
}

impl SizedObject for InputBoolean {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.width
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InputString {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: NullableObjectId,
    pub input_attributes: NullableObjectId,
    pub options: InputStringOptions,
    pub variable_reference: NullableObjectId,
    pub justification: Alignment,
    pub value: VtString,
    pub enabled: bool,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for InputString {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputNumber {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: NullableObjectId,
    pub options: NumberOptions,
    pub variable_reference: NullableObjectId,
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

impl SizedObject for InputNumber {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InputList {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub variable_reference: NullableObjectId,
    pub value: u8,
    pub options: InputListOptions,
    pub list_items: Vec<NullableObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for InputList {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputString {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: NullableObjectId,
    pub options: OutputStringOptions,
    pub variable_reference: NullableObjectId,
    pub justification: Alignment,
    pub value: VtString,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputString {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputNumber {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub background_colour: u8,
    pub font_attributes: NullableObjectId,
    pub options: NumberOptions,
    pub variable_reference: NullableObjectId,
    pub value: u32,
    pub offset: i32,
    pub scale: f32,
    pub nr_of_decimals: u8,
    pub format: FormatType,
    pub justification: Alignment,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputNumber {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputList {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub variable_reference: NullableObjectId,
    pub value: u8,
    pub list_items: Vec<NullableObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputList {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputLine {
    pub id: ObjectId,
    pub line_attributes: NullableObjectId,
    pub width: u16,
    pub height: u16,
    pub line_direction: LineDirection,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputLine {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputRectangle {
    pub id: ObjectId,
    pub line_attributes: NullableObjectId,
    pub width: u16,
    pub height: u16,
    pub line_suppression: u8,
    pub fill_attributes: NullableObjectId,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputRectangle {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputEllipse {
    pub id: ObjectId,
    pub line_attributes: NullableObjectId,
    pub width: u16,
    pub height: u16,
    pub ellipse_type: u8,
    pub start_angle: u8,
    pub end_angle: u8,
    pub fill_attributes: NullableObjectId,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputEllipse {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputPolygon {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub line_attributes: NullableObjectId,
    pub fill_attributes: NullableObjectId,
    pub polygon_type: u8,
    pub points: Vec<Point<u16>>,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputPolygon {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputMeter {
    pub id: ObjectId,
    pub width: u16,
    pub needle_colour: u8,
    pub border_colour: u8,
    pub arc_and_tick_colour: u8,
    pub options: OutputMeterOptions,
    pub nr_of_ticks: u8,
    pub start_angle: u8,
    pub end_angle: u8,
    pub min_value: u16,
    pub max_value: u16,
    pub variable_reference: NullableObjectId,
    pub value: u16,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputMeter {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.width
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputLinearBarGraph {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub colour: u8,
    pub target_line_colour: u8,
    pub options: OutputLinearBarGraphOptions,
    pub nr_of_ticks: u8,
    pub min_value: u16,
    pub max_value: u16,
    pub variable_reference: NullableObjectId,
    pub value: u16,
    pub target_value_variable_reference: NullableObjectId,
    pub target_value: u16,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputLinearBarGraph {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputArchedBarGraph {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub colour: u8,
    pub target_line_colour: u8,
    pub options: OutputArchedBarGraphOptions,
    pub start_angle: u8,
    pub end_angle: u8,
    pub bar_graph_width: u16,
    pub min_value: u16,
    pub max_value: u16,
    pub variable_reference: NullableObjectId,
    pub value: u16,
    pub target_value_variable_reference: NullableObjectId,
    pub target_value: u16,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for OutputArchedBarGraph {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PictureGraphic {
    pub id: ObjectId,
    pub width: u16,
    pub actual_width: u16,
    pub actual_height: u16,
    pub format: PictureGraphicFormat,
    pub options: PictureGraphicOptions,
    pub transparency_colour: u8,
    pub data: Vec<u8>,
    pub macro_refs: Vec<MacroRef>,
}

impl PictureGraphic {
    pub fn data_as_raw_encoded(&self) -> Vec<u8> {
        match self.options.data_code_type {
            DataCodeType::Raw => self.data.clone(),
            DataCodeType::RunLength => {
                let mut raw_data = vec![];
                let mut i = 0;
                while i < self.data.len() {
                    let count = self.data[i] as usize;
                    let value = self.data[i + 1];
                    raw_data.extend(vec![value; count]);
                    i += 2;
                }
                raw_data
            }
        }
    }

    pub fn data_as_run_length_encoded(&self) -> Vec<u8> {
        match self.options.data_code_type {
            DataCodeType::Raw => {
                let mut run_length_data = vec![];
                let mut i = 0;
                while i < self.data.len() {
                    let value = self.data[i];
                    let count = self.data[i..].iter().take_while(|&&x| x == value).count();
                    run_length_data.push(count as u8);
                    run_length_data.push(value);
                    i += count;
                }
                run_length_data
            }
            DataCodeType::RunLength => self.data.clone(),
        }
    }
}

impl SizedObject for PictureGraphic {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        (self.actual_height as f32 * (self.width as f32 / self.actual_width as f32)) as u16
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberVariable {
    pub id: ObjectId,
    pub value: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StringVariable {
    pub id: ObjectId,
    pub value: VtString,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FontAttributes {
    pub id: ObjectId,
    pub font_colour: u8,
    pub font_size: FontSize,
    pub font_type: FontType,
    pub font_style: FontStyle,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LineAttributes {
    pub id: ObjectId,
    pub line_colour: u8,
    pub line_width: u8,
    pub line_art: u16,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FillAttributes {
    pub id: ObjectId,
    pub fill_type: u8,
    pub fill_colour: u8,
    pub fill_pattern: NullableObjectId,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InputAttributes {
    pub id: ObjectId,
    pub validation_type: ValidationType,
    pub validation_string: String,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CharacterRange {
    pub first_character: u16,
    pub last_character: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CodePlane {
    pub number: u8,
    pub character_ranges: Vec<CharacterRange>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExtendedInputAttributes {
    pub id: ObjectId,
    pub validation_type: ValidationType,
    pub code_planes: Vec<CodePlane>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ObjectPointer {
    pub id: ObjectId,
    pub value: NullableObjectId,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub id: ObjectId,
    pub commands: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuxiliaryFunctionType1 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_type: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuxiliaryInputType1 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_type: u8,
    pub input_id: u8,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuxiliaryFunctionType2 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_attributes: FunctionAttributes,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuxiliaryInputType2 {
    pub id: ObjectId,
    pub background_colour: u8,
    pub function_attributes: FunctionAttributes,
    pub object_refs: Vec<ObjectRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuxiliaryControlDesignatorType2 {
    pub id: ObjectId,
    pub pointer_type: u8,
    pub auxiliary_object_id: NullableObjectId,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ColourMap {
    pub id: ObjectId,
    pub colour_map: Vec<u8>,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
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
    pub font_attributes_object: NullableObjectId,
    pub line_attributes_object: NullableObjectId,
    pub fill_attributes_object: NullableObjectId,
    pub format: ColorFormat,
    pub options: GraphicsContextOptions,
    pub transparency_colour: u8,
}

impl SizedObject for GraphicsContext {
    fn width(&self) -> u16 {
        self.viewport_width
    }

    fn height(&self) -> u16 {
        self.viewport_height
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowMask {
    pub id: ObjectId,
    pub cell_format: WindowMaskCellFormat,
    pub window_type: WindowType,
    pub background_colour: u8,
    pub options: WindowMaskOptions,
    pub name: ObjectId,
    pub window_title: NullableObjectId,
    pub window_icon: NullableObjectId,
    pub objects: Vec<NullableObjectId>,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyGroup {
    pub id: ObjectId,
    pub options: KeyGroupOptions,
    pub name: ObjectId,
    pub key_group_icon: NullableObjectId,
    pub objects: Vec<ObjectId>,
    pub macro_refs: Vec<MacroRef>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ObjectLabelReferenceList {
    pub id: ObjectId,
    pub object_labels: Vec<ObjectLabel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalObjectDefinition {
    pub id: ObjectId,
    pub options: ExternalObjectDefinitionOptions,
    pub name: NAME,
    pub objects: Vec<NullableObjectId>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalReferenceName {
    pub id: ObjectId,
    pub options: ExternalReferenceNameOptions,
    pub name: NAME,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalObjectPointer {
    pub id: ObjectId,
    pub default_object_id: NullableObjectId,
    pub external_reference_name_id: NullableObjectId,
    pub external_object_id: NullableObjectId,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    pub options: AnimationOptions,
    pub object_refs: Vec<ObjectRef>,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for Animation {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ColourPalette {
    pub id: ObjectId,
    pub options: ColourPaletteOptions,
    pub colours: Vec<Colour>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GraphicData {
    pub id: ObjectId,
    pub format: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ScaledGraphic {
    pub id: ObjectId,
    pub width: u16,
    pub height: u16,
    pub scale_type: u8,
    pub options: ScaledGraphicOptions,
    pub value: NullableObjectId,
    pub macro_refs: Vec<MacroRef>,
}

impl SizedObject for ScaledGraphic {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WorkingSetSpecialControls {
    pub id: ObjectId,
    pub id_of_colour_map: NullableObjectId,
    pub id_of_colour_palette: NullableObjectId,
    pub language_pairs: Vec<(String, String)>,
}

const ISO_8859_15: [u16; 128] = [
    0x0080, 0x0081, 0x0082, 0x0083, 0x0084, 0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008A, 0x008B,
    0x008C, 0x008D, 0x008E, 0x008F, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094, 0x0095, 0x0096, 0x0097,
    0x0098, 0x0099, 0x009A, 0x009B, 0x009C, 0x009D, 0x009E, 0x009F, 0x00A0, 0x00A1, 0x00A2, 0x00A3,
    0x20AC, 0x00A5, 0x0160, 0x00A7, 0x0161, 0x00A9, 0x00AA, 0x00AB, 0x00AC, 0x00AD, 0x00AE, 0x00AF,
    0x00B0, 0x00B1, 0x00B2, 0x00B3, 0x017D, 0x00B5, 0x00B6, 0x00B7, 0x017E, 0x00B9, 0x00BA, 0x00BB,
    0x0152, 0x0153, 0x0178, 0x00BF, 0x00C0, 0x00C1, 0x00C2, 0x00C3, 0x00C4, 0x00C5, 0x00C6, 0x00C7,
    0x00C8, 0x00C9, 0x00CA, 0x00CB, 0x00CC, 0x00CD, 0x00CE, 0x00CF, 0x00D0, 0x00D1, 0x00D2, 0x00D3,
    0x00D4, 0x00D5, 0x00D6, 0x00D7, 0x00D8, 0x00D9, 0x00DA, 0x00DB, 0x00DC, 0x00DD, 0x00DE, 0x00DF,
    0x00E0, 0x00E1, 0x00E2, 0x00E3, 0x00E4, 0x00E5, 0x00E6, 0x00E7, 0x00E8, 0x00E9, 0x00EA, 0x00EB,
    0x00EC, 0x00ED, 0x00EE, 0x00EF, 0x00F0, 0x00F1, 0x00F2, 0x00F3, 0x00F4, 0x00F5, 0x00F6, 0x00F7,
    0x00F8, 0x00F9, 0x00FA, 0x00FB, 0x00FC, 0x00FD, 0x00FE, 0x00FF,
];

const ISO_8859_2: [u16; 128] = [
    0x0080, 0x0081, 0x0082, 0x0083, 0x0084, 0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008A, 0x008B,
    0x008C, 0x008D, 0x008E, 0x008F, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094, 0x0095, 0x0096, 0x0097,
    0x0098, 0x0099, 0x009A, 0x009B, 0x009C, 0x009D, 0x009E, 0x009F, 0x00A0, 0x0104, 0x02D8, 0x0141,
    0x00A4, 0x013D, 0x015A, 0x00A7, 0x00A8, 0x0160, 0x015E, 0x0164, 0x0179, 0x00AD, 0x017D, 0x017B,
    0x00B0, 0x0105, 0x02DB, 0x0142, 0x00B4, 0x013E, 0x015B, 0x02C7, 0x00B8, 0x0161, 0x015F, 0x0165,
    0x017A, 0x02DD, 0x017E, 0x017C, 0x0154, 0x00C1, 0x00C2, 0x0102, 0x00C4, 0x0139, 0x0106, 0x00C7,
    0x010C, 0x00C9, 0x0118, 0x00CB, 0x011A, 0x00CD, 0x00CE, 0x010E, 0x0110, 0x0143, 0x0147, 0x00D3,
    0x00D4, 0x0150, 0x00D6, 0x00D7, 0x0158, 0x016E, 0x00DA, 0x0170, 0x00DC, 0x00DD, 0x0162, 0x00DF,
    0x0155, 0x00E1, 0x00E2, 0x0103, 0x00E4, 0x013A, 0x0107, 0x00E7, 0x010D, 0x00E9, 0x0119, 0x00EB,
    0x011B, 0x00ED, 0x00EE, 0x010F, 0x0111, 0x0144, 0x0148, 0x00F3, 0x00F4, 0x0151, 0x00F6, 0x00F7,
    0x0159, 0x016F, 0x00FA, 0x0171, 0x00FC, 0x00FD, 0x0163, 0x02D9,
];

const ISO_8859_4: [u16; 128] = [
    0x0080, 0x0081, 0x0082, 0x0083, 0x0084, 0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008A, 0x008B,
    0x008C, 0x008D, 0x008E, 0x008F, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094, 0x0095, 0x0096, 0x0097,
    0x0098, 0x0099, 0x009A, 0x009B, 0x009C, 0x009D, 0x009E, 0x009F, 0x00A0, 0x0104, 0x0138, 0x0156,
    0x00A4, 0x0128, 0x013B, 0x00A7, 0x00A8, 0x0160, 0x0112, 0x0122, 0x0166, 0x00AD, 0x017D, 0x00AF,
    0x00B0, 0x0105, 0x02DB, 0x0157, 0x00B4, 0x0129, 0x013C, 0x02C7, 0x00B8, 0x0161, 0x0113, 0x0123,
    0x0167, 0x014A, 0x017E, 0x014B, 0x0100, 0x00C1, 0x00C2, 0x00C3, 0x00C4, 0x00C5, 0x00C6, 0x012E,
    0x010C, 0x00C9, 0x0118, 0x00CB, 0x0116, 0x00CD, 0x00CE, 0x012A, 0x0110, 0x0145, 0x014C, 0x0136,
    0x00D4, 0x00D5, 0x00D6, 0x00D7, 0x00D8, 0x0172, 0x00DA, 0x00DB, 0x00DC, 0x0168, 0x016A, 0x00DF,
    0x0101, 0x00E1, 0x00E2, 0x00E3, 0x00E4, 0x00E5, 0x00E6, 0x012F, 0x010D, 0x00E9, 0x0119, 0x00EB,
    0x0117, 0x00ED, 0x00EE, 0x012B, 0x0111, 0x0146, 0x014D, 0x0137, 0x00F4, 0x00F5, 0x00F6, 0x00F7,
    0x00F8, 0x0173, 0x00FA, 0x00FB, 0x00FC, 0x0169, 0x016B, 0x02D9,
];

const ISO_8859_5: [u16; 128] = [
    0x0080, 0x0081, 0x0082, 0x0083, 0x0084, 0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008A, 0x008B,
    0x008C, 0x008D, 0x008E, 0x008F, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094, 0x0095, 0x0096, 0x0097,
    0x0098, 0x0099, 0x009A, 0x009B, 0x009C, 0x009D, 0x009E, 0x009F, 0x00A0, 0x0401, 0x0402, 0x0403,
    0x0404, 0x0405, 0x0406, 0x0407, 0x0408, 0x0409, 0x040A, 0x040B, 0x040C, 0x00AD, 0x040E, 0x040F,
    0x0410, 0x0411, 0x0412, 0x0413, 0x0414, 0x0415, 0x0416, 0x0417, 0x0418, 0x0419, 0x041A, 0x041B,
    0x041C, 0x041D, 0x041E, 0x041F, 0x0420, 0x0421, 0x0422, 0x0423, 0x0424, 0x0425, 0x0426, 0x0427,
    0x0428, 0x0429, 0x042A, 0x042B, 0x042C, 0x042D, 0x042E, 0x042F, 0x0430, 0x0431, 0x0432, 0x0433,
    0x0434, 0x0435, 0x0436, 0x0437, 0x0438, 0x0439, 0x043A, 0x043B, 0x043C, 0x043D, 0x043E, 0x043F,
    0x0440, 0x0441, 0x0442, 0x0443, 0x0444, 0x0445, 0x0446, 0x0447, 0x0448, 0x0449, 0x044A, 0x044B,
    0x044C, 0x044D, 0x044E, 0x044F, 0x2116, 0x0451, 0x0452, 0x0453, 0x0454, 0x0455, 0x0456, 0x0457,
    0x0458, 0x0459, 0x045A, 0x045B, 0x045C, 0x00A7, 0x045E, 0x045F,
];

const ISO_8859_7: [u16; 128] = [
    0x0080, 0x0081, 0x0082, 0x0083, 0x0084, 0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008A, 0x008B,
    0x008C, 0x008D, 0x008E, 0x008F, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094, 0x0095, 0x0096, 0x0097,
    0x0098, 0x0099, 0x009A, 0x009B, 0x009C, 0x009D, 0x009E, 0x009F, 0x00A0, 0x2018, 0x2019, 0x00A3,
    0x20AC, 0x20AF, 0x00A6, 0x00A7, 0x00A8, 0x00A9, 0x037A, 0x00AB, 0x00AC, 0x00AD, 0xFFFD, 0x2015,
    0x00B0, 0x00B1, 0x00B2, 0x00B3, 0x0384, 0x0385, 0x0386, 0x00B7, 0x0388, 0x0389, 0x038A, 0x00BB,
    0x038C, 0x00BD, 0x038E, 0x038F, 0x0390, 0x0391, 0x0392, 0x0393, 0x0394, 0x0395, 0x0396, 0x0397,
    0x0398, 0x0399, 0x039A, 0x039B, 0x039C, 0x039D, 0x039E, 0x039F, 0x03A0, 0x03A1, 0xFFFD, 0x03A3,
    0x03A4, 0x03A5, 0x03A6, 0x03A7, 0x03A8, 0x03A9, 0x03AA, 0x03AB, 0x03AC, 0x03AD, 0x03AE, 0x03AF,
    0x03B0, 0x03B1, 0x03B2, 0x03B3, 0x03B4, 0x03B5, 0x03B6, 0x03B7, 0x03B8, 0x03B9, 0x03BA, 0x03BB,
    0x03BC, 0x03BD, 0x03BE, 0x03BF, 0x03C0, 0x03C1, 0x03C2, 0x03C3, 0x03C4, 0x03C5, 0x03C6, 0x03C7,
    0x03C8, 0x03C9, 0x03CA, 0x03CB, 0x03CC, 0x03CD, 0x03CE, 0xFFFD,
];

#[cfg(test)]
mod vt_string_tests {
    use super::super::object_attributes::{FontType, NonProportionalFontSize};
    use super::*;

    #[test]
    fn imported_strings_decode_and_keep_their_original_bytes() {
        let latin9 = VtString::Encoded {
            encoded_bytes: vec![0xA4, 0xA6],
        };
        assert_eq!(latin9.decode(FontType::Latin9).unwrap(), "€Š");
        assert_eq!(latin9.wire_bytes(), vec![0xA4, 0xA6]);

        let wide = VtString::Encoded {
            encoded_bytes: vec![0xFF, 0xFE, 0x3D, 0xD8, 0x00, 0xDE, 0x61],
        };
        assert_eq!(wide.decode(FontType::Reserved(3)).unwrap(), "😀");
        assert_eq!(
            wide.wire_bytes(),
            vec![0xFF, 0xFE, 0x3D, 0xD8, 0x00, 0xDE, 0x61]
        );
    }

    #[test]
    fn edited_and_legacy_json_strings_are_unicode() {
        let legacy: VtString = serde_json::from_str("\"hello\"").unwrap();
        assert_eq!(legacy, VtString::Text("hello".to_owned()));
        assert_eq!(
            legacy.wire_bytes(),
            vec![0xFF, 0xFE, b'h', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0]
        );
        let unsupported = VtString::Encoded {
            encoded_bytes: vec![0x41],
        };
        assert!(unsupported.decode(FontType::Reserved(3)).is_err());

        let too_long = Object::StringVariable(StringVariable {
            id: ObjectId::new(1).unwrap(),
            value: VtString::Text("x".repeat(32_767)),
        });
        assert!(too_long.try_write().is_err());
        let mut pool = crate::object_pool::ObjectPool::new();
        pool.add(too_long);
        assert!(pool.try_as_iop().is_err());
    }

    #[test]
    fn font_size_uses_proportional_style_bit_as_the_mode() {
        // Font Attributes: ID, type, colour, size, font type, style, macro count.
        let proportional_wire = [0x01, 0x00, 23, 2, 12, 0, 0x80, 0];
        let mut proportional = Object::read(&mut proportional_wire.into_iter()).unwrap();
        assert!(matches!(
            proportional,
            Object::FontAttributes(FontAttributes {
                font_size: FontSize::Proportional(12),
                ..
            })
        ));
        if let Object::FontAttributes(attributes) = &mut proportional {
            attributes.font_style.proportional = false;
        }
        assert_eq!(proportional.write(), proportional_wire);

        let fixed_wire = [0x01, 0x00, 23, 2, 3, 0, 0, 0];
        let fixed = Object::read(&mut fixed_wire.into_iter()).unwrap();
        assert!(matches!(
            fixed,
            Object::FontAttributes(FontAttributes {
                font_size: FontSize::NonProportional(NonProportionalFontSize::Px12x16),
                ..
            })
        ));
        assert_eq!(fixed.write(), fixed_wire);
    }
}
