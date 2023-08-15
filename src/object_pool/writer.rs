use super::*;

impl Object {
    pub fn write(&self) -> Vec<u8> {
        let mut data = Vec::new();

        match self {
            Object::WorkingSet(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::WorkingSet);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u8(&mut data, o.selectable);
                Self::write_u16(&mut data, o.active_mask);
                Self::write_u8(&mut data, o.object_refs.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);
                Self::write_u8(&mut data, o.language_codes.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
                Self::write_macro_refs(&mut data, &o.macro_refs);
                Self::write_language_codes(&mut data, &o.language_codes);
            }
            Object::DataMask(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::DataMask);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u16(&mut data, o.soft_key_mask);
                Self::write_u8(&mut data, o.object_refs.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::AlarmMask(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::AlarmMask);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u16(&mut data, o.soft_key_mask);
                Self::write_u8(&mut data, o.priority);
                Self::write_u8(&mut data, o.acoustic_signal);
                Self::write_u8(&mut data, o.object_refs.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::Container(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::Container);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.hidden);
                Self::write_u8(&mut data, o.object_refs.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::SoftKeyMask(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::SoftKeyMask);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u8(&mut data, o.objects.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_objects(&mut data, &o.objects);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::Key(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::Key);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u8(&mut data, o.key_code);
                Self::write_u8(&mut data, o.object_refs.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::Button(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::Button);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u8(&mut data, o.border_colour);
                Self::write_u8(&mut data, o.key_code);
                Self::write_u8(&mut data, o.options);
                Self::write_u8(&mut data, o.object_refs.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::InputBoolean(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::InputBoolean);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.foreground_colour);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u8(&mut data, o.value);
                Self::write_u8(&mut data, o.enabled);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::InputString(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::InputString);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u16(&mut data, o.font_attributes);
                Self::write_u16(&mut data, o.input_attributes);
                Self::write_u8(&mut data, o.options);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u8(&mut data, o.justification);
                Self::write_string(&mut data, &o.value);
                Self::write_u8(&mut data, o.enabled);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::InputNumber(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::InputNumber);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u16(&mut data, o.font_attributes);
                Self::write_u8(&mut data, o.options);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u32(&mut data, o.value);
                Self::write_u32(&mut data, o.min_value);
                Self::write_u32(&mut data, o.max_value);
                Self::write_i32(&mut data, o.offset);
                Self::write_f32(&mut data, o.scale);
                Self::write_u8(&mut data, o.nr_of_decimals);
                Self::write_u8(&mut data, o.format);
                Self::write_u8(&mut data, o.justification);
                Self::write_u8(&mut data, o.options2);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::InputList(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::InputList);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u8(&mut data, o.value);
                Self::write_u8(&mut data, o.list_items.len() as u8);
                Self::write_u8(&mut data, o.options);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_objects(&mut data, &o.list_items);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::OutputString(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputString);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u16(&mut data, o.font_attributes);
                Self::write_u8(&mut data, o.options);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u8(&mut data, o.justification);
                Self::write_u16(&mut data, o.value.len() as u16);
                Self::write_string(&mut data, &o.value);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::OutputNumber(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputNumber);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u16(&mut data, o.font_attributes);
                Self::write_u8(&mut data, o.options);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u32(&mut data, o.value);
                Self::write_i32(&mut data, o.offset);
                Self::write_f32(&mut data, o.scale);
                Self::write_u8(&mut data, o.nr_of_decimals);
                Self::write_u8(&mut data, o.format);
                Self::write_u8(&mut data, o.justification);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::OutputLine(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputLine);
                Self::write_u16(&mut data, o.line_attributes);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.line_direction);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::OutputRectangle(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputRectangle);
                Self::write_u16(&mut data, o.line_attributes);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.line_suppression);
                Self::write_u16(&mut data, o.fill_attributes);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::OutputEllipse(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputEllipse);
                Self::write_u16(&mut data, o.line_attributes);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.ellipse_type);
                Self::write_u8(&mut data, o.start_angle);
                Self::write_u8(&mut data, o.end_angle);
                Self::write_u16(&mut data, o.fill_attributes);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::OutputPolygon(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputPolygon);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u16(&mut data, o.line_attributes);
                Self::write_u16(&mut data, o.fill_attributes);
                Self::write_u8(&mut data, o.polygon_type);
                Self::write_u8(&mut data, o.points.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_points(&mut data, &o.points);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::OutputMeter(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputMeter);
                Self::write_u16(&mut data, o.width);
                Self::write_u8(&mut data, o.needle_colour);
                Self::write_u8(&mut data, o.border_colour);
                Self::write_u8(&mut data, o.arc_and_tick_colour);
                Self::write_u8(&mut data, o.options);
                Self::write_u8(&mut data, o.nr_of_ticks);
                Self::write_u8(&mut data, o.start_angle);
                Self::write_u8(&mut data, o.end_angle);
                Self::write_u16(&mut data, o.min_value);
                Self::write_u16(&mut data, o.max_value);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u16(&mut data, o.value);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::OutputLinearBarGraph(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputLinearBarGraph);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.colour);
                Self::write_u8(&mut data, o.target_line_colour);
                Self::write_u8(&mut data, o.options);
                Self::write_u8(&mut data, o.nr_of_ticks);
                Self::write_u16(&mut data, o.min_value);
                Self::write_u16(&mut data, o.max_value);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u16(&mut data, o.value);
                Self::write_u16(&mut data, o.target_value_variable_reference);
                Self::write_u16(&mut data, o.target_value);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::OutputArchedBarGraph(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputArchedBarGraph);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.colour);
                Self::write_u8(&mut data, o.target_line_colour);
                Self::write_u8(&mut data, o.options);
                Self::write_u8(&mut data, o.start_angle);
                Self::write_u8(&mut data, o.end_angle);
                Self::write_u16(&mut data, o.bar_graph_width);
                Self::write_u16(&mut data, o.min_value);
                Self::write_u16(&mut data, o.max_value);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u16(&mut data, o.value);
                Self::write_u16(&mut data, o.target_value_variable_reference);
                Self::write_u16(&mut data, o.target_value);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::PictureGraphic(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::PictureGraphic);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.actual_width);
                Self::write_u16(&mut data, o.actual_height);
                Self::write_u8(&mut data, o.format);
                Self::write_u8(&mut data, o.options);
                Self::write_u8(&mut data, o.transparency_colour);
                Self::write_u32(&mut data, o.data.len() as u32);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_bytes(&mut data, &o.data);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::NumberVariable(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::NumberVariable);
                Self::write_u32(&mut data, o.value);
            }
            Object::StringVariable(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::StringVariable);
                Self::write_string(&mut data, &o.value);
            }
            Object::FontAttributes(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::FontAttributes);
                Self::write_u8(&mut data, o.font_colour);
                Self::write_u8(&mut data, o.font_size);
                Self::write_u8(&mut data, o.font_type);
                Self::write_u8(&mut data, o.font_style);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::LineAttributes(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::LineAttributes);
                Self::write_u8(&mut data, o.line_colour);
                Self::write_u8(&mut data, o.line_width);
                Self::write_u16(&mut data, o.line_art);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::FillAttributes(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::FillAttributes);
                Self::write_u8(&mut data, o.fill_type);
                Self::write_u8(&mut data, o.fill_colour);
                Self::write_u16(&mut data, o.fill_pattern);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::InputAttributes(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::InputAttributes);
                Self::write_u8(&mut data, o.validation_type);
                Self::write_string(&mut data, &o.validation_string);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::ObjectPointer(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::ObjectPointer);
                Self::write_u16(&mut data, o.value);
            }
            Object::Macro(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::Macro);
                Self::write_u16(&mut data, o.commands.len() as u16);

                Self::write_bytes(&mut data, &o.commands);
            }
            Object::AuxiliaryFunctionType1(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::AuxiliaryFunctionType1);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u8(&mut data, o.function_type);
                Self::write_u8(&mut data, o.object_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
            }
            Object::AuxiliaryInputType1(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::AuxiliaryInputType1);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u8(&mut data, o.function_type);
                Self::write_u8(&mut data, o.input_id);
                Self::write_u8(&mut data, o.object_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
            }
            Object::AuxiliaryFunctionType2(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::AuxiliaryFunctionType2);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u8(&mut data, o.function_attributes);
                Self::write_u8(&mut data, o.object_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
            }
            Object::AuxiliaryInputType2(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::AuxiliaryInputType2);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u8(&mut data, o.function_attributes);
                Self::write_u8(&mut data, o.object_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
            }
            Object::AuxiliaryControlDesignatorType2(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::AuxiliaryControlDesignatorType2);
                Self::write_u8(&mut data, o.pointer_type);
                Self::write_u16(&mut data, o.auxiliary_object_id);
            }
            Object::WindowMask(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::WindowMask);
                Self::write_u8(&mut data, o.width);
                Self::write_u8(&mut data, o.height);
                Self::write_u8(&mut data, o.window_type);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u8(&mut data, o.options);
                Self::write_u16(&mut data, o.name);
                Self::write_u16(&mut data, o.window_title);
                Self::write_u16(&mut data, o.window_icon);
                Self::write_u8(&mut data, o.objects.len() as u8);
                Self::write_u8(&mut data, o.object_refs.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_objects(&mut data, &o.objects);
                Self::write_object_refs(&mut data, &o.object_refs);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::KeyGroup(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::KeyGroup);
                Self::write_u8(&mut data, o.options);
                Self::write_u16(&mut data, o.name);
                Self::write_u16(&mut data, o.key_group_icon);
                Self::write_u8(&mut data, o.objects.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_objects(&mut data, &o.objects);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::GraphicsContext(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::GraphicsContext);
                Self::write_u16(&mut data, o.viewport_width);
                Self::write_u16(&mut data, o.viewport_height);
                Self::write_i16(&mut data, o.viewport_x);
                Self::write_i16(&mut data, o.viewport_y);
                Self::write_u16(&mut data, o.canvas_width);
                Self::write_u16(&mut data, o.canvas_height);
                Self::write_f32(&mut data, o.viewport_zoom);
                Self::write_i16(&mut data, o.graphics_cursor_x);
                Self::write_i16(&mut data, o.graphics_cursor_y);
                Self::write_u8(&mut data, o.foreground_colour);
                Self::write_u8(&mut data, o.background_colour);
                Self::write_u16(&mut data, o.font_attributes_object);
                Self::write_u16(&mut data, o.line_attributes_object);
                Self::write_u16(&mut data, o.fill_attributes_object);
                Self::write_u8(&mut data, o.format);
                Self::write_u8(&mut data, o.options);
                Self::write_u8(&mut data, o.transparency_colour);
            }
            Object::OutputList(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::OutputList);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u16(&mut data, o.variable_reference);
                Self::write_u8(&mut data, o.value);
                Self::write_u8(&mut data, o.list_items.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_objects(&mut data, &o.list_items);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::ExtendedInputAttributes(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::ExtendedInputAttributes);
                Self::write_u8(&mut data, o.validation_type);
                Self::write_u8(&mut data, o.nr_of_code_planes);
                // TODO
            }
            Object::ColourMap(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::ColourMap);
                Self::write_u16(&mut data, o.colour_map.len() as u16);

                Self::write_bytes(&mut data, &o.colour_map);
            }
            Object::ObjectLabelReferenceList(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::ObjectLabelReferenceList);
                Self::write_u16(&mut data, o.object_labels.len() as u16);

                Self::write_object_labels(&mut data, &o.object_labels);
            }
            Object::ExternalObjectDefinition(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::ExternalObjectDefinition);
                Self::write_u8(&mut data, o.options);
                Self::write_name(&mut data, o.name);
                Self::write_u8(&mut data, o.objects.len() as u8);

                Self::write_objects(&mut data, &o.objects);
            }
            Object::ExternalReferenceName(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::ExternalReferenceName);
                Self::write_u8(&mut data, o.options);
                Self::write_name(&mut data, o.name);
            }
            Object::ExternalObjectPointer(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::ExternalObjectPointer);
                Self::write_u16(&mut data, o.default_object_id);
                Self::write_u16(&mut data, o.external_reference_name_id);
                Self::write_u16(&mut data, o.external_object_id);
            }
            Object::Animation(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::Animation);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u16(&mut data, o.refresh_interval);
                Self::write_u8(&mut data, o.value);
                Self::write_u8(&mut data, o.enabled);
                Self::write_u8(&mut data, o.first_child_index);
                Self::write_u8(&mut data, o.last_child_index);
                Self::write_u8(&mut data, o.default_child_index);
                Self::write_u8(&mut data, o.options);
                Self::write_u8(&mut data, o.object_refs.len() as u8);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_object_refs(&mut data, &o.object_refs);
                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
            Object::ColourPalette(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::ColourPalette);
                Self::write_u16(&mut data, o.options);
                Self::write_u16(&mut data, o.colours.len() as u16);

                Self::write_colours(&mut data, &o.colours);
            }
            Object::GraphicData(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::GraphicData);
                Self::write_u8(&mut data, o.format);
                Self::write_u32(&mut data, o.data.len() as u32);

                Self::write_bytes(&mut data, &o.data);
            }
            Object::WorkingSetSpecialControls(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::WorkingSetSpecialControls);
                Self::write_u16(&mut data, o.id_of_colour_map);
                Self::write_u16(&mut data, o.id_of_colour_palette);
                Self::write_u8(&mut data, o.language_pairs.len() as u8);

                Self::write_language_pairs(&mut data, &o.language_pairs);
            }
            Object::ScalesGraphic(o) => {
                Self::write_u16(&mut data, o.id);
                Self::write_u8(&mut data, ObjectType::ScalesGraphic);
                Self::write_u16(&mut data, o.width);
                Self::write_u16(&mut data, o.height);
                Self::write_u8(&mut data, o.scale_type);
                Self::write_u8(&mut data, o.options);
                Self::write_u16(&mut data, o.value);
                Self::write_u8(&mut data, o.macro_refs.len() as u8);

                Self::write_macro_refs(&mut data, &o.macro_refs);
            }
        }
        data
    }

    fn write_objects(data: &mut Vec<u8>, objects: &Vec<ObjectId>) {
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
    fn write_name(data: &mut Vec<u8>, val: impl Into<Name>) {
        let val: Name = val.into();
        data.extend::<[u8; 8]>(val.into());
    }
}
