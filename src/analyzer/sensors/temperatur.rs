/// Temperatur sensor
/// Anschlus:  `Analog:IN04`
/// Model:     `presurei877`
///
use serde_derive::{Deserialize, Serialize};
use crate::systime;
// use crate::MioError;



use lazy_static::lazy_static;
use std::sync::RwLock;
lazy_static! {
    static ref TEMP01 : RwLock<Temperatur> = {
        RwLock::new(Temperatur::from_analog16(1021))
    };
}

/// Presure value model
///
/// fsr - full scale range
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Temperatur {
    pub updated : u64,
    pub value: f32,
    pub broken: bool,
}

impl Temperatur {
    pub fn from_analog16(value : u16) -> Temperatur {
        let mut signal =  value as f32 / 10.0;
        let broken = value>1000;

        Temperatur{
            updated:systime::now_sec(),
            value: signal,
            broken: broken,
        }
    }
}


pub async fn get_temperatur01() -> Temperatur {
    return TEMP01.read().unwrap().clone();
}




// pub async fn setup() -> Result<(),MioError> {
    // Ok(())
// }
//
// pub async fn Temperatur_value() -> Result<Temperatur, MioError> {
    // let analog_value  = io::analog_input16(0x4).await?;
    // Ok(Temperatur::from_analog16(analog_value))
// }
//
