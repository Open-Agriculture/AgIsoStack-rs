// Copyright 2023 Raven Industries inc.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum NameFieldError {
    /// The value of the field is out of bounds
    OutOfBounds(NameField),
}

impl std::fmt::Display for NameFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for NameFieldError {}

const DEFAULT_NAME: u64 = 0xFFFFFFFFFFFFFFFF;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum NameField {
    IdentityNumber,
    ShortIdentityNumber,
    ExtendedIdentityNumber,
    ManufacturerCode,
    EcuInstance,
    FunctionInstance,
    Function,
    DeviceClass,
    DeviceClassInstance,
    IndustryGroup,
    SelfConfigurableAddress,
}

#[derive(Copy, Clone)]
pub struct NameFieldValue {
    pub value: u32,
    pub field: NameField,
}

#[derive(Copy, Clone)]
pub struct NAME {
    pub raw_name: u64,
}

impl NAME {
    pub fn new(raw_name: u64) -> Self {
        Self { raw_name }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build(
        short_identity_number: u16,
        extended_identity_number: u8,
        manufacturer_code: u16,
        ecu_instance: u8,
        function_instance: u8,
        function: u8,
        device_class: u8,
        device_class_instance: u8,
        industry_group: u8,
        self_configurable_address: bool,
    ) -> Result<NAME, NameFieldError> {
        let mut new_name = NAME::new(0);
        new_name.set_short_identity_number(short_identity_number);
        new_name
            .set_extended_identity_number(extended_identity_number)
            .ok();
        new_name.set_manufacturer_code(manufacturer_code).ok();
        new_name.set_ecu_instance(ecu_instance).ok();
        new_name.set_function_instance(function_instance).ok();
        new_name.set_function(function);
        new_name.set_device_class(device_class).ok();
        new_name
            .set_device_class_instance(device_class_instance)
            .ok();
        new_name.set_industry_group(industry_group).ok();
        new_name.set_self_configurable_address(self_configurable_address);
        Ok(new_name)
    }

    pub fn get_value_by_field(&self, field: NameField) -> u32 {
        match field {
            NameField::IdentityNumber => self.get_identity_number(),
            NameField::ShortIdentityNumber => self.get_short_identity_number() as u32,
            NameField::ExtendedIdentityNumber => self.get_extended_identity_number() as u32,
            NameField::ManufacturerCode => self.get_manufacturer_code() as u32,
            NameField::EcuInstance => self.get_ecu_instance() as u32,
            NameField::FunctionInstance => self.get_function_instance() as u32,
            NameField::Function => self.get_function() as u32,
            NameField::DeviceClass => self.get_device_class() as u32,
            NameField::DeviceClassInstance => self.get_device_class_instance() as u32,
            NameField::IndustryGroup => self.get_industry_group() as u32,
            NameField::SelfConfigurableAddress => self.get_self_configurable_address() as u32,
        }
    }

    pub fn has_field_values(&self, name_fields: &[NameFieldValue]) -> bool {
        let fields_present = name_fields.iter().fold(0_u16, |acc, name_field| {
            return acc | 1 << name_field.field as u16;
        });

        let fields_satisfied = name_fields.iter().fold(0_u16, |acc, name_field| {
            if self.has_field_value(*name_field) {
                return acc | 1 << name_field.field as u16;
            } else {
                return acc;
            }
        });
        return fields_satisfied == fields_present;
    }

    pub fn has_field_value(&self, field_value: NameFieldValue) -> bool {
        self.raw_name != DEFAULT_NAME
            && self.get_value_by_field(field_value.field) == field_value.value
    }

    pub fn get_device_class(&self) -> u8 {
        ((self.raw_name >> 49) & 0x7F) as u8
    }

    pub fn set_device_class(&mut self, device_class: u8) -> Result<(), NameFieldError> {
        if (device_class & !0x7F) != 0 {
            return Err(NameFieldError::OutOfBounds(NameField::DeviceClass));
        }
        self.raw_name &= !0x00FE000000000000_u64;
        self.raw_name |= ((device_class & 0x7F) as u64) << 49;
        Ok(())
    }

    pub fn get_device_class_instance(&self) -> u8 {
        ((self.raw_name >> 56) & 0x0F) as u8
    }

    pub fn set_device_class_instance(
        &mut self,
        device_class_instance: u8,
    ) -> Result<(), NameFieldError> {
        if (device_class_instance & !0x0F) != 0 {
            return Err(NameFieldError::OutOfBounds(NameField::DeviceClassInstance));
        }
        self.raw_name &= !0x0F00000000000000;
        self.raw_name |= ((device_class_instance & 0x0F) as u64) << 56;
        Ok(())
    }

    pub fn get_ecu_instance(&self) -> u8 {
        ((self.raw_name >> 32) & 0x07) as u8
    }

    pub fn set_ecu_instance(&mut self, ecu_instance: u8) -> Result<(), NameFieldError> {
        if (ecu_instance & !0x07) != 0 {
            return Err(NameFieldError::OutOfBounds(NameField::EcuInstance));
        }
        self.raw_name &= !0x0000000700000000;
        self.raw_name |= ((ecu_instance & 0x07) as u64) << 32;
        Ok(())
    }

    pub fn get_extended_identity_number(&self) -> u8 {
        ((self.raw_name >> 16) & 0x1F) as u8
    }

    pub fn set_extended_identity_number(
        &mut self,
        extended_identity_number: u8,
    ) -> Result<(), NameFieldError> {
        if (extended_identity_number & !0x1F) != 0 {
            return Err(NameFieldError::OutOfBounds(
                NameField::ExtendedIdentityNumber,
            ));
        }
        self.raw_name &= !0x00000000001F0000;
        self.raw_name |= ((extended_identity_number & 0x1F) as u64) << 16;
        Ok(())
    }

    pub fn get_function(&self) -> u8 {
        ((self.raw_name >> 40) & 0xFF) as u8
    }

    pub fn set_function(&mut self, function: u8) {
        self.raw_name &= !0x0000FF0000000000;
        self.raw_name |= (function as u64) << 40;
    }

    pub fn get_function_instance(&self) -> u8 {
        ((self.raw_name >> 35) & 0x1F) as u8
    }

    pub fn set_function_instance(&mut self, function: u8) -> Result<(), NameFieldError> {
        if (function & !0x1F) != 0 {
            return Err(NameFieldError::OutOfBounds(NameField::FunctionInstance));
        }
        self.raw_name &= !0x000000F800000000;
        self.raw_name |= ((function & 0x1F) as u64) << 35;
        Ok(())
    }

    pub fn get_identity_number(&self) -> u32 {
        (self.raw_name & 0x001FFFFF) as u32
    }

    pub fn set_identity_number(&mut self, identity_number: u32) -> Result<(), NameFieldError> {
        if (identity_number & !0x001FFFFF) != 0 {
            return Err(NameFieldError::OutOfBounds(NameField::IdentityNumber));
        }
        self.raw_name &= !0x00000000001FFFFF;
        self.raw_name |= (identity_number & 0x001FFFFF) as u64;
        Ok(())
    }

    pub fn get_industry_group(&self) -> u8 {
        ((self.raw_name >> 60) & 0x07) as u8
    }

    pub fn set_industry_group(&mut self, industry_group: u8) -> Result<(), NameFieldError> {
        if (industry_group & !0x07) != 0 {
            return Err(NameFieldError::OutOfBounds(NameField::IndustryGroup));
        }
        self.raw_name &= !0x7000000000000000;
        self.raw_name |= ((industry_group & 0x07) as u64) << 60;
        Ok(())
    }

    pub fn get_manufacturer_code(&self) -> u16 {
        ((self.raw_name >> 21) & 0x07FF) as u16
    }

    pub fn set_manufacturer_code(&mut self, manufacturer_code: u16) -> Result<(), NameFieldError> {
        if (manufacturer_code & !0x7FF) != 0 {
            return Err(NameFieldError::OutOfBounds(NameField::ManufacturerCode));
        }
        self.raw_name &= !0x00000000FFE00000;
        self.raw_name |= ((manufacturer_code & 0x7FF) as u64) << 21;
        Ok(())
    }

    pub fn get_self_configurable_address(&self) -> bool {
        (self.raw_name >> 63) != 0
    }

    pub fn set_self_configurable_address(&mut self, self_configurable_address: bool) {
        self.raw_name &= !0x8000000000000000;
        self.raw_name |= (self_configurable_address as u64) << 63;
    }

    pub fn get_short_identity_number(&self) -> u16 {
        (self.raw_name & 0x0000FFFF) as u16
    }

    pub fn set_short_identity_number(&mut self, short_identity_number: u16) {
        self.raw_name &= !0x000000000000FFFF;
        self.raw_name |= short_identity_number as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_properties() {
        let mut name_under_test = NAME::new(0);

        name_under_test.set_self_configurable_address(true);
        assert_eq!(name_under_test.set_industry_group(1), Ok(()));
        assert_eq!(name_under_test.set_device_class(2), Ok(()));
        name_under_test.set_function(3);
        assert_eq!(name_under_test.set_identity_number(4), Ok(()));
        assert_eq!(name_under_test.set_ecu_instance(5), Ok(()));
        assert_eq!(name_under_test.set_function_instance(6), Ok(()));
        assert_eq!(name_under_test.set_device_class_instance(7), Ok(()));
        assert_eq!(name_under_test.set_manufacturer_code(8), Ok(()));

        assert_eq!(true, name_under_test.get_self_configurable_address());
        assert_eq!(1, name_under_test.get_industry_group());
        assert_eq!(2, name_under_test.get_device_class());
        assert_eq!(3, name_under_test.get_function());
        assert_eq!(4, name_under_test.get_identity_number());
        assert_eq!(5, name_under_test.get_ecu_instance());
        assert_eq!(6, name_under_test.get_function_instance());
        assert_eq!(7, name_under_test.get_device_class_instance());
        assert_eq!(8, name_under_test.get_manufacturer_code());
        assert_eq!(0, name_under_test.get_extended_identity_number());
        assert_eq!(4, name_under_test.get_short_identity_number());
        assert_eq!(10881826125818888196_u64, name_under_test.raw_name);
    }

    #[test]
    fn test_builder() {
        let name_under_test = NAME::build(4, 0, 8, 5, 6, 3, 2, 7, 1, true).unwrap();

        assert_eq!(10881826125818888196_u64, name_under_test.raw_name);
    }

    #[test]
    fn test_out_of_range_properties() {
        let mut name_under_test = NAME::new(0);

        assert_eq!(
            name_under_test.set_industry_group(8),
            Err(NameFieldError::OutOfBounds(NameField::IndustryGroup))
        );
        assert_eq!(
            name_under_test.set_device_class(128),
            Err(NameFieldError::OutOfBounds(NameField::DeviceClass))
        );
        assert_eq!(
            name_under_test.set_identity_number(2097152),
            Err(NameFieldError::OutOfBounds(NameField::IdentityNumber))
        );
        assert_eq!(
            name_under_test.set_ecu_instance(8),
            Err(NameFieldError::OutOfBounds(NameField::EcuInstance))
        );
        assert_eq!(
            name_under_test.set_function_instance(32),
            Err(NameFieldError::OutOfBounds(NameField::FunctionInstance))
        );
        assert_eq!(
            name_under_test.set_device_class_instance(16),
            Err(NameFieldError::OutOfBounds(NameField::DeviceClassInstance))
        );
        assert_eq!(
            name_under_test.set_manufacturer_code(2048),
            Err(NameFieldError::OutOfBounds(NameField::ManufacturerCode))
        );

        // Check if the values are still the same
        assert_eq!(name_under_test.get_industry_group(), 0);
        assert_eq!(name_under_test.get_device_class_instance(), 0);
        assert_eq!(name_under_test.get_device_class(), 0);
        assert_eq!(name_under_test.get_identity_number(), 0);
        assert_eq!(name_under_test.get_ecu_instance(), 0);
        assert_eq!(name_under_test.get_function_instance(), 0);
        assert_eq!(name_under_test.get_manufacturer_code(), 0);
    }

    #[test]
    fn test_name_equality() {
        let test_value: u64 = 10376445291390828545;
        let name_under_test1 = NAME::new(test_value);
        let name_under_test2 = NAME::new(test_value);

        assert_eq!(test_value, name_under_test1.raw_name);
        assert_eq!(name_under_test1.raw_name, name_under_test2.raw_name);
    }

    #[test]
    fn test_filter_matching() {
        let mut test_name = NAME::new(0);
        let mut filters_to_test = Vec::new();
        let identity_number_filter = NameFieldValue {
            value: 1,
            field: NameField::IdentityNumber,
        };
        filters_to_test.push(identity_number_filter);
        assert_eq!(false, test_name.has_field_values(&filters_to_test));
        assert_eq!(false, test_name.has_field_value(identity_number_filter));

        assert_eq!(test_name.set_identity_number(1), Ok(()));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
        assert_eq!(true, test_name.has_field_value(identity_number_filter));

        let manufacturer_number_filter = NameFieldValue {
            value: 2,
            field: NameField::ManufacturerCode,
        };
        filters_to_test.push(manufacturer_number_filter);
        assert_eq!(false, test_name.has_field_values(&filters_to_test));
        assert_eq!(false, test_name.has_field_value(manufacturer_number_filter));

        assert_eq!(test_name.set_manufacturer_code(2), Ok(()));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
        assert_eq!(true, test_name.has_field_value(manufacturer_number_filter));

        let ecu_instance_filter = NameFieldValue {
            value: 3,
            field: NameField::EcuInstance,
        };
        filters_to_test.push(ecu_instance_filter);
        assert_eq!(false, test_name.has_field_values(&filters_to_test));
        assert_eq!(false, test_name.has_field_value(ecu_instance_filter));

        assert_eq!(test_name.set_ecu_instance(3), Ok(()));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
        assert_eq!(true, test_name.has_field_value(ecu_instance_filter));

        let function_instance_filter = NameFieldValue {
            value: 4,
            field: NameField::FunctionInstance,
        };
        filters_to_test.push(function_instance_filter);
        assert_eq!(false, test_name.has_field_values(&filters_to_test));
        assert_eq!(false, test_name.has_field_value(function_instance_filter));

        assert_eq!(test_name.set_function_instance(4), Ok(()));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
        assert_eq!(true, test_name.has_field_value(function_instance_filter));

        let function_filter = NameFieldValue {
            value: 5,
            field: NameField::Function,
        };
        filters_to_test.push(function_filter);
        assert_eq!(false, test_name.has_field_values(&filters_to_test));
        assert_eq!(false, test_name.has_field_value(function_filter));

        test_name.set_function(5);
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
        assert_eq!(true, test_name.has_field_value(function_filter));

        let device_class_filter = NameFieldValue {
            value: 6,
            field: NameField::DeviceClass,
        };
        filters_to_test.push(device_class_filter);
        assert_eq!(false, test_name.has_field_values(&filters_to_test));
        assert_eq!(false, test_name.has_field_value(device_class_filter));

        assert_eq!(test_name.set_device_class(6), Ok(()));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
        assert_eq!(true, test_name.has_field_value(device_class_filter));

        let industry_group_filter = NameFieldValue {
            value: 7,
            field: NameField::IndustryGroup,
        };
        filters_to_test.push(industry_group_filter);
        assert_eq!(false, test_name.has_field_values(&filters_to_test));
        assert_eq!(false, test_name.has_field_value(industry_group_filter));

        assert_eq!(test_name.set_industry_group(7), Ok(()));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
        assert_eq!(true, test_name.has_field_value(industry_group_filter));

        let device_class_instance_filter = NameFieldValue {
            value: 8,
            field: NameField::DeviceClassInstance,
        };
        filters_to_test.push(device_class_instance_filter);
        assert_eq!(false, test_name.has_field_values(&filters_to_test));
        assert_eq!(
            false,
            test_name.has_field_value(device_class_instance_filter)
        );

        assert_eq!(test_name.set_device_class_instance(8), Ok(()));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
        assert_eq!(
            true,
            test_name.has_field_value(device_class_instance_filter)
        );

        let self_configurable_address_filter = NameFieldValue {
            value: true as u32,
            field: NameField::SelfConfigurableAddress,
        };
        filters_to_test.push(self_configurable_address_filter);

        assert_eq!(false, test_name.has_field_values(&filters_to_test));
        assert_eq!(
            false,
            test_name.has_field_value(self_configurable_address_filter)
        );

        test_name.set_self_configurable_address(true);
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
        assert_eq!(
            true,
            test_name.has_field_value(self_configurable_address_filter)
        );
    }
}
