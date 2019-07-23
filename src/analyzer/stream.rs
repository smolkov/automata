use serde_derive::{Deserialize, Serialize};
use wqa_settings::{
    ConfigError,
    ron::Config
};

// use std::time::{Duration};
// use std::fmt;
use walkdir::{WalkDir,DirEntry};
// use serde_yaml;
use super::{
    adjustment::Adjustment,
};
use std::{
    fs::create_dir_all,
    // io::{Read, Write},
    path::{PathBuf},
};
use crate::workspace as store;
use crate::error::*;

/// Channel model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub value:   f64,
    pub name:    String,
    pub unit:    String,
    pub adjust:  Vec<Adjustment>,
}

impl Channel {
    /// create new channel
    pub fn new() -> Channel {
        Channel {
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

pub struct Channels {
    values: Vec<Channel>,
}

/// Stream
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Stream {
    pub number:   u64,
    pub name:     String,
    pub updated:  u64,
    pub channels: Vec<Channel>,
}


impl Default for Stream {
    fn default()-> Self {
        Self {
            number:   1,
            name:     "stream1".to_owned(),
            updated:  0,
            channels: Vec::new(),
        }
    }
}

pub fn local_directory(number:u64) -> Result<PathBuf> {
    let dir = store::streams_dir()?.join(format!("{}/",number));
    Ok(dir)
}

pub async fn list() ->Result<Vec<Stream>> {
    let path = store::streams_dir()?;
    let mut streams: Vec<Stream> = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let metadate = entry.metadata()?;
        if metadate.is_dir(){
            streams.push(Stream::load_no_fallback(entry.into_path().join("stream.ron"))?);
        }
    }
    Ok(streams)
}

pub async fn get_stream(number: u64) -> Result<Stream> {
    let dir = store::streams_dir()?.join(format!("{}/",number)).join("stream.ron");
    let stream = Stream::load_no_fallback(dir)?;
    Ok(stream)
}

pub async fn set_stream(stream:Stream) -> Result<()> {
    let stream_file = store::streams_dir()?.join(format!("{}/",stream.number)).join("stream.ron");
    stream.write(stream_file)?;
    Ok(())
}

