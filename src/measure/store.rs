use crate::error::*;
// use failure::{format_err};
// use regex::Regex;
use dirs;
use std::fs;
use walkdir::{WalkDir};

use analyzer::Device;
// use analyzer::flow::*;
// use analyzer::*;
use settings::ron::Config;
use std::{
    path::PathBuf,
    fs::{create_dir_all},
};

use super::rule::*;
use super::stream::*;
use super::channel::*;
use super::measurement::*;
// use super::calibration::*;

use std::collections::HashMap;
use analyzer::flow::MonitoringSetting;

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
    static ref ROOTDIR: &'static str = ".";
}

// change project root directory
pub fn change_root_dir(root: PathBuf) -> Result<()> {
    // let path = PathBuf::f
    if !root.exists() {
        fs::create_dir_all(&root)?;
    }
    // ROOTDIR = root;
    Ok(())
}

/// data dir
pub fn data_dir() -> Result<PathBuf> {
    let path = PathBuf::from(".wqa");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
///
pub fn stream_dir() -> Result<PathBuf> {
    let path = data_dir()?;
    let path = path.join("stream");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

/// signal dir
pub fn signal_dir() -> Result<PathBuf> {
    let path = data_dir()?;
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
    let path = data_dir()?.join("rule");
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
    // pub fn setup(path: &str) -> Result<Workspace> {

    // }
}

pub async fn setup(path: &str) -> Result<()> {
    let path = PathBuf::from(path);
    change_root_dir(path.join(".wqa"))?;
    let file = path.join("device.ron");
    if !file.exists() {
        Device::default().write(file)?;
    }
    let file = path.join("monitoring.ron");
    if !file.exists() {
        // FlowSetting::default().write(file)?;
    }

    // let path = stream_dir()?.join("1");
    // if !path.exists() {

        // fs::create_dir_all(&path)?;
        // let st1 = Stream::default();
        // let ch1 = Channel::default();
        // stream_save(&st1).await?;
        // stream_channel_save(&st1,&ch1).await?
   // }
    Ok(())
}


pub async fn get_device() -> Result<Device> {
    let device = Device::load_no_fallback(data_dir()?.join("device.ron"))?;
    Ok(device)
}

pub async fn set_device(device: Device) -> Result<()> {
    device.write(data_dir()?.join("device.ron"))?;
    Ok(())
}

pub async fn get_flow_monitoring_settings() -> Result<MonitoringSetting> {
    let settings = MonitoringSetting::load_no_fallback(data_dir()?.join("flowmonitoring.ron"))?;
    Ok(settings)
}

pub async fn set_flow_monitoring_settings(settings:MonitoringSetting) -> Result<()> {
    settings.write(data_dir()?.join("flowmonitoring.ron"))?;
    Ok(())
}

// pub async fn set_serial( serial: String ) -> Result<()> {
    // let mut device = get_().await?;
    // device.set_serial(serial);
    // device.write(data_dir()?.join("device.ron"))?;
    // Ok(())
// }

pub async fn get_streams() ->Result<Vec<Stream>> {
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
fn get_stream_path(stream:&Stream) -> Result<PathBuf> {
    let path = stream_dir()?.join(format!("{}/",stream.id));
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
fn get_channel_path(channel:&Channel) -> Result<PathBuf> {
    let path = stream_dir()?.join(format!("{}/{}",channel.channel_number(),channel.stream_number()));
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}
/// Search in stream/:id/stream.ron
pub async fn get_stream_from_id(id: u64) -> Result<Stream> {
    let dir = stream_dir()?.join(format!("{}/",id)).join("stream.ron");
    let stream = Stream::load_no_fallback(dir)?;
    Ok(stream)
}

pub async fn set_stream(stream:&Stream) -> Result<()> {
    let path = get_stream_path(stream)?.join("stream.ron");
    stream.write(path)?;
    Ok(())
}

pub async fn get_stream_channels(stream:u64) -> Result<Vec<Channel>> {
    let path = stream_dir()?.join(format!("{}/",stream));
    let mut channels:Vec<Channel> = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let file_type = entry.file_type();
        if file_type.is_dir(){
            let cfg = entry.path().join("channel.ron");
            if cfg.exists()  {
                channels.push(Channel::load_no_fallback(cfg)?);
            }
        }
    }
    Ok(channels)
}

// pub async fn get_stream_channel(stream:u64,channel:u64) ->
pub async fn set_channel(channel:&Channel) -> Result<()> {
    let path = get_channel_path(channel)?;
    channel.write(path.join("channel.ron"))?;
    Ok(())
}

pub async fn measurement_evaluation_save() {}

// pub async fn stream_channel(stream:&Stream,channel:&Channel),

pub async fn get_rules() ->Result<Vec<Rule>> {
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

pub async fn set_rule(rule: Rule) -> Result<()> {
    let path = rules_dir()?.join(format!("{}/",rule.id));
    if !path.exists() {
        create_dir_all(path.as_path())?;
    }
    rule.write(path.join("rule.ron"))?;
    Ok(())
}

pub async fn get_rule_from_id(id:u64) -> Result<Rule> {
    let rule = Rule::load_no_fallback(rules_dir()?.join(format!("{}/",id)).join("rule.ron"))?;
    Ok(rule)
}

pub async fn get_measure_stats() -> Result<MeasureStats> {
    let path = data_dir()?.join("mstats.ron");
    let mut mstat = MeasureStats::load_no_fallback(path)?;
    Ok(mstat)
}


pub async fn next_measurement() -> Result<MeasureStats> {
    let path = data_dir()?.join("mstat.ron");
    let mut mstat = MeasureStats::load_no_fallback(path.clone())?;
    mstat.counter +=1;
    mstat.write(path)?;
    Ok(mstat)
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
        set_stream(&stream).await.unwrap();
        let rule = Rule::new(1);
        set_rule(rule).await.unwrap();
        // let x = Workspace::from_str("abc");
        // assert!(x.is_ok());
    }

}
