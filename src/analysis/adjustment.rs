use serde::{Deserialize, Serialize};

pub type Range = core::ops::Range<f64>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Adjustment {
    pub slope : f64,
    pub intercept: f64,
    pub range: Range,
}


impl Adjustment {
    pub fn new(slope:f64, intercept:f64) -> Adjustment {
        Self {
            slope : slope,
            intercept : intercept,
            range: Range{start:0.0,end:500.0},
        }
    }
}

impl Default for Adjustment {
    fn default() -> Self {
        Self::new(1.0,0.0)
    }
}

