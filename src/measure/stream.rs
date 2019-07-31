use serde::{Deserialize, Serialize};
use crate::error::*;
use super::channel::*;
use super::store;
use {
    futures::{
        executor::block_on,
    },
};
// use super::measurement::*;
// use super::calibration::*;

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
    pub channels:    Vec<Channel>,
    // pub measu:     String,
}


impl Default for Stream {
    fn default() -> Self {
        Self {
            id:          1,
            name:        "#1".to_owned(),
            updated:     0,
            description: "none".to_owned(),
            channels:    Vec::new(),
        }
    }
}


impl Stream {
    pub fn new(id:u8) -> Stream {
        Self {
            id:          id as u64,
            name:        format!("#{}",id),
            updated:     0,
            description: "none".to_owned(),
            channels:    Vec::new(),
        }
    }
    pub fn get_channels(&self) -> Vec<Channel> {
        self.channels.clone()
    }
    pub fn update_channels(&mut self) -> Result<usize> {
        let channels = block_on(store::get_stream_channels(self.id))?;
        self.channels = channels;
        Ok(self.channels.len())
    }
}

