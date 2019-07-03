//! `TIC`
//! Wiki: https://en.wikipedia.org/wiki/Total_organic_carbon
//! 
//! * Total Inorganic Carbon (TIC) – often referred to as inorganic carbon (IC), carbonate, bicarbonate, and dissolved carbon dioxide (CO2).
//! 


use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TIC{
    value: f64,
}
