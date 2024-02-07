// Copyright 2023 Raven Industries inc.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(u8);

impl Address {
    /// Address representing broadcasts for destination specific PGNs
    pub const GLOBAL: Address = Self::BROADCAST;
    /// Alias for the global address
    pub const BROADCAST: Address = Address(0xFF);
    /// The null address is used by ECUs without an address such as during address claiming
    pub const NULL: Address = Address(0xFE);

    pub fn new(raw_address: u8) -> Self {
        Self { 0: raw_address }
    }

    pub fn raw(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn is_global(&self) -> bool {
        self.is_broadcast()
    }

    #[inline]
    pub fn is_broadcast(self) -> bool {
        self == Self::BROADCAST
    }

    #[inline]
    pub fn is_null(self) -> bool {
        self == Self::NULL
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::NULL
    }
}

// TODO: custom Debug impl and helpers
