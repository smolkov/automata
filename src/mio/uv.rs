//! Uv monitor api
//!
//!
//!
use crate::error::*;




pub async fn sample_pump(start: bool) -> Result<()> {
    Ok(())
}

pub async fn stream1_valve( open: bool)  -> Result<()> {
    Ok(())
}

pub async fn stream2_valve( open: bool)  -> Result<()> {
    Ok(())
}
pub async fn stream3_valve( open: bool) -> Result<()> {
    Ok(())
}
pub async fn stream4_valve( open: bool) -> Result<()> {
    Ok(())
}
pub async fn stream5_valve( open: bool) -> Result<()> {
    Ok(())
}
pub async fn stream6_valve( open: bool) -> Result<()> {
    Ok(())
}

pub async fn zeroflow_valve( open:bool) -> Result<()> {
    Ok(())
}
pub async fn tic_valve( open:bool) -> Result<()> {
    Ok(())
}
pub async fn calibration_valve( open:bool) -> Result<()> {
    Ok(())
}

pub async fn ndir1_value( open:bool) -> Result<f64> {
    Ok(0.0)
}

pub async fn ndir2_value( open: bool) -> Result<f64> {
    Ok(0.0)
}

pub async fn is_fluid1() -> Result<bool> { Ok(false) }
pub async fn is_fluid2() -> Result<bool> { Ok(false) }
pub async fn is_fluid3() -> Result<bool> { Ok(false) }
pub async fn is_fluid4() -> Result<bool> { Ok(false) }
pub async fn is_fluid5() -> Result<bool> { Ok(false) }
pub async fn is_fluid6() -> Result<bool> { Ok(false) }

