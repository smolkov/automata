//! Default structur
//! .
//! ├── /wqa
//! │   ├── stats.ron
//! │   ├── wqa.toml
//! │   ├──stream
//! │   │   └── 1
//! │   │   │   └── stream.ron
//! │   │       ├── 1
//! │   │       │   ├── channel.ron
//! │   ├── examples
//! │   │   ├── kaggle
//! │   │   │   └── schema-kaggle.toml
//! │   │   └── movies
//! │   │       ├── movies.csv
//! │   │       ├── README.md
//! │   │       └── schema-movies.toml
//! │   ├── LICENSE
//! │   ├── meilidb
//! │   │   ├── Cargo.toml
//! │   │   ├── examples
//! │   │   │   ├── create-database.rs
//! │   │   │   └── query-database.rs
//! │   │   └── src
//! │   │       ├── lib.rs
//! │   │       └── sort_by_attr.rs
//! │   ├── meilidb-core
//! │   │   ├── Cargo.toml
//! │   │   └── src
//! │   │       ├── automaton.rs
//! │   │       ├── criterion
//! │   │       │   ├── document_id.rs
//! │   │       │   ├── exact.rs
//! │   │       │   ├── mod.rs
//! │   │       │   ├── number_of_words.rs
//! │   │       │   ├── sum_of_typos.rs
//! │   │       │   ├── sum_of_words_attribute.rs
//! │   │       │   ├── sum_of_words_position.rs
//! │   │       │   └── words_proximity.rs
//! │   │       ├── distinct_map.rs
//! │   │       ├── lib.rs
//! │   │       ├── query_builder.rs
//! │   │       ├── reordered_attrs.rs
//! │   │       └── store.rs
//! //! .wqa/
//! //!      streams/
//!             1/
//!               stream.ron
//!               1/
//!
//!      history/
//!      logs/
use crate::Result;
// use failure::{format_err};
// use regex::Regex;
// use dirs;
use std::fs;
// use walkdir::{WalkDir};
use log::info;
use analyzer::Device;
// use analyzer::flow::*;
// use analyzer::*;
use settings::ron::Config;
use std::{
    path::PathBuf,
};
pub mod machine_id;
pub mod integration;
pub mod adjustment;
pub mod measurement;
pub mod signal;
pub mod sensor;
pub mod statistic;

pub mod device;
pub mod rule;
pub mod stream;
pub mod channel;

use self::measurement::MeasureStats;
// pub use self::rule;
// pub use self::stream;
// pub use self::channel;
// pub use self::machine_id;
// use super::mio::airflow::AirflowSetting;
use std::env;
use std::ffi::OsStr;
// use super::calibration::*;

// use std::collections::HashMap;
use analyzer::flow::MonitoringSetting;

lazy_static! {
    pub static ref ROOT_DIR: PathBuf = {
        let path:PathBuf = env::var_os("WQA_ROOT")
            .unwrap_or_else(|| OsStr::new(".wqa").to_os_string())
            .into();
        if !path.exists() {
            info!("init new store:{}",path.as_path().to_string_lossy());
            fs::create_dir_all(path.as_path()).unwrap();
        }
        path
    };
    pub static ref LOCAL_DIR: PathBuf = ROOT_DIR.join("local");

    pub static ref CARGO_HOME: String = LOCAL_DIR.join("cargo-home").to_string_lossy().into();
    pub static ref RUSTUP_HOME: String = LOCAL_DIR.join("rustup-home").to_string_lossy().into();

    // Where cargo puts its output, when running outside a docker container,
    // CARGO_TARGET_DIR
    pub static ref TARGET_DIR: PathBuf = LOCAL_DIR.join("target-dirs");

    // The directory crates are unpacked to for running tests, mounted
    // in docker containers
    pub static ref TEST_SOURCE_DIR: PathBuf = LOCAL_DIR.join("test-source");

    // Where GitHub crate mirrors are stored
    pub static ref GH_MIRRORS_DIR: PathBuf = LOCAL_DIR.join("gh-mirrors");

    // Where crates.io sources are stores
    pub static ref CRATES_DIR: PathBuf = ROOT_DIR.join("shared/crates");

    pub static ref EXPERIMENT_DIR: PathBuf = ROOT_DIR.join("ex");
    pub static ref LOG_DIR: PathBuf = ROOT_DIR.join("logs");

    pub static ref LOCAL_CRATES_DIR: PathBuf = "local-crates".into();

    pub static ref SOURCE_CACHE_DIR: PathBuf = ROOT_DIR.join("cache").join("sources");
}

// pub(crate) fn crate_source_dir(ex: &Experiment, tc: &Toolchain, krate: &Crate) -> PathBuf {
    // EXPERIMENT_DIR
        // .join(&ex.name)
        // .join("sources")
        // .join(tc.to_path_component())
        // .join(krate.id())
// }

// lazy_static! {
    // static ref HASHMAP: HashMap<u32, &'static str> = {
    //     let mut m = HashMap::new();
    //     m.insert(0, "foo");
    //     m.insert(1, "bar");
    //     m.insert(2, "baz");
    //     m
    // };
    // static ref WS:Workspace = Workspace::default();
    // static ref Func:
    // static ref COUNT: usize = HASHMAP.len();
    // static ref NUMBER: u32 = times_two(21);
    // static ref ROOTDIR: &'static str = ".";
// }


/// data dir
pub fn data_dir() -> Result<PathBuf> {
    let path = ROOT_DIR.clone();
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

/// sensor dir
pub fn sensor_dir() -> Result<PathBuf> {
    let path = data_dir()?;
    let path = path.join("sensor");
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


#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
    root: PathBuf,

    // list:
}


impl Default for Workspace {
    fn default() -> Self {
        Self{
            root: PathBuf::from("./.wqa"),
        }
    }
}

impl Workspace {
    pub fn setup(&mut self, root: &str) -> Result<()> {
        let path = PathBuf::from(root);
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        self.root = path.clone();
        Ok(())
    }
}

// pub async fn setup_root_directory() -> Result<()> {

    // Ok(())
// }

pub async fn setup(path: &str) -> Result<()> {
    let path = PathBuf::from(path);
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



pub async fn get_flow_monitoring_settings() -> Result<MonitoringSetting> {
    let settings = MonitoringSetting::load_no_fallback(data_dir()?.join("flowmonitoring.ron"))?;
    Ok(settings)
}

pub async fn set_flow_monitoring_settings(settings:MonitoringSetting) -> Result<()> {
    settings.write(data_dir()?.join("airflow.ron"))?;
    Ok(())
}

// pub async fn set_serial( serial: String ) -> Result<()> {
    // let mut device = get_().await?;
    // device.set_serial(serial);
    // device.write(data_dir()?.join("device.ron"))?;
    // Ok(())
// }



pub async fn measurement_evaluation_save() {}

// pub async fn stream_channel(stream:&Stream,channel:&Channel),



pub async fn get_measure_stats() -> Result<MeasureStats> {
    let path = data_dir()?.join("mstats.ron");
    let mstat = MeasureStats::load_no_fallback(path)?;
    Ok(mstat)
}


///next measurement
pub async fn next_measurement() -> Result<MeasureStats> {
    let path = data_dir()?.join("mstat.ron");
    let mut mstat = MeasureStats::load_no_fallback(path.clone())?;
    mstat.counter +=1;
    mstat.write(path)?;
    Ok(mstat)
}


// pub async fn

// pub async fn get_airflow_setting() -> Result<



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
        let first = stream::Stream::default();
        stream::save(&first).await.unwrap();
        let measrule = rule::Rule::new(1);
        rule::save(measrule).await.unwrap();
        // let x = Workspace::from_str("abc");
        // assert!(x.is_ok());
    }

}
