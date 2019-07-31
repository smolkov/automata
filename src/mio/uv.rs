//! Uv monitor api
//!
//!
//!
use crate::error::*;
use super::io;


/// Sample pump start stop.
pub async fn sample_pump(start: bool) -> Result<()> {
    io::set_digital1_output(2,start).await
}
/// Open measurement sample N
pub async fn open_sample_valve(sample:u8)  -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn close_sample_valve(sample:u8)  -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn lamp_switch_on() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn lamp_switch_off() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn open_zeroflow_valve() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn close_zeroflow_valve() -> Result<()> {
    io::set_digital1_output(4,false).await
}
pub async fn open_tic_valve() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn close_tic_valve() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn open_calibration_valve() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn close_calibration_valve( ) -> Result<()> {
    io::set_digital1_output(4,false).await
}

pub async fn is_fluid(sample:u8) -> Result<bool> {
    io::get_digital2_input(2).await
}

pub async fn get_ndir1_value() -> Result<f64> {
    Ok(0.0)
}

pub async fn get_ndir2_value() -> Result<f64> {
    Ok(0.0)
}


