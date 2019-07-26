/// Temperatur sensor
/// Anschlus:  `Analog:IN04`
/// Model:     `presurei877`
///
use serde_derive::{Deserialize, Serialize};
use analyzer::{
    Airflow,
    Humidity,
    Pressure,
};
use crate::error::*;


use lazy_static::lazy_static;
// use std::sync::RwLock;
lazy_static! {
    
    // static ref SENSOR : RwLock<Humidity> = {
        // RwLock::new(Humidity::from_analog16(0))
    // };
    // static ref DATA : RwLock<Vec<f32>> = {
        // RwLock::new(Vec::new())
    // };
}


pub async fn get_airflow_input() -> Result<Airflow> {
    Ok(Airflow::from_analog16(0))
}
pub async fn get_airflow_output() -> Result<Airflow> {
    Ok(Airflow::from_analog16(0))
}
pub async fn get_humidity() -> Result<Humidity> {
    Ok(Humidity::from_analog16(0))
}
pub async fn get_pressure() -> Result<Pressure> {
    Ok(Pressure::from_analog16(0))
}
