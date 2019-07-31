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
    pub fn channel_id(&self) -> u64 {
        self.id
    }
}

impl Default for Channel {
    fn default() -> Self {
        Channel::new(1)
    }
}

// pub struct Channels {
    // values: Vec<Channel>,
// }

/// Stream
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Stream {
    pub id:          u64,
    pub name:        String,
    pub updated:     u64,
    pub description: String,
    // pub measu:     String,
}


impl Default for Stream {
    fn default()-> Self {
        Self {
            id:          1,
            name:        "#1".to_owned(),
            updated:     0,
            description: "none".to_owned(),
        }
    }
}


impl Stream {
    pub fn new(id:u8) -> Stream {
        Self {
            id:       id as u64,
            name:     format!("#{}",id),
            updated:  0,
            description: "none".to_owned(),
        }
    }
}
