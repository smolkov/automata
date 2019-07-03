//! Chemical oxygen demand

//! Summenparameter in Umweltanalytik 
//! Wiki: https://de.wikipedia.org/wiki/Summenparameter
//! 
//! 
//! `COD`
//! Wiki: https://en.wikipedia.org/wiki/Chemical_oxygen_demand
//! 
//! In environmental chemistry, the chemical oxygen demand (COD) is an indicative measure of the amount of oxygen that can be consumed by reactions in a measured solution.
//! It is commonly expressed in mass of oxygen consumed over volume of solution which in SI units is milligrams per litre (mg/L). 
//! A COD test can be used to easily quantify the amount of organics in water. 
//! The most common application of COD is in quantifying the amount of oxidizable pollutants found in surface water (e.g. lakes and rivers) or wastewater. 
//! COD is useful in terms of water quality by providing a metric to determine the effect an effluent will have on the receiving body, much like biochemical oxygen demand (BOD).
//! 
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct COD{
    value: f64,
}

impl  COD{
    pub fn new(value:f64) -> Self {
        Self {value}
    }
}
