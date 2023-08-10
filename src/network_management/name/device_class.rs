// Copyright 2023 Raven Industries inc.

use super::IndustryGroup;

/// Enum containing all Device Classes.
/// Some Device classes belong to multiple Industry Groups.
///
/// # Examples
///
/// ```rust
/// # use ag-iso-stack::name::{IndustryGroup, DeviceClass};
/// let device_class: DeviceClass = DeviceClass::Fertilizers;
///
/// assert_eq!(device_class, Into::<DeviceClass>::into((5, Some(IndustryGroup::AgriculturalAndForestryEquipment))));
/// assert_eq!(Into::<u8>::into(device_class), 5);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum DeviceClass {
    // Shared
    #[default]
    NotAvailable,
    NonSpecificSystem(IndustryGroup),
    Tractor(IndustryGroup),

    // On Highway Equipment
    Trailer,

    // Agricultural And Forestry Equipment
    Tillage,
    SecondaryTillage,
    PlantersOrSeeders,
    Fertilizers,
    Sprayers,
    Harvesters,
    RootHarvesters,
    Forage,
    Irrigation,
    TransportOrTrailer,
    FarmYardOperations,
    PoweredAuxiliaryDevices,
    SpecialCrops,
    EarthWork,
    Skidder,
    SensorSystems,
    TimberHarvesters,
    Forwarders,
    TimberLoaders,
    TimberProcessingMachines,
    Mulchers,
    UtilityVehicles,
    SlurryOrManureApplicators,
    FeedersOrMixers,
    Weeders,

    // Construction Equipment
    SkidSteerLoader,
    ArticulatedDumpTruck,
    Backhoe,
    Crawler,
    Excavator,
    Forklift,
    FourWheelDriveLoader,
    Grader,
    MillingMachine,
    RecyclerAndSoilStabilizer,
    BindingAgentSpreader,
    Paver,
    Feeder,
    ScreeningPlant,
    Stacker,
    Roller,
    Crusher,

    // Marine Equipment
    SystemTools,
    SafetySystems,
    Gateway,
    PowerManagementAndLightingSystems,
    Steeringsystems,
    NavigationSystems,
    CommunicationsSystems,
    InstrumentationOrGeneralSystems,
    EnvironmentalSystems,
    DeckCargoAndFishingEquipmentSystems,

    // Industrial Process Control
    IndustrialProcessControlStationary,
}

/// Display the Device Class name.
///
/// # Examples
///
/// ```rust
/// # use ag-iso-stack::name::DeviceClass;
/// let device_class: DeviceClass = DeviceClass::Fertilizers;
///
/// assert_eq!("Fertilizers", format!("{}", device_class));
/// ```
impl core::fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<DeviceClass> for u8 {
    fn from(value: DeviceClass) -> Self {
        match value {
            // Shared
            DeviceClass::NotAvailable => 127,
            DeviceClass::NonSpecificSystem(_) => 0,
            DeviceClass::Tractor(_) => 1,

            // On Highway Equipment
            DeviceClass::Trailer => 2,

            // Agricultural And Forestry Equipment
            DeviceClass::Tillage => 2,
            DeviceClass::SecondaryTillage => 3,
            DeviceClass::PlantersOrSeeders => 4,
            DeviceClass::Fertilizers => 5,
            DeviceClass::Sprayers => 6,
            DeviceClass::Harvesters => 7,
            DeviceClass::RootHarvesters => 8,
            DeviceClass::Forage => 9,
            DeviceClass::Irrigation => 10,
            DeviceClass::TransportOrTrailer => 11,
            DeviceClass::FarmYardOperations => 12,
            DeviceClass::PoweredAuxiliaryDevices => 13,
            DeviceClass::SpecialCrops => 14,
            DeviceClass::EarthWork => 15,
            DeviceClass::Skidder => 16,
            DeviceClass::SensorSystems => 17,
            DeviceClass::TimberHarvesters => 19,
            DeviceClass::Forwarders => 20,
            DeviceClass::TimberLoaders => 21,
            DeviceClass::TimberProcessingMachines => 22,
            DeviceClass::Mulchers => 23,
            DeviceClass::UtilityVehicles => 24,
            DeviceClass::SlurryOrManureApplicators => 25,
            DeviceClass::FeedersOrMixers => 26,
            DeviceClass::Weeders => 27,

            // Construction Equipment
            DeviceClass::SkidSteerLoader => 1,
            DeviceClass::ArticulatedDumpTruck => 2,
            DeviceClass::Backhoe => 3,
            DeviceClass::Crawler => 4,
            DeviceClass::Excavator => 5,
            DeviceClass::Forklift => 6,
            DeviceClass::FourWheelDriveLoader => 7,
            DeviceClass::Grader => 8,
            DeviceClass::MillingMachine => 9,
            DeviceClass::RecyclerAndSoilStabilizer => 10,
            DeviceClass::BindingAgentSpreader => 11,
            DeviceClass::Paver => 12,
            DeviceClass::Feeder => 13,
            DeviceClass::ScreeningPlant => 14,
            DeviceClass::Stacker => 15,
            DeviceClass::Roller => 16,
            DeviceClass::Crusher => 17,

            // Marine Equipment
            DeviceClass::SystemTools => 10,
            DeviceClass::SafetySystems => 20,
            DeviceClass::Gateway => 25,
            DeviceClass::PowerManagementAndLightingSystems => 30,
            DeviceClass::Steeringsystems => 40,
            DeviceClass::NavigationSystems => 60,
            DeviceClass::CommunicationsSystems => 70,
            DeviceClass::InstrumentationOrGeneralSystems => 80,
            DeviceClass::EnvironmentalSystems => 90,
            DeviceClass::DeckCargoAndFishingEquipmentSystems => 100,

            // Industrial Process Control
            DeviceClass::IndustrialProcessControlStationary => 0,
        }
    }
}

impl From<(u8, Option<IndustryGroup>)> for DeviceClass {
    fn from(value: (u8, Option<IndustryGroup>)) -> Self {
        match value {
            (0, Some(IndustryGroup::IndustrialProcessControl)) => {
                DeviceClass::IndustrialProcessControlStationary
            }
            (0, Some(ig)) => DeviceClass::NonSpecificSystem(ig),

            (1, Some(IndustryGroup::OnHighwayEquipment)) => {
                DeviceClass::Tractor(IndustryGroup::OnHighwayEquipment)
            }
            (2, Some(IndustryGroup::OnHighwayEquipment)) => DeviceClass::Trailer,

            (1, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::Tractor(IndustryGroup::AgriculturalAndForestryEquipment)
            }
            (2, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Tillage,
            (3, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::SecondaryTillage
            }
            (4, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::PlantersOrSeeders
            }
            (5, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Fertilizers,
            (6, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Sprayers,
            (7, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Harvesters,
            (8, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::RootHarvesters
            }
            (9, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Forage,
            (10, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Irrigation,
            (11, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::TransportOrTrailer
            }
            (12, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::FarmYardOperations
            }
            (13, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::PoweredAuxiliaryDevices
            }
            (14, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::SpecialCrops
            }
            (15, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::EarthWork,
            (16, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Skidder,
            (17, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::SensorSystems
            }
            (19, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::TimberHarvesters
            }
            (20, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Forwarders,
            (21, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::TimberLoaders
            }
            (22, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::TimberProcessingMachines
            }
            (23, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Mulchers,
            (24, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::UtilityVehicles
            }
            (25, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::SlurryOrManureApplicators
            }
            (26, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => {
                DeviceClass::FeedersOrMixers
            }
            (27, Some(IndustryGroup::AgriculturalAndForestryEquipment)) => DeviceClass::Weeders,

            (1, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::SkidSteerLoader,
            (2, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::ArticulatedDumpTruck,
            (3, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Backhoe,
            (4, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Crawler,
            (5, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Excavator,
            (6, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Forklift,
            (7, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::FourWheelDriveLoader,
            (8, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Grader,
            (9, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::MillingMachine,
            (10, Some(IndustryGroup::ConstructionEquipment)) => {
                DeviceClass::RecyclerAndSoilStabilizer
            }
            (11, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::BindingAgentSpreader,
            (12, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Paver,
            (13, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Feeder,
            (14, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::ScreeningPlant,
            (15, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Stacker,
            (16, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Roller,
            (17, Some(IndustryGroup::ConstructionEquipment)) => DeviceClass::Crusher,

            (10, Some(IndustryGroup::MarineEquipment)) => DeviceClass::SystemTools,
            (20, Some(IndustryGroup::MarineEquipment)) => DeviceClass::SafetySystems,
            (25, Some(IndustryGroup::MarineEquipment)) => DeviceClass::Gateway,
            (30, Some(IndustryGroup::MarineEquipment)) => {
                DeviceClass::PowerManagementAndLightingSystems
            }
            (40, Some(IndustryGroup::MarineEquipment)) => DeviceClass::Steeringsystems,
            (60, Some(IndustryGroup::MarineEquipment)) => DeviceClass::NavigationSystems,
            (70, Some(IndustryGroup::MarineEquipment)) => DeviceClass::CommunicationsSystems,
            (80, Some(IndustryGroup::MarineEquipment)) => {
                DeviceClass::InstrumentationOrGeneralSystems
            }
            (90, Some(IndustryGroup::MarineEquipment)) => DeviceClass::EnvironmentalSystems,
            (100, Some(IndustryGroup::MarineEquipment)) => {
                DeviceClass::DeckCargoAndFishingEquipmentSystems
            }

            _ => DeviceClass::NotAvailable,
        }
    }
}

impl From<DeviceClass> for IndustryGroup {
    fn from(value: DeviceClass) -> Self {
        match value {
            // Shared
            DeviceClass::NotAvailable => IndustryGroup::Global,
            DeviceClass::NonSpecificSystem(ig) | DeviceClass::Tractor(ig) => ig,

            // On Highway Equipment
            DeviceClass::Trailer => IndustryGroup::OnHighwayEquipment,

            // Agricultural And Forestry Equipment
            DeviceClass::Tillage
            | DeviceClass::SecondaryTillage
            | DeviceClass::PlantersOrSeeders
            | DeviceClass::Fertilizers
            | DeviceClass::Sprayers
            | DeviceClass::Harvesters
            | DeviceClass::RootHarvesters
            | DeviceClass::Forage
            | DeviceClass::Irrigation
            | DeviceClass::TransportOrTrailer
            | DeviceClass::FarmYardOperations
            | DeviceClass::PoweredAuxiliaryDevices
            | DeviceClass::SpecialCrops
            | DeviceClass::EarthWork
            | DeviceClass::Skidder
            | DeviceClass::SensorSystems
            | DeviceClass::TimberHarvesters
            | DeviceClass::Forwarders
            | DeviceClass::TimberLoaders
            | DeviceClass::TimberProcessingMachines
            | DeviceClass::Mulchers
            | DeviceClass::UtilityVehicles
            | DeviceClass::SlurryOrManureApplicators
            | DeviceClass::FeedersOrMixers
            | DeviceClass::Weeders => IndustryGroup::AgriculturalAndForestryEquipment,

            // Construction Equipment
            DeviceClass::SkidSteerLoader
            | DeviceClass::ArticulatedDumpTruck
            | DeviceClass::Backhoe
            | DeviceClass::Crawler
            | DeviceClass::Excavator
            | DeviceClass::Forklift
            | DeviceClass::FourWheelDriveLoader
            | DeviceClass::Grader
            | DeviceClass::MillingMachine
            | DeviceClass::RecyclerAndSoilStabilizer
            | DeviceClass::BindingAgentSpreader
            | DeviceClass::Paver
            | DeviceClass::Feeder
            | DeviceClass::ScreeningPlant
            | DeviceClass::Stacker
            | DeviceClass::Roller
            | DeviceClass::Crusher => IndustryGroup::ConstructionEquipment,

            // Marine Equipment
            DeviceClass::SystemTools
            | DeviceClass::SafetySystems
            | DeviceClass::Gateway
            | DeviceClass::PowerManagementAndLightingSystems
            | DeviceClass::Steeringsystems
            | DeviceClass::NavigationSystems
            | DeviceClass::CommunicationsSystems
            | DeviceClass::InstrumentationOrGeneralSystems
            | DeviceClass::EnvironmentalSystems
            | DeviceClass::DeckCargoAndFishingEquipmentSystems => IndustryGroup::MarineEquipment,

            // Industrial Process Control
            DeviceClass::IndustrialProcessControlStationary => {
                IndustryGroup::IndustrialProcessControl
            }
        }
    }
}
