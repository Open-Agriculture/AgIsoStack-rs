use crate::object_pool::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectId {
    id: u16,
}

impl ObjectId {
    const NULL: ObjectId = ObjectId { id: u16::MAX };

    pub fn new(id: u16) -> Result<Self, ParseError> {
        if id == Self::NULL.id {
            Err(ParseError::UnknownObjectType)
        } else {
            Ok(ObjectId { id })
        }
    }
}

pub struct NullableObjectId(Option<ObjectId>);

impl NullableObjectId {
    pub const NULL: NullableObjectId = NullableObjectId(None);
    pub fn new(id: u16) -> Self {
        if id == ObjectId::NULL.id {
            NullableObjectId(None)
        } else {
            NullableObjectId(Some(ObjectId::new(id).unwrap()))
        }
    }
}

impl Default for NullableObjectId {
    fn default() -> Self {
        NullableObjectId::NULL
    }
}

impl From<u16> for NullableObjectId {
    fn from(id: u16) -> Self {
        NullableObjectId::new(id)
    }
}

impl From<NullableObjectId> for u16 {
    fn from(id: NullableObjectId) -> Self {
        match id.0 {
            Some(id) => id.id,
            None => u16::from(ObjectId::NULL),
        }
    }
}

impl From<ObjectId> for NullableObjectId {
    fn from(id: ObjectId) -> Self {
        if id == ObjectId::NULL {
            NullableObjectId(None)
        } else {
            NullableObjectId(Some(id))
        }
    }
}

impl Default for ObjectId {
    fn default() -> Self {
        Self::new(0).unwrap()
    }
}
impl TryFrom<u16> for ObjectId {
    type Error = ParseError;

    fn try_from(id: u16) -> Result<Self, Self::Error> {
        ObjectId::new(id)
    }
}

impl From<ObjectId> for u16 {
    fn from(val: ObjectId) -> Self {
        val.id
    }
}
impl TryFrom<[u8; 2]> for ObjectId {
    type Error = ParseError;

    fn try_from(val: [u8; 2]) -> Result<Self, Self::Error> {
        ObjectId::new(u16::from_le_bytes(val))
    }
}
impl From<ObjectId> for [u8; 2] {
    fn from(val: ObjectId) -> Self {
        val.id.to_le_bytes()
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
impl TryFrom<&[u8]> for ObjectId {
    type Error = ParseError;

    fn try_from(val: &[u8]) -> Result<Self, Self::Error> {
        match val.len() {
            2.. => Ok(ObjectId::new(u16::from_le_bytes([val[0], val[1]]))?),
            _ => Err(ParseError::DataEmpty),
        }
    }
}
