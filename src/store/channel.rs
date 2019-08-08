
use crate::local::rootdir;
use super::stream;
// use walkdir::{WalkDir,DirEntry};
// use log::info;
// use analyzer::flow::*;
use settings::ron::Config;
use serde::{Deserialize, Serialize};
use failure::Fallible;
use std::{
    fs,
    path::PathBuf,
};
// use crate::error::*;
// use crate::sensor::Sensor;

/// Channel model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub value:      f64,
    pub name:       String,
    pub unit:       String,
    pub min:        f64,
    pub max:        f64,
}

pub struct Limit{
    pub name: String,
    pub min: f64,
    pub max: f64,
}

impl Channel {
    /// create new channel
    pub fn new(id:u64) -> Channel {
        Channel {
            path:  PathBuf::new("/channel-1/"),
            value: 0.0,
            name: "CH".to_owned(),
            unit:"mg/l".to_owned(),
        }
    }

}

impl Default for Channel{
    fn default() -> Self {
        Channel::new(1)
    }
}


pub fn directory(id: u64) -> Fallible<PathBuf> {
    let path = rootdir()?.join(format!("/channel-{}/",id));
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

pub async fn save(channel: &Channel) -> Fallible<()> {
    let path = directory(channel.id)?.join("config");
    channel.write(path)?;
    Ok(())
}
pub async fn read(route:Route) -> Fallible<Channel> {
    let path = directory(id)?.join("config");
    let channel = Channel::load_no_fallback(path)?;
    Ok(channel)
}
// pub async fn add_sensor(channel:Channel,sensor:ndir::Sensor) -> Result<()>{
    // let path =
// }


pub async fn adjustment(channel: Channel ) -> Fallible<Adjustment> {
    let path = directory(channel.id)?.join("adjust");
    let adj = Adjustment::load_no_fallback(path)?;
    Ok(adj)
}


pub struct Channels {
    route: crate::Route
}

// pub struct Chan {
    // pub fn
// }



// pub async fn get_stream_channel(stream:u64,channel:u64) ->
// pub async fn save(channel:&Channel) -> Result<()> {
    // let path = get_path(channel)?;
    // channel.write(path.join("channel.ron"))?;
    // Ok(())
// }
