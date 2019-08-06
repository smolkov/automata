/// Sensors
/// NDir1,NDir2, Sauerstoff,
///
///

use serde::{Deserialize, Serialize};



/// Sensor model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    None,
    Simulation,
    Edinburgh,
    Aide,
    No,
    Zirox
}


pub type Range = std::ops::Range<f64>;

/// Serial sensor.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sensor{
    // updated: u64,
    pub fsr   : f64,
    pub model : Model,
    pub range:  Range,
}

impl Sensor {
    pub fn  new() -> Sensor  {
        Sensor {
            fsr : 0.0,
            model: Model::None,
            range: Range { start: 0.0, end: 1.0 }
        }
    }
}

impl Default for Sensor {
    fn default() -> Sensor {
        Sensor::new()
    }
}


// pub trait NDirs{
//     type Error;
//     fn ndir1(&self) -> Result<NDir,Self::Error>;
//     fn ndir2(&self) -> Result<NDir,Self::Error>;
//     fn ndir1_fsr(&self) -> Result<f64,Self::Error>;
//     fn ndir2_fsr(&self) -> Result<f64,Self::Error>;
//     fn ndir1_set_range(&self,range:NdirRange);
//     fn ndir2_set_range(&self,range:NdirRange);
// }


