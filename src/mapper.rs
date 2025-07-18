use crate::models::{
    model_iobroker::IoBrokerResponse, model_lambda::LambdaData, model_temperature::TemperatureData,
};

use crate::entity::{
    heatpump::ActiveModel as HeatPumpModel, temperature_data::ActiveModel as TemperatureModel,
};
use chrono::Utc;
use sea_orm::Set;
use serde_json::Value;
use std::fmt;
use std::io::Error;
use strum::IntoEnumIterator;

impl std::error::Error for ConversionError {}

#[derive(Debug)]
pub enum ConversionError {
    InvalidData(String),
    KeyNotFound(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConversionError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            ConversionError::KeyNotFound(key) => write!(f, "Key not found: {}", key),
        }
    }
}

fn get_enum<T: IntoEnumIterator>(
    broker_value: &IoBrokerResponse,
    key: String,
) -> Result<T, ConversionError> {
    let result = match get_value(broker_value, key.clone()) {
        Ok(val) => T::iter()
            .nth(val)
            .ok_or(ConversionError::InvalidData(key.clone())),
        Err(e) => Err(e),
    };
    result
}

fn get_value<T: std::str::FromStr>(
    broker_value: &IoBrokerResponse,
    key: String,
) -> Result<T, ConversionError> {
    // First, try to get the IoBrokerValue for the given key
    let value = broker_value
        .get(&key)
        .ok_or_else(|| ConversionError::KeyNotFound(key.clone()))?;

    // Then try to parse the val field into type T
    value
        .val
        .parse::<T>()
        .map_err(|_| ConversionError::InvalidData(key.clone()))
}

pub fn map_lamda_data(broker_value: &IoBrokerResponse) -> Result<LambdaData, ConversionError> {
    let model = LambdaData {
        ambient_state: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.40002_Ambient_State"),
        )?,
        ambient_temperature_calculated: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.40005_Ambient_Calculated_Temp"),
        )?,
        emanager_operating_state: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.40102_E_Manager_State"),
        )?,
        emanager_actual_power: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.40104_E_Manager_Actual"),
        )?,
        emanager_pv_power: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.40103_E-Manager_ExcessPower"),
        )?,
        emanager_power_setpoint: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.40105_E_Manager_Setpoint"),
        )?,
        heatpump_error_state: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.41001_HP1_Error_State"),
        )?,
        heatpump_error_number: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41002_HP1_Error"),
        )?,
        heatpump_state: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.41003_HP1_State"),
        )?,
        heatpump_operating_state: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.41004_HP1_OperatingState"),
        )?,
        heatpump_flowline_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41005_HP1_T_Flow"),
        )?,
        heatpump_return_line_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41006_HP1_T_Return"),
        )?,
        heatpump_volume_sink: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41007_HP1_Vol_Sink"),
        )?,
        heatpump_energy_source_inlet_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41008_HP1_T_EQin"),
        )?,
        heatpump_volume_source_flow: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41010_HP1_Vol_Source"),
        )?,
        heatpump_compressor_rating: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41011_HP1_CompressorRating"),
        )?,
        heatpump_actual_heating_capacity: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41012_HP1_QpHeating"),
        )?,
        heatpump_inverter_actual_power: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41013_HP1_FI_PowerConsumption"),
        )?,
        heatpump_current_cop: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41014_HP1_COP"),
        )?,
        heatpump_request_type: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.41016_HP1_RequestType"),
        )?,
        heatpump_request_flow_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41017_HP1_RequestT_Flow"),
        )?,
        heatpump_request_return_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41018_HP1_RequestT_Return"),
        )?,
        heatpump_request_temp_diff: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41019_HP1_RequestT_Diff"),
        )?,
        heatpump_electric_energy: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41021_HP1_VdA_E"),
        )?,
        heatpump_heat_energy: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.41023_HP1_VdA_Q"),
        )?,
        boiler_state: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.42002_Boiler1_OperatingState"),
        )?,
        boiler_high_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.42003_Boiler1_ActualHighTemp"),
        )?,
        boiler_low_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.42004_Boiler1_ActualLowTemp"),
        )?,
        boiler_max_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.42051_Boiler1_MaximumTemp"),
        )?,
        buffer_state: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.43002_Buffer1_OperatingState"),
        )?,
        buffer_high_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.43003_Buffer1_ActualHighTemp"),
        )?,
        buffer_low_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.43004_Buffer1_ActualLowTemp"),
        )?,
        buffer_max_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.43051_Buffer1_MaximumTemp"),
        )?,
        heating_circuit_1_state: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.45002_Heating1_OperatingState"),
        )?,
        heating_circuit_2_state: get_enum(
            broker_value,
            String::from("modbus.0.holdingRegisters.45102_Heating2_OperatingState"),
        )?,
        heating_circuit_1_flow_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.45003_Heating1_T_Flow"),
        )?,
        heating_circuit_2_flow_temp: get_value(
            broker_value,
            String::from("modbus.0.holdingRegisters.45103_Heating2_T_Flow"),
        )?,
    };
    Ok(model)
}

pub fn map_to_temperature(response: IoBrokerResponse) -> Result<Vec<TemperatureData>, Error> {
    let temperature_data: Vec<TemperatureData> = response
        .into_iter()
        .filter(|(_, value)| !value.val.is_empty())
        .filter_map(|(device, value)| {
            let json_result: Value = match serde_json::from_str(&value.val) {
                Ok(result) => result,
                Err(_) => return None,
            };

            if !json_result["temperature"].is_null() {
                let temperature = match json_result["temperature"].to_string().parse::<f64>() {
                    Ok(temp) => temp,
                    Err(_) => return None,
                };

                let formatted_device = device.split("_").collect::<Vec<&str>>()[1].to_string();

                Some(TemperatureData {
                    device: formatted_device,
                    value: temperature,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(temperature_data)
}

pub trait ToLambdaDataModel {
    fn to_lambda_data(self) -> HeatPumpModel;
}

pub trait ToTemperatureDataModel {
    fn to_temperature_data(self) -> TemperatureModel;
}

impl ToTemperatureDataModel for Vec<TemperatureData> {
    fn to_temperature_data(self) -> TemperatureModel {
        let json_value = serde_json::to_value(&self).unwrap_or_default();

        TemperatureModel {
            event_timestamp: Set(
                Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
            ),
            data: Set(json_value),
        }
    }
}

impl ToLambdaDataModel for LambdaData {
    fn to_lambda_data(self) -> HeatPumpModel {
        HeatPumpModel {
            event_timestamp: Set(
                Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
            ),
            ambient_state: Set(self.ambient_state.to_string()),
            ambient_temperaturecalculated: Set(self.ambient_temperature_calculated),
            boiler_hightemp: Set(self.boiler_high_temp),
            boiler_lowtemp: Set(self.boiler_low_temp),
            boiler_maxtemp: Set(self.boiler_max_temp),
            boiler_state: Set(self.boiler_state.to_string()),
            buffer_hightemp: Set(self.buffer_high_temp),
            buffer_lowtemp: Set(self.buffer_low_temp),
            buffer_maxtemp: Set(self.buffer_max_temp),
            buffer_state: Set(self.buffer_state.to_string()),
            heatingcircuit_1_flowtemp: Set(self.heating_circuit_1_flow_temp),
            heatingcircuit_1_state: Set(self.heating_circuit_1_state.to_string()),
            heatingcircuit_2_flowtemp: Set(self.heating_circuit_2_flow_temp),
            heatingcircuit_2_state: Set(self.heating_circuit_2_state.to_string()),
            heatpump_actualheatingcapacity: Set(self.heatpump_actual_heating_capacity),
            heatpump_compressorrating: Set(self.heatpump_compressor_rating),
            heatpump_currentcop: Set(self.heatpump_current_cop),
            heatpump_electricenergy: Set(self.heatpump_electric_energy),
            heatpump_energysourceinlettemp: Set(self.heatpump_energy_source_inlet_temp),
            heatpump_errornumber: Set(self.heatpump_error_number as f64),
            heatpump_errorstate: Set(self.heatpump_error_state.to_string()),
            heatpump_flowlinetemp: Set(self.heatpump_flowline_temp),
            heatpump_heatenergy: Set(self.heatpump_heat_energy),
            heatpump_inverteractualpower: Set(self.heatpump_inverter_actual_power),
            heatpump_operatingstate: Set(self.heatpump_operating_state.to_string()),
            heatpump_requestflowtemp: Set(self.heatpump_request_flow_temp),
            heatpump_requestreturntemp: Set(self.heatpump_request_return_temp),
            heatpump_requesttempdiff: Set(self.heatpump_request_temp_diff),
            heatpump_requesttype: Set(self.heatpump_request_type.to_string()),
            heatpump_returnlinetemp: Set(self.heatpump_return_line_temp),
            heatpump_state: Set(self.heatpump_state.to_string()),
            heatpump_volumesink: Set(self.heatpump_volume_sink),
            heatpump_volumesourceflow: Set(self.heatpump_volume_source_flow),
        }
    }
}
