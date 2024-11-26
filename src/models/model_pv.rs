use std::fmt::{Display, Error, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PvData {
    pub battery_power:i64,
    pub battery_percentage:f64,
    pub grid:i64,
    pub home:i64,
    pub pv:i64,
    pub wallbox:i64
}

impl Display for crate::models::model_pv::PvData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(),Error> {
        writeln!(f, "Battery power : {}", self.battery_power)?;
        writeln!(f, "Battery percentage : {}", self.battery_percentage)?;
        writeln!(f, "Grid Power: {}", self.grid)?;
        writeln!(f, "Home Power{}", self.home)?;
        writeln!(f, "PV Power{}", self.pv)?;
        writeln!(f, "Wallbox Power{}", self.wallbox)
    }
}
