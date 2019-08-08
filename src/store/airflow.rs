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
use rand::prelude::*;
use failure::Fallible;
use std::path::PathBuf;

use crate::local::*;

use serde::{Deserialize, Serialize};

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
pub struct Airflow {
    pub value:   f32,
    pub broken:  bool,
    pub correlation:  f32,
    pub soll_value:   f32,
    pub input_deviation:      f32,
    pub output_deviation:     f32,
    pub monitorin_interval:   u64,
}


use crate::Route;


use settings::ron::Config;

impl Airflow {
    pub fn from_voltage(&self,voltage:f32) -> f32{
        let broken = voltage > 1.0;
        let airflow = 60.0 * ((((((A6*voltage +A5)*voltage +A4) *voltage +A3) *voltage + A2) * voltage + A1) *voltage + A0);
        Airflow {
            value: airflow,
            broken: broken,
        }
    }
    pub fn from_analog16(&self,value:u16) -> f32{
        let voltage = value as f32 / 4095.0 * 5.0;
        Airflow::from_voltage(voltage)
    }
}

pub async fn simulate_airflow_5v() -> Fallible<f32> {
    let mut rng = thread_rng();
    let r5v:f32 = rng.gen_range(1.0, 5.0);
    Ok(r5v)
}

pub async fn calibrate_airflow() -> Fallible<()> {

    Ok(())
}

pub async fn get_input(airflow:Airflow) -> Fallible<f32> {
    // let value = io::get_analog1_input01().await?;
    Ok(airflow.from_analog16(0 as u16))
}

pub async fn get_output(airflow:Airflow) -> Fallible<f32> {
    // let value = io::get_analog1_input02().await?;
    Ok(airflow.from_analog16(0))
}



pub fn directory() -> Fallible<PathBuf> {
    let path = rootdir()?.join("/airflow/");
    Ok(path)
}

pub async fn read(route:Route) -> Fallible<Airflow> {
    let path = directory()?.join("config");
    let airflow = Airflow::load_no_fallback(path)?;
    Ok(airflow)
}

pub async fn save(airflow: Airflow) -> Fallible<()> {
    let path = directory()?.join("config");
    airflow.write(path)?;
    Ok(())
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
