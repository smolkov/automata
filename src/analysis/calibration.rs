use serde::{Deserialize, Serialize};
use super::*;
use solution::Solution;
// use std::time::Duration;
// use crate::*;

use std::{
    path::{Path,PathBuf},
};
// use automata_settings::ron::Config;

/// Linear
/// An indicator calibration and adjust
///

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Linear {
    pub slope : f64,
    pub intercept: f64,
    pub min: f64,
    pub max: f64,
}

/// Polygon calibration
pub type Polygon = Vec<Linear>;

/// Calibration
/// An indicator calibration and adjust
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum  Adjustment{
    None,
    Lineal(Linear),
    Polygon(Polygon),
}


impl Linear {
    pub fn new(slope:f64, intercept:f64) -> Linear {
        Self {
            slope : slope,
            intercept : intercept,
            min: 0.0,
            max: 500.0,
        }
    }
}

impl Default for Linear {
    fn default() -> Self {
        Self{
            slope : 1.0,
            intercept : 0.0,
            min: 0.0,
            max: 500.0,
        }
    }
}



#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Point {
    pub sol: Solution,
    pub val: Vec<f64>
    // pub cal: MeasurementIter,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Calibration {
    pub method: PathBuf,
    pub path: PathBuf,
    // pub timestamp: u64,
    // pub adjustment: Adjustment,
    // pub points: Vec<Point>,
}


pub async fn history(method:&Method) -> Result<Calibration> {
    let path = method.path.join("history");
    let method = method.path.clone();
    Ok(Calibration{
        method:method,
        path :path,
        })
}

// pub async fn last(method: &Method) -> Result<Calibration> {
    // let path = method.path.join("last");
    // let path = path.to_path_buf();
    // Ok(Calibration{path })
// }

// pub async fn
// pub async fn setup() -> Result<Calibration> {

// }

// impl Default for Calibration {
    // fn default() -> Self {
        // Self{
            // slope : 1.0,
            // intercept : 0.0,
            // min: 0.0,
            // max: 500.0,
        // }
    // }
// }

