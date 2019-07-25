use serde::{Deserialize, Serialize};
use analyzer::Adjustment;
// use crate::error::*;


/// Channel model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub id:      u64,
    pub value:   f64,
    pub name:    String,
    pub unit:    String,
    pub adjust:  Vec<Adjustment>,
}

impl Channel {
    /// create new channel
    pub fn new() -> Channel {
        Channel {
            id: 0,
            value: 0.0,
            name: "CH".to_owned(),
            unit:"mg/l".to_owned(),
            adjust: Vec::new(),
        }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Channel::new()
    }
}

// pub struct Channels {
    // values: Vec<Channel>,
// }

/// Stream
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Stream {
    pub id:   u64,
    pub name:     String,
    pub updated:  u64,
    // pub measu:     String,
}


impl Default for Stream {
    fn default()-> Self {
        Self {
            id:   1,
            name:     "stream1".to_owned(),
            updated:  0,
        }
    }
}


