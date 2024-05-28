use crate::object_pool::colour::Colour;
use crate::object_pool::object::{
    AlarmMask, Button, Container, DataMask, GraphicsContext, InputBoolean, InputList, InputNumber,
    InputString, Key, KeyGroup, LineAttributes, Object, OutputLine, OutputList, OutputNumber,
    OutputString, PictureGraphic, SoftKeyMask, WindowMask, WorkingSet,
};
use crate::object_pool::object_id::ObjectId;
use crate::object_pool::vt_version::VtVersion;
use crate::object_pool::ObjectType;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectPool {
    objects: Vec<Object>,
    colour_map: [u8; 256],
    colour_palette: [Colour; 256],
    _supported_vt_version: VtVersion,
}

impl ObjectPool {
    pub fn new() -> Self {
        // Setup the default colour map
        let mut colour_map = [0xFFu8; 256];
        for i in 0..colour_map.len() {
            colour_map[i] = i as u8;
        }

        ObjectPool {
            objects: Vec::new(),
            colour_map,
            colour_palette: Colour::COLOUR_PALETTE,
            _supported_vt_version: VtVersion::default(),
        }
    }

    pub fn size(&self) -> usize {
        self.objects.len()
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

    pub fn remove(&mut self, id: ObjectId) {
        self.objects.retain(|x| x.id() != id);
    }

    pub fn object_by_id(&self, id: ObjectId) -> Option<&Object> {
        self.objects.iter().find(|&o| o.id() == id)
    }

    pub fn object_mut_by_id(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.objects.iter_mut().find(|o| o.id() == id)
    }

    pub fn objects_by_type(&self, object_type: ObjectType) -> Vec<&Object> {
        self.objects
            .iter()
            .filter(|&o| o.object_type() == object_type)
            .collect()
    }

    pub fn objects_by_types(&self, object_types: &[ObjectType]) -> Vec<&Object> {
        self.objects
            .iter()
            .filter(|&o| object_types.contains(&o.object_type()))
            .collect()
    }

    pub fn parent_objects(&self, id: ObjectId) -> Vec<&Object> {
        self.objects
            .iter()
            .filter(|&o| o.referenced_objects().contains(&id))
            .collect()
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
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

    pub fn color_to_index(&self, color: Colour) -> Option<u8> {
        self.colour_map
            .iter()
            .find(|&&c| self.colour_palette[c as usize] == color)
            .map(|&c| c)
    }

    ///
    /// Returns the needed width and height of the object to fit its content.
    ///
    pub fn content_size(&self, object: &Object) -> (u16, u16) {
        // If the object is a sized object, return its size
        if let Some(sized) = object.as_sized_object() {
            return (sized.width(), sized.height());
        }

        // Some special cases where the content is not an object ref
        match object {
            Object::SoftKeyMask(o) => {
                let mut width = 0;
                let mut height = 0;
                for object_id in o.objects.iter() {
                    if let Some(object) = self.object_by_id(*object_id) {
                        let (object_width, object_height) = self.content_size(object);
                        width = width.max(object_width);
                        height = height.max(object_height);
                    }
                }
                return (width, height);
            }
            Object::ObjectPointer(o) => {
                if let Some(id) = o.value.into() {
                    if let Some(object) = self.object_by_id(id) {
                        return self.content_size(object);
                    }
                }
            }
            _ => (),
        }

        // Otherwise, return the largest x and y reached by one of the object refs
        let object_refs = match object {
            Object::WorkingSet(o) => o.object_refs.iter(),
            Object::DataMask(o) => o.object_refs.iter(),
            Object::AlarmMask(o) => o.object_refs.iter(),
            Object::Key(o) => o.object_refs.iter(),
            Object::AuxiliaryFunctionType1(o) => o.object_refs.iter(),
            Object::AuxiliaryInputType1(o) => o.object_refs.iter(),
            Object::AuxiliaryFunctionType2(o) => o.object_refs.iter(),
            Object::AuxiliaryInputType2(o) => o.object_refs.iter(),
            _ => return (0, 0),
        };

        let mut width = 0;
        let mut height = 0;
        for object_ref in object_refs {
            if let Some(object) = self.object_by_id(object_ref.id) {
                let (object_width, object_height) = self.content_size(object);
                width = width.max(object_width as i16 + object_ref.offset.x);
                height = height.max(object_height as i16 + object_ref.offset.y);
            }
        }
        (width.max(0) as u16, height.max(0) as u16)
    }

    ///
    /// Calculates the minimum size for all data masks and keys in soft key masks
    ///
    pub fn get_minimum_mask_sizes(&self) -> (u16, (u16, u16)) {
        let mut mask_size = 0;
        let mut soft_key_size = (0, 0);

        for mask in self.objects_by_types(&[ObjectType::DataMask, ObjectType::AlarmMask]) {
            let size = self.content_size(mask);
            mask_size = mask_size.max(size.0.max(size.1));

            let soft_key_mask_id = match mask {
                Object::DataMask(o) => o.soft_key_mask.0,
                Object::AlarmMask(o) => o.soft_key_mask.0,
                _ => None,
            };
            if let Some(soft_key_mask_id) = soft_key_mask_id {
                if let Some(soft_key_mask) = self.object_by_id(soft_key_mask_id) {
                    soft_key_size = soft_key_size.max(self.content_size(soft_key_mask));
                }
            }
        }

        (mask_size, soft_key_size)
    }
}

impl Default for ObjectPool {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for ObjectPool {
    type Item = Object;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.into_iter()
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
