use crate::models::model_iobroker::IoBrokerResponse;
use reqwest::{self, Client, ClientBuilder};
use std::fmt;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub enum ClientError {
    NetworkError(reqwest::Error),
    ParseError(String),  // Changed to store more detailed error info
    HttpError(reqwest::StatusCode),
    TimeoutError,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::NetworkError(e) => write!(f, "Network error: {}", e),
            ClientError::ParseError(e) => write!(f, "Parse error: {}", e),
            ClientError::HttpError(code) => write!(f, "HTTP error: {}", code),
            ClientError::TimeoutError => write!(f, "Request timed out"),
        }
    }
}

impl std::error::Error for ClientError {}

#[derive(Clone)]
pub struct IoBrokerClient {
    client: Client,
    base_url: String,
    max_retries: u32,
}

impl IoBrokerClient {
    pub fn new(base_url: String) -> Result<Self, ClientError> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(5)
            .build()
            .map_err(ClientError::NetworkError)?;

        Ok(Self {
            client,
            base_url,
            max_retries: 1,
        })
    }

    pub async fn fetch_data(&self, path: String) -> Result<IoBrokerResponse, ClientError> {
        let mut attempts = 0;
        let mut last_error = None;
        println!("Fetching data from: {}", path);
        
        while attempts < self.max_retries {
            match self.try_fetch_data(path.clone()).await {
                Ok(data) => return Ok(data),
                Err(e) => {
                    last_error = Some(e);
                    attempts += 1;
                    if attempts < self.max_retries {
                        sleep(Duration::from_secs(2u64.pow(attempts))).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or(ClientError::TimeoutError))
    }

    async fn try_fetch_data(&self,path:String) -> Result<IoBrokerResponse, ClientError> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(ClientError::NetworkError)?;

        if !response.status().is_success() {
            return Err(ClientError::HttpError(response.status()));
        }
        
        let text = response.text().await.map_err(ClientError::NetworkError)?;
        serde_json::from_str(&text).map_err(|e| {
            ClientError::ParseError(format!("Parse error: {}. Response: {}", e, text))
        })
    }
}