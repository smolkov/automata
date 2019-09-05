use super::*;
use chrono::prelude::*;
// use walkdir::{WalkDir,DirEntry};
// use log::info;
// use analyzer::flow::*;
use serde::{Deserialize, Serialize};

use async_std::fs;
use std::path::PathBuf;
use super::adjustment::Adjustment;
// use crate::error::*;
// use crate::sensor::Sensor;


pub type Range = std::ops::Range<f64>;

/// Channel model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub path:       PathBuf,
    pub label:      String,
    pub unit:       String,
    pub range:      Range,
    pub value:      f64,
}

pub struct Limit{
    pub name: String,
    pub min: f64,
    pub max: f64,
}



pub async fn is_channel(path: PathBuf ) -> io::Result<bool> {
    Ok(true)
}



// impl Channel {
//     /// create new channel
//     pub fn new(id:u64) -> Channel {
//         Channel {
//             path:  PathBuf::new("/channel-1/"),
//             value: 0.0,
//             name: "CH".to_owned(),
//             unit:"mg/l".to_owned(),
//         }
//     }
// }

// impl Default for Channel{
//     fn default() -> Self {
//         Channel::new(1)
//     }
// }


// pub fn directory(id: u64) -> Fallible<PathBuf> {
    // let path = rootdir()?.join(format!("/channel-{}/",id));
    // if !path.exists() {
        // fs::create_dir_all(&path)?;
    // }
    // Ok(path)
// }

pub async fn channel(path:PathBuf) -> Result<Channel> {
    Ok(
        Channel{
            path:path,
            label:"CH".to_owned(),
            unit: "mg/l".to_owned(),
            range: Range{start:0.0,end:0.0},
            value: 0.0,

        }
    )
}


// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Measurement {
    // pub timestamp: u64,
    // pub value: f64,
// }

// pub async fn read_measurement(channel:&Channel) -> io::Result<Measurement> {
//     let path = channel.path.join("measurement");
//     let mut file = fs::File::open(path.as_path()).await?;
//     let mut buf = Vec::new();
//     file.read_to_end(&mut buf).await?;
//     let measurement: Measurement = from_slice(buf.as_slice())?;
//     Ok(measurement)
// }

/// read config
// pub async fn read_config(channel:&Channel) -> io::Result<Config>{
//     let path = channel.path.join("config");
//     let mut file = fs::File::open(path.as_path()).await?;
//     let mut buf = Vec::new();
//     file.read_to_end(&mut buf).await?;
//     let config: Config = from_slice(buf.as_slice())?;
//     Ok(config)
// }

/// write config
// pub async fn write_config(channel:&Channel,config:&Config) -> io::Result<()> {
//     let path = channel.path.join("config");
//     let mut file = fs::File::create(path.as_path()).await?;
//     let state = serde_json::to_vec(config).unwrap();
//     file.write_all(state.as_slice()).await?;
//     file.sync_data().await?;
//     // info!("c[{}] turn {}", Paint::cyan(format!("{}", id)), Paint::cyan("close"));
//     Ok(())
// }

/// read channel adjustment
pub async fn adjustment(channel: &Channel) -> io::Result<()> {
    let path = channel.path.join("adjustment");
    let adj = fs::read_to_string(&path).await?;
    // let adj : Adjustment = from_slice(adj.as_slice())?;
    Ok(())
}


/// read channel sensor component
pub async fn sensor(channel: &Channel) -> Result<PathBuf> {
    Ok(channel.path.join("sensor"))
}

/// get channel calibration component
pub async fn calibration(channel: &Channel) -> Result<PathBuf> {
    Ok(channel.path.join("calibration"))
}

// pub async fn write_config(channel:&Channel,config:&Config) -> io::Result<()> {
//     let path = channel.path.join("config");
//     channel.write(path)?;
//     Ok(())
// }
// pub async fn read(route:Route) -> Fallible<Channel> {
//     let path = directory(id)?.join("config");
//     let channel = Channel::load_no_fallback(path)?;
//     Ok(channel)
// }
// pub async fn add_sensor(channel:Channel,sensor:ndir::Sensor) -> Result<()>{
// let path =
// }



// pub async fn get_stream_channel(stream:u64,channel:u64) ->
// pub async fn save(channel:&Channel) -> Result<()> {
    // let path = get_path(channel)?;
    // channel.write(path.join("channel.ron"))?;
    // Ok(())
// }
