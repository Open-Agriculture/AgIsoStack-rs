use core::cell::Cell;

use super::*;

#[derive(Debug)]
pub struct ObjectPool {
    objects: Vec<Object>,
    colour_map: [u8; 256],
    colour_palette: [Colour; 256],
    _supported_vt_version: VtVersion,

    size_cache: Cell<Option<usize>>,
}

impl ObjectPool {
    pub fn new() -> Self {
        // Setup the default colour map
        let mut colour_map = [0xFFu8; 256];
        for i in 0..(colour_map.len() as u8) {
            colour_map[i as usize] = i;
        }

        ObjectPool {
            objects: Vec::new(),
            colour_map,
            colour_palette: Colour::COLOUR_PALETTE,
            _supported_vt_version: VtVersion::default(),

            size_cache: Cell::new(None),
        }
    }

    pub fn size(&self) -> usize {
        if self.size_cache.get().is_none() {
            self.size_cache.set(Some(self.as_iop().len()));
        }
        self.size_cache.get().unwrap_or_default()
    }

    ///
    /// Loads the binary encoded object pool from a buffer in accordance
    /// with ISO 11783-6 Annex B (object definitions) and returns the
    /// parsed [`ObjectPool`].
    ///
    /// # Arguments
    ///
    /// * `data` - A buffer containing the binary encoded object pool
    ///
    /// # Examples
    /// ```
    /// use std::fs::File;
    /// use std::io::Read;
    /// use std::path::Path;
    /// use ag_iso_stack::object_pool::ObjectPool;
    ///
    /// let example_path = Path::new("C:/project/resources/test/AgIsoStack-rs-test-pool.iop");
    /// let mut pool_file = match File::open(example_path) {
    ///             Err(why) => panic!("couldn't open {:?}: {}", example_path.to_str(), why),
    ///             Ok(file) => file,
    /// };
    ///
    /// let mut buffer = Vec::new();
    /// match pool_file.read_to_end(&mut buffer) {
    ///             Ok(size) => size,
    ///             Err(why) => panic!("Could not read object pool file: {why}"),
    /// };
    ///
    /// let object_pool = ObjectPool::from_iop(buffer);
    /// ```
    ///
    pub fn from_iop<I>(data: I) -> Self
    where
        I: IntoIterator<Item = u8>,
    {
        let mut data = data.into_iter();

        let mut op = Self::new();

        while let Ok(o) = Object::read(&mut data) {
            op.objects.push(o);
        }

        op
    }

    pub fn as_iop(&self) -> Vec<u8> {
        let mut data = Vec::new();

        for obj in &self.objects {
            data.extend(obj.write());
        }

        data
    }

    pub fn add(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    pub fn object_by_id(&self, id: ObjectId) -> Option<&Object> {
        self.objects.iter().find(|&o| o.id() == id)
    }

    pub fn objects_by_type(&self, object_type: ObjectType) -> Vec<&Object> {
        self.objects
            .iter()
            .filter(|&o| o.object_type() == object_type)
            .collect()
    }

    // Get objects by type

    pub fn working_set_object(&self) -> Option<&WorkingSet> {
        match &self.objects_by_type(ObjectType::WorkingSet).first() {
            Some(Object::WorkingSet(o)) => Some(o),
            _ => None,
        }
    }

    pub fn data_mask_objects(&self) -> Vec<&DataMask> {
        let r: Vec<&DataMask> = self
            .objects_by_type(ObjectType::DataMask)
            .iter()
            .filter_map(|&o| match o {
                Object::DataMask(o) => Some(o),
                _ => None,
            })
            .collect();
        r
    }

    pub fn picture_graphic_objects(&self) -> Vec<&PictureGraphic> {
        let r: Vec<&PictureGraphic> = self
            .objects_by_type(ObjectType::PictureGraphic)
            .iter()
            .filter_map(|&o| match o {
                Object::PictureGraphic(o) => Some(o),
                _ => None,
            })
            .collect();
        r
    }

    // Get typed objects by id

    pub fn data_mask_object_by_id(&self, id: ObjectId) -> Option<&DataMask> {
        match &self.object_by_id(id) {
            Some(Object::DataMask(o)) => Some(o),
            _ => None,
        }
    }

    pub fn alarm_mask_object_by_id(&self, id: ObjectId) -> Option<&AlarmMask> {
        match &self.object_by_id(id) {
            Some(Object::AlarmMask(o)) => Some(o),
            _ => None,
        }
    }

    pub fn soft_key_mask_object_by_id(&self, id: ObjectId) -> Option<&SoftKeyMask> {
        match &self.object_by_id(id) {
            Some(Object::SoftKeyMask(o)) => Some(o),
            _ => None,
        }
    }

    pub fn key_group_object_by_id(&self, id: ObjectId) -> Option<&KeyGroup> {
        match &self.object_by_id(id) {
            Some(Object::KeyGroup(o)) => Some(o),
            _ => None,
        }
    }

    pub fn window_mask_object_by_id(&self, id: ObjectId) -> Option<&WindowMask> {
        match &self.object_by_id(id) {
            Some(Object::WindowMask(o)) => Some(o),
            _ => None,
        }
    }

    pub fn container_object_by_id(&self, id: ObjectId) -> Option<&Container> {
        match &self.object_by_id(id) {
            Some(Object::Container(o)) => Some(o),
            _ => None,
        }
    }

    pub fn key_object_by_id(&self, id: ObjectId) -> Option<&Key> {
        match &self.object_by_id(id) {
            Some(Object::Key(o)) => Some(o),
            _ => None,
        }
    }

    pub fn button_object_by_id(&self, id: ObjectId) -> Option<&Button> {
        match &self.object_by_id(id) {
            Some(Object::Button(o)) => Some(o),
            _ => None,
        }
    }

    pub fn input_boolean_object_by_id(&self, id: ObjectId) -> Option<&InputBoolean> {
        match &self.object_by_id(id) {
            Some(Object::InputBoolean(o)) => Some(o),
            _ => None,
        }
    }

    pub fn input_string_object_by_id(&self, id: ObjectId) -> Option<&InputString> {
        match &self.object_by_id(id) {
            Some(Object::InputString(o)) => Some(o),
            _ => None,
        }
    }

    pub fn input_number_object_by_id(&self, id: ObjectId) -> Option<&InputNumber> {
        match &self.object_by_id(id) {
            Some(Object::InputNumber(o)) => Some(o),
            _ => None,
        }
    }

    pub fn input_list_object_by_id(&self, id: ObjectId) -> Option<&InputList> {
        match &self.object_by_id(id) {
            Some(Object::InputList(o)) => Some(o),
            _ => None,
        }
    }

    pub fn output_string_object_by_id(&self, id: ObjectId) -> Option<&OutputString> {
        match &self.object_by_id(id) {
            Some(Object::OutputString(o)) => Some(o),
            _ => None,
        }
    }

    pub fn output_number_object_by_id(&self, id: ObjectId) -> Option<&OutputNumber> {
        match &self.object_by_id(id) {
            Some(Object::OutputNumber(o)) => Some(o),
            _ => None,
        }
    }

    pub fn output_list_object_by_id(&self, id: ObjectId) -> Option<&OutputList> {
        match &self.object_by_id(id) {
            Some(Object::OutputList(o)) => Some(o),
            _ => None,
        }
    }

    pub fn output_line_object_by_id(&self, id: ObjectId) -> Option<&OutputLine> {
        match &self.object_by_id(id) {
            Some(Object::OutputLine(o)) => Some(o),
            _ => None,
        }
    }

    pub fn line_attributes_object_by_id(&self, id: ObjectId) -> Option<&LineAttributes> {
        match &self.object_by_id(id) {
            Some(Object::LineAttributes(o)) => Some(o),
            _ => None,
        }
    }

    pub fn graphics_context_object_by_id(&self, id: ObjectId) -> Option<&GraphicsContext> {
        match &self.object_by_id(id) {
            Some(Object::GraphicsContext(o)) => Some(o),
            _ => None,
        }
    }

    pub fn color_by_index(&self, index: u8) -> Colour {
        self.colour_palette[self.colour_map[index as usize] as usize]
    }
}

impl Default for ObjectPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    fn get_pool_path() -> Box<Path> {
        Box::from(Path::new(&format!(
            "{}/resources/test/AgIsoStack-rs-test-pool.iop",
            match std::env::var("CARGO_MANIFEST_DIR") {
                Err(_why) =>
                    panic!("could not find environment variable 'CARGO_MANIFEST_DIR': {_why}!"),
                Ok(path) => path,
            }
        )))
    }

    fn get_pool_file() -> File {
        match File::open(get_pool_path()) {
            Err(why) => panic!("couldn't open {:?}: {}", get_pool_path().to_str(), why),
            Ok(file) => file,
        }
    }

    #[test]
    fn test_from_iop() {
        let mut buffer = Vec::new();
        match get_pool_file().read_to_end(&mut buffer) {
            Ok(size) => size,
            Err(why) => panic!("Could not read object pool file: {why}"),
        };
        let object_pool = ObjectPool::from_iop(buffer);

        /*CHECK WORKING SET*/

        let line_obj_ws_ref = ObjectRef {
            id: ObjectId::try_from(13000)
                .unwrap_or_else(|_| panic!("Could not convert 13000 to ObjectId!")),
            offset: Point { x: 12, y: 16 },
        };

        assert_eq!(
            WorkingSet {
                id: ObjectId::try_from(0)
                    .unwrap_or_else(|_| panic!("Could not convert 0 to ObjectId!")),
                background_colour: 7,
                selectable: true,
                active_mask: ObjectId::try_from(1000)
                    .unwrap_or_else(|_| panic!("Could not convert 1000 to ObjectId!")),
                object_refs: vec![line_obj_ws_ref],
                macro_refs: vec![],
                language_codes: vec!["en".to_string(), "de".to_string()],
            },
            *object_pool.working_set_object().unwrap()
        );

        /*CHECK DATA MASK*/

        let line_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(13000)
                .unwrap_or_else(|_| panic!("Could not convert 13000 to ObjectId!")),
            offset: Point { x: 0, y: 0 },
        };

        let input_boolean_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(7000)
                .unwrap_or_else(|_| panic!("Could not convert 7000 to ObjectId!")),
            offset: Point { x: 100, y: 5 },
        };

        let input_string_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(8000)
                .unwrap_or_else(|_| panic!("Could not convert 8000 to ObjectId!")),
            offset: Point { x: 120, y: 10 },
        };

        let input_number_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(9000)
                .unwrap_or_else(|_| panic!("Could not convert 9000 to ObjectId!")),
            offset: Point { x: 120, y: 30 },
        };

        let input_list_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(10000)
                .unwrap_or_else(|_| panic!("Could not convert 10000 to ObjectId!")),
            offset: Point { x: 128, y: 56 },
        };

        let output_number_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(12000)
                .unwrap_or_else(|_| panic!("Could not convert 12000 to ObjectId!")),
            offset: Point { x: 5, y: 110 },
        };

        let output_list_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(37000)
                .unwrap_or_else(|_| panic!("Could not convert 37000 to ObjectId!")),
            offset: Point { x: 50, y: 110 },
        };

        let meter_list_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(17000)
                .unwrap_or_else(|_| panic!("Could not convert 17000 to ObjectId!")),
            offset: Point { x: 120, y: 110 },
        };

        let linear_bar_graph_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(18000)
                .unwrap_or_else(|_| panic!("Could not convert 18000 to ObjectId!")),
            offset: Point { x: 180, y: 10 },
        };

        let arched_bar_graph_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(19000)
                .unwrap_or_else(|_| panic!("Could not convert 19000 to ObjectId!")),
            offset: Point { x: 160, y: 110 },
        };

        let button_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(6000)
                .unwrap_or_else(|_| panic!("Could not convert 6000 to ObjectId!")),
            offset: Point { x: 10, y: 150 },
        };

        let aux_object_pointer_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(33000)
                .unwrap_or_else(|_| panic!("Could not convert 33000 to ObjectId!")),
            offset: Point { x: 130, y: 145 },
        };

        let external_object_pointer_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(43000)
                .unwrap_or_else(|_| panic!("Could not convert 43000 to ObjectId!")),
            offset: Point { x: 80, y: 185 },
        };

        let graphics_context_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(36000)
                .unwrap_or_else(|_| panic!("Could not convert 36000 to ObjectId!")),
            offset: Point { x: 5, y: 15 },
        };

        let scaled_graphic_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(48000)
                .unwrap_or_else(|_| panic!("Could not convert 48000 to ObjectId!")),
            offset: Point { x: 50, y: 5 },
        };

        let polygon_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(16000)
                .unwrap_or_else(|_| panic!("Could not convert 16000 to ObjectId!")),
            offset: Point { x: 80, y: 20 },
        };

        let rectangle_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(14000)
                .unwrap_or_else(|_| panic!("Could not convert 14000 to ObjectId!")),
            offset: Point { x: 10, y: 70 },
        };

        let output_string_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(11000)
                .unwrap_or_else(|_| panic!("Could not convert 11000 to ObjectId!")),
            offset: Point { x: 60, y: 70 },
        };

        let animation_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(44000)
                .unwrap_or_else(|_| panic!("Could not convert 44000 to ObjectId!")),
            offset: Point { x: 10, y: 45 },
        };

        let container_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(3000)
                .unwrap_or_else(|_| panic!("Could not convert 3000 to ObjectId!")),
            offset: Point { x: 70, y: 25 },
        };

        let object_pointer_obj_dm_ref = ObjectRef {
            id: ObjectId::try_from(27000)
                .unwrap_or_else(|_| panic!("Could not convert 27000 to ObjectId!")),
            offset: Point { x: 60, y: 160 },
        };

        assert_eq!(
            DataMask {
                id: ObjectId::try_from(1000)
                    .unwrap_or_else(|_| panic!("Could not convert 1000 to ObjectId!")),
                background_colour: 7,
                soft_key_mask: ObjectId::NULL,
                object_refs: vec![
                    line_obj_dm_ref,
                    input_boolean_obj_dm_ref,
                    input_string_obj_dm_ref,
                    input_number_obj_dm_ref,
                    input_list_obj_dm_ref,
                    output_number_obj_dm_ref,
                    output_list_obj_dm_ref,
                    meter_list_obj_dm_ref,
                    linear_bar_graph_obj_dm_ref,
                    arched_bar_graph_obj_dm_ref,
                    button_obj_dm_ref,
                    aux_object_pointer_obj_dm_ref,
                    external_object_pointer_obj_dm_ref,
                    graphics_context_obj_dm_ref,
                    animation_obj_dm_ref,
                    scaled_graphic_obj_dm_ref,
                    polygon_obj_dm_ref,
                    rectangle_obj_dm_ref,
                    output_string_obj_dm_ref,
                    container_obj_dm_ref,
                    object_pointer_obj_dm_ref
                ],
                macro_refs: vec![],
            },
            *object_pool
                .data_mask_object_by_id(
                    ObjectId::try_from(1000)
                        .unwrap_or_else(|_| panic!("Could not convert 1000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK ALARM MASK*/

        let line_obj_am_ref = ObjectRef {
            id: ObjectId::try_from(13000)
                .unwrap_or_else(|_| panic!("Could not convert 13000 to ObjectId!")),
            offset: Point { x: 0, y: 0 },
        };

        assert_eq!(
            AlarmMask {
                id: ObjectId::try_from(2000)
                    .unwrap_or_else(|_| panic!("Could not convert 2000 to ObjectId!")),
                background_colour: 7,
                soft_key_mask: ObjectId::NULL,
                priority: 1,
                acoustic_signal: 1,
                object_refs: vec![line_obj_am_ref],
                macro_refs: vec![],
            },
            *object_pool
                .alarm_mask_object_by_id(
                    ObjectId::try_from(2000)
                        .unwrap_or_else(|_| panic!("Could not convert 2000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK SOFT KEY MASK*/

        let soft_key_obj_sm_id = ObjectId::try_from(5000)
            .unwrap_or_else(|_| panic!("Could not convert 5000 to ObjectId!"));

        assert_eq!(
            SoftKeyMask {
                id: ObjectId::try_from(4000)
                    .unwrap_or_else(|_| panic!("Could not convert 4000 to ObjectId!")),
                background_colour: 7,
                objects: vec![soft_key_obj_sm_id],
                macro_refs: vec![],
            },
            *object_pool
                .soft_key_mask_object_by_id(
                    ObjectId::try_from(4000)
                        .unwrap_or_else(|_| panic!("Could not convert 4000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK KEY GROUP*/

        let key_group_options = KeyGroupOptions {
            available: false,
            transparent: false,
        };

        let object_pointer_obj_id = ObjectId::try_from(27001)
            .unwrap_or_else(|_| panic!("Could not convert 27001 to ObjectId!"));

        assert_eq!(
            KeyGroup {
                id: ObjectId::try_from(35000)
                    .unwrap_or_else(|_| panic!("Could not convert 35000 to ObjectId!")),
                options: key_group_options,
                name: ObjectId::try_from(11000)
                    .unwrap_or_else(|_| panic!("Could not convert 11000 to ObjectId!")),
                key_group_icon: ObjectId::NULL,
                objects: vec![object_pointer_obj_id],
                macro_refs: vec![],
            },
            *object_pool
                .key_group_object_by_id(
                    ObjectId::try_from(35000)
                        .unwrap_or_else(|_| panic!("Could not convert 35000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK WINDOW MASK*/

        let output_string_obj_id = ObjectId::try_from(11000)
            .unwrap_or_else(|_| panic!("Could not convert 11000 to ObjectId!"));
        let line_obj_wm_ref = ObjectRef {
            id: ObjectId::try_from(13000)
                .unwrap_or_else(|_| panic!("Could not convert 13000 to ObjectId!")),
            offset: Point { x: 0, y: 0 },
        };

        assert_eq!(
            WindowMask {
                id: ObjectId::try_from(34000)
                    .unwrap_or_else(|_| panic!("Could not convert 34000 to ObjectId!")),
                cell_format: WindowMaskCellFormat::CF1x1,
                window_type: WindowType::FreeForm,
                background_colour: 7,
                options: WindowMaskOptions {
                    available: true,
                    transparent: true,
                },
                name: output_string_obj_id,
                window_title: ObjectId::NULL,
                window_icon: ObjectId::NULL,
                objects: vec![],
                object_refs: vec![line_obj_wm_ref],
                macro_refs: vec![],
            },
            *object_pool
                .window_mask_object_by_id(
                    ObjectId::try_from(34000)
                        .unwrap_or_else(|_| panic!("Could not convert 34000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK CONTAINER*/

        let polygon_obj_container_ref = ObjectRef {
            id: ObjectId::try_from(16000)
                .unwrap_or_else(|_| panic!("Could not convert 16000 to ObjectId!")),
            offset: Point { x: 70, y: 0 },
        };

        let rectangle_obj_container_ref = ObjectRef {
            id: ObjectId::try_from(14000)
                .unwrap_or_else(|_| panic!("Could not convert 14000 to ObjectId!")),
            offset: Point { x: 0, y: 50 },
        };

        let output_string_obj_container_ref = ObjectRef {
            id: ObjectId::try_from(11000)
                .unwrap_or_else(|_| panic!("Could not convert 11000 to ObjectId!")),
            offset: Point { x: 50, y: 50 },
        };

        assert_eq!(
            Container {
                id: ObjectId::try_from(3000)
                    .unwrap_or_else(|_| panic!("Could not convert 3000 to ObjectId!")),
                width: 110,
                height: 80,
                hidden: false,
                object_refs: vec![
                    polygon_obj_container_ref,
                    rectangle_obj_container_ref,
                    output_string_obj_container_ref,
                ],
                macro_refs: vec![],
            },
            *object_pool
                .container_object_by_id(
                    ObjectId::try_from(3000)
                        .unwrap_or_else(|_| panic!("Could not convert 3000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK SOFT KEY*/

        let line_obj_soft_key_ref = ObjectRef {
            id: ObjectId::try_from(13000)
                .unwrap_or_else(|_| panic!("Could not convert 13000 to ObjectId!")),
            offset: Point { x: 0, y: 0 },
        };

        assert_eq!(
            Key {
                id: ObjectId::try_from(5000)
                    .unwrap_or_else(|_| panic!("Could not convert 5000 to ObjectId!")),
                background_colour: 8,
                key_code: 1,
                object_refs: vec![line_obj_soft_key_ref],
                macro_refs: vec![],
            },
            *object_pool
                .key_object_by_id(
                    ObjectId::try_from(5000)
                        .unwrap_or_else(|_| panic!("Could not convert 5000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK BUTTON*/

        let button_options = ButtonOptions {
            latchable: false,
            state: ButtonState::RELEASED,
            suppress_border: false,
            transparent_background: false,
            disabled: false,
            no_border: false,
        };

        let output_string_obj_button_ref = ObjectRef {
            id: ObjectId::try_from(11001)
                .unwrap_or_else(|_| panic!("Could not convert 11001 to ObjectId!")),
            offset: Point { x: -4, y: -6 },
        };

        assert_eq!(
            Button {
                id: ObjectId::try_from(6000)
                    .unwrap_or_else(|_| panic!("Could not convert 6000 to ObjectId!")),
                width: 30,
                height: 20,
                background_colour: 8,
                border_colour: 8,
                key_code: 1,
                options: button_options,
                object_refs: vec![output_string_obj_button_ref],
                macro_refs: vec![],
            },
            *object_pool
                .button_object_by_id(
                    ObjectId::try_from(6000)
                        .unwrap_or_else(|_| panic!("Could not convert 6000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK INPUT BOOLEAN*/

        assert_eq!(
            InputBoolean {
                id: ObjectId::try_from(7000)
                    .unwrap_or_else(|_| panic!("Could not convert 7000 to ObjectId!")),
                background_colour: 1,
                width: 10,
                foreground_colour: ObjectId::try_from(23000)
                    .unwrap_or_else(|_| panic!("Could not convert 23000 to ObjectId!")),
                variable_reference: ObjectId::NULL,
                value: false,
                enabled: true,
                macro_refs: vec![],
            },
            *object_pool
                .input_boolean_object_by_id(
                    ObjectId::try_from(7000)
                        .unwrap_or_else(|_| panic!("Could not convert 7000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK INPUT STRING*/

        assert_eq!(
            InputString {
                id: ObjectId::try_from(8000)
                    .unwrap_or_else(|_| panic!("Could not convert 8000 to ObjectId!")),
                width: 50,
                height: 15,
                background_colour: 1,
                font_attributes: ObjectId::try_from(23002)
                    .unwrap_or_else(|_| panic!("Could not convert 23002 to ObjectId!")),
                input_attributes: ObjectId::NULL,
                options: InputStringOptions {
                    transparent: false,
                    auto_wrap: true,
                    wrap_on_hyphen: false,
                },
                variable_reference: ObjectId::NULL,
                justification: Alignment {
                    horizontal: HorizontalAlignment::Left,
                    vertical: VerticalAlignment::Top,
                },
                value: "abc ".to_string(),
                enabled: true,
                macro_refs: vec![],
            },
            *object_pool
                .input_string_object_by_id(
                    ObjectId::try_from(8000)
                        .unwrap_or_else(|_| panic!("Could not convert 8000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK INPUT NUMBER*/

        assert_eq!(
            InputNumber {
                id: ObjectId::try_from(9000)
                    .unwrap_or_else(|_| panic!("Could not convert 9000 to ObjectId!")),
                width: 50,
                height: 20,
                background_colour: 1,
                font_attributes: ObjectId::try_from(23001)
                    .unwrap_or_else(|_| panic!("Could not convert 23001 to ObjectId!")),
                options: NumberOptions {
                    transparent: false,
                    truncate: false,
                    display_zero_as_blank: false,
                    display_leading_zeros: false,
                },
                variable_reference: ObjectId::NULL,
                value: 123,
                min_value: 0,
                max_value: 65535,
                offset: 0,
                scale: 1.0,
                nr_of_decimals: 0,
                format: FormatType::Decimal,
                justification: Alignment {
                    horizontal: HorizontalAlignment::Left,
                    vertical: VerticalAlignment::Top,
                },
                options2: InputNumberOptions {
                    enabled: true,
                    real_time_editing: false,
                },
                macro_refs: vec![],
            },
            *object_pool
                .input_number_object_by_id(
                    ObjectId::try_from(9000)
                        .unwrap_or_else(|_| panic!("Could not convert 9000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK INPUT LIST*/

        assert_eq!(
            InputList {
                id: ObjectId::try_from(10000)
                    .unwrap_or_else(|_| panic!("Could not convert 10000 to ObjectId!")),
                width: 56,
                height: 48,
                variable_reference: ObjectId::NULL,
                value: 0,
                options: InputListOptions {
                    enabled: true,
                    real_time_editing: false,
                },
                list_items: vec![ObjectId::try_from(14000)
                    .unwrap_or_else(|_| panic!("Could not convert 14000 to ObjectId!"))],
                macro_refs: vec![],
            },
            *object_pool
                .input_list_object_by_id(
                    ObjectId::try_from(10000)
                        .unwrap_or_else(|_| panic!("Could not convert 10000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK OUTPUT STRING*/

        assert_eq!(
            OutputString {
                id: ObjectId::try_from(11000)
                    .unwrap_or_else(|_| panic!("Could not convert 11000 to ObjectId!")),
                width: 40,
                height: 20,
                background_colour: 1,
                font_attributes: ObjectId::try_from(23003)
                    .unwrap_or_else(|_| panic!("Could not convert 23003 to ObjectId!")),
                options: OutputStringOptions {
                    transparent: false,
                    auto_wrap: false,
                    wrap_on_hyphen: false,
                },
                variable_reference: Default::default(),
                justification: Alignment {
                    horizontal: HorizontalAlignment::Left,
                    vertical: VerticalAlignment::Top,
                },
                value: "Text".to_string(),
                macro_refs: vec![],
            },
            *object_pool
                .output_string_object_by_id(
                    ObjectId::try_from(11000)
                        .unwrap_or_else(|_| panic!("Could not convert 11000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK OUTPUT NUMBER*/

        assert_eq!(
            OutputNumber {
                id: ObjectId::try_from(12000)
                    .unwrap_or_else(|_| panic!("Could not convert 12000 to ObjectId!")),
                width: 30,
                height: 20,
                background_colour: 1,
                font_attributes: ObjectId::try_from(23004)
                    .unwrap_or_else(|_| panic!("Could not convert 23004 to ObjectId!")),
                options: NumberOptions {
                    transparent: false,
                    truncate: false,
                    display_zero_as_blank: false,
                    display_leading_zeros: false,
                },
                variable_reference: ObjectId::NULL,
                value: 0,
                offset: 0,
                scale: 1.0,
                nr_of_decimals: 0,
                format: FormatType::Decimal,
                justification: Alignment {
                    horizontal: HorizontalAlignment::Left,
                    vertical: VerticalAlignment::Top,
                },
                macro_refs: vec![],
            },
            *object_pool
                .output_number_object_by_id(
                    ObjectId::try_from(12000)
                        .unwrap_or_else(|_| panic!("Could not convert 12000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK OUTPUT LIST*/

        assert_eq!(
            OutputList {
                id: ObjectId::try_from(37000)
                    .unwrap_or_else(|_| panic!("Could not convert 37000 to ObjectId!")),
                width: 60,
                height: 40,
                variable_reference: ObjectId::NULL,
                value: 0,
                list_items: vec![ObjectId::try_from(7000)
                    .unwrap_or_else(|_| panic!("Could not convert 7000 to ObjectId!"))],
                macro_refs: vec![],
            },
            *object_pool
                .output_list_object_by_id(
                    ObjectId::try_from(37000)
                        .unwrap_or_else(|_| panic!("Could not convert 37000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK OUTPUT LINE*/

        assert_eq!(
            OutputLine {
                id: ObjectId::try_from(13000)
                    .unwrap_or_else(|_| panic!("Could not convert 13000 to ObjectId!")),
                line_attributes: ObjectId::try_from(24000)
                    .unwrap_or_else(|_| panic!("Could not convert 24000 to ObjectId!")),
                width: 50,
                height: 15,
                line_direction: LineDirection::BottomLeftToTopRight,
                macro_refs: vec![],
            },
            *object_pool
                .output_line_object_by_id(
                    ObjectId::try_from(13000)
                        .unwrap_or_else(|_| panic!("Could not convert 13000 to ObjectId!"))
                )
                .unwrap()
        );

        /*CHECK GRAPHICS CONTEXT */

        assert_eq!(
            GraphicsContext {
                id: ObjectId::try_from(36000)
                    .unwrap_or_else(|_| panic!("Could not convert 36000 to ObjectId!")),
                viewport_width: 40,
                viewport_height: 25,
                viewport_x: 0,
                viewport_y: 0,
                canvas_width: 40,
                canvas_height: 20,
                viewport_zoom: 0.0,
                graphics_cursor_x: 0,
                graphics_cursor_y: 0,
                foreground_colour: 0,
                background_colour: 1,
                font_attributes_object: ObjectId::NULL,
                line_attributes_object: ObjectId::NULL,
                fill_attributes_object: ObjectId::NULL,
                format: ColorFormat::ColorMonochrome,
                options: GraphicsContextOptions {
                    transparent: false,
                    color: ColorOption::ForegroundBackground,
                },
                transparency_colour: 0,
            },
            *object_pool
                .graphics_context_object_by_id(
                    ObjectId::try_from(36000)
                        .unwrap_or_else(|_| panic!("Could not convert 36000 to ObjectId!"))
                )
                .unwrap()
        );

        todo!("continue checking objects of pool of 'resources/test/AgIsoStack-rs-test-pool.iop'")
    }
}
