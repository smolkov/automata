/// Sensors: NDir1, NDir2
/// Anschlus:  `DoppelMotor::UART01`
/// Model:     `presurei877`
// use serde_derive::{Deserialize, Serialize};

use crate::error::*;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    None,
    Edin500,
    Edin3000,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Edinburgh {
    pub model: Model,
    pub min : f64,
    pub max: f64,
}

impl Default for Edinburgh {
    fn default() -> Edinburgh {
        Edinburgh {
            model: Model::None,
            min: 0.0,
            max: 1.0,
        }
    }
}



async fn decode() -> Result<f64> {

    Ok((0.0))
}



// pub async fn setup_ndir1(min:f64,max:f64,read: impl Future<Output = Result<f64>>) -> Result<()> {
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
