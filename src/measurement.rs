use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration};
// use super::common::*;
use analyzer::Statistic;
use chrono::Utc;
use super::stream::*;

use lazy_static::lazy_static;
// use std::sync::RwLock;
lazy_static! {
    static ref MEASUREMENTS:u64 = 0;
}

pub struct Measurement {
    pub timestamp:   u64,
    pub value:       f64,
    pub counter:     u64,
    pub outlier:     bool,
}

fn add_counter() -> u64 {
    let counter  = *MEASUREMENTS+1;
    *MEASUREMENTS =  counter;
    counter
}



impl Measurement {
    pub fn new (counter:u64) -> Self{
        Self{
            timestamp:     Utc::now().timestamp() as u64,
            value:         0.0,
            counter:       counter,
            outlier:       false,
        }
    }
}



pub struct MeasurementBuilder  {
    stream: u64,
    channel: u64,
    measurement: u64,
    statistic: Statistic,
    // values: Vec<Replicate>,
}

impl MeasurementBuilder {
    pub fn new(stream: u64,channel: u64) -> Self {
       Self{
           stream: stream,
           channel:channel,
           measurement: add_counter(),
           statistic:Statistic::default(),
           values:Vec::new(),
       }
    }
}
//
