#[allow(dead_code)]
use serde::{Deserialize, Serialize};
// use crate::error::*;
use super::channel::*;
use settings::ron::Config;
use crate::Result;

use std::fs;
use walkdir::{WalkDir,DirEntry};
// use log::info;
// use analyzer::flow::*;
// use analyzer::*;
use std::{
    path::PathBuf,
    fs::{create_dir_all},
};
// use {
    // futures::{
        // executor::block_on,
    // },
// };
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
    // pub fn update_channels(&mut self) -> Result<usize> {
    //     let channels = block_on(store::get_stream_channels(self.id))?;
    //     self.channels = channels;
    //     Ok(self.channels.len())
    // }
}

/// Get streams work directory
pub fn get_directory() -> Result<PathBuf> {
    let path = super::data_dir()?;
    let path = path.join("stream");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

fn entry_is_stream(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with("stream.ron")
}

pub async fn search_all() ->Result<Vec<Stream>> {
    let path = get_directory()?;
    let mut streams: Vec<Stream> = Vec::new();
    for entry in WalkDir::new(path).into_iter()
        .filter_entry(|e| entry_is_stream(e))
        .filter_map(|e| e.ok()) {
        let file_type = entry.file_type();
        if file_type.is_file(){
            let cfg = entry.path();
            streams.push(Stream::load_no_fallback(cfg)?);
        }
    }
    Ok(streams)
}
pub fn get_path(id:u64) -> Result<PathBuf> {
    let path = get_directory()?.join(format!("{}/",id));
    if !path.exists() {
        create_dir_all(&path)?;
    }
    Ok(path)
}

/// Load from stream/:id/stream.ron
pub async fn read(id: u64) -> Result<Stream> {
    let dir = get_directory()?.join(format!("{}/",id)).join("stream.ron");
    let stream = Stream::load_no_fallback(dir)?;
    Ok(stream)
}

pub async fn save(stream:&Stream) -> Result<()> {
    let path = get_path(stream.id)?.join("stream.ron");
    stream.write(path)?;
    Ok(())
}
fn entry_is_channel(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with("channel.ron")
}

pub async fn channels(id:u64) -> Result<Vec<Channel>> {
    let path = get_path(id)?;
    let mut channels:Vec<Channel> = Vec::new();
    for entry in WalkDir::new(path).into_iter()
        .filter_entry(|e| entry_is_channel(e))
        .filter_map(|e| e.ok()) {
        let file_type = entry.file_type();
        if file_type.is_file(){
            let cfg = entry.path();
            channels.push(Channel::load_no_fallback(cfg)?);
        }
    }
    Ok(channels)
}


// pub async get1() -> Result<Stream> {

// }

//
