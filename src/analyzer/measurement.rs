use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration};




// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct MeasurementIter {
//     pub value: f64,
//     pub outlier: bool,
//     // pub signal: Signal,
// }


/// Measurement data result
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Measurement {
    pub changed : u64,
    pub value : f64,
    pub data:  Vec<f64>,
}







//
