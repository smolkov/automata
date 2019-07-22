//! Summenparameter in Umweltanalytik 
//! Wiki: https://de.wikipedia.org/wiki/Summenparameter
//! 
//! 
//! `BOD`
//!  Wiki: https://en.wikipedia.org/wiki/Total_organic_carbon
//! 
//! Biochemical Oxygen Demand (BOD, also called Biological Oxygen Demand) is the amount of dissolved oxygen needed (i.e. demanded) 
//! by aerobic biological organisms to break down organic material present in a given water sample at certain temperature over a specific time period. 
//! The BOD value is most commonly expressed in milligrams of oxygen consumed per litre of sample during 5 days of incubation at 20 °C and is often used as a surrogate of the degree of organic pollution of water.[1]
//! BOD can be used as a gauge of the effectiveness of wastewater treatment plants. BOD is similar in function to chemical oxygen demand (COD), in that both measure the amount of organic compounds in water.
//! However, COD is less specific, since it measures everything that can be chemically oxidized, rather than just levels of biodegradable organic matter.
//! 

use serde_derive::{Deserialize, Serialize};
// use super::channel::{Channel};
use std::time::{SystemTime};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BODItem {
    time: u64,
    value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BOD{
    value: f64,
}

impl  BOD{
    pub fn new(value:f64) -> Self {
        Self {value}
    }
}

