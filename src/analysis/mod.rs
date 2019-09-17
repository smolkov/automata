#[allow(dead_code)]
use serde::{Deserialize, Serialize};
pub mod channel;
pub mod adjustment;
pub mod indicator;
pub mod solution;
pub mod calibration;
// pub mod statistic;
// use {
    // futures::{
        // channel::mpsc,
    // },
// };
use chrono::Utc;
use std::path::{PathBuf,Path};
use async_std::fs;
use async_std::io;
// use async_std::prelude::*;
// use async_std::task;
// use async_std::stream;
use async_std::stream::{Stream};

pub use crate::error::Result;
pub use channel::{Channel};

//
// use async_std::fs;
// use async_std::io;
// use async_std::prelude::*;
// use async_std::task;
// use super::rootdir;

// use async_std::fs;
// use async_std::fs::;
// use async_std::io;
// use async_std::os::unix::fs::symlink;
// use async_std::os::unix::net::UnixDatagram;
// use async_std::prelude::*;
// use async_std::task;
// use crate::error::*;

// use walkdir::{WalkDir,DirEntry};
// use log::info;
// use analyzer::flow::*;
// use analyzer::*;



pub type License = String;

/// Method
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Method{
    pub updated:     u64,
    pub label:       String,
    pub description: String,
    pub path :       PathBuf,
}

/// workdir
pub async fn workdir() -> PathBuf {
    super::workdir().join("methods")
}

pub async fn method(path: &Path) -> Result<Method> {
    let path = path.to_path_buf();
    let label = path.join("label");
    let description = path.join("description.md");
    let label = fs::read_to_string(&label);
    let desctription= fs::read_to_string(&description);
    Ok(Method{
        updated: Utc::now().timestamp_millis() as u64,
        label: label.await?,
        description: desctription.await?,
        path: path,
    })
}


pub async fn channels(_method: &Method) ->  Result<Vec<Channel>> {
    let channels: Vec<Channel> = Vec::new();
    Ok(channels)
}


pub async fn methods(path:&Path) -> Result<Vec<Method>>{
    let methods: Vec<Method> = Vec::new();
    let mut dir = fs::read_dir(&path).await?;
    while let Some(entry) = dir.next().await {
        println!("{}", entry?.file_name().to_string_lossy());
    }
//     let mut dir = fs::read_dir(path).await?;
//     let methods:Vec<Method> = Vec::new();

//     while let Some(entry) = dir.next().await {

//         println!("{}", entry?.file_name().to_string_lossy());
//     }
    Ok(methods)
}


// Load from stream/:id/stream.ron
// pub async fn stream(path : &str) -> Fallible<Stream> {
    // let dir = directory()?.join(format!("{}/",id)).join("config");
    // let stream = Stream::load_no_fallback(dir)?;
    // Ok(stream)
// }

// fn entry_is_stream(entry: &DirEntry) -> bool {
    // entry.file_name().to_string_lossy().ends_with("stream")
// }

// pub async fn search_all() ->io::Result<()> {
//     let path = rootdir();
//     for entry in WalkDir::new(path).into_iter()
//         .filter_entry(|e| entry_is_stream(e))
//         .filter_map(|e| e.ok()) {
//         let file_type = entry.file_type();
//         if file_type.is_file(){
//             let cfg = entry.path();
//             streams.push(Stream::load_no_fallback(cfg)?);
//         }
//     }
//     Ok(streams)
   // for entry in fs::read_dir(path).await? {
        // println!("{}", entry?.file_name().to_string_lossy());
    // }
    // Ok(())
    // task::block_on(async {
        // let mut dir = fs::read_dir(&path).await?;

        // while let Some(entry) = dir.next().await {
        // }

    // })

        // let entry = entry?;
        // let file_type = entry.file_type();
        // println!("{}", entry?.file_name().to_string_lossy());
    // }
// }



//
// pub fn get_path(id:u64) -> Fallible<PathBuf> {
    // let path = directory()?.join(format!("{}/",id));
    // if !path.exists() {
        // fs::create_dir_all(&path)?;
    // }
    // Ok(path)
// }
// pub async fn save(route:Route,stream:Stream) -> Fallible<()> {
    // let path = get_path(stream.id)?.join("config");
    // stream.write(path)?;
    // Ok(())
// }
// fn entry_is_channel(entry: &DirEntry) -> bool {
    // entry.file_name().to_string_lossy().ends_with("channel.ron")
// }
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
// pub async fn default_path() -> String {
//    "/{stream}/config".to_owned()
// }
// pub async get1() -> Result<Stream> {
// }
//
