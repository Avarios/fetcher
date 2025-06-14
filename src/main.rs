mod models;
mod mapper;
mod client;
mod postgres_client;
mod entity;

use std::env;
use crate::client::IoBrokerClient;
use crate::mapper::{map_lamda_data, map_to_temperature};
use std::error::Error;
use chrono::Utc;
use tokio::time::{self, Duration};
use tokio::signal::ctrl_c;
use tokio::sync::oneshot;
use crate::postgres_client::PostgresClient;

// Example handler functions
async fn handle_short_interval(io_broker:&IoBrokerClient,database_client:&PostgresClient) -> Result<(), Box<dyn Error>> {
    let lambda_data = match io_broker.fetch_data("/states?filter=modbus.0.holdingRegisters.*".to_string()).await {
        Ok(data) => data,
        Err(e) => Err(e)?
    };

    let mapped_lambda_data = match map_lamda_data(&lambda_data) {
        Ok(mapped_data) => mapped_data,
        Err(e) => {
            eprintln!("Error mapping data: {}", e);
            Err(e)?
        }
    };
    print!("Lambda data: {:#?} \n\n", &mapped_lambda_data );
    // Save the mapped data to the database     
    print!("Saving data to database...\n");
    database_client.write_lambda_data(mapped_lambda_data.clone()).await?;
    println!("Lambda data saved: {} \n {} \n\n", Utc::now().naive_local() , &mapped_lambda_data);
    Ok(())
}

async fn handle_long_interval(io_broker:&IoBrokerClient,database_client:&PostgresClient) -> Result<(), Box<dyn Error>> {

    let temperature_data = match io_broker.fetch_data("/states?filter=mqtt.0.adfhome.Temperatur*".to_string()).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching temperature: {}", e);
            Err(e)?
        }
    };

    let mapped_temperature_data = match map_to_temperature(temperature_data) {
        Ok(mapped_data) => mapped_data,
        Err(e) => {
            eprintln!("Error mapping data: {}", e);
            Err(e)?
        }
    };

    database_client.write_temperature_data(mapped_temperature_data.clone()).await?;
    println!("Temperature data saved: {} \n {:#?} \n\n", Utc::now().naive_local(), &mapped_temperature_data );
    Ok(())
} 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   
    
    let broker_url = env::var("IOBROKER_URL").map_err(|e| format!("BROKER_URL environment variable error: {}", e))?;
    let postgres_host = env::var("POSTGRES_HOST").map_err(|e| format!("POSTGRES_HOST environment variable error: {}", e))?;
    let postgres_port:u16 = env::var("POSTGRES_PORT").map_err(|e| format!("POSTGRES_PORT environment variable error: {}", e))?.parse().unwrap();
    let postgres_user = env::var("POSTGRES_USER").map_err(|e| format!("POSTGRES_USER error: {}", e))?;
    let postgres_password = env::var("POSTGRES_PASSWORD").map_err(|e| format!("POSTGRES_PASSWORD environment variable error: {}", e))?;
    let postgres_database = env::var("POSTGRES_DATABASE").map_err(|e| format!("POSTGRES_DATABASE environment variable error: {}", e))?;
    let database_client = PostgresClient::new(
        postgres_user,
        postgres_password,
        postgres_host,
        postgres_database,
        postgres_port,
    ).await?;
    let io_broker_client = IoBrokerClient::new(broker_url.to_string())?;

    let mut short_interval = time::interval(Duration::from_secs(30));
    let mut long_interval = time::interval(Duration::from_secs(60 * 15));

    // Create a shutdown channel
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

    // Spawn a task to handle Ctrl+C
    tokio::spawn(async move {
        if let Ok(_) = ctrl_c().await {
            let _ = shutdown_tx.send(());
        }
    });

    loop {
        tokio::select! {
            _ = short_interval.tick() => {
                println!("30-Seconds interval triggered");
                if let Err(e) = handle_short_interval(&io_broker_client, &database_client).await {
                    eprintln!("Error l: {}", e);
                }
            }
             _ = long_interval.tick() => {
                println!("30-Minutes interval triggered");
                if let Err(e) = handle_long_interval(&io_broker_client, &database_client).await {
                    eprintln!("Error in long interval: {}", e);
                }
            } 
            _ = &mut shutdown_rx => {
                println!("Shutdown signal received, cleaning up...");
                break;
            }
        }
    }

    println!("Program terminated successfully");
    Ok(())
}