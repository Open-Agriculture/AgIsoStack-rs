pub mod colour;
pub mod reader;
pub mod writer;

pub mod object;
pub mod object_attributes;
mod object_id;
mod object_pool;
mod object_type;
mod vt_version;

use crate::network_management::name::NAME;

pub use colour::Colour;
pub use object_attributes::ObjectRef;
pub use object_id::NullableObjectId;
pub use object_id::ObjectId;
pub use object_pool::ObjectPool;
pub use object_type::ObjectType;

#[derive(Debug)]
pub enum ParseError {
    DataEmpty,
    UnknownObjectType,
    UnexpectedNullObjectId,
    BooleanOutOfRange,
    UnsupportedVtVersion,
}
