pub mod colour;
pub mod reader;
pub mod writer;

mod object;
mod object_attributes;
mod object_id;
mod object_pool;
mod object_type;
mod vt_version;

use crate::network_management::name::NAME;

pub use colour::Colour;
pub use object_pool::ObjectPool;
pub use object_type::ObjectType;

#[derive(Debug)]
pub enum ParseError {
    DataEmpty,
    UnknownObjectType,
}
