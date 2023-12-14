use crate::object_pool::colour::Colour;
use crate::object_pool::object::{
    AlarmMask, Button, Container, DataMask, GraphicsContext, InputBoolean, InputList, InputNumber,
    InputString, Key, KeyGroup, LineAttributes, Object, OutputLine, OutputList, OutputNumber,
    OutputString, PictureGraphic, SoftKeyMask, WindowMask, WorkingSet,
};
use crate::object_pool::object_id::ObjectId;
use crate::object_pool::vt_version::VtVersion;
use crate::object_pool::ObjectType;
use core::cell::Cell;

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
        let mut op = Self::new();
        op.extend_with_iop(data);
        op
    }

    pub fn extend_with_iop<I>(&mut self, data: I)
    where
        I: IntoIterator<Item = u8>,
    {
        let mut data = data.into_iter();

        while let Ok(o) = Object::read(&mut data) {
            // By the standard, if there already is an object with the same ID, the new object
            // replaces the old one
            self.objects.retain(|x| x.id() != o.id());
            self.objects.push(o);
        }
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

/* !todo: implement tests / fix tests
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

        let _object_pool = ObjectPool::from_iop(buffer);
    }
}
*/
