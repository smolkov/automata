

use serde::{Deserialize, Serialize};
use analyzer::Adjustment;
// use crate::error::*;


/// Channel model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub id:         u64,
    pub stream_id:  u64,
    pub value:      f64,
    pub name:       String,
    pub unit:       String,
    pub adjust:     Vec<Adjustment>,
}

impl Channel {
    /// create new channel
    pub fn new(stream_id:u64) -> Channel {
        Channel {
            id: 0,
            stream_id:stream_id,
            value: 0.0,
            name: "CH".to_owned(),
            unit:"mg/l".to_owned(),
            adjust: Vec::new(),
        }
    }
    pub fn channel_number(&self) -> u64 {
        self.id
    }
    pub fn stream_number(&self) -> u64 {
        self.stream_id
    }
}

impl Default for Channel {
    fn default() -> Self {
        Channel::new(1)
    }
}
