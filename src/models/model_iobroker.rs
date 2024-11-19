use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt;

pub type IoBrokerResponse = HashMap<String, IoBrokerValue>;

// Added serde rename attributes to match exact API response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // If API uses camelCase
pub struct IoBrokerValue {
    #[serde(deserialize_with = "deserialize_string_or_number")]
    #[serde(rename = "val")]
    pub val: String,
    #[serde(rename = "ack")]
    pub ack: bool,
    #[serde(rename = "ts")]
    pub ts: i64,
    #[serde(rename = "q")]
    pub q: i32,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "lc")]
    pub lc: i64,
}

// Custom deserializer function that handles both string and number
fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumber;

    impl<'de> serde::de::Visitor<'de> for StringOrNumber {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or number")
        }

        fn visit_i64<E>(self, value: i64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_f64<E>(self, value: f64) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_str<E>(self, value: &str) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<String, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }
    }

    deserializer.deserialize_any(StringOrNumber)
}