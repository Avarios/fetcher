use std::error::Error;
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection};
use crate::{mapper::{ToLambdaDataModel, ToTemperatureDataModel}, models::{model_lambda::LambdaData, model_temperature::TemperatureData}};


pub struct PostgresClient {

    db:DatabaseConnection
}

impl PostgresClient {
    pub async  fn new(
        user: String,
        password: String,
        hostname: String,
        database_name: String,
        port: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = ConnectOptions::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, hostname, port, database_name
        ));
        let db = Database::connect(config.clone()).await.unwrap();
        Ok(PostgresClient { db })
    }

    pub async fn write_lambda_data(&self, data: LambdaData) -> Result<i64, Box<dyn Error>> {
        println!("Writing data to database");
        let model = data.to_lambda_data();
        model.insert(&self.db).await?;
        println!("Data written to database");
        return Ok(1);
    }

    pub async fn write_temperature_data(&self, data: Vec<TemperatureData>) -> Result<i64, Box<dyn Error>> {
        println!("Writing temperature data to database");
        let model = data.to_temperature_data();
        model.insert(&self.db).await?;
        println!("Temperature data written to database");
        return Ok(1);
    }
}

