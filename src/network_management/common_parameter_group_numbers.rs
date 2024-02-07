// Copyright 2023 Raven Industries inc.

use crate::j1939::Pgn;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommonParameterGroupNumbers {
    ParameterGroupNumberRequest = 0x00EA00,
    AddressClaim = 0x00EE00,
}

impl CommonParameterGroupNumbers {
    pub fn get_pgn(&self) -> Pgn {
        match self {
            CommonParameterGroupNumbers::AddressClaim => Pgn::new(false, false, 0xEE, 0x00),
            CommonParameterGroupNumbers::ParameterGroupNumberRequest => {
                Pgn::new(false, false, 0xEA, 0x00)
            }
        }
    }
}
