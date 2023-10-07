use core::cell::Cell;


use super::*;

#[derive(Debug)]
pub struct ObjectPool {
    objects: Vec<Object>,
    colour_map: [u8; 256],
    colour_palette: [Colour; 256],
    supported_vt_version: VtVersion,

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
            supported_vt_version: VtVersion::default(),

            size_cache: Cell::new(None),
        }
    }

    pub fn size(&self) -> usize {
        if self.size_cache.get().is_none() {
            self.size_cache.set(Some(self.as_iop().len()));
        }
        self.size_cache.get().unwrap_or_default()
    }

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

    pub fn line_attributes_object_by_id(&self, id: ObjectId) -> Option<&LineAttributes> {
        match &self.object_by_id(id) {
            Some(Object::LineAttributes(o)) => Some(o),
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
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use super::*;



    fn get_pool_path() -> Box<Path> {
        Box::from(Path::new(
            &format!(
                "{}/resources/test/AgIsoStack-rs-test-pool.iop",
                match std::env::var("CARGO_MANIFEST_DIR") {
                    Err(why) =>
                        panic!("could not find environment variable 'CARGO_MANIFEST_DIR': {why}!"),
                    Ok(path) => path,
                }
            )
        ))
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
            id: ObjectId::from(13000),
            offset: Point {x: 12, y: 16}
        };

        assert_eq!(WorkingSet {
            id: ObjectId::from(0),
            background_colour: 7,
            selectable: true,
            active_mask: ObjectId::from(1000),
            object_refs: vec![line_obj_ws_ref],
            macro_refs: vec![],
            language_codes: vec!["en".to_string(), "de".to_string()],
        }, *object_pool.working_set_object().unwrap());

        /*CHECK DATA MASK*/

        let line_obj_dm_ref = ObjectRef {
            id: ObjectId::from(13000),
            offset: Point {x: 0, y: 0}
        };

        let input_boolean_obj_dm_ref = ObjectRef {
            id: ObjectId::from(7000),
            offset: Point {x: 100, y: 5}
        };

        let input_string_obj_dm_ref = ObjectRef {
            id: ObjectId::from(8000),
            offset: Point {x: 120, y: 10}
        };

        let input_number_obj_dm_ref = ObjectRef {
            id: ObjectId::from(9000),
            offset: Point {x: 120, y: 30}
        };

        let input_list_obj_dm_ref = ObjectRef {
            id: ObjectId::from(10000),
            offset: Point {x: 128, y: 56}
        };

        let output_number_obj_dm_ref = ObjectRef {
            id: ObjectId::from(12000),
            offset: Point {x: 5, y: 110}
        };

        let output_list_obj_dm_ref = ObjectRef {
            id: ObjectId::from(37000),
            offset: Point {x: 50, y: 110}
        };

        let meter_list_obj_dm_ref = ObjectRef {
            id: ObjectId::from(17000),
            offset: Point {x: 120, y: 110}
        };

        let linear_bargraph_obj_dm_ref = ObjectRef {
            id: ObjectId::from(18000),
            offset: Point {x: 180, y: 10}
        };

        let arched_bargraph_obj_dm_ref = ObjectRef {
            id: ObjectId::from(19000),
            offset: Point {x: 160, y: 110}
        };

        let button_obj_dm_ref = ObjectRef {
            id: ObjectId::from(6000),
            offset: Point {x: 10, y: 150}
        };

        let aux_object_pointer_obj_dm_ref = ObjectRef {
            id: ObjectId::from(33000),
            offset: Point {x: 130, y: 145}
        };

        let external_object_pointer_obj_dm_ref = ObjectRef {
            id: ObjectId::from(43000),
            offset: Point {x: 80, y: 185}
        };

        let graphics_context_obj_dm_ref = ObjectRef {
            id: ObjectId::from(36000),
            offset: Point {x: 5, y: 15}
        };

        let animation_obj_dm_ref = ObjectRef {
            id: ObjectId::from(44000),
            offset: Point {x: 10, y: 45}
        };

        let scaled_graphic_obj_dm_ref = ObjectRef {
            id: ObjectId::from(48000),
            offset: Point {x: 50, y: 5}
        };

        let polygon_obj_dm_ref = ObjectRef {
            id: ObjectId::from(16000),
            offset: Point {x: 80, y: 20}
        };

        let rectangle_obj_dm_ref = ObjectRef {
            id: ObjectId::from(14000),
            offset: Point {x: 10, y: 70}
        };

        let output_string_obj_dm_ref = ObjectRef {
            id: ObjectId::from(11000),
            offset: Point {x: 60, y: 70}
        };

        let animation_obj_dm_ref = ObjectRef {
            id: ObjectId::from(44000),
            offset: Point {x: 10, y: 45}
        };

        let container_obj_dm_ref = ObjectRef {
            id: ObjectId::from(3000),
            offset: Point {x: 70, y: 25}
        };

        let object_pointer_obj_dm_ref = ObjectRef {
            id: ObjectId::from(27000),
            offset: Point {x: 60, y: 160}
        };

        assert_eq!(DataMask {
            id: ObjectId::from(1000),
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
                linear_bargraph_obj_dm_ref,
                arched_bargraph_obj_dm_ref,
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
        }, *object_pool.data_mask_object_by_id(ObjectId::from(1000)).unwrap());

        /*CHECK ALARM MASK*/
        
        let line_obj_am_ref = ObjectRef {
            id: ObjectId::from(13000),
            offset: Point {x: 0, y: 0},
        };
        
        assert_eq!(AlarmMask {
            id: ObjectId::from(2000),
            background_colour: 7,
            soft_key_mask: ObjectId::NULL,
            priority: 1,
            acoustic_signal: 1,
            object_refs: vec![line_obj_am_ref],
            macro_refs: vec![],
        }, *object_pool.alarm_mask_object_by_id(ObjectId::from(2000)).unwrap());

        todo!("continue checking objects of pool of 'resources/test/AgIsoStack-rs-test-pool.iop'")
    }
}