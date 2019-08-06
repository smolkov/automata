/// Pressure sensor
/// Anschlus:  `Analog:IN04`
/// Model:     `presurei877`
///

use serde::{Deserialize, Serialize};
// use crate::error::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PressureSetting {
    pub pressure_warn_level:  f32,
    pub pressure_crit_level:  f32,
    pub monitorin_interval:   u64,
    // pub injection_threshold:  f32,
}



impl Default for PressureSetting {
    fn default() -> Self {
        Self {
            pressure_warn_level:  300.0,
            pressure_crit_level:  600.0,
            monitorin_interval:   0,
        }
    }
}


// use lazy_static::lazy_static;
// use std::sync::RwLock;
// lazy_static! {
//     static ref PRESSURE : RwLock<Pressure> = {
//         RwLock::new(Pressure::from_analog16(0))
//     };
//     static ref DATA : RwLock<Vec<f32>> = {
//         RwLock::new(Vec::new())
//     };
// }

/// Presure value model
///
/// fsr - full scale range
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pressure {
    pub value: f32,
    pub broken: bool,
}

impl Pressure {
    pub fn from_analog16(value: u16) -> Pressure {
        let voltage =  value as f32 / 4096.0 * 5.0;
        Pressure::from_voltage(voltage)
    }
    pub fn from_voltage(voltage: f32) -> Pressure {
        let broken = voltage< 1.0 * 4.0 / 5.0;
        let pressure = ((voltage - 1.0)  / (5.0 - 1.0))*1000.0;
        Pressure{
            value:pressure,
            broken: broken,
        }
    }
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
