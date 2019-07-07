/// Pressure sensor
/// Anschlus:  `Analog:IN04`
/// Model:     `presurei877`
///
use serde_derive::{Deserialize, Serialize};
use crate::WqmError;



use lazy_static::lazy_static;
use std::sync::RwLock;
lazy_static! {
    static ref PRESSURE : RwLock<Pressure> = {
        RwLock::new(Pressure::from_analog16(0))
    };
}

/// Presure value model
///
/// fsr - full scale range
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pressure {
    pub fsr: f32,
    pub broken: bool,
}

impl Pressure {
    pub fn from_analog16(value: u16) -> Pressure {
        let mut signal =  value as f32 / 4096.0 * 5.0;
        let broken = signal < 1.0 * 4.0 / 5.0;
        signal = (signal - 1.0)  / (5.0 - 1.0);
        Pressure{
            fsr: signal,
            broken: broken,
        }
    }
}


pub async fn pressure() -> Result<Pressure,WqmError> {
    Ok(Pressure::from_analog16(0))
}


// pub async fn setup() -> Result<(),MioError> {
    // Ok(())
// }
//
// pub async fn pressure_value() -> Result<Pressure, MioError> {
    // let analog_value  = io::analog_input16(0x4).await?;
    // Ok(Pressure::from_analog16(analog_value))
// }
//
