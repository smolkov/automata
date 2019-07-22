use serde_derive::{Deserialize, Serialize};
use wqa_settings::ron::Config;
// use std::time::{Duration};
// use std::fmt;
use walkdir::{WalkDir,DirEntry};
// use serde_yaml;
use super::{
    store,
};
use std::{
    fs::create_dir_all,
    // io::{Read, Write},
    path::{PathBuf},
};

use crate::error::*;

/// Stream
#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Stream {
    pub number: u64,
    pub name  : String,
}


impl Default for Stream {
    fn default()-> Self {
        Self {
            number:1,
            name: "stream1".to_owned(),
        }
    }
}

impl Stream {
    pub fn ron_file_path(&self) -> PathBuf {
        store::root_directory().join(format!("stream{}/",self.number)).join("stream.ron")
    }
    /// local configuration directory
    pub fn directory(&self) -> Result<PathBuf,WqaError> {
        let dir = store::root_directory().join(format!("stream{}",self.number));
        create_dir_all(&dir)?;
        Ok(dir)
    }
}


pub fn local_directory(number:u64) -> Result<PathBuf,WqaError> {
    let dir = store::root_directory().join(format!("stream{}/",number));
    create_dir_all(&dir)?;
    Ok(dir)
}


pub async fn find_all() ->Result<Vec<Stream>,WqaError> {
    let path = store::streams_directory();
    let mut streams: Vec<Stream> = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let metadate = entry.metadata()?;
        if metadate.is_dir(){
            match entry.file_name().to_str() {
                Some(sp) => {
                    let part: Vec<_> = sp.matches("stream").collect();
                    if part.len() > 0 {
                        streams.push(Stream::load_no_fallback(entry.into_path().join("stream.ron"))?);
                    }
                },
                None => { }
            }
        }
    }
    Ok(streams)
}

