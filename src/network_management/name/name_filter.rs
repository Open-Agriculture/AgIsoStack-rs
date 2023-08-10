// Copyright 2023 Raven Industries inc.

use super::{DeviceClass, FunctionCode, IndustryGroup, NAME};

/// A struct that associates a NAME parameter with a value of that parameter.
/// This struct is used to match a partner control function with specific criteria that
/// defines it. Use these to define what device you want to talk to.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NameFilter {
    IdentityNumber(u32),
    ManufacturerCode(u16),
    EcuInstance(u8),
    FunctionInstance(u8),
    FunctionCode(FunctionCode),
    DeviceClass(DeviceClass),
    DeviceClassInstance(u8),
    IndustryGroup(IndustryGroup),
    SelfConfigurableAddress(bool),
}

impl NameFilter {
    /// Returns true if a NAME matches this filter's component.
    pub fn match_filter(&self, name: &NAME) -> bool {
        match self {
            NameFilter::IdentityNumber(val) => name.identity_number() == *val,
            NameFilter::ManufacturerCode(val) => name.manufacturer_code() == *val,
            NameFilter::EcuInstance(val) => name.ecu_instance() == *val,
            NameFilter::FunctionInstance(val) => name.function_instance() == *val,
            NameFilter::FunctionCode(val) => name.function_code() == *val,
            NameFilter::DeviceClass(val) => name.device_class() == *val,
            NameFilter::DeviceClassInstance(val) => name.device_class_instance() == *val,
            NameFilter::IndustryGroup(val) => name.industry_group() == *val,
            NameFilter::SelfConfigurableAddress(val) => name.self_configurable_address() == *val,
        }
    }
}
