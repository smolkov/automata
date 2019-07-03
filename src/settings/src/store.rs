use log::{debug, error, info, warn};
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use amethyst::{
    utils::application_root_dir,
};

use serde_derive::{Deserialize, Serialize};
use ron;
use lazy_static::lazy_static;
use std::sync::RwLock;

use directories::{
    BaseDirs,
    UserDirs,
    ProjectDirs,
};


lazy_static! {
    pub static ref LOCAL: RwLock<Store> = RwLock::new(Store::new());
}

/// Storage
#[derive(Debug)]
pub struct Store {
   base_dirs: BaseDirs,
   project_dirs: ProjectDirs,
   user_dirs: UserDirs,
}

const TLD: &str = "com";
const SLD: &str = "larag";
const METHOD: &str = "methods";
const CHAN: &str = "channels";
const SENSOR: &str = "sensors";
const MEAS: &str = "measurements";
const CAL: &str = "calibrations";
const LOGS: &str = "logs"; 

impl Store {
    //TODO: return Result 
    pub fn new() -> Store {
        let base_dirs =BaseDirs::new().expect("Couldn't find base dirs for this platform");
        let user_dirs = UserDirs::new().expect("Couldn't find base dirs for this platform");
        let project_dirs = ProjectDirs::from(TLD, SLD, env!("CARGO_PKG_NAME")) .expect("Couldn't find project dirs for this platform");
        let config_dir = project_dirs.config_dir();
        create_dir_all(&config_dir).unwrap();
        let methods = config_dir.join(METHOD);
        create_dir_all(&methods).unwrap();
        let channels = config_dir.join(CHAN);
        create_dir_all(&channels).unwrap();
        let measurements = config_dir.join(MEAS);
        create_dir_all(&measurements).unwrap();
        let calibrations= config_dir.join(CAL);
        create_dir_all(&calibrations).unwrap();
        let sensors= config_dir.join(SENSOR);
        create_dir_all(&sensors).unwrap();
        let logs = config_dir.join(LOGS);
        create_dir_all(&logs).unwrap();
         Store {
            base_dirs,
            user_dirs,
            project_dirs,
        }
    }
}


pub fn methods_dir() ->  PathBuf {
    let local = LOCAL.read().unwrap();
    let config_dir = local.project_dirs.config_dir();;
    config_dir.join(METHOD)
} 

pub fn channels_dir() ->  PathBuf {
    let local = LOCAL.read().unwrap();
    let config_dir = local.project_dirs.config_dir();;
    config_dir.join(CHAN)
} 
pub fn sensors_dir() ->  PathBuf {
    let local = LOCAL.read().unwrap();
    let config_dir = local.project_dirs.config_dir();;
    config_dir.join(SENSOR)
} 
pub fn measurements_dir() ->  PathBuf {
    let local = LOCAL.read().unwrap();
    let config_dir = local.project_dirs.config_dir();;
    config_dir.join(METHOD)
} 
pub fn calibrations_dir() ->  PathBuf {
    let local = LOCAL.read().unwrap();
    let config_dir = local.project_dirs.config_dir();;
    config_dir.join(METHOD)
} 
pub fn logs_dir() ->  PathBuf {
    let local = LOCAL.read().unwrap();
    let config_dir = local.project_dirs.config_dir();;
    config_dir.join(METHOD)
} 