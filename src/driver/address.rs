// Copyright 2023 Raven Industries inc.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(pub u8);

impl Address {
    pub const GLOBAL: Address = Address(0xFF);
    pub const NULL: Address = Address(0xFE);
}

// TODO: custom Debug impl and helpers
