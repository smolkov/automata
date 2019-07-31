use crate::error::*;
// use failure::{format_err};
// use regex::Regex;
use dirs;
use std::fs;
use walkdir::{WalkDir};

use analyzer::Device;
// use analyzer::*;
use wqa_settings::ron::Config;
use std::{
    path::PathBuf,
    fs::{create_dir_all},
};

use super::rules::*;
use super::stream::*;

use std::collections::HashMap;


lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
    // static ref Func:
    static ref COUNT: usize = HASHMAP.len();
    // static ref NUMBER: u32 = times_two(21);
    static ref DATADIR: Option<PathBuf> = None;
}

/// data dir
pub fn data_dir() -> Result<PathBuf> {
    let path = DATADIR.map_or(dirs::home_dir().or(Some(PathBuf::from("./"))).unwrap(),|d| d);
    let path = path.join(".wqa");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
pub fn stream_dir() -> Result<PathBuf> {
    let path = data_dir()?;
    let path = path.join("stream");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
/// history path
pub fn history_path() -> Result<PathBuf> {
    let path = data_dir()?;
    let path = path.join("history");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
/// measurements dir
pub fn measurements_dir() -> Result<PathBuf> {
    let path = history_path()?;
    let path = path.join("measurements");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
pub fn signal_dir() -> Result<PathBuf> {
    let path = history_path()?;
    let path = path.join("signal");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
pub fn module_dir() -> Result<PathBuf> {
    let path = data_dir()?;
    let path = path.join("modules");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}


pub fn rules_dir() -> Result<PathBuf> {
    let path = data_dir()?.join("rules");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}


#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
    root: PathBuf,

    // list:
}

impl Workspace {
    // pub fn setup() -> Result<()> {

    // }
}

pub async fn setup() -> Result<()> {
    let path = stream_dir()?.join("1");
    if !path.exists() {
        fs::create_dir_all(&path)?;
        let st1 = Stream::default();
        let ch1 = Channel::default();
        stream_save(&st1).await?;
        stream_channel_save(&st1,&ch1).await?
    }
    Ok(())
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

pub async fn stream_get_list() ->Result<Vec<Stream>> {
    let path = stream_dir()?;
    let mut streams: Vec<Stream> = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let file_type = entry.file_type();
        if file_type.is_dir(){
            let cfg = entry.path().join("stream.ron");
            if cfg.exists()  {
                streams.push(Stream::load_no_fallback(cfg)?);
            }
        }
    }
    Ok(streams)
}
pub fn stream_get_path(stream:&Stream) -> Result<PathBuf> {
    let path = stream_dir()?.join(format!("{}/",stream.id));
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
pub async fn stream_get_from_id(id: u64) -> Result<Stream> {
    let dir = stream_dir()?.join(format!("{}/",id)).join("stream.ron");
    let stream = Stream::load_no_fallback(dir)?;
    Ok(stream)
}

pub async fn stream_save(stream:&Stream) -> Result<()> {
    let path = stream_get_path(stream)?.join("stream.ron");
    stream.write(path)?;
    Ok(())
}

pub async fn stream_channel_list(stream:Stream) -> Result<Vec<Channel>> {
    let path = stream_get_path(&stream)?;
    let mut channels:Vec<Channel> = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let metadate = entry.metadata()?;
        if metadate.is_dir(){

            channels.push(Channel::load_no_fallback(entry.into_path())?);
        }
    }
    Ok(channels)
}

pub async fn stream_channel_save(stream:&Stream,channel:&Channel) -> Result<()> {
    let path = stream_get_path(stream)?.join(format!("{}/",channel.id));
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    channel.write(path.join("channel.ron"))?;
    Ok(())
}

// pub async fn stream_channel(stream:&Stream,channel:&Channel),

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



// pub async fn airflow_input_set(airflow:Airflow) {

// }

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
        stream_save(&stream).await.unwrap();
        let rule = Rule::new(1);
        rule_save(rule).await.unwrap();
        // let x = Workspace::from_str("abc");
        // assert!(x.is_ok());
    }

}
