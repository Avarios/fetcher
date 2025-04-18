use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaData {
    #[serde(rename = "Ambient_State")]
    pub ambient_state: AmbientStateEnum,
    #[serde(rename = "Ambient_TemperatureCalculated")]
    pub ambient_temperature_calculated: f64,
    #[serde(rename = "EManager_OperatingState")]
    pub emanager_operating_state: EManagerStateEnum,
    #[serde(rename = "EManager_ActualPower")]
    pub emanager_actual_power: f64,
    #[serde(rename = "EManager_PVPower")]
    pub emanager_pv_power: f64,
    #[serde(rename = "EManager_PowerSetpoint")]
    pub emanager_power_setpoint: f64,
    #[serde(rename = "Heatpump_ErrorState")]
    pub heatpump_error_state: EManagerErrorStateEnum,
    #[serde(rename = "Heatpump_ErrorNumber")]
    pub heatpump_error_number: i32,
    #[serde(rename = "Heatpump_State")]
    pub heatpump_state: HeatPumpStateEnum,
    #[serde(rename = "Heatpump_OperatingState")]
    pub heatpump_operating_state: HeatPumpOperatingStateEnum,
    #[serde(rename = "Heatpump_FlowlineTemp")]
    pub heatpump_flowline_temp: f64,
    #[serde(rename = "Heatpump_ReturnLineTemp")]
    pub heatpump_return_line_temp: f64,
    #[serde(rename = "Heatpump_VolumeSink")]
    pub heatpump_volume_sink: f64,
    #[serde(rename = "Heatpump_EnergySourceInletTemp")]
    pub heatpump_energy_source_inlet_temp: f64,
    #[serde(rename = "Heatpump_VolumeSourceFlow")]
    pub heatpump_volume_source_flow: f64,
    #[serde(rename = "Heatpump_CompressorRating")]
    pub heatpump_compressor_rating: f64,
    #[serde(rename = "Heatpump_ActualHeatingCapacity")]
    pub heatpump_actual_heating_capacity: f64,
    #[serde(rename = "Heatpump_InverterActualPower" )]
    pub heatpump_inverter_actual_power: f64,
    #[serde(rename = "Heatpump_CurrentCop")]
    pub heatpump_current_cop: f64,
    #[serde(rename = "Heatpump_RequestType")]
    pub heatpump_request_type: HeatPumpRequestType,
    #[serde(rename = "Heatpump_RequestFlowTemp")]
    pub heatpump_request_flow_temp: f64,
    #[serde(rename = "Heatpump_RequestReturnTemp")]
    pub heatpump_request_return_temp: f64,
    #[serde(rename = "Heatpump_RequestTempDiff")]
    pub heatpump_request_temp_diff: f64,
    #[serde(rename = "Heatpump_ElectricEnergy")]
    pub heatpump_electric_energy: f64,
    #[serde(rename = "Heatpump_HeatEnergy")]
    pub heatpump_heat_energy: f64,
    #[serde(rename = "Boiler_State")]
    pub boiler_state: BoilerStateEnum,
    #[serde(rename = "Boiler_HighTemp")]
    pub boiler_high_temp: f64,
    #[serde(rename = "Boiler_LowTemp")]
    pub boiler_low_temp: f64,
    #[serde(rename = "Boiler_MaxTemp")]
    pub boiler_max_temp: f64,
    #[serde(rename = "Buffer_State")]
    pub buffer_state: BufferState,
    #[serde(rename = "Buffer_HighTemp")]
    pub buffer_high_temp: f64,
    #[serde(rename = "Buffer_LowTemp")]
    pub buffer_low_temp: f64,
    #[serde(rename = "Buffer_MaxTemp")]
    pub buffer_max_temp: f64,
    #[serde(rename = "HeatingCircuit_1_State")]
    pub heating_circuit_1_state: HeatingCircuitState,
    #[serde(rename = "HeatingCircuit_2_State")]
    pub heating_circuit_2_state: HeatingCircuitState,
    #[serde(rename = "HeatingCircuit_1_FlowTemp")]
    pub heating_circuit_1_flow_temp: f64,
    #[serde(rename = "HeatingCircuit_2_FlowTemp")]
    pub heating_circuit_2_flow_temp: f64,
}

impl Display for LambdaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
        writeln!(f, "===== Lambda Data Report =====")?;

        // Ambient section
        writeln!(f, "\n[Ambient]")?;
        writeln!(f, "State: {:?}", self.ambient_state)?;
        writeln!(f, "Temperature: {:.1}°C", self.ambient_temperature_calculated)?;

        // Energy Manager section
        writeln!(f, "\n[Energy Manager]")?;
        writeln!(f, "Operating State: {:?}", self.emanager_operating_state)?;
        writeln!(f, "Actual Power: {:.2} kW", self.emanager_actual_power)?;
        writeln!(f, "PV Power: {:.2} kW", self.emanager_pv_power)?;
        writeln!(f, "Power Setpoint: {:.2} kW", self.emanager_power_setpoint)?;

        // Heat Pump section
        writeln!(f, "\n[Heat Pump]")?;
        writeln!(f, "State: {:?}", self.heatpump_state)?;
        writeln!(f, "Operating State: {:?}", self.heatpump_operating_state)?;
        writeln!(f, "Error State: {:?}", self.heatpump_error_state)?;
        if self.heatpump_error_number != 0 {
            writeln!(f, "Error Number: {}", self.heatpump_error_number)?;
        }
        writeln!(f, "Flow Temperature: {:.1}°C", self.heatpump_flowline_temp)?;
        writeln!(f, "Return Temperature: {:.1}°C", self.heatpump_return_line_temp)?;
        writeln!(f, "Volume Sink: {:.1} l/h", self.heatpump_volume_sink)?;
        writeln!(f, "Source Inlet Temperature: {:.1}°C", self.heatpump_energy_source_inlet_temp)?;
        writeln!(f, "Source Flow: {:.1} l/h", self.heatpump_volume_source_flow)?;
        writeln!(f, "Compressor Rating: {:.1}%", self.heatpump_compressor_rating)?;
        writeln!(f, "Actual Heating Capacity: {:.2} kW", self.heatpump_actual_heating_capacity)?;
        writeln!(f, "Inverter Power: {:.2} kW", self.heatpump_inverter_actual_power)?;
        writeln!(f, "Current COP: {:.2}", self.heatpump_current_cop)?;
        writeln!(f, "Request Type: {:?}", self.heatpump_request_type)?;
        writeln!(f, "Request Flow Temperature: {:.1}°C", self.heatpump_request_flow_temp)?;
        writeln!(f, "Request Return Temperature: {:.1}°C", self.heatpump_request_return_temp)?;
        writeln!(f, "Request Temperature Difference: {:.1}K", self.heatpump_request_temp_diff)?;
        writeln!(f, "Electric Energy: {:.2} kWh", self.heatpump_electric_energy)?;
        writeln!(f, "Heat Energy: {:.2} kWh", self.heatpump_heat_energy)?;
        writeln!(f, "Calculated COP: {:.2}", self.heatpump_heat_energy / self.heatpump_electric_energy)?;

        // Boiler section
        writeln!(f, "\n[Boiler]")?;
        writeln!(f, "State: {:?}", self.boiler_state)?;
        writeln!(f, "High Temperature: {:.1}°C", self.boiler_high_temp)?;
        writeln!(f, "Low Temperature: {:.1}°C", self.boiler_low_temp)?;
        writeln!(f, "Max Temperature: {:.1}°C", self.boiler_max_temp)?;

        // Buffer section
        writeln!(f, "\n[Buffer]")?;
        writeln!(f, "State: {:?}", self.buffer_state)?;
        writeln!(f, "High Temperature: {:.1}°C", self.buffer_high_temp)?;
        writeln!(f, "Low Temperature: {:.1}°C", self.buffer_low_temp)?;
        writeln!(f, "Max Temperature: {:.1}°C", self.buffer_max_temp)?;

        // Heating Circuits section
        writeln!(f, "\n[Heating Circuit 1]")?;
        writeln!(f, "State: {:?}", self.heating_circuit_1_state)?;
        writeln!(f, "Flow Temperature: {:.1}°C", self.heating_circuit_1_flow_temp)?;

        writeln!(f, "\n[Heating Circuit 2]")?;
        writeln!(f, "State: {:?}", self.heating_circuit_2_state)?;
        writeln!(f, "Flow Temperature: {:.1}°C", self.heating_circuit_2_flow_temp)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Serialize, Deserialize, Display)]
#[repr(u8)]
pub enum AmbientStateEnum {
    #[serde(rename = "OFF")]
    Off = 0,
    #[serde(rename = "AUTOMATIC")]
    Automatic = 1,
    #[serde(rename = "MANUAL")]
    Manual = 2,
    #[serde(rename = "ERROR")]
    Error = 3
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize, Display)]
#[repr(u8)]
pub enum EManagerStateEnum {
    #[serde(rename = "OFF")]
    Off = 0,
    #[serde(rename = "AUTOMATIC")]
    Automatic = 1,
    #[serde(rename = "MANUAL")]
    Manual = 2,
    #[serde(rename = "ERROR")]
    Error = 3,
    #[serde(rename = "OFFLINE")]
    Offline = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize, Display)]
#[repr(u8)]
pub enum EManagerErrorStateEnum {
    #[serde(rename = "NONE")]
    None = 0,
    #[serde(rename = "MESSAGE")]
    Message = 1,
    #[serde(rename = "WARNING")]
    Warning = 2,
    #[serde(rename = "ALARM")]
    Alarm = 3,
    #[serde(rename = "FAULT")]
    Fault = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize, Display)]
#[repr(u8)]
pub enum HeatPumpStateEnum {
    #[serde(rename = "INIT")]
    Init = 0,
    #[serde(rename = "REFERENCE")]
    Reference = 1,
    #[serde(rename = "RESTART_BLOCK")]
    RestartBlock = 2,
    #[serde(rename = "READY")]
    Ready = 3,
    #[serde(rename = "START_PUMPS")]
    StartPumps = 4,
    #[serde(rename = "START_COMPRESSOR")]
    StartCompressor = 5,
    #[serde(rename = "PRE_REGULATION")]
    PreRegulation = 6,
    #[serde(rename = "REGULATION")]
    Regulation = 7,
    #[serde(rename = "NOT_USED")]
    NotUsed = 8,
    #[serde(rename = "COOLING")]
    Cooling = 9,
    #[serde(rename = "DEFROSTING")]
    Defrosting = 10,
    #[serde(rename = "STOPPING")]
    Stopping = 20,
    #[serde(rename = "FAULT_LOCK")]
    FaultLock = 30,
    #[serde(rename = "ALARM_BLOCK")]
    AlarmBlock = 31,
    #[serde(rename = "ERROR_RESET")]
    ErrorReset = 40,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize, Display)]
#[repr(u8)]
pub enum HeatPumpOperatingStateEnum {
    #[serde(rename = "STANDBY")]
    Stby = 0,
    #[serde(rename = "CH")]
    Ch = 1,
    #[serde(rename = "DHW")]
    Dhw = 2,
    #[serde(rename = "CC")]
    Cc = 3,
    #[serde(rename = "CIRCULATE")]
    Circulate = 4,
    #[serde(rename = "DEFROST")]
    Defrost = 5,
    #[serde(rename = "OFF")]
    Off = 6,
    #[serde(rename = "FROST")]
    Frost = 7,
    #[serde(rename = "STBY_FROST")]
    StbyFrost = 8,
    #[serde(rename = "NOT_USED")]
    NotUsed = 9,
    #[serde(rename = "SUMMER")]
    Summer = 10,
    #[serde(rename = "HOLIDAY")]
    Holiday = 11,
    #[serde(rename = "ERROR")]
    Error = 12,
    #[serde(rename = "WARNING")]
    Warning = 13,
    #[serde(rename = "INFO_MESSAGE")]
    InfoMessage = 14,
    #[serde(rename = "TIME_BLOCK")]
    TimeBlock = 15,
    #[serde(rename = "RELEASE_BLOCK")]
    ReleaseBlock = 16,
    #[serde(rename = "MIN_TEMP_BLOCK")]
    MintempBlock = 17,
    #[serde(rename = "FIRMWARE_DOWNLOAD")]
    FirmwareDownload = 18,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize, Display)]
#[repr(u8)]
pub enum HeatPumpRequestType {
    #[serde(rename = "NO_REQUEST")]
    NoRequest = 0,
    #[serde(rename = "FLOW_PUMP_CIRCULATION")]
    FlowPumpCirculation = 1,
    #[serde(rename = "CENTRAL_HEATING")]
    CentralHeating = 2,
    #[serde(rename = "CENTRAL_COOLING")]
    CentralCooling = 3,
    #[serde(rename = "DOMESTIC_HOT_WATER")]
    DomesticHotWater = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize, Display)]
#[repr(u8)]
pub enum BoilerStateEnum {
    #[serde(rename = "STBY")]
    Stby = 0,
    #[serde(rename = "DHW")]
    Dhw = 1,
    #[serde(rename = "LEGIO")]
    Legio = 2,
    #[serde(rename = "SUMMER")]
    Summer = 3,
    #[serde(rename = "FROST")]
    Frost = 4,
    #[serde(rename = "HOLIDAY")]
    Holiday = 5,
    #[serde(rename = "PRIO_STOP")]
    PrioStop = 6,
    #[serde(rename = "ERROR")]
    Error = 7,
    #[serde(rename = "OFF")]
    Off = 8,
    #[serde(rename = "PROMPT_DHW")]
    PromptDhw = 9,
    #[serde(rename = "TRAILING_STOP")]
    TrailingStop = 10,
    #[serde(rename = "TEMP_LOCK")]
    TempLock = 11,
    #[serde(rename = "STBY_FROST")]
    StbyFrost = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize, Display)]
#[repr(u8)]
pub enum BufferState {
    #[serde(rename = "STBY")]
    Stby = 0,
    #[serde(rename = "HEATING")]
    Heating = 1,
    #[serde(rename = "COOLING")]
    Cooling = 2,
    #[serde(rename = "SUMMER")]
    Summer = 3,
    #[serde(rename = "FROST")]
    Frost = 4,
    #[serde(rename = "HOLIDAY")]
    Holiday = 5,
    #[serde(rename = "PRIO_STOP")]
    PrioStop = 6,
    #[serde(rename = "ERROR")]
    Error = 7,
    #[serde(rename = "OFF")]
    Off = 8,
    #[serde(rename = "STBY_FROST")]
    StbyFrost = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize, Display)]
#[repr(u8)]
pub enum HeatingCircuitState {
    #[serde(rename = "HEATING")]
    Heating = 0,
    #[serde(rename = "ECO")]
    Eco = 1,
    #[serde(rename = "COOLING")]
    Cooling = 2,
    #[serde(rename = "FLOOR_DRY")]
    Floordry = 3,
    #[serde(rename = "FROST")]
    Frost = 4,
    #[serde(rename = "MAX_TEMP")]
    MaxTemp = 5,
    #[serde(rename = "ERROR")]
    Error = 6,
    #[serde(rename = "SERVICE")]
    Service = 7,
    #[serde(rename = "HOLIDAY")]
    Holiday = 8,
    #[serde(rename = "CH_SUMMER")]
    ChSummer = 9,
    #[serde(rename = "CC_WINTER")]
    CcWinter = 10,
    #[serde(rename = "PRIO_STOP")]
    PrioStop = 11,
    #[serde(rename = "OFF")]
    Off = 12,
    #[serde(rename = "RELEASE_OFF")]
    ReleaseOff = 13,
    #[serde(rename = "TIME_OFF")]
    TimeOff = 14,
    #[serde(rename = "STBY")]
    Stby = 15,
    #[serde(rename = "STBY_HEATING")]
    StbyHeating = 16,
    #[serde(rename = "STBY_ECO")]
    StbyEco = 17,
    #[serde(rename = "STBY_COOLING")]
    StbyCooling = 18,
    #[serde(rename = "STBY_FROST")]
    StbyFrost = 19,
    #[serde(rename = "STBY_FLOOR_DRY")]
    StbyFloordry = 20,
}