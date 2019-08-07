

use serde::{Deserialize, Serialize};
use super::adjustment::Adjustment;
use super::stream;
use crate::Result;
use settings::ron::Config;
// use walkdir::{WalkDir,DirEntry};
// use log::info;
// use analyzer::flow::*;
// use analyzer::*;
use std::{
    fs,
    path::PathBuf,
};
// use crate::error::*;
// use crate::sensor::Sensor;

/// Channel model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub id:         u64,
    pub stream:     u64,
    pub value:      f64,
    pub name:       String,
    pub unit:       String,
    pub sensor:     u64,
    pub adjust:     Vec<Adjustment>,
    pub path:       String,
}

impl Channel {
    /// create new channel
    pub fn new(id:u64) -> Channel {
        Channel {
            id:  id,
            stream: 1,
            value: 0.0,
            name: "CH".to_owned(),
            unit:"mg/l".to_owned(),
            sensor: 0,
            adjust: Vec::new(),
            path: ".".to_owned(),
        }
    }
    pub fn get_path(&self) -> Result<PathBuf> {
        let path = stream::get_path(self.stream)?.join(format!("/{}",self.id));
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        Ok(path)
    }
}




impl Default for Channel {
    fn default() -> Self {
        Channel::new(1)
    }
}

pub async fn save(channel: &Channel) -> Result<()> {
    let path = channel.get_path()?.join("channel.ron");
    channel.write(path)?;
    Ok(())
}

// pub async fn get_stream_channel(stream:u64,channel:u64) ->
// pub async fn save(channel:&Channel) -> Result<()> {
    // let path = get_path(channel)?;
    // channel.write(path.join("channel.ron"))?;
    // Ok(())
// }
