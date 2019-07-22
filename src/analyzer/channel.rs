// use std::time::Duration;
use super::{
    adjustment::Adjustment,
    WqaError,
};
use walkdir::{WalkDir};
use std::{
    path::{PathBuf},
};
use serde_derive::{Deserialize, Serialize};
use wqa_settings::ron::Config;
use super::store;

/// Channel model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub id:     u64,
    pub on:     bool,
    pub name:   String,
    pub unit:   String,
    pub min:    f64,
    pub max:    f64,
    pub adjust: Vec<Adjustment>,
    path:       PathBuf,
}

impl Channel {
    /// create new channel
    pub fn new(id:u64) -> Channel {
        Channel {
            id: id,
            on : false,
            name: "CH".to_owned(),
            unit:"mg/l".to_owned(),
            min: 0.0,
            max: 0.0,
            adjust: Vec::new(),
            path: store::channel_directory().join(format!("channel{}",id)),
        }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Channel::new(1)
    }
}

// pub async fn local_path(channel: Channel) -> Result<(),WqaError> {
    // let path = store::channel_directory(num:u64)?;
// }

/// local channels in directory
pub async fn local_directory(path: PathBuf) -> Result<Vec<Channel>,WqaError> {
    // let path = store::streams_directory().join(format!"")
    let mut channels:Vec<Channel> = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let metadate = entry.metadata()?;
        if metadate.is_dir() {
            match entry.file_name().to_str() {
                Some(sp) => {
                    let part: Vec<_> = sp.matches("channel").collect();
                    if part.len() > 0 {
                        let path = entry.into_path();
                        channels.push(Channel::load_no_fallback(path.join("channel.ron"))?);
                    }
                },
                None => { }
            }
        }
     }
    Ok(channels)
}

pub async fn lacal_save(channel:&Channel) -> Result<(),WqaError> {
    let path = channel.path.clone();
    channel.write(path)?;
    Ok(())
}

//
