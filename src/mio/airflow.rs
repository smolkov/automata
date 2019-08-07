/// Airlow sensor
/// IN  Anschlus `Analog:IN01`
/// OUT Anschlus `Analog:IN02`
///
///

// use crate::io;
// use std::ops::Range;
// use crate::error::*;
// use super::common::*;

// use tempfile::{
    // tempdir_in,
    // TempDir,
// };
use serde::{Deserialize, Serialize};
use rand::prelude::*;
use super::io;
use crate::Result;

// use lazy_static::lazy_static;
// use std::sync::RwLock;

// lazy_static! {
    // static ref IN : RwLock<Airflow> = {
        // RwLock::new(Airflow::from_analog16(0))
    // };
    // static ref OUT : RwLock<Airflow> = {
        // RwLock::new(Airflow::from_analog16(0))
    // };
// }


// static max:f64 = 0.5;
// static
// static max:f64 = 0.4;

/// für 0..60:   0.230197;
static A6:f32 = 0.003836617;
/// für 0..60:  -3.616438;
static A5:f32 = -0.06027397;
/// für 0..60:  22.36370;
static A4:f32 = 0.3727283;
/// für 0..60: -68.58285;
static A3:f32 = -1.1430475;
/// für 0..60: 110.3052;
static A2:f32 = 1.83842;
/// für 0..60: -84.19201;
static A1:f32 = -1.4032;
/// für 0..60:  23.49542;
static A0:f32 = 0.39159;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AirflowSetting {

    // pub injection_threshold:  f32,
}




// pub type FlowRange = Range<f32>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Airflow {
    pub value:   f32,
    pub broken:  bool,
    pub correlation:  f32,
    pub soll_value:   f32,
    pub input_deviation:      f32,
    pub output_deviation:     f32,
    pub monitorin_interval:   u64,
}



impl Airflow {
    pub fn from_voltage(voltage:f32) -> Airflow{
        let broken = voltage > 1.0;
        let airflow = 60.0 * ((((((A6*voltage +A5)*voltage +A4) *voltage +A3) *voltage + A2) * voltage + A1) *voltage + A0);
        Airflow {
            value: airflow,
            broken: broken,
        }
    }
    pub fn from_analog16(value:u16) -> Airflow{
        let voltage = value as f32 / 4095.0 * 5.0;
        Airflow::from_voltage(voltage)
    }
}

pub async fn simulate_airflow_5v() -> Result<f32> {
    let mut rng = thread_rng();
    let r5v:f32 = rng.gen_range(1.0, 5.0);
    Ok(r5v)
}

pub async fn calibrate_airflow() -> Result<()> {

    Ok(())
}

pub async fn get_input() -> Result<Airflow> {
    let value = io::get_analog1_input01().await?;
    Ok(Airflow::from_analog16(value))
}

pub async fn get_output() -> Result<Airflow> {
    let value = io::get_analog1_input02().await?;
    Ok(Airflow::from_analog16(value))
}
use crate::local::*;

pub async read() -> Airflow {
    let path = WQ
}




// pub async fn warning_input() -> Option<Range> {
    // None
// }
// pub async fn critical_range_input() -> Option<Range> {
    // None
// }

// pub async fn setup_range_input(warn: Option<Range>, critical : Option<Range>) {

// }



// pub async fn airflow_input_value(input_read: ReadAnalogu16) -> Result<Airflow,MioError>{
//     let analog_value = input_read.await?;
//     Ok(Airflow::from_analog16(analog_value))
// }
// pub async fn airflow_output_value(output_read:ReadAnalogu16) -> Result<Airflow,MioError>{
//     let analog_value = output_read.await?;
//     Ok(Airflow::from_analog16(analog_value))
// }
