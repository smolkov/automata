use crate::error::*;
use failure::{format_err};
use regex::Regex;
use dirs;
use std::fs;
use walkdir::{WalkDir};

use analyzer::Device;
use wqa_settings::ron::Config;
use std::{
    path::PathBuf,
    fs::{create_dir_all},
};

use super::rules::*;
use super::stream::*;

pub fn data_dir() -> Result<PathBuf> {
    let path = dirs::data_dir().ok_or_else(|| format_err!("Failed to find data directory"))?;
    let path = path.join("wqa");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create data directory")?;
    }
    Ok(path)
}

pub fn history_path() -> Result<PathBuf> {
    let path = data_dir()?;
    let path = path.join("history");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create history directory")?;
    }
    Ok(path)
}
pub fn measurements_dir() -> Result<PathBuf> {
    let path = history_path()?;
    let path = path.join("measurements");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create miasurements directory")?;
    }
    Ok(path)
}
pub fn signal_dir() -> Result<PathBuf> {
    let path = history_path()?;
    let path = path.join("signal");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create miasurements directory")?;
    }
    Ok(path)
}
pub fn module_dir() -> Result<PathBuf> {
    let path = data_dir()?;
    let path = path.join("modules");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create module directory")?;
    }
    Ok(path)
}

pub fn streams_dir() -> Result<PathBuf> {
    let path = data_dir()?.join("streams");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create streams directory")?;
    }
    Ok(path)
}

pub fn rules_dir() -> Result<PathBuf> {
    let path = data_dir()?.join("rules");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create rules directory")?;
    }
    Ok(path)
}


#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
    root: PathBuf,
}



pub async fn device_get() -> Result<Device> {
    let device = Device::load_no_fallback(data_dir()?.join("device.ron"))?;
    Ok(device)
}

pub async fn device_save(device: Device) -> Result<()> {
    device.write(data_dir()?.join("device.ron"))?;
    Ok(())
}

// pub async fn set_serial( serial: String ) -> Result<()> {
    // let mut device = get_().await?;
    // device.set_serial(serial);
    // device.write(data_dir()?.join("device.ron"))?;
    // Ok(())
// }

pub async fn get_stream_list() ->Result<Vec<Stream>> {
    let path = streams_dir()?;
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
pub fn stream_get_path(stream:&Stream) -> Result<PathBuf> {
    let path = streams_dir()?.join(format!("{}/",stream.id));
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create stream directory")?;
    }
    Ok(path)
}
pub async fn stream_get_id(id: u64) -> Result<Stream> {
    let dir = streams_dir()?.join(format!("{}/",id)).join("stream.ron");
    let stream = Stream::load_no_fallback(dir)?;
    Ok(stream)
}

pub async fn stream_save(stream:Stream) -> Result<()> {
    let path = stream_get_path(&stream)?.join("stream.ron");
    stream.write(path)?;
    Ok(())
}

pub async fn get_stream_channel_list(stream:Stream) -> Result<Vec<Channel>> {
    let path = stream_get_path(&stream)?.join("channels");
    let mut channels:Vec<Channel> = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let metadate = entry.metadata()?;
        if metadate.is_file(){
            channels.push(Channel::load_no_fallback(entry.into_path())?);
        }
    }
    Ok(channels)
}


pub async fn rule_get_list() ->Result<Vec<Rule>> {
    let path = rules_dir()?;
    let mut rules = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let metadate = entry.metadata()?;
        if metadate.is_dir(){
            rules.push(Rule::load_no_fallback(entry.into_path().join("rule.ron"))?);
        }
    }
    Ok(rules)
}

pub async fn rule_save(rule: Rule) -> Result<()> {
    let path = rules_dir()?.join(format!("{}/",rule.id));
    if !path.exists() {
        create_dir_all(path.as_path())?;
    }
    rule.write(path.join("rule.ron"))?;
    Ok(())
}

pub async fn rule_get_id(id:u64) -> Result<Rule> {
    let rule = Rule::load_no_fallback(rules_dir()?.join(format!("{}/",id)).join("rule.ron"))?;
    Ok(rule)
}

// pub async fn

#[cfg(test)]
mod tests {
    use super::*;
    // use futures::executor::block_on;


    #[runtime::test]
    async fn test_valid_workspace() {
        let path = data_dir().unwrap();
        println!("PATH:{:?}",path);
        let stream = Stream::default();
        stream_save(stream).await.unwrap();
        let rule = Rule::new(1);
        rule_save(rule).await.unwrap();
        

        // let x = Workspace::from_str("abc");
        // assert!(x.is_ok());
    }

}
