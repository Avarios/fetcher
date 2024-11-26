use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};
use strum::EnumIter;

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

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Serialize, Deserialize)]
#[repr(u8)]
pub enum AmbientStateEnum {
    OFF = 0,
    AUTOMATIC = 1,
    MANUAL = 2,
    ERROR = 3
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize)]
#[repr(u8)]
pub enum EManagerStateEnum {
    Off = 0,
    Automatik = 1,
    Manual = 2,
    Error = 3,
    Offline = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize)]
#[repr(u8)]
pub enum EManagerErrorStateEnum {
    None = 0,
    Message = 1,
    Warning = 2,
    Alarm = 3,
    Fault = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize)]
#[repr(u8)]
pub enum HeatPumpStateEnum {
    Init = 0,
    Reference = 1,
    RestartBlock = 2,
    Ready = 3,
    StartPumps = 4,
    StartCompressor = 5,
    PreRegulation = 6,
    Regulation = 7,
    NotUsed = 8,
    Cooling = 9,
    Defrosting = 10,
    Stopping = 20,
    FaultLock = 30,
    AlarmBlock = 31,
    ErrorReset = 40,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize)]
#[repr(u8)]
pub enum HeatPumpOperatingStateEnum {
    Stby = 0,
    Ch = 1,
    Dhw = 2,
    Cc = 3,
    Circulate = 4,
    Defrost = 5,
    Off = 6,
    Frost = 7,
    StbyFrost = 8,
    NotUsed = 9,
    Summer = 10,
    Holiday = 11,
    Error = 12,
    Warning = 13,
    InfoMessage = 14,
    TimeBlock = 15,
    ReleaseBlock = 16,
    MintempBlock = 17,
    FirmwareDownload = 18,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize)]
#[repr(u8)]
pub enum HeatPumpRequestType {
    NoRequest = 0,
    FlowPumpCirculation = 1,
    CentralHeating = 2,
    CentralCooling = 3,
    DomesticHotWater = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize)]
#[repr(u8)]
pub enum BoilerStateEnum {
    Stby = 0,
    Dhw = 1,
    Legio = 2,
    Summer = 3,
    Frost = 4,
    Holiday = 5,
    PrioStop = 6,
    Error = 7,
    Off = 8,
    PromptDhw = 9,
    TrailingStop = 10,
    TempLock = 11,
    StbyFrost = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize)]
#[repr(u8)]
pub enum BufferState {
    Stby = 0,
    Heating = 1,
    Cooling = 2,
    Summer = 3,
    Frost = 4,
    Holiday = 5,
    PrioStop = 6,
    Error = 7,
    Off = 8,
    StbyFrost = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter,Serialize, Deserialize)]
#[repr(u8)]
pub enum HeatingCircuitState {
    Heating = 0,
    Eco = 1,
    Cooling = 2,
    Floordry = 3,
    Frost = 4,
    MaxTemp = 5,
    Error = 6,
    Service = 7,
    Holiday = 8,
    ChSummer = 9,
    CcWinter = 10,
    PrioStop = 11,
    Off = 12,
    ReleaseOff = 13,
    TimeOff = 14,
    Stby = 15,
    StbyHeating = 16,
    StbyEco = 17,
    StbyCooling = 18,
    StbyFrost = 19,
    StbyFloordry = 20,
}