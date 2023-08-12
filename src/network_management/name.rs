// Copyright 2023 Raven Industries inc.

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NameField {
    IdentityNumber(u32),
    ShortIdentityNumber(u16),
    ExtendedIdentityNumber(u8),
    ManufacturerCode(u16),
    EcuInstance(u8),
    FunctionInstance(u8),
    Function(u8),
    DeviceClass(u8),
    DeviceClassInstance(u8),
    IndustryGroup(u8),
    SelfConfigurableAddress(bool),
}

#[derive(Copy, Clone, PartialEq)]
pub struct NAME {
    raw_name: u64,
}

impl NAME {
    pub fn new(raw_name: u64) -> Self {
        Self { raw_name }
    }

    pub fn builder() -> NameBuilder {
        NameBuilder::default()
    }

    pub fn has_field_value(&self, field_value: NameField) -> bool {
        *self != NAME::default()
            && match field_value {
                NameField::IdentityNumber(value) => self.get_identity_number() == value,
                NameField::ShortIdentityNumber(value) => self.get_short_identity_number() == value,
                NameField::ExtendedIdentityNumber(value) => {
                    self.get_extended_identity_number() == value
                }
                NameField::ManufacturerCode(value) => self.get_manufacturer_code() == value,
                NameField::EcuInstance(value) => self.get_ecu_instance() == value,
                NameField::FunctionInstance(value) => self.get_function_instance() == value,
                NameField::Function(value) => self.get_function() == value,
                NameField::DeviceClass(value) => self.get_device_class() == value,
                NameField::DeviceClassInstance(value) => self.get_device_class_instance() == value,
                NameField::IndustryGroup(value) => self.get_industry_group() == value,
                NameField::SelfConfigurableAddress(value) => {
                    self.get_self_configurable_address() == value
                }
            }
    }

    pub fn has_field_values(&self, name_fields: &[NameField]) -> bool {
        /// A helper function to get the index of a field
        /// This is used to set the bits in a mask to check if all supplied fields are satisfied
        fn get_index(field: &NameField) -> u8 {
            match field {
                NameField::IdentityNumber(_) => 0,
                NameField::ShortIdentityNumber(_) => 1,
                NameField::ExtendedIdentityNumber(_) => 2,
                NameField::ManufacturerCode(_) => 3,
                NameField::EcuInstance(_) => 4,
                NameField::FunctionInstance(_) => 5,
                NameField::Function(_) => 6,
                NameField::DeviceClass(_) => 7,
                NameField::DeviceClassInstance(_) => 8,
                NameField::IndustryGroup(_) => 9,
                NameField::SelfConfigurableAddress(_) => 10,
            }
        }

        // Make a mask of all the fields present in the supplied array
        let fields_present = name_fields.iter().fold(0_u16, |acc, name_field| {
            return acc | 1 << get_index(name_field);
        });
        // Make a mask of all the fields satisfied in the supplied array
        let fields_satisfied = name_fields.iter().fold(0_u16, |acc, name_field| {
            if self.has_field_value(*name_field) {
                return acc | 1 << get_index(name_field);
            } else {
                return acc;
            }
        });
        return fields_satisfied == fields_present;
    }

    pub fn get_device_class(&self) -> u8 {
        ((self.raw_name >> 49) & 0x7F) as u8
    }

    pub fn set_device_class(&mut self, device_class: u8) {
        self.raw_name &= !0x00FE000000000000_u64;
        self.raw_name |= ((device_class & 0x7F) as u64) << 49;
    }

    pub fn get_device_class_instance(&self) -> u8 {
        ((self.raw_name >> 56) & 0x0F) as u8
    }

    pub fn set_device_class_instance(&mut self, device_class_instance: u8) {
        self.raw_name &= !0x0F00000000000000;
        self.raw_name |= ((device_class_instance & 0x0F) as u64) << 56;
    }

    pub fn get_ecu_instance(&self) -> u8 {
        ((self.raw_name >> 32) & 0x07) as u8
    }

    pub fn set_ecu_instance(&mut self, ecu_instance: u8) {
        self.raw_name &= !0x0000000700000000;
        self.raw_name |= ((ecu_instance & 0x07) as u64) << 32;
    }

    pub fn get_extended_identity_number(&self) -> u8 {
        ((self.raw_name >> 16) & 0x1F) as u8
    }

    pub fn set_extended_identity_number(&mut self, extended_identity_number: u8) {
        self.raw_name &= !0x00000000001F0000;
        self.raw_name |= ((extended_identity_number & 0x1F) as u64) << 16;
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

    pub fn set_function_instance(&mut self, function: u8) {
        self.raw_name &= !0x000000F800000000;
        self.raw_name |= ((function & 0x1F) as u64) << 35;
    }

    pub fn get_identity_number(&self) -> u32 {
        (self.raw_name & 0x001FFFFF) as u32
    }

    pub fn set_identity_number(&mut self, identity_number: u32) {
        self.raw_name &= !0x00000000001FFFFF;
        self.raw_name |= (identity_number & 0x00000000001FFFFF) as u64;
    }

    pub fn get_industry_group(&self) -> u8 {
        ((self.raw_name >> 60) & 0x07) as u8
    }

    pub fn set_industry_group(&mut self, industry_group: u8) {
        self.raw_name &= !0x7000000000000000;
        self.raw_name |= ((industry_group & 0x07) as u64) << 60;
    }

    pub fn get_manufacturer_code(&self) -> u16 {
        ((self.raw_name >> 21) & 0x07FF) as u16
    }

    pub fn set_manufacturer_code(&mut self, manufacturer_code: u16) {
        self.raw_name &= !0x00000000FFE00000;
        self.raw_name |= ((manufacturer_code & 0x07FF) as u64) << 21;
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

impl Default for NAME {
    fn default() -> Self {
        Self {
            raw_name: 0xFFFFFFFFFFFFFFFF,
        }
    }
}

impl From<NAME> for u64 {
    fn from(name: NAME) -> Self {
        name.raw_name
    }
}

#[derive(Default)]
pub struct NameBuilder {
    self_configurable_address: bool,
    industry_group: u8,
    device_class_instance: u8,
    device_class: u8,
    function_code: u8,
    function_instance: u8,
    ecu_instance: u8,
    manufacturer_code: u16,
    identity_number: u32,
}

impl NameBuilder {
    pub fn new() -> NameBuilder {
        NameBuilder::default()
    }

    pub fn build(&self) -> NAME {
        NAME {
            raw_name: (self.self_configurable_address as u64) << 63
                | (self.industry_group as u64 & 0x7) << 60
                | (self.device_class_instance as u64 & 0xF) << 56
                | (self.device_class as u64 & 0x7F) << 49
                | (self.function_code as u64 & 0xFF) << 40
                | (self.function_instance as u64 & 0x1F) << 35
                | (self.ecu_instance as u64 & 0x7) << 32
                | (self.manufacturer_code as u64 & 0x7FF) << 21
                | self.identity_number as u64 & 0x1FFFFF,
        }
    }

    pub fn self_configurable_address(&mut self, value: impl Into<bool>) -> &mut NameBuilder {
        self.self_configurable_address = value.into();
        self
    }
    pub fn industry_group(&mut self, value: impl Into<u8>) -> &mut NameBuilder {
        self.industry_group = value.into();
        self
    }
    pub fn device_class_instance(&mut self, value: impl Into<u8>) -> &mut NameBuilder {
        self.device_class_instance = value.into();
        self
    }
    pub fn device_class(&mut self, value: impl Into<u8>) -> &mut NameBuilder {
        self.device_class = value.into();
        self
    }
    pub fn function_code(&mut self, value: impl Into<u8>) -> &mut NameBuilder {
        self.function_code = value.into();
        self
    }
    pub fn function_instance(&mut self, value: impl Into<u8>) -> &mut NameBuilder {
        self.function_instance = value.into();
        self
    }
    pub fn ecu_instance(&mut self, value: impl Into<u8>) -> &mut NameBuilder {
        self.ecu_instance = value.into();
        self
    }
    pub fn manufacturer_code(&mut self, value: impl Into<u16>) -> &mut NameBuilder {
        self.manufacturer_code = value.into();
        self
    }
    pub fn identity_number(&mut self, value: impl Into<u32>) -> &mut NameBuilder {
        self.identity_number = value.into();
        self
    }
}

impl From<NAME> for NameBuilder {
    fn from(value: NAME) -> Self {
        let value: u64 = value.into();
        NameBuilder {
            self_configurable_address: (value >> 63) != 0,
            industry_group: (value >> 60 & 0x7) as u8,
            device_class_instance: (value >> 56 & 0xF) as u8,
            device_class: (value >> 49 & 0x7F) as u8,
            function_code: (value >> 40 & 0xFF) as u8,
            function_instance: (value >> 35 & 0x1F) as u8,
            ecu_instance: (value >> 32 & 0x7) as u8,
            manufacturer_code: (value >> 21 & 0x7FF) as u16,
            identity_number: (value & 0x1FFFFF) as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_properties() {
        let mut name_under_test = NAME::new(0);

        name_under_test.set_self_configurable_address(true);
        name_under_test.set_industry_group(1);
        name_under_test.set_device_class(2);
        name_under_test.set_function(3);
        name_under_test.set_identity_number(4);
        name_under_test.set_ecu_instance(5);
        name_under_test.set_function_instance(6);
        name_under_test.set_device_class_instance(7);
        name_under_test.set_manufacturer_code(8);

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
    fn test_name_builder() {
        let name_under_test = NAME::builder()
            .identity_number(4_u32)
            .manufacturer_code(8_u16)
            .ecu_instance(5)
            .function_instance(6)
            .function_code(3)
            .device_class(2)
            .device_class_instance(7)
            .industry_group(1)
            .self_configurable_address(true)
            .build();

        assert_eq!(10881826125818888196_u64, name_under_test.into());
    }

    #[test]
    fn test_out_of_range_properties() {
        let mut name_under_test = NAME::new(0);

        name_under_test.set_industry_group(8);
        name_under_test.set_device_class_instance(16);
        name_under_test.set_device_class(128);
        name_under_test.set_identity_number(2097152);
        name_under_test.set_ecu_instance(8);
        name_under_test.set_function_instance(32);
        name_under_test.set_manufacturer_code(2048);

        assert_ne!(name_under_test.get_industry_group(), 8);
        assert_ne!(name_under_test.get_device_class_instance(), 16);
        assert_ne!(name_under_test.get_device_class(), 128);
        assert_ne!(name_under_test.get_identity_number(), 2097151);
        assert_ne!(name_under_test.get_ecu_instance(), 8);
        assert_ne!(name_under_test.get_function_instance(), 32);
        assert_ne!(name_under_test.get_manufacturer_code(), 2048);
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

        let identity_number_filter = NameField::IdentityNumber(1);
        filters_to_test.push(identity_number_filter);
        assert_eq!(false, test_name.has_field_value(identity_number_filter));
        assert_eq!(false, test_name.has_field_values(&filters_to_test));

        test_name.set_identity_number(1);
        assert_eq!(true, test_name.has_field_value(identity_number_filter));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));

        let manufacturer_number_filter = NameField::ManufacturerCode(2);
        filters_to_test.push(manufacturer_number_filter);
        assert_eq!(false, test_name.has_field_value(manufacturer_number_filter));
        assert_eq!(false, test_name.has_field_values(&filters_to_test));

        test_name.set_manufacturer_code(2);
        assert_eq!(true, test_name.has_field_value(manufacturer_number_filter));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));

        let ecu_instance_filter = NameField::EcuInstance(3);
        filters_to_test.push(ecu_instance_filter);
        assert_eq!(false, test_name.has_field_value(ecu_instance_filter));
        assert_eq!(false, test_name.has_field_values(&filters_to_test));

        test_name.set_ecu_instance(3);
        assert_eq!(true, test_name.has_field_value(ecu_instance_filter));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));

        let function_instance_filter = NameField::FunctionInstance(4);
        filters_to_test.push(function_instance_filter);
        assert_eq!(false, test_name.has_field_value(function_instance_filter));
        assert_eq!(false, test_name.has_field_values(&filters_to_test));

        test_name.set_function_instance(4);
        assert_eq!(true, test_name.has_field_value(function_instance_filter));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));

        let function_filter = NameField::Function(5);
        filters_to_test.push(function_filter);
        assert_eq!(false, test_name.has_field_value(function_filter));
        assert_eq!(false, test_name.has_field_values(&filters_to_test));

        test_name.set_function(5);
        assert_eq!(true, test_name.has_field_value(function_filter));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));

        let device_class_filter = NameField::DeviceClass(6);
        filters_to_test.push(device_class_filter);
        assert_eq!(false, test_name.has_field_value(device_class_filter));
        assert_eq!(false, test_name.has_field_values(&filters_to_test));

        test_name.set_device_class(6);
        assert_eq!(true, test_name.has_field_value(device_class_filter));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));

        let industry_group_filter = NameField::IndustryGroup(7);
        filters_to_test.push(industry_group_filter);
        assert_eq!(false, test_name.has_field_value(industry_group_filter));
        assert_eq!(false, test_name.has_field_values(&filters_to_test));

        test_name.set_industry_group(7);
        assert_eq!(true, test_name.has_field_value(industry_group_filter));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));

        let device_class_inst_filter = NameField::DeviceClassInstance(8);
        filters_to_test.push(device_class_inst_filter);
        assert_eq!(false, test_name.has_field_value(device_class_inst_filter));
        assert_eq!(false, test_name.has_field_values(&filters_to_test));

        test_name.set_device_class_instance(8);
        assert_eq!(true, test_name.has_field_value(device_class_inst_filter));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));

        let self_config_address_filter = NameField::SelfConfigurableAddress(true);
        filters_to_test.push(self_config_address_filter);
        assert_eq!(false, test_name.has_field_value(self_config_address_filter));
        assert_eq!(false, test_name.has_field_values(&filters_to_test));

        test_name.set_self_configurable_address(true);
        assert_eq!(true, test_name.has_field_value(self_config_address_filter));
        assert_eq!(true, test_name.has_field_values(&filters_to_test));
    }
}
