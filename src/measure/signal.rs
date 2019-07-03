use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SignalIter {
    value :  f64,
    status:  usize, 
}

/// Sensor signal record;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Signal {
/// start time in mili seconds
    pub start:   u64,
/// Pereodic 
    pub period:  u64,
/// Sensor data
    pub data  :  Vec<SignalIter>,
/// Position of anomaly.
    pub anomaly: Vec<usize>,
}


impl Default for Signal{
    fn default() -> Self {
        Self {
            start:  0,
            period: 300,
            data: Vec::new(),
            anomaly: Vec::new(),
        }
    }
}
