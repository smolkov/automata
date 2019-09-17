// use metrics_core::{Builder, Drain, Observe};
// use metrics_runtime::{observers::PrometheusBuilder, Controller, Receiver};
// use metrics_runtime::{Controller};


// #[macro_use]
// extern crate rust_embed;
pub mod node;
pub mod templates;
// pub mod local;
pub mod error;
pub mod config;
pub mod analysis;
pub mod emoji;
// pub mod store;
pub mod state;
pub mod machine;
// pub mod api;
pub mod metric;
pub mod mio;
// pub mod sensor;
pub mod server;

// pub mod collectors;
// pub use data::*;
// pub use asset::Asset;
// pub use config::GLobalConfig;
// pub use store::Store;
pub use state::State;
use node::Node;
// pub use mio::Mio;
// pub use templates::Template;

// pub use machine::Machine;
use std::path::{PathBuf,Path};
use std::env;
// use std::ffi::OsStr;
// use path_table::PathTable;
// const README_MD: &'static str = include_str!("../README.md");



use metrics_runtime::{Controller, Receiver};

use async_std::fs;
use async_std::os::unix::fs::symlink;
// use async_std::io;
// use async_std::prelude::*;
// use async_std::task;
use error::Result;

use lazy_static::lazy_static;


lazy_static! {
    pub static ref WORKDIR: PathBuf = PathBuf::from("/var/run/automata");
    pub static ref SYSGPIO: PathBuf = PathBuf::from("/sys/class/gpio");
    // {
    //     let path:PathBuf = env::var_os("AUTOMATA_PATH").ok_or_else(err: F)
    //         .unwrap_or_else(|| OsStr::new("/var/run/automata").to_os_string())
    //         .unwrap_or_else(|| )
    //         .unwrap_or_else(|| )
    //         .into();
    //     // if !path.exists() {
    //         // info!("init new store:{}",path.as_path().to_string_lossy());
    //         // fs::create_dir_all(path.as_path()).unwrap();
    //     // }
    //     path
    // };
    pub static ref RUNDIR: PathBuf = {
       PathBuf::from("/var/run/automata")
    };
    // pub static ref MIO:TempDir =  {
        // Builder::new().prefix("automataio").tempdir()
    // };

    pub static ref LOCAL_DIR: PathBuf = WORKDIR.join("local");
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
    pub static ref CRATES_DIR: PathBuf = WORKDIR.join("shared/crates");
    pub static ref EXPERIMENT_DIR: PathBuf = WORKDIR.join("ex");
    pub static ref LOG_DIR: PathBuf = WORKDIR.join("logs");
    pub static ref LOCAL_CRATES_DIR: PathBuf = "local-crates".into();
    pub static ref SOURCE_CACHE_DIR: PathBuf = WORKDIR.join("cache").join("sources");
}


pub fn rootdir() -> PathBuf {
    WORKDIR.clone()
}



pub fn workdir() -> PathBuf {
    rootdir()
}

pub fn rundir() -> PathBuf {
    rootdir()
}

pub fn cfgdir() -> PathBuf {
   if let Some(home) = env::var_os("HOME") {
        let path = PathBuf::from(home).join("/.automata/");
        path
   }else {
        let path = PathBuf::from("/.automata/");
        path
   }
}

pub async fn setup() -> Result<()>{
    let workdir = workdir();
    let cfgdir = cfgdir();
    if ! cfgdir.exists() {
        fs::DirBuilder::new().recursive(true).create(cfgdir.as_path()).await?;
    }
    if !workdir.exists() {
        fs::DirBuilder::new().recursive(true).create(workdir.as_path()).await?;
        symlink(cfgdir.as_path(),workdir.join("store").as_path()).await?;
    }
    Ok(())
}

/// Various forms of authentication credentials supported by Github
#[derive(Debug, PartialEq, Clone)]
pub enum Credentials {
    /// Oauth token string
    /// https://developer.github.com/v3/#oauth2-token-sent-in-a-header
    Token(String),
    /// Oauth client id and secret
    /// https://developer.github.com/v3/#oauth2-keysecret
    Client(String, String),
}


// Entry point interface for interacting with Github API
pub struct Automata {
    pub path: PathBuf,
    pub template: PathBuf,
    pub store: PathBuf,
    pub controller: Controller,
    // pub store:
}


pub async fn automata(path: &Path) -> Result<Automata> {
    let path = path.to_path_buf();
    let template = path.join("/template");
    let store = path.join("/store");
    let receiver = Receiver::builder().build()?;
    Ok(Automata{
            path: path,
            template: template,
            store: store,
            controller: receiver.get_controller(),
        })
}

impl Automata {


    pub fn rootdir(&self) -> &Path{
        self.path.as_path()
    }
    pub fn rundir(&self) -> PathBuf{
        self.path.join("/run/")
    }

//     pub fn config(&self) -> GLobalConfig{
//         GLobalConfig::new(self.clone())
//     }
    // pub fn client(&self) -> Option<Client>{
        // self.address = address;
    // }
    // pub fn store(&self) -> Store  {
        // Store::new(self.clone())
    // }
    // pub fn control() -> Control {

    // }
    // pub fn mio(&self) -> Mio {
        // Mio::new(self.clone())
    // }
    // pub fn state(&self) -> State{
        // State::new(self.clone())
    // }

    // pub fn template(&self) -> Template {
        // Template::new(self.clone())
    // }

}


pub async fn link_store(store_path :&Path) -> Result<Node> {
    let path = workdir();
    if !path.exists() {
        fs::DirBuilder::new().recursive(true).create(path.as_path()).await?;
    }
    symlink(store_path,path.join("/store").as_path()).await?;
    Ok(Node::new(path.as_path()))
}


