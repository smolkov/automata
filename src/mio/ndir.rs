/// Sensors: NDir1, NDir2
/// Anschlus:  `DoppelMotor::UART01`
/// Model:     `presurei877`
// use serde_derive::{Deserialize, Serialize};

use crate::error::*;

// use lazy_static::lazy_static;



// lazy_static!{
// static ref NDIR1:RwLock<Sensor> = {
//    }
// }


pub async fn setup_ndir1(min:f64,max:f64) -> Result<()> {
    Ok(())
}
pub async fn setup_ndir2(min:f64,max:f64) -> Result<()> {
    Ok(())
}

pub async fn get_ndir1_value() -> Result<f64> {
    Ok(0.0)
}
pub async fn get_ndir2_value() -> Result<f64> {
    Ok(0.0)
}
