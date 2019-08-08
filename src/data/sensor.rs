use serde::{Deserialize, Serialize};
// use std::future::Future;
use std::fmt;
// use std::pin::Pin;
// use settings::ron::Config;
// use csv;
// use std::{
    // fs,
    // path::PathBuf,
// };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
/// time in mili seconds
    pub time:   u64,
/// Pereodic
    pub value:  u64,
/// state
    pub state: u8,
}



impl Default for Record {
    fn default() -> Self {
        Self {
            start:  0,
            period: 300,
            data:  Vec::new(),
        }
    }
}

pub type Range = std::ops::Range<f64>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AverageSignal {
//sart time
    pub start:   u64,
// Pereodic
    pub meas_id:  u64,
// Sensor data
    pub value  :  f64,
}


impl Default for AverageSignal{
    fn default() -> Self {
        Self {
            start:   0,
            meas_id: 0,
            value:   0.0,
        }
    }
}

impl AverageSignal {
    fn new(meas_id:u64) -> AverageSignal {
         Self {
            start:   0,
            meas_id: 0,
            value:   0.0,
        }
    }

}

/// Sensor model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Simulation,
    Edinburgh,
    Aide,
    No,
    Zirox
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::Simulation => write!(f, "simulation"),
            Model::Edinburgh => write!(f, "edinburgh"),
            Model::Aide => write!(f, "aide"),
            Model::No => write!(f, "aide"),
            Model::Zirox => write!(f, "aide"),
            _ => write!(f, "not supported"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sensor {
    pub name: String,
    pub interval: u64,
    pub range: Range,
}


impl Default for Sensor {
    fn default() -> Self {
        Sensor::new(Model::Simulation)
    }
}

impl Sensor {
    pub fn  new(id:u64) -> Sensor  {
        Sensor {
            interval : 0.0,
            model: Model::Simulation,
            range: Range { start: 0.0, end: 1.0 }
        }
    }
}

