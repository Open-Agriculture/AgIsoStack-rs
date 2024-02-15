use crate::j1939::standard_id::StandardId;
use crate::j1939::ExtendedId;

#[derive(Debug)]
pub enum ParseIdError {
    Priority,
    Pgn,
    SourceAddress,
    StandardId,
    ExtendedId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    Standard(StandardId),
    Extended(ExtendedId),
}

impl Default for Id {
    fn default() -> Self {
        Id::Extended(ExtendedId::default())
    }
}
