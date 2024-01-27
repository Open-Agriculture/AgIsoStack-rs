// Copyright 2024 Raven Industries inc
use super::*;
use crate::object_pool::colour::Colour;
use crate::object_pool::object::*;
use crate::object_pool::object_attributes::{MacroRef, ObjectLabel, ObjectRef, Point};
use crate::object_pool::object_id::ObjectId;

impl Object {
    pub fn read(data: &mut dyn Iterator<Item = u8>) -> Result<Self, ParseError> {
        let id = Self::read_u16(data)?.try_into()?;
        let object_type = Self::read_u8(data)?.try_into()?;

        match object_type {
            ObjectType::WorkingSet => Self::read_working_set(id, data),
            ObjectType::DataMask => Self::read_data_mask(id, data),
            ObjectType::AlarmMask => Self::read_alarm_mask(id, data),
            ObjectType::Container => Self::read_container(id, data),
            ObjectType::SoftKeyMask => Self::read_soft_key_mask(id, data),
            ObjectType::Key => Self::read_key(id, data),
            ObjectType::Button => Self::read_button(id, data),
            ObjectType::InputBoolean => Self::read_input_boolean(id, data),
            ObjectType::InputString => Self::read_input_string(id, data),
            ObjectType::InputNumber => Self::read_input_number(id, data),
            ObjectType::InputList => Self::read_input_list(id, data),
            ObjectType::OutputString => Self::read_output_string(id, data),
            ObjectType::OutputNumber => Self::read_output_number(id, data),
            ObjectType::OutputLine => Self::read_output_line(id, data),
            ObjectType::OutputRectangle => Self::read_output_rectangle(id, data),
            ObjectType::OutputEllipse => Self::read_output_ellipse(id, data),
            ObjectType::OutputPolygon => Self::read_output_polygon(id, data),
            ObjectType::OutputMeter => Self::read_output_meter(id, data),
            ObjectType::OutputLinearBarGraph => Self::read_output_linear_bar_graph(id, data),
            ObjectType::OutputArchedBarGraph => Self::read_output_arched_bar_graph(id, data),
            ObjectType::PictureGraphic => Self::read_picture_graphic(id, data),
            ObjectType::NumberVariable => Self::read_number_variable(id, data),
            ObjectType::StringVariable => Self::read_string_variable(id, data),
            ObjectType::FontAttributes => Self::read_font_attributes(id, data),
            ObjectType::LineAttributes => Self::read_line_attributes(id, data),
            ObjectType::FillAttributes => Self::read_fill_attributes(id, data),
            ObjectType::InputAttributes => Self::read_input_attributes(id, data),
            ObjectType::ObjectPointer => Self::read_object_pointer(id, data),
            ObjectType::Macro => Self::read_macro(id, data),
            ObjectType::AuxiliaryFunctionType1 => Self::read_auxiliary_function_type1(id, data),
            ObjectType::AuxiliaryInputType1 => Self::read_auxiliary_input_type1(id, data),
            ObjectType::AuxiliaryFunctionType2 => Self::read_auxiliary_function_type2(id, data),
            ObjectType::AuxiliaryInputType2 => Self::read_auxiliary_input_type2(id, data),
            ObjectType::AuxiliaryControlDesignatorType2 => {
                Self::read_auxiliary_control_designator_type2(id, data)
            }
            ObjectType::WindowMask => Self::read_window_mask(id, data),
            ObjectType::KeyGroup => Self::read_key_group(id, data),
            ObjectType::GraphicsContext => Self::read_graphics_context(id, data),
            ObjectType::OutputList => Self::read_output_list(id, data),
            ObjectType::ExtendedInputAttributes => Self::read_extended_input_attributes(id, data),
            ObjectType::ColourMap => Self::read_colour_map(id, data),
            ObjectType::ObjectLabelReferenceList => {
                Self::read_object_label_reference_list(id, data)
            }
            ObjectType::ExternalObjectDefinition => Self::read_external_object_definition(id, data),
            ObjectType::ExternalReferenceName => Self::read_external_reference_name(id, data),
            ObjectType::ExternalObjectPointer => Self::read_external_object_pointer(id, data),
            ObjectType::Animation => Self::read_animation(id, data),
            ObjectType::ColourPalette => Self::read_colour_palette(id, data),
            ObjectType::GraphicData => Self::read_graphic_data(id, data),
            ObjectType::WorkingSetSpecialControls => {
                Self::read_working_set_special_controls(id, data)
            }
            ObjectType::ScaledGraphic => Self::read_scaled_graphic(id, data),
        }
    }

    /* READ COMMON TYPES */

    fn read_objects(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_objects: usize,
    ) -> Result<Vec<ObjectId>, ParseError> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_objects {
            objs.push(Self::read_u16(data)?.try_into()?);
        }
        Ok(objs)
    }

    fn read_object_refs(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_objects: usize,
    ) -> Result<Vec<ObjectRef>, ParseError> {
        let mut refs = Vec::new();
        for _ in 0..nr_of_objects {
            refs.push(ObjectRef {
                id: Self::read_u16(data)?.try_into()?,
                offset: Point {
                    x: Self::read_i16(data)?,
                    y: Self::read_i16(data)?,
                },
            })
        }
        Ok(refs)
    }
    fn read_macro_refs(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_macros: usize,
    ) -> Result<Vec<MacroRef>, ParseError> {
        let mut refs = Vec::new();
        for _ in 0..nr_of_macros {
            refs.push(MacroRef {
                event_id: Self::read_u8(data)?,
                macro_id: Self::read_u8(data)?,
            })
        }
        Ok(refs)
    }
    fn read_bytes(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_bytes: usize,
    ) -> Result<Vec<u8>, ParseError> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_bytes {
            objs.push(Self::read_u8(data)?)
        }
        Ok(objs)
    }
    fn read_points(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_points: usize,
    ) -> Result<Vec<Point<u16>>, ParseError> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_points {
            objs.push(Point {
                x: Self::read_u16(data)?,
                y: Self::read_u16(data)?,
            })
        }
        Ok(objs)
    }
    fn read_colours(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_colours: usize,
    ) -> Result<Vec<Colour>, ParseError> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_colours {
            let b = Self::read_u8(data)?;
            let g = Self::read_u8(data)?;
            let r = Self::read_u8(data)?;
            let a = Self::read_u8(data)?;

            objs.push(Colour::new_by_rgba(r, g, b, a))
        }
        Ok(objs)
    }
    fn read_object_labels(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_objects: usize,
    ) -> Result<Vec<ObjectLabel>, ParseError> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_objects {
            objs.push(ObjectLabel {
                id: Self::read_u16(data)?.try_into()?,
                string_variable_reference: Self::read_u16(data)?.try_into()?,
                font_type: Self::read_u8(data)?,
                graphic_representation: Self::read_u16(data)?.try_into()?,
            })
        }
        Ok(objs)
    }
    fn read_language_pairs(
        data: &mut dyn Iterator<Item = u8>,
        nr_of_objects: usize,
    ) -> Result<Vec<(String, String)>, ParseError> {
        let mut objs = Vec::new();
        for _ in 0..nr_of_objects {
            objs.push((Self::read_string(2, data)?, Self::read_string(2, data)?))
        }
        Ok(objs)
    }

    fn read_character_ranges(
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Vec<CharacterRange>, ParseError> {
        let nr_of_character_ranges = Self::read_u8(data)? as usize;
        let mut character_ranges = Vec::new();

        for _ in 0..nr_of_character_ranges {
            let character_range = CharacterRange {
                first_character: Self::read_u16(data)?,
                last_character: Self::read_u16(data)?,
            };
            character_ranges.push(character_range);
        }

        Ok(character_ranges)
    }

    fn read_code_planes(data: &mut dyn Iterator<Item = u8>) -> Result<Vec<CodePlane>, ParseError> {
        let mut code_planes = Vec::new();
        let nr_of_code_planes = Self::read_u8(data)? as usize;

        for _ in 0..nr_of_code_planes {
            let number = Self::read_u8(data)?;
            let character_ranges = Self::read_character_ranges(data)?;
            let code_plane = CodePlane {
                number,
                character_ranges,
            };

            code_planes.push(code_plane);
        }

        Ok(code_planes)
    }

    fn read_bool(data: &mut dyn Iterator<Item = u8>) -> Result<bool, ParseError> {
        match data.next() {
            Some(d) => {
                if d == 0 || d == 1 {
                    Ok(d == 1)
                } else {
                    Err(ParseError::UnknownObjectType)
                }
            }
            None => Err(ParseError::DataEmpty),
        }
    }
    fn read_u8(data: &mut dyn Iterator<Item = u8>) -> Result<u8, ParseError> {
        match data.next() {
            Some(d) => Ok(d),
            None => Err(ParseError::DataEmpty),
        }
    }
    fn read_u16(data: &mut dyn Iterator<Item = u8>) -> Result<u16, ParseError> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();

        if a.is_none() || b.is_none() {
            return Err(ParseError::DataEmpty);
        }

        Ok(u16::from_le_bytes([a.unwrap(), b.unwrap()]))
    }
    fn read_i16(data: &mut dyn Iterator<Item = u8>) -> Result<i16, ParseError> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();

        if a.is_none() || b.is_none() {
            return Err(ParseError::DataEmpty);
        }

        Ok(i16::from_le_bytes([a.unwrap(), b.unwrap()]))
    }
    fn read_u32(data: &mut dyn Iterator<Item = u8>) -> Result<u32, ParseError> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();
        let c: Option<u8> = data.next();
        let d: Option<u8> = data.next();

        if a.is_none() || b.is_none() || c.is_none() || d.is_none() {
            return Err(ParseError::DataEmpty);
        }

        Ok(u32::from_le_bytes([
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
        ]))
    }
    fn read_i32(data: &mut dyn Iterator<Item = u8>) -> Result<i32, ParseError> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();
        let c: Option<u8> = data.next();
        let d: Option<u8> = data.next();

        if a.is_none() || b.is_none() || c.is_none() || d.is_none() {
            return Err(ParseError::DataEmpty);
        }

        Ok(i32::from_le_bytes([
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
        ]))
    }
    fn read_f32(data: &mut dyn Iterator<Item = u8>) -> Result<f32, ParseError> {
        let a: Option<u8> = data.next();
        let b: Option<u8> = data.next();
        let c: Option<u8> = data.next();
        let d: Option<u8> = data.next();

        if a.is_none() || b.is_none() || c.is_none() || d.is_none() {
            return Err(ParseError::DataEmpty);
        }

        Ok(f32::from_le_bytes([
            a.unwrap(),
            b.unwrap(),
            c.unwrap(),
            d.unwrap(),
        ]))
    }
    fn read_string(len: usize, data: &mut dyn Iterator<Item = u8>) -> Result<String, ParseError> {
        let mut s = String::new();
        for _ in 0..len {
            if let Some(c) = data.next() {
                s.push(c as char);
            } else {
                return Err(ParseError::DataEmpty);
            };
        }
        Ok(s)
    }
    fn read_name(data: &mut dyn Iterator<Item = u8>) -> Result<NAME, ParseError> {
        let name: [Option<u8>; 8] = [
            data.next(),
            data.next(),
            data.next(),
            data.next(),
            data.next(),
            data.next(),
            data.next(),
            data.next(),
        ];

        if name.contains(&None) {
            return Err(ParseError::DataEmpty);
        }

        Ok(NAME::new(u64::from_le_bytes(name.map(|v| v.unwrap()))))
    }

    /* READ ISOBUS OBJECTS */

    fn read_working_set(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = WorkingSet {
            id,
            background_colour: Self::read_u8(data)?.into(),
            selectable: Self::read_bool(data)?,
            active_mask: Self::read_u16(data)?.try_into()?,
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
            language_codes: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        for _ in 0..o.language_codes.capacity() {
            o.language_codes.push(Self::read_string(2, data)?)
        }

        Ok(Object::WorkingSet(o))
    }

    fn read_data_mask(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = DataMask {
            id,
            background_colour: Self::read_u8(data)?,
            soft_key_mask: Self::read_u16(data)?.try_into()?,
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::DataMask(o))
    }

    fn read_alarm_mask(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = AlarmMask {
            id,
            background_colour: Self::read_u8(data)?,
            soft_key_mask: Self::read_u16(data)?.try_into()?,
            priority: Self::read_u8(data)?,
            acoustic_signal: Self::read_u8(data)?,
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::AlarmMask(o))
    }

    fn read_container(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = Container {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            hidden: Self::read_bool(data)?,
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::Container(o))
    }

    fn read_soft_key_mask(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = SoftKeyMask {
            id,
            background_colour: Self::read_u8(data)?,
            objects: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.objects
            .extend(Self::read_objects(data, o.objects.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::SoftKeyMask(o))
    }

    fn read_key(id: ObjectId, data: &mut dyn Iterator<Item = u8>) -> Result<Self, ParseError> {
        let mut o = Key {
            id,
            background_colour: Self::read_u8(data)?,
            key_code: Self::read_u8(data)?,
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::Key(o))
    }

    fn read_button(id: ObjectId, data: &mut dyn Iterator<Item = u8>) -> Result<Self, ParseError> {
        let mut o = Button {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            background_colour: Self::read_u8(data)?,
            border_colour: Self::read_u8(data)?,
            key_code: Self::read_u8(data)?,
            options: Self::read_u8(data)?.into(),
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::Button(o))
    }

    fn read_input_boolean(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = InputBoolean {
            id,
            background_colour: Self::read_u8(data)?,
            width: Self::read_u16(data)?,
            foreground_colour: Self::read_u16(data)?.try_into()?,
            variable_reference: Self::read_u16(data)?.try_into()?,
            value: Self::read_bool(data)?,
            enabled: Self::read_bool(data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::InputBoolean(o))
    }

    fn read_input_string(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = InputString {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            background_colour: Self::read_u8(data)?,
            font_attributes: Self::read_u16(data)?.try_into()?,
            input_attributes: Self::read_u16(data)?.try_into()?,
            options: Self::read_u8(data)?.into(),
            variable_reference: Self::read_u16(data)?.try_into()?,
            justification: Self::read_u8(data)?.into(),
            value: Self::read_string(Self::read_u8(data)?.into(), data)?,
            enabled: Self::read_bool(data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::InputString(o))
    }

    fn read_input_number(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = InputNumber {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            background_colour: Self::read_u8(data)?,
            font_attributes: Self::read_u16(data)?.try_into()?,
            options: Self::read_u8(data)?.into(),
            variable_reference: Self::read_u16(data)?.try_into()?,
            value: Self::read_u32(data)?,
            min_value: Self::read_u32(data)?,
            max_value: Self::read_u32(data)?,
            offset: Self::read_i32(data)?,
            scale: Self::read_f32(data)?,
            nr_of_decimals: Self::read_u8(data)?,
            format: Self::read_bool(data)?.into(),
            justification: Self::read_u8(data)?.into(),
            options2: Self::read_u8(data)?.into(),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::InputNumber(o))
    }

    fn read_input_list(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = InputList {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            variable_reference: Self::read_u16(data)?.try_into()?,
            value: Self::read_u8(data)?,
            list_items: Vec::with_capacity(Self::read_u8(data)?.into()),
            options: Self::read_u8(data)?.into(),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.list_items
            .extend(Self::read_objects(data, o.list_items.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::InputList(o))
    }

    fn read_output_string(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputString {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            background_colour: Self::read_u8(data)?,
            font_attributes: Self::read_u16(data)?.try_into()?,
            options: Self::read_u8(data)?.into(),
            variable_reference: Self::read_u16(data)?.try_into()?,
            justification: Self::read_u8(data)?.into(),
            value: Self::read_string(Self::read_u16(data)?.into(), data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputString(o))
    }

    fn read_output_number(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputNumber {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            background_colour: Self::read_u8(data)?,
            font_attributes: Self::read_u16(data)?.try_into()?,
            options: Self::read_u8(data)?.into(),
            variable_reference: Self::read_u16(data)?.try_into()?,
            value: Self::read_u32(data)?,
            offset: Self::read_i32(data)?,
            scale: Self::read_f32(data)?,
            nr_of_decimals: Self::read_u8(data)?,
            format: Self::read_bool(data)?.into(),
            justification: Self::read_u8(data)?.into(),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputNumber(o))
    }

    fn read_output_line(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputLine {
            id,
            line_attributes: Self::read_u16(data)?.try_into()?,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            line_direction: Self::read_u8(data)?.into(),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputLine(o))
    }

    fn read_output_rectangle(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputRectangle {
            id,
            line_attributes: Self::read_u16(data)?.try_into()?,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            line_suppression: Self::read_u8(data)?,
            fill_attributes: Self::read_u16(data)?.try_into()?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputRectangle(o))
    }

    fn read_output_ellipse(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputEllipse {
            id,
            line_attributes: Self::read_u16(data)?.try_into()?,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            ellipse_type: Self::read_u8(data)?,
            start_angle: Self::read_u8(data)?,
            end_angle: Self::read_u8(data)?,
            fill_attributes: Self::read_u16(data)?.try_into()?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputEllipse(o))
    }

    fn read_output_polygon(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputPolygon {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            line_attributes: Self::read_u16(data)?.try_into()?,
            fill_attributes: Self::read_u16(data)?.try_into()?,
            polygon_type: Self::read_u8(data)?,
            points: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.points
            .extend(Self::read_points(data, o.points.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputPolygon(o))
    }

    fn read_output_meter(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputMeter {
            id,
            width: Self::read_u16(data)?,
            needle_colour: Self::read_u8(data)?,
            border_colour: Self::read_u8(data)?,
            arc_and_tick_colour: Self::read_u8(data)?,
            options: Self::read_u8(data)?.into(),
            nr_of_ticks: Self::read_u8(data)?,
            start_angle: Self::read_u8(data)?,
            end_angle: Self::read_u8(data)?,
            min_value: Self::read_u16(data)?,
            max_value: Self::read_u16(data)?,
            variable_reference: Self::read_u16(data)?.try_into()?,
            value: Self::read_u16(data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputMeter(o))
    }

    fn read_output_linear_bar_graph(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputLinearBarGraph {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            colour: Self::read_u8(data)?,
            target_line_colour: Self::read_u8(data)?,
            options: Self::read_u8(data)?.into(),
            nr_of_ticks: Self::read_u8(data)?,
            min_value: Self::read_u16(data)?,
            max_value: Self::read_u16(data)?,
            variable_reference: Self::read_u16(data)?.try_into()?,
            value: Self::read_u16(data)?,
            target_value_variable_reference: Self::read_u16(data)?.try_into()?,
            target_value: Self::read_u16(data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputLinearBarGraph(o))
    }

    fn read_output_arched_bar_graph(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputArchedBarGraph {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            colour: Self::read_u8(data)?,
            target_line_colour: Self::read_u8(data)?,
            options: Self::read_u8(data)?.into(),
            start_angle: Self::read_u8(data)?,
            end_angle: Self::read_u8(data)?,
            bar_graph_width: Self::read_u16(data)?,
            min_value: Self::read_u16(data)?,
            max_value: Self::read_u16(data)?,
            variable_reference: Self::read_u16(data)?.try_into()?,
            value: Self::read_u16(data)?,
            target_value_variable_reference: Self::read_u16(data)?.try_into()?,
            target_value: Self::read_u16(data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputArchedBarGraph(o))
    }

    fn read_picture_graphic(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = PictureGraphic {
            id,
            width: Self::read_u16(data)?,
            actual_width: Self::read_u16(data)?,
            actual_height: Self::read_u16(data)?,
            format: Self::read_u8(data)?,
            options: Self::read_u8(data)?.into(),
            transparency_colour: Self::read_u8(data)?,
            data: Vec::with_capacity(Self::read_u32(data)? as usize),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.data.extend(Self::read_bytes(data, o.data.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::PictureGraphic(o))
    }

    fn read_number_variable(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let o = NumberVariable {
            id,
            value: Self::read_u32(data)?,
        };

        Ok(Object::NumberVariable(o))
    }

    fn read_string_variable(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let o = StringVariable {
            id,
            value: Self::read_string(Self::read_u16(data)?.into(), data)?,
        };

        Ok(Object::StringVariable(o))
    }

    fn read_font_attributes(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = FontAttributes {
            id,
            font_colour: Self::read_u8(data)?,
            font_size: Self::read_u8(data)?,
            font_type: Self::read_u8(data)?,
            font_style: Self::read_u8(data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::FontAttributes(o))
    }

    fn read_line_attributes(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = LineAttributes {
            id,
            line_colour: Self::read_u8(data)?,
            line_width: Self::read_u8(data)?,
            line_art: Self::read_u16(data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::LineAttributes(o))
    }

    fn read_fill_attributes(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = FillAttributes {
            id,
            fill_type: Self::read_u8(data)?,
            fill_colour: Self::read_u8(data)?,
            fill_pattern: Self::read_u16(data)?.try_into()?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::FillAttributes(o))
    }

    fn read_input_attributes(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = InputAttributes {
            id,
            validation_type: Self::read_u8(data)?,
            validation_string: Self::read_string(Self::read_u8(data)?.into(), data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::InputAttributes(o))
    }

    fn read_object_pointer(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let o = ObjectPointer {
            id,
            value: Self::read_u16(data)?.try_into()?,
        };

        Ok(Object::ObjectPointer(o))
    }

    fn read_macro(id: ObjectId, data: &mut dyn Iterator<Item = u8>) -> Result<Self, ParseError> {
        let mut o = Macro {
            id,
            commands: Vec::with_capacity(Self::read_u16(data)?.into()),
        };

        o.commands
            .extend(Self::read_bytes(data, o.commands.capacity())?);

        Ok(Object::Macro(o))
    }

    fn read_auxiliary_function_type1(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = AuxiliaryFunctionType1 {
            id,
            background_colour: Self::read_u8(data)?,
            function_type: Self::read_u8(data)?,
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);

        Ok(Object::AuxiliaryFunctionType1(o))
    }

    fn read_auxiliary_input_type1(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = AuxiliaryInputType1 {
            id,
            background_colour: Self::read_u8(data)?,
            function_type: Self::read_u8(data)?,
            input_id: Self::read_u8(data)?,
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);

        Ok(Object::AuxiliaryInputType1(o))
    }

    fn read_auxiliary_function_type2(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = AuxiliaryFunctionType2 {
            id,
            background_colour: Self::read_u8(data)?,
            function_attributes: Self::read_u8(data)?,
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);

        Ok(Object::AuxiliaryFunctionType2(o))
    }

    fn read_auxiliary_input_type2(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = AuxiliaryInputType2 {
            id,
            background_colour: Self::read_u8(data)?,
            function_attributes: Self::read_u8(data)?,
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);

        Ok(Object::AuxiliaryInputType2(o))
    }

    fn read_auxiliary_control_designator_type2(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let o = AuxiliaryControlDesignatorType2 {
            id,
            pointer_type: Self::read_u8(data)?,
            auxiliary_object_id: Self::read_u16(data)?.try_into()?,
        };

        Ok(Object::AuxiliaryControlDesignatorType2(o))
    }

    fn read_window_mask(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = WindowMask {
            id,
            cell_format: Self::read_u16(data)?.into(),
            window_type: Self::read_u8(data)?.into(),
            background_colour: Self::read_u8(data)?,
            options: Self::read_u8(data)?.into(),
            name: Self::read_u16(data)?.try_into()?,
            window_title: Self::read_u16(data)?.try_into()?,
            window_icon: Self::read_u16(data)?.try_into()?,
            objects: Vec::with_capacity(Self::read_u8(data)?.into()),
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.objects
            .extend(Self::read_objects(data, o.objects.capacity())?);
        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::WindowMask(o))
    }

    fn read_key_group(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = KeyGroup {
            id,
            options: Self::read_u8(data)?.into(),
            name: Self::read_u16(data)?.try_into()?,
            key_group_icon: Self::read_u16(data)?.try_into()?,
            objects: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.objects
            .extend(Self::read_objects(data, o.objects.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::KeyGroup(o))
    }

    fn read_graphics_context(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let o = GraphicsContext {
            id,
            viewport_width: Self::read_u16(data)?,
            viewport_height: Self::read_u16(data)?,
            viewport_x: Self::read_i16(data)?,
            viewport_y: Self::read_i16(data)?,
            canvas_width: Self::read_u16(data)?,
            canvas_height: Self::read_u16(data)?,
            viewport_zoom: Self::read_f32(data)?,
            graphics_cursor_x: Self::read_i16(data)?,
            graphics_cursor_y: Self::read_i16(data)?,
            foreground_colour: Self::read_u8(data)?,
            background_colour: Self::read_u8(data)?,
            font_attributes_object: Self::read_u16(data)?.try_into()?,
            line_attributes_object: Self::read_u16(data)?.try_into()?,
            fill_attributes_object: Self::read_u16(data)?.try_into()?,
            format: Self::read_u8(data)?.into(),
            options: Self::read_u8(data)?.into(),
            transparency_colour: Self::read_u8(data)?,
        };
        Ok(Object::GraphicsContext(o))
    }

    fn read_output_list(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = OutputList {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            variable_reference: Self::read_u16(data)?.try_into()?,
            value: Self::read_u8(data)?,
            list_items: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.list_items
            .extend(Self::read_objects(data, o.list_items.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::OutputList(o))
    }

    fn read_extended_input_attributes(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let o = ExtendedInputAttributes {
            id,
            validation_type: Self::read_u8(data)?.into(),
            code_planes: Self::read_code_planes(data)?,
        };

        Ok(Object::ExtendedInputAttributes(o))
    }

    fn read_colour_map(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = ColourMap {
            id,
            colour_map: Vec::with_capacity(Self::read_u16(data)?.into()),
        };

        o.colour_map
            .extend(Self::read_bytes(data, o.colour_map.capacity())?);

        Ok(Object::ColourMap(o))
    }

    fn read_object_label_reference_list(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = ObjectLabelReferenceList {
            id,
            object_labels: Vec::with_capacity(Self::read_u16(data)?.into()),
        };

        o.object_labels
            .extend(Self::read_object_labels(data, o.object_labels.capacity())?);

        Ok(Object::ObjectLabelReferenceList(o))
    }

    fn read_external_object_definition(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = ExternalObjectDefinition {
            id,
            options: Self::read_u8(data)?.into(),
            name: Self::read_name(data)?,
            objects: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.objects
            .extend(Self::read_objects(data, o.objects.capacity())?);

        Ok(Object::ExternalObjectDefinition(o))
    }

    fn read_external_reference_name(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let o = ExternalReferenceName {
            id,
            options: Self::read_u8(data)?.into(),
            name: Self::read_name(data)?,
        };

        Ok(Object::ExternalReferenceName(o))
    }

    fn read_external_object_pointer(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let o = ExternalObjectPointer {
            id,
            default_object_id: Self::read_u16(data)?.try_into()?,
            external_reference_name_id: Self::read_u16(data)?.try_into()?,
            external_object_id: Self::read_u16(data)?.try_into()?,
        };

        Ok(Object::ExternalObjectPointer(o))
    }

    fn read_animation(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = Animation {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            refresh_interval: Self::read_u16(data)?,
            value: Self::read_u8(data)?,
            enabled: Self::read_bool(data)?,
            first_child_index: Self::read_u8(data)?,
            last_child_index: Self::read_u8(data)?,
            default_child_index: Self::read_u8(data)?,
            options: Self::read_u8(data)?.into(),
            object_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.object_refs
            .extend(Self::read_object_refs(data, o.object_refs.capacity())?);
        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::Animation(o))
    }

    fn read_colour_palette(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = ColourPalette {
            id,
            options: Self::read_u8(data)?.into(),
            colours: Vec::with_capacity(Self::read_u16(data)?.into()),
        };

        o.colours
            .extend(Self::read_colours(data, o.colours.capacity())?);

        Ok(Object::ColourPalette(o))
    }

    fn read_graphic_data(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = GraphicData {
            id,
            format: Self::read_u8(data)?,
            data: Vec::with_capacity(Self::read_u32(data)?.try_into().unwrap()),
        };

        o.data.extend(Self::read_bytes(data, o.data.capacity())?);

        Ok(Object::GraphicData(o))
    }

    fn read_working_set_special_controls(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = WorkingSetSpecialControls {
            id,
            id_of_colour_map: Self::read_u16(data)?.try_into()?,
            id_of_colour_palette: Self::read_u16(data)?.try_into()?,
            language_pairs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.language_pairs.extend(Self::read_language_pairs(
            data,
            o.language_pairs.capacity(),
        )?);

        Ok(Object::WorkingSetSpecialControls(o))
    }

    fn read_scaled_graphic(
        id: ObjectId,
        data: &mut dyn Iterator<Item = u8>,
    ) -> Result<Self, ParseError> {
        let mut o = ScaledGraphic {
            id,
            width: Self::read_u16(data)?,
            height: Self::read_u16(data)?,
            scale_type: Self::read_u8(data)?,
            options: Self::read_u8(data)?.into(),
            value: Self::read_u16(data)?,
            macro_refs: Vec::with_capacity(Self::read_u8(data)?.into()),
        };

        o.macro_refs
            .extend(Self::read_macro_refs(data, o.macro_refs.capacity())?);

        Ok(Object::ScaledGraphic(o))
    }
}

#[cfg(test)]
mod tests {
    use crate::object_pool::object::{Object, WorkingSet};
    use crate::object_pool::object_attributes::{ObjectRef, Point};
    use crate::object_pool::object_id::ObjectId;
    use crate::object_pool::{Colour, ObjectPool, ObjectType};
    use std::vec::IntoIter;

    fn read_id_type(data: &mut dyn Iterator<Item = u8>) -> ObjectId {
        let id = Object::read_u16(data)
            .unwrap_or_else(|_| panic!("Failed to read object ID",))
            .try_into()
            .unwrap_or_else(|why| panic!("Failed to convert object ID: {:#?}", why));

        let _object_type: ObjectType = Object::read_u8(data)
            .unwrap_or_else(|_| panic!("Failed to read object type",))
            .try_into()
            .unwrap_or_else(|_| panic!("Failed to read object type",));

        id
    }

    #[test]
    fn read_working_set_test() {
        let mut data: IntoIter<u8> = vec![
            0x34, 0x12, //Object ID
            0x00, //Type
            0xF0, //Background colour
            0x01, //Selectable
            0x00, 0x00, //Active mask
            0x02, //Number of object references
            0x00, //Number of macro references
            0x00, //Number of language codes
            0xF1, 0x00, // Object ID reference 1
            0x7B, 0x00, // X Location reference 1
            0xC8, 0x01, // Y Location reference 1
            0xF2, 0x00, // Object ID reference 2
            0x15, 0x03, // X Location reference 2
            0x0C, 0x00, // Y Location reference 2
            0x00, // Event ID reference 1
            0x00, // Macro ID reference 1
            0x00, // Event ID reference 2
            0x00, // Macro ID reference 2
            0x00, 0x00, // Language code 1
            0x00, 0x00, // Language code 2
        ]
        .into_iter();

        let id = read_id_type(&mut data);

        let mut pool = ObjectPool::new();
        pool.add(
            Object::read_working_set(id, &mut data)
                .unwrap_or_else(|why| panic!("Failed to read working set: {:?}", why)),
        );

        let _working_set_act = pool
            .working_set_object()
            .unwrap_or_else(|| panic!("Failed to get working set of object pool",));

        let _working_set_exp = WorkingSet {
            id,
            background_colour: Colour::new_by_id(0xF0),
            selectable: true,
            active_mask: ObjectId::default(),
            object_refs: vec![
                ObjectRef {
                    id: ObjectId::new(0xF1).unwrap(),
                    offset: Point { x: 123, y: 456 },
                },
                ObjectRef {
                    id: ObjectId::new(0xF2).unwrap(),
                    offset: Point { x: 789, y: 12 },
                },
            ],
            macro_refs: vec![],
            language_codes: vec![],
        };

        assert_eq!(*_working_set_act, _working_set_exp);
    }
}
