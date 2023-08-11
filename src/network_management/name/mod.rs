// Copyright 2023 Raven Industries inc.

mod name_filter;
pub use name_filter::NameFilter;
mod industry_group;
pub use industry_group::IndustryGroup;
mod device_class;
pub use device_class::DeviceClass;
mod function_code;
pub use function_code::FunctionCode;

#[derive(Default, Copy, Clone, PartialEq)]
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

    /// Match `self` against the provided `NameFilter`s
    ///
    /// Returns true, only if all filters match
    pub fn match_filters(&self, name_filters: &[NameFilter]) -> bool {
        name_filters
            .iter()
            .all(|name_filter| name_filter.match_filter(self))
    }

    /// Raven specific
    pub fn short_identity_number(&self) -> u16 {
        (self.raw_name & 0x0000FFFF) as u16
    }

    /// Raven specific
    pub fn set_short_identity_number(&mut self, short_identity_number: u16) {
        self.raw_name &= !0x000000000000FFFF;
        self.raw_name |= short_identity_number as u64;
    }

    /// Raven specific
    pub fn extended_identity_number(&self) -> u8 {
        ((self.raw_name >> 16) & 0x1F) as u8
    }

    /// Raven specific
    pub fn set_extended_identity_number(&mut self, extended_identity_number: u8) {
        self.raw_name &= !0x00000000001F0000;
        self.raw_name |= ((extended_identity_number & 0x1F) as u64) << 16;
    }

    pub fn identity_number(&self) -> u32 {
        (self.raw_name & 0x001FFFFF) as u32
    }

    pub fn set_identity_number(&mut self, identity_number: u32) {
        self.raw_name &= !0x00000000001FFFFF;
        self.raw_name |= (identity_number & 0x00000000001FFFFF) as u64;
    }

    pub fn manufacturer_code(&self) -> u16 {
        ((self.raw_name >> 21) & 0x07FF) as u16
    }

    pub fn set_manufacturer_code(&mut self, manufacturer_code: u16) {
        self.raw_name &= !0x00000000FFE00000;
        self.raw_name |= ((manufacturer_code & 0x07FF) as u64) << 21;
    }

    pub fn ecu_instance(&self) -> u8 {
        ((self.raw_name >> 32) & 0x07) as u8
    }

    pub fn set_ecu_instance(&mut self, ecu_instance: u8) {
        self.raw_name &= !0x0000000700000000;
        self.raw_name |= ((ecu_instance & 0x07) as u64) << 32;
    }

    pub fn function_instance(&self) -> u8 {
        ((self.raw_name >> 35) & 0x1F) as u8
    }

    pub fn set_function_instance(&mut self, function: u8) {
        self.raw_name &= !0x000000F800000000;
        self.raw_name |= ((function & 0x1F) as u64) << 35;
    }

    pub fn function_code(&self) -> FunctionCode {
        (((self.raw_name >> 40) & 0xFF) as u8).into()
    }

    pub fn set_function_code(&mut self, function_code: impl Into<u8>) {
        self.raw_name &= !0x0000FF0000000000;
        self.raw_name |= (function_code.into() as u64) << 40;
    }

    pub fn device_class(&self) -> DeviceClass {
        (((self.raw_name >> 49) & 0x7F) as u8, self.industry_group()).into()
    }

    pub fn set_device_class(&mut self, device_class: DeviceClass) {
        self.set_industry_group(device_class.into()); // Derive the industrygroup from the device class

        self.raw_name &= !0x00FE000000000000_u64;
        self.raw_name |= ((u8::from(device_class) & 0x7F) as u64) << 49;
    }

    pub fn device_class_instance(&self) -> u8 {
        ((self.raw_name >> 56) & 0x0F) as u8
    }

    pub fn set_device_class_instance(&mut self, device_class_instance: u8) {
        self.raw_name &= !0x0F00000000000000;
        self.raw_name |= ((device_class_instance & 0x0F) as u64) << 56;
    }

    pub fn industry_group(&self) -> IndustryGroup {
        (((self.raw_name >> 60) & 0x07) as u8).into()
    }

    pub fn set_industry_group(&mut self, industry_group: IndustryGroup) {
        self.raw_name &= !0x7000000000000000;
        self.raw_name |= ((u8::from(industry_group) & 0x07) as u64) << 60;
    }

    pub fn self_configurable_address(&self) -> bool {
        (self.raw_name >> 63) != 0
    }

    pub fn set_self_configurable_address(&mut self, self_configurable_address: bool) {
        self.raw_name &= !0x8000000000000000;
        self.raw_name |= (self_configurable_address as u64) << 63;
    }
}

impl From<NAME> for u64 {
    fn from(name: NAME) -> Self {
        name.raw_name
    }
}

impl core::fmt::Display for NAME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:08X}", self.raw_name)
    }
}

impl core::fmt::Debug for NAME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NAME")
            .field(
                "identity_number",
                &format_args!("{}", self.identity_number()),
            )
            .field(
                "manufacturer_code",
                &format_args!("{}", self.manufacturer_code()),
            )
            .field("ecu_instance", &format_args!("{}", self.ecu_instance()))
            .field(
                "function_instance",
                &format_args!("{}", self.function_instance()),
            )
            .field("function_code", &format_args!("{}", self.function_code()))
            .field("device_class", &format_args!("{}", self.device_class()))
            .field(
                "device_class_instance",
                &format_args!("{}", self.device_class_instance()),
            )
            .field("industry_group", &format_args!("{}", self.industry_group()))
            .field(
                "self_configurable_address",
                &format_args!("{}", self.self_configurable_address()),
            )
            .finish()
    }
}

#[derive(Default)]
pub struct NameBuilder {
    identity_number: u32,
    manufacturer_code: u16,
    ecu_instance: u8,
    function_instance: u8,
    function_code: FunctionCode,
    device_class: DeviceClass,
    device_class_instance: u8,
    industry_group: IndustryGroup,
    self_configurable_address: bool,
}

impl NameBuilder {
    pub fn new() -> NameBuilder {
        NameBuilder::default()
    }

    pub fn build(&self) -> NAME {
        let mut name = NAME::default();
        name.set_identity_number(self.identity_number);
        name.set_manufacturer_code(self.manufacturer_code);
        name.set_ecu_instance(self.ecu_instance);
        name.set_function_instance(self.function_instance);
        name.set_function_code(self.function_code);
        name.set_device_class(self.device_class);
        name.set_device_class_instance(self.device_class_instance);
        name.set_industry_group(self.industry_group);
        name.set_self_configurable_address(self.self_configurable_address);
        name
    }

    /// Raven specific
    pub fn short_identity_number(&mut self, value: u16) -> &mut NameBuilder {
        self.identity_number &= !0x0000FFFF;
        self.identity_number |= value as u32;
        self
    }
    /// Raven specific
    pub fn extended_identity_number(&mut self, value: u8) -> &mut NameBuilder {
        self.identity_number &= !0x001F0000;
        self.identity_number |= ((value & 0x1F) as u32) << 16;
        self
    }
    pub fn identity_number(&mut self, value: u32) -> &mut NameBuilder {
        self.identity_number = value;
        self
    }
    pub fn manufacturer_code(&mut self, value: u16) -> &mut NameBuilder {
        self.manufacturer_code = value;
        self
    }
    pub fn ecu_instance(&mut self, value: u8) -> &mut NameBuilder {
        self.ecu_instance = value;
        self
    }
    pub fn function_instance(&mut self, value: u8) -> &mut NameBuilder {
        self.function_instance = value;
        self
    }
    pub fn function_code(&mut self, value: FunctionCode) -> &mut NameBuilder {
        self.function_code = value;
        self
    }
    pub fn device_class(&mut self, value: DeviceClass) -> &mut NameBuilder {
        self.device_class = value;
        self
    }
    pub fn device_class_instance(&mut self, value: u8) -> &mut NameBuilder {
        self.device_class_instance = value;
        self
    }
    pub fn industry_group(&mut self, value: IndustryGroup) -> &mut NameBuilder {
        self.industry_group = value;
        self
    }
    pub fn self_configurable_address(&mut self, value: bool) -> &mut NameBuilder {
        self.self_configurable_address = value;
        self
    }
}

impl From<NAME> for NameBuilder {
    fn from(value: NAME) -> Self {
        NameBuilder {
            identity_number: value.identity_number(),
            manufacturer_code: value.manufacturer_code(),
            ecu_instance: value.ecu_instance(),
            function_instance: value.function_instance(),
            function_code: value.function_code(),
            device_class: value.device_class(),
            device_class_instance: value.device_class_instance(),
            industry_group: value.industry_group(),
            self_configurable_address: value.self_configurable_address(),
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
        name_under_test.set_industry_group(IndustryGroup::OnHighwayEquipment);
        name_under_test.set_device_class(DeviceClass::Tractor(IndustryGroup::OnHighwayEquipment));
        name_under_test.set_function_code(3);
        name_under_test.set_identity_number(4);
        name_under_test.set_ecu_instance(5);
        name_under_test.set_function_instance(6);
        name_under_test.set_device_class_instance(7);
        name_under_test.set_manufacturer_code(8);

        assert_eq!(true, name_under_test.self_configurable_address());
        assert_eq!(1, u8::from(name_under_test.industry_group()));
        assert_eq!(2, u8::from(name_under_test.device_class()));
        assert_eq!(3, u8::from(name_under_test.function_code()));
        assert_eq!(4, name_under_test.identity_number());
        assert_eq!(5, name_under_test.ecu_instance());
        assert_eq!(6, name_under_test.function_instance());
        assert_eq!(7, name_under_test.device_class_instance());
        assert_eq!(8, name_under_test.manufacturer_code());
        assert_eq!(0, name_under_test.extended_identity_number());
        assert_eq!(4, name_under_test.short_identity_number());
        assert_eq!(10881826125818888196_u64, name_under_test.raw_name);
    }

    #[test]
    fn test_name_builder() {
        let name_under_test = NAME::builder()
            .identity_number(4_u32)
            .manufacturer_code(8_u16)
            .ecu_instance(5)
            .function_instance(6)
            .function_code(FunctionCode::from(3))
            .device_class(DeviceClass::from((2, IndustryGroup::from(1))))
            .device_class_instance(7)
            .industry_group(IndustryGroup::from(1))
            .self_configurable_address(true)
            .build();

        assert_eq!(10881826125818888196_u64, name_under_test.into());
    }

    #[test]
    fn test_out_of_range_properties() {
        let mut name_under_test = NAME::new(0);

        name_under_test.set_device_class_instance(16);
        name_under_test.set_identity_number(2097152);
        name_under_test.set_ecu_instance(8);
        name_under_test.set_function_instance(32);
        name_under_test.set_manufacturer_code(2048);

        assert_ne!(name_under_test.device_class_instance(), 16);
        assert_ne!(name_under_test.identity_number(), 2097151);
        assert_ne!(name_under_test.ecu_instance(), 8);
        assert_ne!(name_under_test.function_instance(), 32);
        assert_ne!(name_under_test.manufacturer_code(), 2048);
    }

    #[test]
    fn test_name_equality() {
        let test_value: u64 = 10376445291390828545;
        let name_under_test1 = NAME::new(test_value);
        let name_under_test2 = NAME::new(test_value);

        assert_eq!(test_value, name_under_test1.raw_name);
        assert_eq!(name_under_test1.raw_name, name_under_test2.raw_name);
        assert_eq!(name_under_test1, name_under_test2);
    }

    #[test]
    fn test_filter_matching() {
        let mut test_name = NAME::new(0);
        let mut filters_to_test = Vec::new();
        let identity_number_filter = NameFilter::IdentityNumber(1);
        filters_to_test.push(identity_number_filter);

        assert_eq!(false, test_name.match_filters(&filters_to_test));
        test_name.set_identity_number(1);
        assert_eq!(true, test_name.match_filters(&filters_to_test));

        let manufacturer_number_filter = NameFilter::ManufacturerCode(2);
        filters_to_test.push(manufacturer_number_filter);

        assert_eq!(false, test_name.match_filters(&filters_to_test));
        test_name.set_manufacturer_code(2);
        assert_eq!(true, test_name.match_filters(&filters_to_test));

        let ecu_instance_filter = NameFilter::EcuInstance(3);
        filters_to_test.push(ecu_instance_filter);

        assert_eq!(false, test_name.match_filters(&filters_to_test));
        test_name.set_ecu_instance(3);
        assert_eq!(true, test_name.match_filters(&filters_to_test));

        let function_instance_filter = NameFilter::FunctionInstance(4);
        filters_to_test.push(function_instance_filter);

        assert_eq!(false, test_name.match_filters(&filters_to_test));
        test_name.set_function_instance(4);
        assert_eq!(true, test_name.match_filters(&filters_to_test));

        let function_filter = NameFilter::FunctionCode(FunctionCode::MachineControl);
        filters_to_test.push(function_filter);

        assert_eq!(false, test_name.match_filters(&filters_to_test));
        test_name.set_function_code(FunctionCode::MachineControl);
        assert_eq!(true, test_name.match_filters(&filters_to_test));

        let device_class_filter = NameFilter::DeviceClass(DeviceClass::Tractor(
            IndustryGroup::AgriculturalAndForestryEquipment,
        ));
        filters_to_test.push(device_class_filter);

        assert_eq!(false, test_name.match_filters(&filters_to_test));
        test_name.set_device_class(DeviceClass::Tractor(
            IndustryGroup::AgriculturalAndForestryEquipment,
        ));
        assert_eq!(true, test_name.match_filters(&filters_to_test));

        let industry_group_filter =
            NameFilter::IndustryGroup(IndustryGroup::AgriculturalAndForestryEquipment);
        filters_to_test.push(industry_group_filter);

        assert_eq!(true, test_name.match_filters(&filters_to_test));

        let device_class_instance_filter = NameFilter::DeviceClassInstance(8);
        filters_to_test.push(device_class_instance_filter);

        assert_eq!(false, test_name.match_filters(&filters_to_test));
        test_name.set_device_class_instance(8);
        assert_eq!(true, test_name.match_filters(&filters_to_test));

        let self_configurable_address_filter = NameFilter::SelfConfigurableAddress(true);
        filters_to_test.push(self_configurable_address_filter);

        assert_eq!(false, test_name.match_filters(&filters_to_test));
        test_name.set_self_configurable_address(true);
        assert_eq!(true, test_name.match_filters(&filters_to_test));
    }
}
