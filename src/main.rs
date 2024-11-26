mod models;
mod mapper;
mod client;
mod influxdb;

use std::env;
use crate::client::IoBrokerClient;
use crate::mapper::{map_lamda_data, map_to_temperature};
use std::error::Error;
use tokio::time::{self, Duration};
use tokio::signal::ctrl_c;
use chrono::{Utc};
use tokio::sync::oneshot;
use crate::influxdb::InfluxClient;

// Example handler functions
async fn handle_short_interval(io_broker:&IoBrokerClient,influx_client:&InfluxClient,bucket_name:String) -> Result<(), Box<dyn Error>> {
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

    println!("Lambda: {} \n {} \n\n", Utc::now().naive_local() , &mapped_lambda_data);
    
    influx_client.write_lambda_data(bucket_name, &mapped_lambda_data).await?;
    println!("Wrote Datapoint");
    Ok(())
}

async fn handle_long_interval(io_broker:&IoBrokerClient,influx_client:&InfluxClient,bucket_name:String) -> Result<(), Box<dyn Error>> {

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

    println!("Temperature: {} \n {:#?} \n\n", Utc::now().naive_local(), &mapped_temperature_data );
    influx_client.write_temperature_data(bucket_name, &mapped_temperature_data).await?;
    println!("Wrote Datapoint");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   
    
    let broker_url = env::var("IOBROKER_URL").map_err(|e| format!("BROKER_URL environment variable error: {}", e))?;
    let influx_url = env::var("INFLUX_URL").map_err(|e| format!("INFLUX_URL environment variable error: {}", e))?;
    let auth_token = env::var("INFLUX_AUTH_TOKEN").map_err(|e| format!("AUTH_TOKEN environment variable error: {}", e))?;
    let bucket_name = env::var("INFLUX_BUCKETNAME").map_err(|e| format!("INFLUX_BUCKETNAME error: {}", e))?;
    let org = env::var("INFLUX_ORG").map_err(|e| format!("ORG environment variable error: {}", e))?;
    

    let io_broker_client = IoBrokerClient::new(broker_url.to_string())?;
    let influx_client = InfluxClient::new(influx_url.to_string(), org.to_string(),auth_token.to_string());
    
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
                if let Err(e) = handle_short_interval(&io_broker_client, &influx_client, bucket_name.clone()).await {
                    eprintln!("Error l: {}", e);
                }
            }
            _ = long_interval.tick() => {
                println!("30-Minutes interval triggered");
                if let Err(e) = handle_long_interval(&io_broker_client, &influx_client, bucket_name.clone()).await {
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