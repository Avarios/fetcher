use std::error::Error;
use influxdb2::{Client, models::DataPoint};
use chrono::Utc;
use serde_json::Value;
use futures::stream;
use influxdb2::models::data_point::DataPointBuilder;
use crate::models::model_lambda::LambdaData;
use crate::models::model_temperature::TemperatureData;

pub struct InfluxClient {
    client: Client,
}

impl InfluxClient {
    pub fn new(influx_url: String, org: String, auth_token: String) -> Self {
        let client: Client = Client::new(influx_url, org, auth_token);
        Self { client }
    }

    pub async fn write_lambda_data(
        &self,
        bucket: String,
        data: &LambdaData,
    ) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_value(data)?;
        let mut data_builder: DataPointBuilder = DataPoint::builder("Heating")
            .timestamp(Utc::now().timestamp_nanos_opt().unwrap() as i64);
        
        if let Value::Object(map) = json {
            for (field_name, value) in map {
                match value {
                    Value::Number(n) => {
                        if let Some(f) = n.as_f64() {
                            data_builder = data_builder.field(&field_name, f);
                        } else if let Some(i) = n.as_i64() {
                            data_builder = data_builder.field(&field_name, i);
                        }
                    }
                    Value::String(s) => {
                        data_builder = data_builder.field(&field_name, s);
                    }
                    Value::Bool(b) => {
                        data_builder = data_builder.field(&field_name, b);
                    }
                    _ => {}
                }
            }
        }

        let point = data_builder.build()?;
        println!("Writing value lambda data {:#?} " , point);
        self.client.write(bucket.as_str(), stream::iter(vec![point])).await?;
        Ok(())
    }

    pub async fn write_temperature_data(
        &self,
        bucket: String,
        temperatures: &[TemperatureData],
    ) -> Result<(), Box<dyn Error>> {
        let points:Vec<DataPoint> = temperatures.iter().map(|data| {
            let mut point = DataPoint::builder("Temperature")
                .timestamp(Utc::now().timestamp_nanos_opt().unwrap());
            point = point.tag("device",&data.device);
            point = point.field("value",data.value);
            return point.build().unwrap();
        }).collect::<Vec<DataPoint>>();
        println!("Writing values temperature data {:#?} " , points);
        self.client.write(bucket.as_str(), stream::iter(points)).await?;
        Ok(())
    }
}