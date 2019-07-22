use log::info;
// use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
// use std::sync::{Mutex,RwLock};
use super::emoji;
use toml;
use yansi::Paint;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
// use std::os::;


// use confy;

// #[derive(Debug, Serialize, Deserialize)]
// struct Config {
    // name: String,
    // comfy: bool,
    // foo: i64,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            address: "localhost".to_string(),
            port: 8000,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    ExternalPr,
    Commands,
}

#[derive(Debug, Deserialize)]
pub struct GitConfig {
    pub webhook_secret: String,
    pub username: String,
    pub ssh_key: String,
    pub api_token: String,
    pub hostname: Option<String>,
}

// impl Default for GitConfig {
//     fn default() -> GitConfig {
//         GitConfig {
//             webhook_secret:
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub git: GitConfig,
    // pub mappings: Vec<Mapping>,
    // pub features: Vec<Feature>,
}

// impl Default for Config {
//     fn default() -> Self {
//         Self{
//             server: ServerConfig::default(),
//         }
//     }
// }

// pub fn load_config () {
// }
//     pub fn feature_enabled(feature: &Feature) -> bool {
//     CONFIG.features.contains(&feature)
// }


// #[derive(Debug, Deserialize)]
// pub struct Mapping {
//     pub github_repo: String,
//     pub gitlab_repo: String,
// }

// lazy_static! {
//     pub static ref HUB_TO_LAB: Mutex<HashMap<String, String>> = {
//         let m: HashMap<String, String> = HashMap::new();
//         Mutex::new(m)
//     };
// }

// lazy_static! {
//     pub static ref LAB_TO_HUB: Mutex<HashMap<String, String>> = {
//         let m: HashMap<String, String> = HashMap::new();
//         Mutex::new(m)
//     };
// }

fn get_toml_path() -> String {
    env::var("WQA_TOML").unwrap_or_else(|_| "wqa.toml".to_string())
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let toml_path = get_toml_path();
        let config: Config = toml::from_str(&read_file_to_string(&toml_path)).unwrap();
        config
    };
}

fn read_file_to_string(filename: &str) -> String {
    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    contents
}

pub fn load_config() {
    info!(
        "{} loaded configuration values from {}",
        emoji::WRENCH,
        get_toml_path()
    );
    info!("CONFIG => {:#?}", Paint::red(&*CONFIG));

    // for mapping in CONFIG.mappings.iter() {
//         let mut hub_to_lab_lock = HUB_TO_LAB.lock();
//         let hub_to_lab = hub_to_lab_lock.as_mut().unwrap();
//         hub_to_lab.insert(mapping.github_repo.clone(), mapping.gitlab_repo.clone());

//         let mut lab_to_hub_lock = LAB_TO_HUB.lock();
//         let lab_to_hub = lab_to_hub_lock.as_mut().unwrap();
//         lab_to_hub.insert(mapping.gitlab_repo.clone(), mapping.github_repo.clone());
//     }
//     info!(
//         "HUB_TO_LAB => {:#?}",
//         Paint::red(HUB_TO_LAB.lock().unwrap())
//     );
//     info!(
//         "LAB_TO_HUB => {:#?}",
//         Paint::red(LAB_TO_HUB.lock().unwrap())
//     );
}
