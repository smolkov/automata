/// Humidity sensor
/// Anschlus Analog:IN04
///

use serde_derive::{Deserialize, Serialize};
use futures::prelude::*;
// use crate::io;
use crate::WqmError;

// use dbus;
use dbus::Error as DBusError;
use dbus::{BusType, Connection, Message};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Humidity {
    pub value: f32,
    pub broken: bool,
    pub warn: bool,
    pub crit: bool,
    pub max: f32,
}

impl Humidity {
    pub fn from_analog16(value: u16) -> Humidity {
        let mut signal =  value as f32 / 4096.0 * 5.0;
        let broken = signal < 0.8 * 4.0 / 5.0;
        signal = (signal - 0.8)  / (3.6 - 0.8);
        Humidity {
            value: signal,
            broken: broken,
            warn:false,
            crit:false,
            max: 60.0,
        }
    }
}





// pub type ReadAInput = impl Future<Output=Result<u16, WqmError>>;


pub async fn humidity() -> Result <Humidity,WqmError> {
    // let analog_value  = read.await?;
    Ok(Humidity::from_analog16(0))
}

//
