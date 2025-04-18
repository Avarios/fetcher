use std::error::Error;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ConnectOptions, Database, DatabaseConnection};
use crate::{mapper::ToActiveModel, models::model_lambda::LambdaData};


pub struct PostgresClient {
    table_name: String,
    connect_options: ConnectOptions,
    db:DatabaseConnection
}

impl PostgresClient {
    pub async  fn new(
        user: String,
        password: String,
        hostname: String,
        database_name: String,
        table_name: String,
        port: u16,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = ConnectOptions::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, hostname, port, database_name
        ));
        let db = Database::connect(config.clone()).await.unwrap();
        Ok(PostgresClient { table_name, connect_options: config, db })
    }

    pub async fn write_lambda_data(&self, data: LambdaData) -> Result<i64, Box<dyn Error>> {
        println!("Writing data to database");
        let model = data.to_active_model();
        model.insert(&self.db).await?;
        println!("Data written to database");
        return Ok(1);
    }
}

