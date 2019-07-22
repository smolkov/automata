use serde_derive::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Adjustment {
    pub slope : f64,
    pub intercept: f64,
    pub min: f64,
    pub max: f64,
}


impl Adjustment {
    pub fn new(slope:f64, intercept:f64) -> Adjustment {
        Self {
            slope : slope,
            intercept : intercept,
            min: 0.0,
            max: 500.0,
        }
    }
}

impl Default for Adjustment {
    fn default() -> Self {
        Self{
            slope : 1.0,
            intercept : 0.0,
            min: 0.0,
            max: 500.0,
        }
    }
}

