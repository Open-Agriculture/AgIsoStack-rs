// Copyright 2023 Raven Industries inc.
const DEFAULT_NAME: u64 = 0xFFFFFFFFFFFFFFFF;

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
    ) -> NAME {
        let mut new_name = NAME::new(0);
        new_name.set_short_identity_number(short_identity_number);
        new_name.set_extended_identity_number(extended_identity_number);
        new_name.set_manufacturer_code(manufacturer_code);
        new_name.set_ecu_instance(ecu_instance);
        new_name.set_function_instance(function_instance);
        new_name.set_function(function);
        new_name.set_device_class(device_class);
        new_name.set_device_class_instance(device_class_instance);
        new_name.set_industry_group(industry_group);
        new_name.set_self_configurable_address(self_configurable_address);
        new_name
    }

    pub fn check_mask(name_to_check: &NAME, name_fields: &Vec<NameField>) -> bool {
        let mut matched = false;
        if (!name_fields.is_empty()) && (DEFAULT_NAME != name_to_check.raw_name) {
            matched = true;

            for field in name_fields {
                if let NameField::IdentityNumber(value) = field {
                    if *value == name_to_check.get_identity_number() {
                        matched = true;
                        break;
                    } else {
                        matched = false;
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::ShortIdentityNumber(value) = field {
                        if *value == name_to_check.get_short_identity_number() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::ExtendedIdentityNumber(value) = field {
                        if *value == name_to_check.get_extended_identity_number() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::ManufacturerCode(value) = field {
                        if *value == name_to_check.get_manufacturer_code() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::EcuInstance(value) = field {
                        if *value == name_to_check.get_ecu_instance() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::FunctionInstance(value) = field {
                        if *value == name_to_check.get_function_instance() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::Function(value) = field {
                        if *value == name_to_check.get_function() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::DeviceClass(value) = field {
                        if *value == name_to_check.get_device_class() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::DeviceClassInstance(value) = field {
                        if *value == name_to_check.get_device_class_instance() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::IndustryGroup(value) = field {
                        if *value == name_to_check.get_industry_group() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }

            if matched {
                for field in name_fields {
                    if let NameField::SelfConfigurableAddress(value) = field {
                        if *value == name_to_check.get_self_configurable_address() {
                            matched = true;
                            break;
                        } else {
                            matched = false;
                        }
                    }
                }
            }
        }
        return matched;
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
    fn test_builder() {
        let name_under_test = NAME::build(4, 0, 8, 5, 6, 3, 2, 7, 1, true);

        assert_eq!(10881826125818888196_u64, name_under_test.raw_name);
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

        assert_eq!(false, NAME::check_mask(&test_name, &filters_to_test));
        test_name.set_identity_number(1);
        assert_eq!(true, NAME::check_mask(&test_name, &filters_to_test));

        let manufacturer_number_filter = NameField::ManufacturerCode(2);
        filters_to_test.push(manufacturer_number_filter);

        assert_eq!(false, NAME::check_mask(&test_name, &filters_to_test));
        test_name.set_manufacturer_code(2);
        assert_eq!(true, NAME::check_mask(&test_name, &filters_to_test));

        let ecu_instance_filter = NameField::EcuInstance(3);
        filters_to_test.push(ecu_instance_filter);

        assert_eq!(false, NAME::check_mask(&test_name, &filters_to_test));
        test_name.set_ecu_instance(3);
        assert_eq!(true, NAME::check_mask(&test_name, &filters_to_test));

        let function_instance_filter = NameField::FunctionInstance(4);
        filters_to_test.push(function_instance_filter);

        assert_eq!(false, NAME::check_mask(&test_name, &filters_to_test));
        test_name.set_function_instance(4);
        assert_eq!(true, NAME::check_mask(&test_name, &filters_to_test));

        let function_filter = NameField::Function(5);
        filters_to_test.push(function_filter);

        assert_eq!(false, NAME::check_mask(&test_name, &filters_to_test));
        test_name.set_function(5);
        assert_eq!(true, NAME::check_mask(&test_name, &filters_to_test));

        let device_class_filter = NameField::DeviceClass(6);
        filters_to_test.push(device_class_filter);

        assert_eq!(false, NAME::check_mask(&test_name, &filters_to_test));
        test_name.set_device_class(6);
        assert_eq!(true, NAME::check_mask(&test_name, &filters_to_test));

        let industry_group_filter = NameField::IndustryGroup(7);
        filters_to_test.push(industry_group_filter);

        assert_eq!(false, NAME::check_mask(&test_name, &filters_to_test));
        test_name.set_industry_group(7);
        assert_eq!(true, NAME::check_mask(&test_name, &filters_to_test));

        let device_class_instance_filter = NameField::DeviceClassInstance(8);
        filters_to_test.push(device_class_instance_filter);

        assert_eq!(false, NAME::check_mask(&test_name, &filters_to_test));
        test_name.set_device_class_instance(8);
        assert_eq!(true, NAME::check_mask(&test_name, &filters_to_test));

        let self_configurable_address_filter = NameField::SelfConfigurableAddress(true);
        filters_to_test.push(self_configurable_address_filter);

        assert_eq!(false, NAME::check_mask(&test_name, &filters_to_test));
        test_name.set_self_configurable_address(true);
        assert_eq!(true, NAME::check_mask(&test_name, &filters_to_test));
    }
}
