

use serde::{Deserialize, Serialize};
use super::adjustment::Adjustment;
// use crate::error::*;
use super::sensor::Sensor;

/// Channel model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub id:         u64,
    pub value:      f64,
    pub name:       String,
    pub unit:       String,
    pub sensor:     Sensor,
    pub adjust:     Vec<Adjustment>,
    pub path:       String,
}

impl Channel {
    /// create new channel
    pub fn new(id:u64) -> Channel {
        Channel {
            id:  id,
            value: 0.0,
            name: "CH".to_owned(),
            unit:"mg/l".to_owned(),
            sensor: Sensor::default(),
            adjust: Vec::new(),
            path: ".".to_owned(),
        }
    }
}




impl Default for Channel {
    fn default() -> Self {
        Channel::new(1)
    }
}


// pub async fn get_stream_channel(stream:u64,channel:u64) ->
// pub async fn save(channel:&Channel) -> Result<()> {
    // let path = get_path(channel)?;
    // channel.write(path.join("channel.ron"))?;
    // Ok(())
// }
