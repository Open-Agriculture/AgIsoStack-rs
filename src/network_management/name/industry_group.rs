// Copyright 2023 Raven Industries inc.

/// Enum containing all Industry Groups.
///
/// # Examples
///
/// ```rust
/// # use ag_iso_stack::network_management::name::IndustryGroup;
/// let industry_group: IndustryGroup = IndustryGroup::AgriculturalAndForestryEquipment;
///
/// assert_eq!(industry_group, 2.into());
/// assert_eq!(industry_group, Into::<IndustryGroup>::into(2));
/// assert_eq!(Into::<u8>::into(industry_group), 2);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[repr(C)]
pub enum IndustryGroup {
    #[default]
    Global = 0,
    OnHighwayEquipment = 1,
    AgriculturalAndForestryEquipment = 2,
    ConstructionEquipment = 3,
    MarineEquipment = 4,
    IndustrialProcessControl = 5,
    ReservedForSAE1 = 6,
    ReservedForSAE2 = 7,
}

/// Display the Industry Group name.
///
/// # Examples
///
/// ```rust
/// # use ag_iso_stack::network_management::name::IndustryGroup;
/// let industry_group: IndustryGroup = IndustryGroup::AgriculturalAndForestryEquipment;
///
/// assert_eq!("AgriculturalAndForestryEquipment", format!("{industry_group}"));
/// ```
impl core::fmt::Display for IndustryGroup {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u8> for IndustryGroup {
    fn from(value: u8) -> Self {
        match value {
            0 => IndustryGroup::Global,
            1 => IndustryGroup::OnHighwayEquipment,
            2 => IndustryGroup::AgriculturalAndForestryEquipment,
            3 => IndustryGroup::ConstructionEquipment,
            4 => IndustryGroup::MarineEquipment,
            5 => IndustryGroup::IndustrialProcessControl,
            6 => IndustryGroup::ReservedForSAE1,
            7 => IndustryGroup::ReservedForSAE2,
            _ => {
                unreachable!("Internal error converting a value larger than 7 to an IndustryGroup")
            }
        }
    }
}

impl From<IndustryGroup> for u8 {
    fn from(value: IndustryGroup) -> Self {
        value as u8
    }
}
