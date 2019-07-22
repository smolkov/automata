/// Humidity sensor
/// Anschlus Analog:IN04
///
use serde_derive::{Deserialize, Serialize};
// use futures::prelude::*;
// use crate::io;
use crate::{
    WqaError,
    systime
};

use lazy_static::lazy_static;
use std::sync::RwLock;
lazy_static! {
    static ref SENSOR : RwLock<Humidity> = {
        RwLock::new(Humidity::from_analog16(0))
    };
    static ref DATA : RwLock<Vec<f32>> = {
        RwLock::new(Vec::new())
    };
}
// use dbus;
// use dbus::Error as DBusError;
// use dbus::{BusType, Connection, Message};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Humidity {
    pub updated: u64,
    pub value: f32,
    pub broken: bool,
}

impl Humidity {
    pub fn from_analog16(value: u16) -> Humidity {
        let mut signal =  value as f32 / 4096.0 * 5.0;
        let broken = signal < 0.8 * 4.0 / 5.0;
        signal = (signal - 0.8)  / (3.6 - 0.8);
        Humidity {
            updated: systime::now_sec(),
            value: signal,
            broken: broken,
        }
    }
    pub fn set_analog16(&mut self, value: u16) {
        let mut signal =  value as f32 / 4096.0 * 5.0;
        let broken = signal < 0.8 * 4.0 / 5.0;
        signal = (signal - 0.8)  / (3.6 - 0.8);
        self.value = signal;
        self.broken = broken;
    }
}



pub async fn set_analog16(value: u16) {
    SENSOR.write().unwrap().set_analog16(value);
}

// pub type ReadAInput = impl Future<Output=Result<u16, WqaError>>;


pub async fn current_value() -> Result <Humidity,WqaError> {
    // let analog_value  = read.await?;
   Ok(SENSOR.read().unwrap().clone())
}

//
