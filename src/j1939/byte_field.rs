/*
Copyright 2023 Raven Industries inc.

@author Jannes Brands
@date 2024-02-22
*/

use bitvec::order::Msb0;
use bitvec::view::BitView;

/// A byte field
pub trait ByteField: Sized {
    /// Get the raw value of the field
    fn raw(self) -> u8;

    /// Get the raw bits of the field
    fn raw_bits(self) -> [bool; 8] {
        let raw = self.raw();
        let field_bits = raw.view_bits::<Msb0>();
        [
            field_bits[0],
            field_bits[1],
            field_bits[2],
            field_bits[3],
            field_bits[4],
            field_bits[5],
            field_bits[6],
            field_bits[7],
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::j1939::byte_field::ByteField;
    use crate::j1939::PduFormat;

    #[test]
    fn test_byte_field() {
        let byte_field = PduFormat::new(0b1010_1010);
        assert_eq!(byte_field.raw(), 0b1010_1010);
        assert_eq!(
            byte_field.raw_bits(),
            [true, false, true, false, true, false, true, false]
        );

        let byte_field = PduFormat::new(0xFF);
        assert_eq!(byte_field.raw(), 0xFF);
        assert_eq!(
            byte_field.raw_bits(),
            [true, true, true, true, true, true, true, true]
        );

        let byte_field = PduFormat::new(0x00);
        assert_eq!(byte_field.raw(), 0x00);
        assert_eq!(
            byte_field.raw_bits(),
            [false, false, false, false, false, false, false, false]
        );
    }
}
