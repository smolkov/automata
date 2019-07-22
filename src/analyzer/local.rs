use log::{debug, error, info, warn};
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use super::WqaError;

use git2::Repository;

use serde_derive::{Deserialize, Serialize};
use ron;
use lazy_static::lazy_static;
use std::sync::RwLock;



// use directories::{
    // BaseDirs,
    // UserDirs,
    // ProjectDirs,
// };


lazy_static! {
    pub static ref STORE: RwLock<Store> = RwLock::new(Store::new(".".into()));
}

/// Storage
#[derive(Debug)]
pub struct Store {
   root: PathBuf,
}

const WQA: &str = ".wqa";

const RULES: &str = "rules";
const STREAMS: &str = "streams";
const CHAN: &str = "channels";
const SENSORS: &str = "sensors";
const RUN: &str= "run";
const MEASUREMENTS: &str = "measurements";
const CALIBRATION: &str = "calibrations";
const LOGS: &str = "logs";

impl Store {
    //TODO: return Result.
    pub fn new(path: PathBuf) -> Store {
        Store{
            root:path.join(WQA),
        }
    }
    pub fn init(&mut self)-> Result<(),WqaError> {
        create_dir_all(&self.root)?;
        // create_dir_all(&self.root.join(STREAMS))?;
        // create_dir_all(&stream_dir.join(MEASUREMENTS))?;
        // create_dir_all(&stream_dir.join(CALIBRATION))?;
        create_dir_all(&self.root.join(CHAN))?;
        create_dir_all(&self.root.join(RULES))?;
        create_dir_all(&self.root.join(SENSORS))?;
        create_dir_all(&self.root.join(LOGS))?;
        Ok(())
    }
    //TODO: init..
    pub fn init_git(&self) -> Result<Repository,git2::Error> {
        Repository::init(self.root.to_path_buf())
    }
}



pub fn streams_directory() -> PathBuf {
    let store = STORE.read().unwrap();
    store.root.clone()
}

pub fn root_directory() -> PathBuf {
    let store = STORE.read().unwrap();
    store.root.clone()
}

pub fn channel_directory() -> PathBuf {
    let store = STORE.read().unwrap();
    store.root.join(CHAN)
}

pub fn rules_directory() -> PathBuf {
    let store = STORE.read().unwrap();
    store.root.join(RULES)
}

// pub fn methods_dir() ->  PathBuf {
//     let local = FSSTORE.read().unwrap();
//     let config_dir = local.project_dirs.config_dir();;
//     config_dir.join(METHOD)
// }

// pub fn channels_dir() ->  PathBuf {
//     let local = FSSTORE.read().unwrap();
//     let config_dir = local.project_dirs.config_dir();;
//     config_dir.join(CHAN)
// }
// pub fn sensors_dir() ->  PathBuf {
//     let local = FSSTORE.read().unwrap();
//     let config_dir = local.project_dirs.config_dir();;
//     config_dir.join(SENSOR)
// }
// pub fn measurements_dir() ->  PathBuf {
//     let local = FSSTORE.read().unwrap();
//     let config_dir = local.project_dirs.config_dir();;
//     config_dir.join(METHOD)
// }
// pub fn calibrations_dir() ->  PathBuf {
//     let local = FSSTORE.read().unwrap();
//     let config_dir = local.project_dirs.config_dir();;
//     config_dir.join(METHOD)
// }
// pub fn logs_dir() ->  PathBuf {
//     let local = FSSTORE.read().unwrap();
//     let config_dir = local.project_dirs.config_dir();;
//     config_dir.join(METHOD)
// }
