/// Humidity sensor
/// Anschlus Analog:IN04
///
use serde::{Deserialize, Serialize};
use super::io;
use crate::Result;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HumiditySetting {
    pub humidity_crit_level:  f32,
    pub monitorin_interval:   u64,
    // pub injection_threshold:  f32,
}



impl Default for HumiditySetting {
    fn default() -> Self {
        Self {
            humidity_crit_level:  70.0,
            monitorin_interval:   0,
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Humidity {
    pub value:   f32,
    pub broken:  bool,
}

impl Humidity {
    pub fn from_analog16(value: u16) -> Humidity {
        let signal =  value as f32 / 4096.0 * 5.0;
        Humidity::from_voltage(signal)
    }
    pub fn from_voltage(voltage:f32) -> Humidity {
        let broken = voltage < 0.8 * 4.0 / 5.0;
        let humidity = ((voltage - 0.8)  / (3.6 - 0.8))*100.0;
        Humidity {
            value:   humidity,
            broken:  broken,
        }
    }
}


pub async fn get_humidity() -> Result<Humidity> {
    let value = io::get_analog1_input03().await?;
    Ok(Humidity::from_analog16(value))
}
