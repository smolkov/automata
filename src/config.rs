#![allow(dead_code, unused_imports)]
use log::info;
use crate::error::*;
use failure::Fallible;
use serde_json::from_slice;
// use super::automata;
// use std::collections::HashMap;
use std::env;
use std::{
    fs::{self,File },
    path::Path,
};

use std::io::prelude::*;
// use std::sync::{Mutex,RwLock};
use super::emoji;
use toml;
use yansi::Paint;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
// use std::os::;
// use std::path::{Path, PathBuf};

const MANIFEST_TOML: &'static str = include_str!("assets/manifest.toml");

#[derive(Debug, Serialize, Deserialize, Clone)]
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

// impl GitConfig {

// }

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
    env::var("automataTOML").unwrap_or_else(|_| "larautomata.toml".to_string())
}

fn read_file_to_string(filename: &str) -> Fallible<String> {
    let path = Path::new(filename);
    if path.exists() {
        let content = fs::read_to_string(path)?;
        return Ok(content);
    }
    Ok(MANIFEST_TOML.to_string())
}

// fn load_or_default() -> Config{
    // let path:PathBuf = get_toml_path().into();
    // if path.exists() {
        // config
    // }
// }

// lazy_static! {
//     pub static ref CONFIG: Config = {
//         let toml_path = get_toml_path();

//         let config: Config = toml::from_str(&read_file_to_string(&toml_path).unwrap()).unwrap();
//         config
//     };
// }

use once_cell::sync::OnceCell;


// pub fn server_confit() -> ServerConfig {
    // CONFIG.server.clone()
// }



// pub struct GLobalConfig {
    // automata: automata,
// }


// impl GLobalConfig {
    // pub fn new(automata: automata) -> GLobalConfig {
        // GLobalConfig{
            // automata:automata
        // }
    // }

// }

fn global() -> &'static Config {
        static CONFIG: OnceCell<Config> = OnceCell::new();
        CONFIG.get_or_init(|| {
            let toml_path = get_toml_path();
            let config: Config = toml::from_str(&read_file_to_string(&toml_path).unwrap()).unwrap();
            config
        })
}
pub fn setup() {
    info!(
            "{} loaded configuration values from {}",
            emoji::WRENCH,
                get_toml_path()
            );
        info!("CONFIG => {:#?}", Paint::red(&*global()));
    }
pub fn server() -> ServerConfig {
    global().server.clone()
}
