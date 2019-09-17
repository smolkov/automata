use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration};
use serde_json::{
    from_slice,
    to_vec
};
use super::*;



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeasurementIter {
    pub value: f64,
    pub outlier: bool,
    // pub signal: Signal,
}



/// Measurement data result
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Measurement{
    pub path: PathBuf,
    pub method: PathBuf,

    pub value : f64,
    pub measurement: u64
    // pub data:  Vec<MeasIter>,
}




// impl struct Measurement {
    // pub fn result(&self) -> {
        // value
    // }
// }


pub struct Process {
    path: PathBuf,
}

pub async fn process(method: &Method) -> Result<Measurement> {

}

pub async fn history(method: &Method) -> Result<Measurement> {

}


pub async fn last(method: &Method) -> Result<Measurement> {

}

pub async fn current(mehod: &Method) -> Result<Measurement> {

}
