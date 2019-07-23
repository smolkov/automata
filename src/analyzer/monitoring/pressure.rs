/// Pressure sensor
/// Anschlus:  `Analog:IN04`
/// Model:     `presurei877`
///

use serde_derive::{Deserialize, Serialize};
use crate::error::*;
use crate::systime;



use lazy_static::lazy_static;
use std::sync::RwLock;
lazy_static! {
    static ref PRESSURE : RwLock<Pressure> = {
        RwLock::new(Pressure::from_analog16(0))
    };
    static ref DATA : RwLock<Vec<f32>> = {
        RwLock::new(Vec::new())
    };
}

/// Presure value model
///
/// fsr - full scale range
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pressure {
    pub updated: u64,
    pub fsr: f32,
    pub broken: bool,
}

impl Pressure {
    pub fn from_analog16(value: u16) -> Pressure {
        let mut signal =  value as f32 / 4096.0 * 5.0;
        let broken = signal < 1.0 * 4.0 / 5.0;
        signal = (signal - 1.0)  / (5.0 - 1.0);

        Pressure{
            updated: systime::now_sec(),
            fsr: signal,
            broken: broken,
        }
    }
    pub fn set_analog16(&mut self , value: u16) {
        let mut signal =  value as f32 / 4096.0 * 5.0;
        let broken = signal < 1.0 * 4.0 / 5.0;
        signal = (signal - 1.0)  / (5.0 - 1.0);
        self.updated= systime::now_sec();
        self.broken = broken;
        self.fsr = signal;
    }
}

pub async fn set_analog16_value(value:u16) {
    PRESSURE.write().unwrap().set_analog16(value);
}

pub async fn pressure() -> Result<Pressure> {
    Ok(PRESSURE.read().unwrap().clone())
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
