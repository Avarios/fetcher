use std::fmt::{Display, Error, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureData {
    pub device: String,
    pub value:f64
}

impl Display for TemperatureData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
        writeln!(f, "{}", self.device)?;
        writeln!(f, "{}", self.value.to_string())
    }
}

