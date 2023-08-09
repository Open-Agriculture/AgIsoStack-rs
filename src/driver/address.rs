// Copyright 2023 Raven Industries inc.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(pub u8);

impl Address {
    /// Address representing broadcasts for destination specific PGNs
    pub const GLOBAL: Address = Address(0xFF);
    /// Alias for the global address
    pub const BROADCAST: Address = Address(0xFF);
    /// The null address is used by ECUs without an address such as during address claiming
    pub const NULL: Address = Address(0xFE);
}

// TODO: custom Debug impl and helpers
