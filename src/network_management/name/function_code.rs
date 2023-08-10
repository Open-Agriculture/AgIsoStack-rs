// TODO: Rewrite like the device class.

/// Enum containing all Function IDs.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[derive(Default)]
pub enum FunctionCode {
    // Shared
    #[default]
    NotAvailable,

    // On Highway Equipment
    MachineControl,

    // Agricultural And Forestry Equipment
    VirtualTerminal,
}



impl core::fmt::Display for FunctionCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<FunctionCode> for u8 {
    fn from(value: FunctionCode) -> Self {
        match value {
            // Shared
            FunctionCode::NotAvailable => 127,

            // Agricultural And Forestry Equipment
            FunctionCode::VirtualTerminal => 29,

            FunctionCode::MachineControl => 132,
        }
    }
}

impl From<u8> for FunctionCode {
    fn from(value: u8) -> Self {
        match value {
            29 => FunctionCode::VirtualTerminal,
            132 => FunctionCode::MachineControl,
            _ => FunctionCode::NotAvailable,
        }
    }
}

// /// Enum containing all Function IDs.
// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
// pub enum FunctionCode {
//     Global(GlobalFunction),
//     OnHighwayEquipment(OnHighwayEquipmentFunction),
//     AgriculturalAndForestryEquipment(AgriculturalAndForestryEquipmentFunction),
//     ConstructionEquipment(ConstructionEquipmentFunction),
//     MarineEquipment(MarineEquipmentFunction),
//     IndustrialProcessControl(IndustrialProcessControlFunction),
// }

// impl core::fmt::Display for Function {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "{}", self)
//     }
// }

// impl From<Function> for u8 {
//     fn from(value: Function) -> Self {
//         match value {
//             Global(value) => value as u8,
//             OnHighwayEquipment(value) => value as u8,
//             AgriculturalAndForestryEquipment(value) => value as u8,
//             ConstructionEquipment(value) => value as u8,
//             MarineEquipment(value) => value as u8,
//             IndustrialProcessControl(value) => value as u8,
//         }
//     }
// }

// /// Struct containing all Global Function ID's
// #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
// pub enum GlobalFunction {
//     Engine = 0,
//     AuxiliaryPowerUnit = 1,
//     ElectricPropulsionControl = 2,
//     Transmission = 3,
//     BatteryPackMonitor = 4,
//     ShiftControlConsole = 5,
//     PowerTakeOff = 6,
//     AxleSteering = 7,
//     AxleDrive = 8,
//     BrakesSystemController = 9,
//     BrakesSteerAxle = 10,
//     BrakesDriveAxle = 11,
//     RetarderEngine = 12,
//     RetarderDriveline = 13,
//     CruiseControl = 14,
//     FuelSystem = 15,
//     SteeringController = 16,
//     SuspensionSteerAxle = 17,
//     SuspensionDriveAxle = 18,
//     InstrumentCluster = 19,
//     TripRecorder = 20,
//     CabClimateControl = 21,
//     AerodynamicControl = 22,
//     VehicleNavigation = 23,
//     VehicleSecurity = 24,
//     NetworkInterconnectECU = 25,
//     BodyController = 26,
//     PowerTakeOff = 27,
//     OffVehicleGateway = 28,
//     VirtualTerminal = 29,
//     ManagementComputer = 30,
//     PropulsionBatteryCharger = 31,
//     HeadwayController = 32,
//     SystemMonitor = 33,
//     HydraulicPumpController = 34,
//     SuspensionSystemController = 35,
//     PneumaticSystemController = 36,
//     CabController = 37,
//     TirePressureControl = 38,
//     IgnitionControlModule = 39,
//     SeatControl = 40,
//     LightingOperatorControls = 41,
//     WaterPumpControl = 42,
//     TransmissionDisplay = 43,
//     ExhaustEmissionControl = 44,
//     VehicleDynamicStabilityControl = 45,
//     OilSensorUnit = 46,
//     InformationSystemController = 47,
//     RampControl = 48,
//     ClutchConverterControl = 49,
//     AuxiliaryHeater = 50,
//     ForwardLookingCollisionWarningSystem = 51,
//     ChassisController = 52,
//     AlternatorChargingSystem = 53,
//     CommunicationsUnitCellular = 54,
//     CommunicationsUnitSatellite = 55,
//     CommunicationsUnitRadio = 56,
//     SteeringColumnUnit = 57,
//     FanDriveControl = 58,
//     Starter = 59,
//     CabDisplay = 60,
//     FileServerPrinter = 61,
//     OnBoardDiagnosticUnit = 62,
//     EngineValveController = 63,
//     EnduranceBraking = 64,
//     GasFlowMeasurement = 65,
//     IOController = 66,
//     ElectricalSystemController = 67,
//     AfterTreatmentSystemGasMeasurement = 68,
//     EngineEmissionAfterTreatmentSystem = 69,
//     AuxiliaryRegenerationDevice = 70,
//     TransferCaseControl = 71,
//     CoolantValveController = 72,
//     RolloverDetectionControl = 73,
//     LubricationSystem = 74,
//     SupplementalFan = 75,
//     TemperatureSensor = 76,
//     FuelPropertiesSensor = 77,
//     FireSuppressionSystem = 78,
//     PowerSystemsManager = 79,
//     ElectricPowertrain = 80,
//     HydraulicPowertrain = 81,
//     FileServer = 82,
//     Printer = 83,
//     StartAidDevice = 84,
//     EngineInjectionControlModule = 85,
//     EVCommunicationController = 86,
//     DriverImpairmentDevice = 87,
//     ElectricPowerConverter = 88,
//     SupplyEquipmentCommunicationController = 89,
//     VehicleAdapterCommunicationController = 90,
//     Reserved = 128,
//     OffBoardDiagnosticServiceTool = 129,
//     OnBoardDataLogger = 130,
//     PCKeyboard = 131,
//     SafetyRestraintSystem = 132,
//     Turbocharger = 133,
//     GroundBasedSpeedSensor = 134,
//     Keypad = 135,
//     HumiditySensor = 136,
//     ThermalManagementSystemController = 137,
//     BrakeStrokeAlert = 138,
//     OnBoardAxleGroupScale = 139,
//     OnBoardAxleGroupDisplay = 140,
//     BatteryCharger = 141,
//     TurbochargerCompressorBypass = 142,
//     TurbochargerWastegate = 143,
//     Throttle = 144,
//     InertialSensor = 145,
//     FuelActuator = 146,
//     EngineExhaustGasRecirculation = 147,
//     EngineExhaustBackpressure = 148,
//     OnBoardBinWeighingScale = 149,
//     OnBoardBinWeighingScaleDisplay = 150,
//     EngineCylinderPressureMonitoringSystem = 151,
//     ObjectDetection = 152,
//     ObjectDetectionDisplay = 153,
//     ObjectDetectionSensor = 154,
//     PersonnelDetectionDevice = 155,
//     #[default] NotAvailable = 255,
// }

// impl core::fmt::Display for GlobalFunctions {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl From<u8> for GlobalFunctions {
//     fn from(value: u8) -> Self {
//         match value {
//             0 => GlobalFunction::Engine,
//             1 => GlobalFunction::AuxiliaryPowerUnit,
//             2 => GlobalFunction::ElectricPropulsionControl,
//             3 => GlobalFunction::Transmission,
//             4 => GlobalFunction::BatteryPackMonitor,
//             5 => GlobalFunction::ShiftControlConsole,
//             6 => GlobalFunction::PowerTakeOff,
//             7 => GlobalFunction::AxleSteering,
//             8 => GlobalFunction::AxleDrive,
//             9 => GlobalFunction::BrakesSystemController,
//             10 => GlobalFunction::BrakesSteerAxle,
//             11 => GlobalFunction::BrakesDriveAxle,
//             12 => GlobalFunction::RetarderEngine,
//             13 => GlobalFunction::RetarderDriveline,
//             14 => GlobalFunction::CruiseControl,
//             15 => GlobalFunction::FuelSystem,
//             16 => GlobalFunction::SteeringController,
//             17 => GlobalFunction::SuspensionSteerAxle,
//             18 => GlobalFunction::SuspensionDriveAxle,
//             19 => GlobalFunction::InstrumentCluster,
//             20 => GlobalFunction::TripRecorder,
//             21 => GlobalFunction::CabClimateControl,
//             22 => GlobalFunction::AerodynamicControl,
//             23 => GlobalFunction::VehicleNavigation,
//             24 => GlobalFunction::VehicleSecurity,
//             25 => GlobalFunction::NetworkInterconnectECU,
//             26 => GlobalFunction::BodyController,
//             27 => GlobalFunction::PowerTakeOff,
//             28 => GlobalFunction::OffVehicleGateway,
//             29 => GlobalFunction::VirtualTerminal,
//             30 => GlobalFunction::ManagementComputer,
//             31 => GlobalFunction::PropulsionBatteryCharger,
//             32 => GlobalFunction::HeadwayController,
//             33 => GlobalFunction::SystemMonitor,
//             34 => GlobalFunction::HydraulicPumpController,
//             35 => GlobalFunction::SuspensionSystemController,
//             36 => GlobalFunction::PneumaticSystemController,
//             37 => GlobalFunction::CabController,
//             38 => GlobalFunction::TirePressureControl,
//             39 => GlobalFunction::IgnitionControlModule,
//             40 => GlobalFunction::SeatControl,
//             41 => GlobalFunction::LightingOperatorControls,
//             42 => GlobalFunction::WaterPumpControl,
//             43 => GlobalFunction::TransmissionDisplay,
//             44 => GlobalFunction::ExhaustEmissionControl,
//             45 => GlobalFunction::VehicleDynamicStabilityControl,
//             46 => GlobalFunction::OilSensorUnit,
//             47 => GlobalFunction::InformationSystemController,
//             48 => GlobalFunction::RampControl,
//             49 => GlobalFunction::ClutchConverterControl,
//             50 => GlobalFunction::AuxiliaryHeater,
//             51 => GlobalFunction::ForwardLookingCollisionWarningSystem,
//             52 => GlobalFunction::ChassisController,
//             53 => GlobalFunction::AlternatorChargingSystem,
//             54 => GlobalFunction::CommunicationsUnitCellular,
//             55 => GlobalFunction::CommunicationsUnitSatellite,
//             56 => GlobalFunction::CommunicationsUnitRadio,
//             57 => GlobalFunction::SteeringColumnUnit,
//             58 => GlobalFunction::FanDriveControl,
//             59 => GlobalFunction::Starter,
//             60 => GlobalFunction::CabDisplay,
//             61 => GlobalFunction::FileServerPrinter,
//             62 => GlobalFunction::OnBoardDiagnosticUnit,
//             63 => GlobalFunction::EngineValveController,
//             64 => GlobalFunction::EnduranceBraking,
//             65 => GlobalFunction::GasFlowMeasurement,
//             66 => GlobalFunction::IOController,
//             67 => GlobalFunction::ElectricalSystemController,
//             68 => GlobalFunction::AfterTreatmentSystemGasMeasurement,
//             69 => GlobalFunction::EngineEmissionAfterTreatmentSystem,
//             70 => GlobalFunction::AuxiliaryRegenerationDevice,
//             71 => GlobalFunction::TransferCaseControl,
//             72 => GlobalFunction::CoolantValveController,
//             73 => GlobalFunction::RolloverDetectionControl,
//             74 => GlobalFunction::LubricationSystem,
//             75 => GlobalFunction::SupplementalFan,
//             76 => GlobalFunction::TemperatureSensor,
//             77 => GlobalFunction::FuelPropertiesSensor,
//             78 => GlobalFunction::FireSuppressionSystem,
//             79 => GlobalFunction::PowerSystemsManager,
//             80 => GlobalFunction::ElectricPowertrain,
//             81 => GlobalFunction::HydraulicPowertrain,
//             82 => GlobalFunction::FileServer,
//             83 => GlobalFunction::Printer,
//             84 => GlobalFunction::StartAidDevice,
//             85 => GlobalFunction::EngineInjectionControlModule,
//             86 => GlobalFunction::EVCommunicationController,
//             87 => GlobalFunction::DriverImpairmentDevice,
//             88 => GlobalFunction::ElectricPowerConverter,
//             89 => GlobalFunction::SupplyEquipmentCommunicationController,
//             90 => GlobalFunction::VehicleAdapterCommunicationController,
//             128 => GlobalFunction::Reserved,
//             129 => GlobalFunction::OffBoardDiagnosticServiceTool,
//             130 => GlobalFunction::OnBoardDataLogger,
//             131 => GlobalFunction::PCKeyboard,
//             132 => GlobalFunction::SafetyRestraintSystem,
//             133 => GlobalFunction::Turbocharger,
//             134 => GlobalFunction::GroundBasedSpeedSensor,
//             135 => GlobalFunction::Keypad,
//             136 => GlobalFunction::HumiditySensor,
//             137 => GlobalFunction::ThermalManagementSystemController,
//             138 => GlobalFunction::BrakeStrokeAlert,
//             139 => GlobalFunction::OnBoardAxleGroupScale,
//             140 => GlobalFunction::OnBoardAxleGroupDisplay,
//             141 => GlobalFunction::BatteryCharger,
//             142 => GlobalFunction::TurbochargerCompressorBypass,
//             143 => GlobalFunction::TurbochargerWastegate,
//             144 => GlobalFunction::Throttle,
//             145 => GlobalFunction::InertialSensor,
//             146 => GlobalFunction::FuelActuator,
//             147 => GlobalFunction::EngineExhaustGasRecirculation,
//             148 => GlobalFunction::EngineExhaustBackpressure,
//             149 => GlobalFunction::OnBoardBinWeighingScale,
//             150 => GlobalFunction::OnBoardBinWeighingScaleDisplay,
//             151 => GlobalFunction::EngineCylinderPressureMonitoringSystem,
//             152 => GlobalFunction::ObjectDetection,
//             153 => GlobalFunction::ObjectDetectionDisplay,
//             154 => GlobalFunction::ObjectDetectionSensor,
//             155 => GlobalFunction::PersonnelDetectionDevice,
//             255 => GlobalFunction::NotAvailable,
//             _ => GlobalFunctions::default(),
//         }
//     }
// }

// impl From<GlobalFunction> for u8 {
//     fn from(value: GlobalFunction) -> Self {
//         value as u8
//     }
// }

// /// Struct containing all On Highway Equipment Function ID's
// #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
// pub enum OnHighwayEquipmentFunction {
//     Tachograph = 128,
//     DoorController = 129,
//     ArticulationTurntableControl = 130,
//     BodyToVehicleInterfaceControl = 131,
//     SlopeSensor = 132,
//     RetarderDisplay = 134,
//     DifferentialLockController = 135,
//     LowVoltageDisconnect = 136,
//     RoadwayInformation = 137,
//     AutomatedDriving = 138,
//     #[default] NotAvailable = 255,
// }

// impl core::fmt::Display for OnHighwayEquipmentFunction {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl From<u8> for OnHighwayEquipmentFunction {
//     fn from(value: u8) -> Self {
//         match value {
//             128 => OnHighwayEquipmentFunction::Tachograph,
//             129 => OnHighwayEquipmentFunction::DoorController,
//             130 => OnHighwayEquipmentFunction::ArticulationTurntableControl,
//             131 => OnHighwayEquipmentFunction::BodyToVehicleInterfaceControl,
//             132 => OnHighwayEquipmentFunction::SlopeSensor,
//             134 => OnHighwayEquipmentFunction::RetarderDisplay,
//             135 => OnHighwayEquipmentFunction::DifferentialLockController,
//             136 => OnHighwayEquipmentFunction::LowVoltageDisconnect,
//             137 => OnHighwayEquipmentFunction::RoadwayInformation,
//             138 => OnHighwayEquipmentFunction::AutomatedDriving,
//             255 => OnHighwayEquipmentFunction::NotAvailable,
//             _ => OnHighwayEquipmentFunction::default(),
//         }
//     }
// }

// impl From<OnHighwayEquipmentFunction> for u8 {
//     fn from(value: OnHighwayEquipmentFunction) -> Self {
//         value as u8
//     }
// }

// TODO: implement other functions

// 128
// 129
// 130
// 131
// 132
// 133
// 255
// 255
// 255

// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 137
// 138
// 139
// 140
// 141
// 142
// 255

// 129
// 130
// 131
// 132
// 134
// 255

// 132
// 135
// 136
// 255

// 132
// 135
// 136
// 255

// 128
// 129
// 131
// 132
// 133
// 134
// 135
// 136
// 137
// 255

// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 255
// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 255
// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 255
// 132
// 133
// 134
// 135
// 255
// 128
// 129
// 131
// 132
// 133
// 135
// 255
// 255
// 132
// 136
// 255
// 255
// 132
// 255
// 132
// 255
// 128
// 132
// 133
// 134
// 135
// 255
// 132
// 255
// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 137
// 138
// 132
// 132
// 132
// 132
// 132
// 132
// 128
// 129
// 130
// 132
// 133
// 134
// 135
// 128
// 129
// 130
// 132
// 133
// 134
// 135
// 132
// 255
// 128
// 129
// 130
// 131
// 132
// 133
// 134
// 135
// 136
// 137
// 138
// 139
// 140
// 141
// 142
// 143
// 144
// 145
// 146
// 255
// 128
// 255
// 255
// 255
// 128
// 255
// 128
// 255
// 255
// 255
// 128
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 255
// 128
// 129
// 130
// 255
// 255
// 255
// 130
// 130
// 140
// 130
// 140
// 150
// 160
// 130
// 140
// 150
// 160
// 170
// 180
// 190
// 200
// 210
// 220
// 130
// 140
// 145
// 150
// 155
// 160
// 170
// 200
// 205
// 210
// 220
// 130
// 140
// 150
// 160
// 170
// 180
// 190
// 130
// 140
// 150
// 160
// 170
// 180
// 190
// 200
// 255
// 255
// 255
// 128
// 129
// 130
// 131
// 132
// 255
// 255
