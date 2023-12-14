use super::object_id::NullableObjectId;
use super::*;
use crate::object_pool::colour::Colour;
use crate::object_pool::object::{
    AlarmMask, Animation, AuxiliaryControlDesignatorType2, AuxiliaryFunctionType1,
    AuxiliaryFunctionType2, AuxiliaryInputType1, AuxiliaryInputType2, Button, CharacterRange,
    CodePlane, ColourMap, ColourPalette, Container, DataMask, ExtendedInputAttributes,
    ExternalObjectDefinition, ExternalObjectPointer, ExternalReferenceName, FillAttributes,
    FontAttributes, GraphicData, GraphicsContext, InputAttributes, InputBoolean, InputList,
    InputNumber, InputString, Key, KeyGroup, LineAttributes, Macro, NumberVariable, Object,
    ObjectLabelReferenceList, ObjectPointer, OutputArchedBarGraph, OutputEllipse, OutputLine,
    OutputLinearBarGraph, OutputList, OutputMeter, OutputNumber, OutputPolygon, OutputRectangle,
    OutputString, PictureGraphic, ScaledGraphic, SoftKeyMask, StringVariable, WindowMask,
    WorkingSet, WorkingSetSpecialControls,
};
use crate::object_pool::object_attributes::{MacroRef, ObjectLabel, ObjectRef, Point};
use crate::object_pool::object_id::ObjectId;

impl Object {
    pub fn write(&self) -> Vec<u8> {
        let mut data = Vec::new();

        match self {
            Object::WorkingSet(o) => Self::write_working_set(&mut data, o),
            Object::DataMask(o) => Self::write_data_mask(&mut data, o),
            Object::AlarmMask(o) => Self::write_alarm_mask(&mut data, o),
            Object::Container(o) => Self::write_container(&mut data, o),
            Object::SoftKeyMask(o) => Self::write_soft_key_mask(&mut data, o),
            Object::Key(o) => Self::write_key(&mut data, o),
            Object::Button(o) => Self::write_button(&mut data, o),
            Object::InputBoolean(o) => Self::write_input_boolean(&mut data, o),
            Object::InputString(o) => Self::write_input_string(&mut data, o),
            Object::InputNumber(o) => Self::write_input_number(&mut data, o),
            Object::InputList(o) => Self::write_input_list(&mut data, o),
            Object::OutputString(o) => Self::write_output_string(&mut data, o),
            Object::OutputNumber(o) => Self::write_output_number(&mut data, o),
            Object::OutputLine(o) => Self::write_output_line(&mut data, o),
            Object::OutputRectangle(o) => Self::write_output_rectangle(&mut data, o),
            Object::OutputEllipse(o) => Self::write_output_ellipse(&mut data, o),
            Object::OutputPolygon(o) => Self::write_output_polygon(&mut data, o),
            Object::OutputMeter(o) => Self::write_output_meter(&mut data, o),
            Object::OutputLinearBarGraph(o) => Self::write_output_linear_bar_graph(&mut data, o),
            Object::OutputArchedBarGraph(o) => Self::write_output_arched_bar_graph(&mut data, o),
            Object::PictureGraphic(o) => Self::write_picture_graphic(&mut data, o),
            Object::NumberVariable(o) => Self::write_number_variable(&mut data, o),
            Object::StringVariable(o) => Self::write_string_variable(&mut data, o),
            Object::FontAttributes(o) => Self::write_font_attributes(&mut data, o),
            Object::LineAttributes(o) => Self::write_line_attributes(&mut data, o),
            Object::FillAttributes(o) => Self::write_fill_attributes(&mut data, o),
            Object::InputAttributes(o) => Self::write_input_attributes(&mut data, o),
            Object::ObjectPointer(o) => Self::write_object_pointer(&mut data, o),
            Object::Macro(o) => Self::write_macro(&mut data, o),
            Object::AuxiliaryFunctionType1(o) => Self::write_auxiliary_function_type1(&mut data, o),
            Object::AuxiliaryInputType1(o) => Self::write_auxiliary_input_type1(&mut data, o),
            Object::AuxiliaryFunctionType2(o) => Self::write_auxiliary_function_type2(&mut data, o),
            Object::AuxiliaryInputType2(o) => Self::write_auxiliary_input_type2(&mut data, o),
            Object::AuxiliaryControlDesignatorType2(o) => {
                Self::write_auxiliary_control_designator_type2(&mut data, o)
            }
            Object::WindowMask(o) => Self::write_window_mask(&mut data, o),
            Object::KeyGroup(o) => Self::write_key_group(&mut data, o),
            Object::GraphicsContext(o) => Self::write_graphics_context(&mut data, o),
            Object::OutputList(o) => Self::write_output_list(&mut data, o),
            Object::ExtendedInputAttributes(o) => {
                Self::write_extended_input_attributes(&mut data, o)
            }
            Object::ColourMap(o) => Self::write_colour_map(&mut data, o),
            Object::ObjectLabelReferenceList(o) => {
                Self::write_object_label_reference_list(&mut data, o)
            }
            Object::ExternalObjectDefinition(o) => {
                Self::write_external_object_definition(&mut data, o)
            }
            Object::ExternalReferenceName(o) => Self::write_external_reference_name(&mut data, o),
            Object::ExternalObjectPointer(o) => Self::write_external_object_pointer(&mut data, o),
            Object::Animation(o) => Self::write_animation(&mut data, o),
            Object::ColourPalette(o) => Self::write_colour_palette(&mut data, o),
            Object::GraphicData(o) => Self::write_graphic_data(&mut data, o),
            Object::WorkingSetSpecialControls(o) => {
                Self::write_working_set_special_controls(&mut data, o)
            }
            Object::ScaledGraphic(o) => Self::write_scaled_graphic(&mut data, o),
        }

        data
    }

    fn write_object_ids(data: &mut Vec<u8>, objects: &Vec<ObjectId>) {
        for d in objects {
            Self::write_u16(data, *d);
        }
    }

    fn write_nullable_object_ids(data: &mut Vec<u8>, objects: &Vec<NullableObjectId>) {
        for d in objects {
            Self::write_u16(data, *d);
        }
    }

    fn write_object_refs(data: &mut Vec<u8>, object_refs: &Vec<ObjectRef>) {
        for d in object_refs {
            Self::write_u16(data, d.id);
            Self::write_i16(data, d.offset.x);
            Self::write_i16(data, d.offset.y);
        }
    }
    fn write_macro_refs(data: &mut Vec<u8>, macro_refs: &Vec<MacroRef>) {
        for d in macro_refs {
            Self::write_u8(data, d.event_id);
            Self::write_u8(data, d.macro_id);
        }
    }
    fn write_bytes(data: &mut Vec<u8>, bytes: &Vec<u8>) {
        for d in bytes {
            Self::write_u8(data, *d);
        }
    }
    fn write_language_codes(data: &mut Vec<u8>, language_codes: &Vec<String>) {
        for d in language_codes {
            Self::write_string(data, d);
        }
    }
    fn write_points(data: &mut Vec<u8>, points: &Vec<Point<u16>>) {
        for d in points {
            Self::write_u16(data, d.x);
            Self::write_u16(data, d.y);
        }
    }
    fn write_colours(data: &mut Vec<u8>, colours: &Vec<Colour>) {
        for d in colours {
            Self::write_u8(data, d.b);
            Self::write_u8(data, d.g);
            Self::write_u8(data, d.r);
            Self::write_u8(data, d.a);
        }
    }
    fn write_object_labels(data: &mut Vec<u8>, object_labels: &Vec<ObjectLabel>) {
        for d in object_labels {
            Self::write_u16(data, d.id);
            Self::write_u16(data, d.string_variable_reference);
            Self::write_u8(data, d.font_type);
            Self::write_u16(data, d.graphic_representation);
        }
    }
    fn write_language_pairs(data: &mut Vec<u8>, language_pairs: &Vec<(String, String)>) {
        for d in language_pairs {
            Self::write_string(data, &d.0);
            Self::write_string(data, &d.1);
        }
    }

    fn write_character_ranges(data: &mut Vec<u8>, character_ranges: &Vec<CharacterRange>) {
        for character_range in character_ranges {
            Self::write_u16(data, character_range.first_character);
            Self::write_u16(data, character_range.last_character);
        }
    }

    fn write_code_planes(data: &mut Vec<u8>, code_planes: &Vec<CodePlane>) {
        Self::write_u8(data, code_planes.len() as u8);

        for code_plane in code_planes {
            Self::write_u8(data, code_plane.number);
            Self::write_u8(data, code_plane.character_ranges.len() as u8);
            Self::write_character_ranges(data, &code_plane.character_ranges);
        }
    }

    fn write_u8(data: &mut Vec<u8>, val: impl Into<u8>) {
        let val: u8 = val.into();
        data.push(val);
    }
    fn write_u16(data: &mut Vec<u8>, val: impl Into<u16>) {
        let val: u16 = val.into();
        data.extend(val.to_le_bytes());
    }
    fn write_i16(data: &mut Vec<u8>, val: impl Into<i16>) {
        let val: i16 = val.into();
        data.extend(val.to_le_bytes());
    }
    fn write_u32(data: &mut Vec<u8>, val: impl Into<u32>) {
        let val: u32 = val.into();
        data.extend(val.to_le_bytes());
    }
    fn write_i32(data: &mut Vec<u8>, val: impl Into<i32>) {
        let val: i32 = val.into();
        data.extend(val.to_le_bytes());
    }
    fn write_f32(data: &mut Vec<u8>, val: impl Into<f32>) {
        let val: f32 = val.into();
        data.extend(val.to_le_bytes());
    }
    fn write_string(data: &mut Vec<u8>, val: impl Into<String>) {
        let val: String = val.into();
        data.extend(val.as_bytes());
    }
    fn write_name(data: &mut Vec<u8>, val: impl Into<NAME>) {
        let val: NAME = val.into();
        data.extend::<[u8; 8]>(val.into());
    }
    fn write_working_set(data: &mut Vec<u8>, o: &WorkingSet) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::WorkingSet);
        Self::write_u8(data, o.background_colour);
        Self::write_u8(data, o.selectable);
        Self::write_u16(data, o.active_mask);
        Self::write_u8(data, o.object_refs.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);
        Self::write_u8(data, o.language_codes.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
        Self::write_macro_refs(data, &o.macro_refs);
        Self::write_language_codes(data, &o.language_codes);
    }
    fn write_data_mask(data: &mut Vec<u8>, o: &DataMask) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::DataMask);
        Self::write_u8(data, o.background_colour);
        Self::write_u16(data, o.soft_key_mask);
        Self::write_u8(data, o.object_refs.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_alarm_mask(data: &mut Vec<u8>, o: &AlarmMask) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::AlarmMask);
        Self::write_u8(data, o.background_colour);
        Self::write_u16(data, o.soft_key_mask);
        Self::write_u8(data, o.priority);
        Self::write_u8(data, o.acoustic_signal);
        Self::write_u8(data, o.object_refs.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_container(data: &mut Vec<u8>, o: &Container) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::Container);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.hidden);
        Self::write_u8(data, o.object_refs.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_soft_key_mask(data: &mut Vec<u8>, o: &SoftKeyMask) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::SoftKeyMask);
        Self::write_u8(data, o.background_colour);
        Self::write_u8(data, o.objects.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_object_ids(data, &o.objects);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_key(data: &mut Vec<u8>, o: &Key) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::Key);
        Self::write_u8(data, o.background_colour);
        Self::write_u8(data, o.key_code);
        Self::write_u8(data, o.object_refs.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_button(data: &mut Vec<u8>, o: &Button) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::Button);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.background_colour);
        Self::write_u8(data, o.border_colour);
        Self::write_u8(data, o.key_code);
        Self::write_u8(data, o.options);
        Self::write_u8(data, o.object_refs.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_input_boolean(data: &mut Vec<u8>, o: &InputBoolean) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::InputBoolean);
        Self::write_u8(data, o.background_colour);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.foreground_colour);
        Self::write_u16(data, o.variable_reference);
        Self::write_u8(data, o.value);
        Self::write_u8(data, o.enabled);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_input_string(data: &mut Vec<u8>, o: &InputString) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::InputString);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.background_colour);
        Self::write_u16(data, o.font_attributes);
        Self::write_u16(data, o.input_attributes);
        Self::write_u8(data, o.options);
        Self::write_u16(data, o.variable_reference);
        Self::write_u8(data, o.justification);
        Self::write_string(data, &o.value);
        Self::write_u8(data, o.enabled);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_input_number(data: &mut Vec<u8>, o: &InputNumber) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::InputNumber);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.background_colour);
        Self::write_u16(data, o.font_attributes);
        Self::write_u8(data, o.options);
        Self::write_u16(data, o.variable_reference);
        Self::write_u32(data, o.value);
        Self::write_u32(data, o.min_value);
        Self::write_u32(data, o.max_value);
        Self::write_i32(data, o.offset);
        Self::write_f32(data, o.scale);
        Self::write_u8(data, o.nr_of_decimals);
        Self::write_u8(data, o.format as u8);
        Self::write_u8(data, o.justification);
        Self::write_u8(data, o.options2);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_input_list(data: &mut Vec<u8>, o: &InputList) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::InputList);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u16(data, o.variable_reference);
        Self::write_u8(data, o.value);
        Self::write_u8(data, o.list_items.len() as u8);
        Self::write_u8(data, o.options);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_nullable_object_ids(data, &o.list_items);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_output_string(data: &mut Vec<u8>, o: &OutputString) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputString);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.background_colour);
        Self::write_u16(data, o.font_attributes);
        Self::write_u8(data, o.options);
        Self::write_u16(data, o.variable_reference);
        Self::write_u8(data, o.justification);
        Self::write_u16(data, o.value.len() as u16);
        Self::write_string(data, &o.value);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_output_number(data: &mut Vec<u8>, o: &OutputNumber) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputNumber);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.background_colour);
        Self::write_u16(data, o.font_attributes);
        Self::write_u8(data, o.options);
        Self::write_u16(data, o.variable_reference);
        Self::write_u32(data, o.value);
        Self::write_i32(data, o.offset);
        Self::write_f32(data, o.scale);
        Self::write_u8(data, o.nr_of_decimals);
        Self::write_u8(data, o.format as u8);
        Self::write_u8(data, o.justification);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_output_line(data: &mut Vec<u8>, o: &OutputLine) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputLine);
        Self::write_u16(data, o.line_attributes);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.line_direction);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_output_rectangle(data: &mut Vec<u8>, o: &OutputRectangle) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputRectangle);
        Self::write_u16(data, o.line_attributes);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.line_suppression);
        Self::write_u16(data, o.fill_attributes);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_output_ellipse(data: &mut Vec<u8>, o: &OutputEllipse) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputEllipse);
        Self::write_u16(data, o.line_attributes);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.ellipse_type);
        Self::write_u8(data, o.start_angle);
        Self::write_u8(data, o.end_angle);
        Self::write_u16(data, o.fill_attributes);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_output_polygon(data: &mut Vec<u8>, o: &OutputPolygon) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputPolygon);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u16(data, o.line_attributes);
        Self::write_u16(data, o.fill_attributes);
        Self::write_u8(data, o.polygon_type);
        Self::write_u8(data, o.points.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_points(data, &o.points);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_output_meter(data: &mut Vec<u8>, o: &OutputMeter) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputMeter);
        Self::write_u16(data, o.width);
        Self::write_u8(data, o.needle_colour);
        Self::write_u8(data, o.border_colour);
        Self::write_u8(data, o.arc_and_tick_colour);
        Self::write_u8(data, o.options);
        Self::write_u8(data, o.nr_of_ticks);
        Self::write_u8(data, o.start_angle);
        Self::write_u8(data, o.end_angle);
        Self::write_u16(data, o.min_value);
        Self::write_u16(data, o.max_value);
        Self::write_u16(data, o.variable_reference);
        Self::write_u16(data, o.value);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_output_linear_bar_graph(data: &mut Vec<u8>, o: &OutputLinearBarGraph) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputLinearBarGraph);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.colour);
        Self::write_u8(data, o.target_line_colour);
        Self::write_u8(data, o.options);
        Self::write_u8(data, o.nr_of_ticks);
        Self::write_u16(data, o.min_value);
        Self::write_u16(data, o.max_value);
        Self::write_u16(data, o.variable_reference);
        Self::write_u16(data, o.value);
        Self::write_u16(data, o.target_value_variable_reference);
        Self::write_u16(data, o.target_value);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_output_arched_bar_graph(data: &mut Vec<u8>, o: &OutputArchedBarGraph) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputArchedBarGraph);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.colour);
        Self::write_u8(data, o.target_line_colour);
        Self::write_u8(data, o.options);
        Self::write_u8(data, o.start_angle);
        Self::write_u8(data, o.end_angle);
        Self::write_u16(data, o.bar_graph_width);
        Self::write_u16(data, o.min_value);
        Self::write_u16(data, o.max_value);
        Self::write_u16(data, o.variable_reference);
        Self::write_u16(data, o.value);
        Self::write_u16(data, o.target_value_variable_reference);
        Self::write_u16(data, o.target_value);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_picture_graphic(data: &mut Vec<u8>, o: &PictureGraphic) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::PictureGraphic);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.actual_width);
        Self::write_u16(data, o.actual_height);
        Self::write_u8(data, o.format);
        Self::write_u8(data, o.options);
        Self::write_u8(data, o.transparency_colour);
        Self::write_u32(data, o.data.len() as u32);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_bytes(data, &o.data);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_number_variable(data: &mut Vec<u8>, o: &NumberVariable) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::NumberVariable);
        Self::write_u32(data, o.value);
    }
    fn write_string_variable(data: &mut Vec<u8>, o: &StringVariable) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::StringVariable);
        Self::write_string(data, &o.value);
    }
    fn write_font_attributes(data: &mut Vec<u8>, o: &FontAttributes) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::FontAttributes);
        Self::write_u8(data, o.font_colour);
        Self::write_u8(data, o.font_size);
        Self::write_u8(data, o.font_type);
        Self::write_u8(data, o.font_style);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_line_attributes(data: &mut Vec<u8>, o: &LineAttributes) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::LineAttributes);
        Self::write_u8(data, o.line_colour);
        Self::write_u8(data, o.line_width);
        Self::write_u16(data, o.line_art);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_fill_attributes(data: &mut Vec<u8>, o: &FillAttributes) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::FillAttributes);
        Self::write_u8(data, o.fill_type);
        Self::write_u8(data, o.fill_colour);
        Self::write_u16(data, o.fill_pattern);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_input_attributes(data: &mut Vec<u8>, o: &InputAttributes) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::InputAttributes);
        Self::write_u8(data, o.validation_type);
        Self::write_string(data, &o.validation_string);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_object_pointer(data: &mut Vec<u8>, o: &ObjectPointer) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ObjectPointer);
        Self::write_u16(data, o.value);
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ObjectPointer);
        Self::write_u16(data, o.value);
    }
    fn write_macro(data: &mut Vec<u8>, o: &Macro) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::Macro);
        Self::write_u16(data, o.commands.len() as u16);

        Self::write_bytes(data, &o.commands);
    }
    fn write_auxiliary_function_type1(data: &mut Vec<u8>, o: &AuxiliaryFunctionType1) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::AuxiliaryFunctionType1);
        Self::write_u8(data, o.background_colour);
        Self::write_u8(data, o.function_type);
        Self::write_u8(data, o.object_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
    }
    fn write_auxiliary_input_type1(data: &mut Vec<u8>, o: &AuxiliaryInputType1) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::AuxiliaryInputType1);
        Self::write_u8(data, o.background_colour);
        Self::write_u8(data, o.function_type);
        Self::write_u8(data, o.input_id);
        Self::write_u8(data, o.object_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
    }
    fn write_auxiliary_function_type2(data: &mut Vec<u8>, o: &AuxiliaryFunctionType2) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::AuxiliaryFunctionType2);
        Self::write_u8(data, o.background_colour);
        Self::write_u8(data, o.function_attributes);
        Self::write_u8(data, o.object_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
    }
    fn write_auxiliary_input_type2(data: &mut Vec<u8>, o: &AuxiliaryInputType2) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::AuxiliaryInputType2);
        Self::write_u8(data, o.background_colour);
        Self::write_u8(data, o.function_attributes);
        Self::write_u8(data, o.object_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
    }
    fn write_auxiliary_control_designator_type2(
        data: &mut Vec<u8>,
        o: &AuxiliaryControlDesignatorType2,
    ) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::AuxiliaryControlDesignatorType2);
        Self::write_u8(data, o.pointer_type);
        Self::write_u16(data, o.auxiliary_object_id);
    }
    fn write_window_mask(data: &mut Vec<u8>, o: &WindowMask) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::WindowMask);
        Self::write_u8(data, o.cell_format.size().x);
        Self::write_u8(data, o.cell_format.size().y);
        Self::write_u8(data, o.window_type);
        Self::write_u8(data, o.background_colour);
        Self::write_u8(data, o.options);
        Self::write_u16(data, o.name);
        Self::write_u16(data, o.window_title);
        Self::write_u16(data, o.window_icon);
        Self::write_u8(data, o.objects.len() as u8);
        Self::write_u8(data, o.object_refs.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_nullable_object_ids(data, &o.objects);
        Self::write_object_refs(data, &o.object_refs);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_key_group(data: &mut Vec<u8>, o: &KeyGroup) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::KeyGroup);
        Self::write_u8(data, o.options);
        Self::write_u16(data, o.name);
        Self::write_u16(data, o.key_group_icon);
        Self::write_u8(data, o.objects.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_object_ids(data, &o.objects);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_graphics_context(data: &mut Vec<u8>, o: &GraphicsContext) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::GraphicsContext);
        Self::write_u16(data, o.viewport_width);
        Self::write_u16(data, o.viewport_height);
        Self::write_i16(data, o.viewport_x);
        Self::write_i16(data, o.viewport_y);
        Self::write_u16(data, o.canvas_width);
        Self::write_u16(data, o.canvas_height);
        Self::write_f32(data, o.viewport_zoom);
        Self::write_i16(data, o.graphics_cursor_x);
        Self::write_i16(data, o.graphics_cursor_y);
        Self::write_u8(data, o.foreground_colour);
        Self::write_u8(data, o.background_colour);
        Self::write_u16(data, o.font_attributes_object);
        Self::write_u16(data, o.line_attributes_object);
        Self::write_u16(data, o.fill_attributes_object);
        Self::write_u8(data, o.format);
        Self::write_u8(data, o.options);
        Self::write_u8(data, o.transparency_colour);
    }
    fn write_output_list(data: &mut Vec<u8>, o: &OutputList) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::OutputList);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u16(data, o.variable_reference);
        Self::write_u8(data, o.value);
        Self::write_u8(data, o.list_items.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_nullable_object_ids(data, &o.list_items);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_extended_input_attributes(data: &mut Vec<u8>, o: &ExtendedInputAttributes) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ExtendedInputAttributes);
        Self::write_u8(data, o.validation_type as u8);
        Self::write_code_planes(data, &o.code_planes);
    }
    fn write_colour_map(data: &mut Vec<u8>, o: &ColourMap) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ColourMap);
        Self::write_u16(data, o.colour_map.len() as u16);

        Self::write_bytes(data, &o.colour_map);
    }
    fn write_object_label_reference_list(data: &mut Vec<u8>, o: &ObjectLabelReferenceList) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ObjectLabelReferenceList);
        Self::write_u16(data, o.object_labels.len() as u16);

        Self::write_object_labels(data, &o.object_labels);
    }
    fn write_external_object_definition(data: &mut Vec<u8>, o: &ExternalObjectDefinition) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ExternalObjectDefinition);
        Self::write_u8(data, o.options);
        Self::write_name(data, o.name);
        Self::write_u8(data, o.objects.len() as u8);

        Self::write_nullable_object_ids(data, &o.objects);
    }
    fn write_external_reference_name(data: &mut Vec<u8>, o: &ExternalReferenceName) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ExternalReferenceName);
        Self::write_u8(data, o.options);
        Self::write_name(data, o.name);
    }
    fn write_external_object_pointer(data: &mut Vec<u8>, o: &ExternalObjectPointer) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ExternalObjectPointer);
        Self::write_u16(data, o.default_object_id);
        Self::write_u16(data, o.external_reference_name_id);
        Self::write_u16(data, o.external_object_id);
    }
    fn write_animation(data: &mut Vec<u8>, o: &Animation) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::Animation);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u16(data, o.refresh_interval);
        Self::write_u8(data, o.value);
        Self::write_u8(data, o.enabled);
        Self::write_u8(data, o.first_child_index);
        Self::write_u8(data, o.last_child_index);
        Self::write_u8(data, o.default_child_index);
        Self::write_u8(data, o.options);
        Self::write_u8(data, o.object_refs.len() as u8);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_object_refs(data, &o.object_refs);
        Self::write_macro_refs(data, &o.macro_refs);
    }
    fn write_colour_palette(data: &mut Vec<u8>, o: &ColourPalette) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ColourPalette);
        Self::write_u8(data, o.options);
        Self::write_u16(data, o.colours.len() as u16);

        Self::write_colours(data, &o.colours);
    }
    fn write_graphic_data(data: &mut Vec<u8>, o: &GraphicData) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::GraphicData);
        Self::write_u8(data, o.format);
        Self::write_u32(data, o.data.len() as u32);

        Self::write_bytes(data, &o.data);
    }
    fn write_working_set_special_controls(data: &mut Vec<u8>, o: &WorkingSetSpecialControls) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::WorkingSetSpecialControls);
        Self::write_u16(data, o.id_of_colour_map);
        Self::write_u16(data, o.id_of_colour_palette);
        Self::write_u8(data, o.language_pairs.len() as u8);

        Self::write_language_pairs(data, &o.language_pairs);
    }
    fn write_scaled_graphic(data: &mut Vec<u8>, o: &ScaledGraphic) {
        Self::write_u16(data, o.id);
        Self::write_u8(data, ObjectType::ScaledGraphic);
        Self::write_u16(data, o.width);
        Self::write_u16(data, o.height);
        Self::write_u8(data, o.scale_type);
        Self::write_u8(data, o.options);
        Self::write_u16(data, o.value);
        Self::write_u8(data, o.macro_refs.len() as u8);

        Self::write_macro_refs(data, &o.macro_refs);
    }
}
