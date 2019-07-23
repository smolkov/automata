use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration};
use wqa_settings::ron::Config;
use crate::error::*;
use crate::workspace as store;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeasIter {
    pub value: f64,
    pub outlier: bool,
    pub signal: u64,
//     // pub signal: Signal,
}


/// Measurement data result
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Measurement {
    pub stream: u64,
    pub changed : u64,
    pub value : f64,
    pub cv: f64,
    pub data:  Vec<MeasIter>,
}

impl Measurement {

}



pub async fn last(measurement:Measurement) -> Result<()> {
    // let path = store::history_path().
    Ok(())
}




//
