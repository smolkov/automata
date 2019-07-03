//! `TC`
//! Wiki: https://en.wikipedia.org/wiki/Total_organic_carbon
//! * Total Carbon (TC) – all the carbon in the sample, including both inorganic and organic carbon
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TC{
    value: f64,
}



