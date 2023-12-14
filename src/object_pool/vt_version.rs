use crate::object_pool::ParseError;
use crate::object_pool::ParseError::UnsupportedVtVersion;

#[derive(Debug, Default)]
pub enum VtVersion {
    Version0,
    Version1,
    Version2,
    #[default]
    Version3,
    Version4,
    Version5,
    Version6,
}

impl From<VtVersion> for u8 {
    fn from(vt_version: VtVersion) -> Self {
        match vt_version {
            VtVersion::Version0 => 0,
            VtVersion::Version1 => 1,
            VtVersion::Version2 => 2,
            VtVersion::Version3 => 3,
            VtVersion::Version4 => 4,
            VtVersion::Version5 => 5,
            VtVersion::Version6 => 6,
        }
    }
}

impl TryFrom<u8> for VtVersion {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VtVersion::Version0),
            1 => Ok(VtVersion::Version1),
            2 => Ok(VtVersion::Version2),
            3 => Ok(VtVersion::Version3),
            4 => Ok(VtVersion::Version4),
            5 => Ok(VtVersion::Version5),
            6 => Ok(VtVersion::Version6),
            _ => Err(UnsupportedVtVersion),
        }
    }
}
