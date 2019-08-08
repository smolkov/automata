#[allow(dead_code)]
use serde::{Deserialize, Serialize};
// use crate::error::*;
use super::channel::Channel;
use settings::ron::Config;
use failure::Fallible;
use walkdir::{WalkDir,DirEntry};
// use log::info;
// use analyzer::flow::*;
// use analyzer::*;
use crate::local::rootdir;
use std::{
    fs,
    path::PathBuf,
};
use crate::Route;

/// Stream
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Stream {
    pub name:        String,
    pub updated:     u64,
    pub description: String,
}


impl Default for Stream {
    fn default() -> Self {
        Self {
            name:        "#1".to_owned(),
            updated:     0,
            description: "none".to_owned(),
        }
    }
}

impl Stream {
    /// new
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
fn directory() -> Fallible<PathBuf> {
    let path = super::local::rootdir()?;
    let path = path.join("stream");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

fn entry_is_stream(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with("STREAM")
}

pub async fn search_all(path: &Path) ->Fallible<Vec<Stream>> {

    
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
pub fn get_path(id:u64) -> Fallible<PathBuf> {
    let path = directory()?.join(format!("{}/",id));
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

/// Load from stream/:id/stream.ron
pub async fn read(path : &str) -> Fallible<Stream> {
    let dir = directory()?.join(format!("{}/",id)).join("config");
    let stream = Stream::load_no_fallback(dir)?;
    Ok(stream)
}

pub async fn save(route:Route,stream:Stream) -> Fallible<()> {
    let path = get_path(stream.id)?.join("config");
    stream.write(path)?;
    Ok(())
}
fn entry_is_channel(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with("channel.ron")
}

// pub async fn read_channels(stream:Stream) -> Result<Vec<Channel>> {
//     let path = get_path(id)?;
//     let mut channels:Vec<Channel> = Vec::new();
//     for entry in WalkDir::new(path).into_iter()
//         .filter_entry(|e| entry_is_channel(e))
//         .filter_map(|e| e.ok()) {
//         let file_type = entry.file_type();
//         if file_type.is_file(){
//             let cfg = entry.path();
//             channels.push(Channel::load_no_fallback(cfg)?);
//         }
//     }
//     Ok(channels)
// }

pub async fn default_path() -> String {
   "/{stream}/config".to_owned()
}



// pub async get1() -> Result<Stream> {

// }

//
