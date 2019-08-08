/// Humidity sensor
/// Anschlus Analog:IN04
///
use serde::{Deserialize, Serialize};
use failure::Fallible;
use lazy_static::lazy_static;

lazy_static! {
    static ref INTERVAL:u64 = 2000;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Humidity {
    id:  u64,
    broken: bool
    // pub injection_threshold:  f32,
}



impl Default for Humidity {
    fn default() -> Self {
        Self {
            id: 0,
            broken: true,
        }
    }
}

impl Humidity {
    pub fn from_analog16(&self, value: u16) -> f32 {
        let signal =  value as f32 / 4096.0 * 5.0;
        Humidity::from_voltage(signal)
    }
    pub fn from_voltage(&self, voltage:f32) -> f32 {
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
