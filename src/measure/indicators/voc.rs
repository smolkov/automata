use serde_derive::{Deserialize, Serialize};
// use super::channel::{Channel};
use std::time::{SystemTime};



/// Purgeable (volatile) Organic Compound (VOC) – organic carbon that has been removed from a neutral, or acidified sample by purging with an inert gas. 
/// These are the same compounds referred to as Volatile Organic Compounds (VOC) and usually determined by Purge and Trap Gas Chromatography.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VOC{
    value: f64,
}
