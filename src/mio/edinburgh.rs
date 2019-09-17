//! Edinburg access
//!
//! GAS MEASUREMENT RANGE
//! Model	CO2	CO
//! Chillcard NG	0-30%	0-10%
//! Chillcard NG	0-100%	0-30%
//! Accuracy	±2% of range
//! Zero stability	±2% or range (over 12 months)
//! Repeatability	At zero: generally <±1% of range. At span: <±2% or range
//! Response time	Bit-switch selectable T90 = variable (determined by bit-switch and firmware)
//! Operating temperature	0 - 45°C
//! Zero Drift due to ambient temperature	<± 0.1% range per C
//! Operating pressure	800 - 1150mbar (lower pressures with reduced accuracy)
//! Power requirements	24V DC (7 - 30V)
//! Warm-up time	Operation time: 1 minute (initial) Full specification: 30 mins
//! Humidity	Measurement unaffected by 0-90% RH non-condensing
//! Output	Linear 4 - 20mA, 0-20mA (bit-switch selectable) up to 500W or voltage output load
//! Controls	Bit-switch selection of various options zero and span adjust buttons
//! Please Note	Equipment is configured for one gas type at a time.
//! The methods exposed by this library are centered around

use serde::{Deserialize, Serialize};
use bincode::{deserialize, serialize};
use crate::error::*;
use super::Scale;
use serial;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Edin500,
    Edin1000,
    Edin3000,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Edinburgh {
    model: Model,
    scale: Scale,

}

impl Default for Edinburgh {
    fn default() -> Edinburgh {
        Edinburgh {
            model: Model::Edin500,
            scale: Scale::new(1.0),
        }
    }
}

// use lazy_static::lazy_static;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config{
    pub interval: u64,
    pub scale: Vec<Scale>,

}

const PORT_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud9600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone,
};

pub async fn model(edin:Edinburgh) -> Result<Model> {

}

pub async fn average()-> Result<Vec<f64>>{

}

pub async fn server(edin:Edinburgh) -> Result<()> {

    Ok(())
}


pub async fn ping(edin:&Edinburgh) -> Result<()> {
    Ok(())
}

pub async fn get_value(edin:&Edinburgh) -> Result<f64> {
    Ok(0.0)

}

// pub async fn decode_uart_data(read: impl Future<Output= Result<Vec<u8>>) -> Result<f64> {

//     Ok(0.0))
// }


// pub async fn setup_ndir1(min:f64,max:f64,read: impl Future<Output = Result<f64>>) -> Result<()> {
// pub async fn setup_ndir1(min:f64,max:f64) -> Result<()> {
    // Ok(())
// }
// pub async fn setup_ndir2(min:f64,max:f64) -> Result<()> {
    // Ok(())
// }


// pub async fn get_ndir1_value() -> Result<f64> {
    // Ok(0.0)
// }
// pub async fn get_ndir2_value() -> Result<f64> {
    // Ok(0.0)
// }
