//! Uv monitor api
//!
//!
//!
use crate::Result;
use super::io;




/// Sample pump start stop.
pub async fn sample_pump(start: bool) -> Result<()> {
    io::set_digital1_output(2,start).await
}
/// Open measurement sample N
pub async fn open_sample_valve(num:u8)  -> Result<()> {
    io::set_digital1_output(num,true).await
}
pub async fn close_sample_valve(num:u8)  -> Result<()> {
    io::set_digital1_output(num,true).await
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

pub async fn is_fluid(_sample:u8) -> Result<bool> {
    io::get_digital2_input(2).await
}

