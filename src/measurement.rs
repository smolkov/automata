use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration};
use super::common::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeasData {
    pub value: f64,
    pub outlier: bool,
    pub signal: u64,
//     // pub signal: Signal,
}


pub struct Measurement {
    pub updated: u64,
    pub cv:      f64,
    pub value:   f64,
    pub datas:    Vec<MeasData>,
}


impl Measurement {
    pub fn new() -> Measurement {
        Measurement{
            updated: 0,
            cv:      0.0,
            value:   0.0,
            datas:    Vec::new(),
        }
    }
    pub fn add(&mut self, data: MeasData ) {
        self.updated = now_sec();
        self.datas.push(data);
    }
}


pub struct Method  {
    values: Vec<Measurement>,
    
}

//
