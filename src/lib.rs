//! Automata
//!
//! [`log`]: https://automata.io
//!
//!
//!
//! ## What is automata?
//!
//! ## Client description?
//!
//! Panic with a given message unless an expression evaluates to true.
//!
//! # Examples
//!
//! ```
//! # #[macro_use] extern crate foo;
//! # fn main() {
//! panic_unless!(1 + 1 == 2, “Math is broken.”);
//! # }
//! ```
//!
//! ```should_panic
//! # #[macro_use] extern crate foo;
//! # fn main() {
//! panic_unless!(true == false, “I’m broken.”);
//! # }
//! ```

#![feature(async_await)]
#![feature(async_closure)]
#[allow(dead_code)]

#[macro_use]
extern crate lazy_static;
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
pub mod metrics;
pub mod emoji;
// pub mod store;
pub mod state;
pub mod machine;
// pub mod api;

pub mod mio;

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
use std::ffi::OsStr;
// use path_table::PathTable;
const README_MD: &'static str = include_str!("../README.md");




use async_std::fs;
use async_std::os::unix::fs::symlink;
// use async_std::io;
// use async_std::prelude::*;
// use async_std::task;
use error::Result;

pub fn hello_lar() {
    use yansi::Paint;
    println!(r#"  "#);
    println!(r#" {:} "#,Paint::blue("   ██╗      █████╗ ██████╗    "));
    println!(r#" {:} "#,Paint::blue("   ██║     ██╔══██╗██╔══██╗   "));
    println!(r#" {:} "#,Paint::blue("   ██║     ███████║██████╔╝   "));
    println!(r#" {:} "#,Paint::blue("   ██║     ██╔══██║██╔══██╗   "));
    println!(r#" {:} "#,Paint::blue("   ███████╗██║  ██║██║  ██║   "));
    println!(r#" {:} "#,Paint::blue("   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝   "));
    println!(r#" {:} "#,Paint::blue("ONLINE WATER QUALITY ANALYShhERS"));
    println!(r#"      "#);
    println!(r#"      "#);
}

pub fn hello()  {
    use yansi::Paint;
println!(r#"  {:}  "#,Paint::blue(r#"              _                        _  "#));
println!(r#"  {:}  "#,Paint::blue(r#"             | |                      | |  "#));
println!(r#"  {:}  "#,Paint::blue(r#"   __ _ _   _| |_ ___  _ __ ___   __ _| |_ __ _  "#));
println!(r#"  {:}  "#,Paint::blue(r#"  / _` | | | | __/ _ \| '_ ` _ \ / _` | __/ _` |  "#));
println!(r#"  {:}  "#,Paint::blue(r#" | (_| | |_| | || (_) | | | | | | (_| | || (_| |  "#));
println!(r#"  {:}  "#,Paint::blue(r#"  \__,_|\__,_|\__\___/|_| |_| |_|\__,_|\__\__,_|  "#));
println!(r#"  {:}  "#,Paint::blue(r#"  "#));

    println!(r#"  {:}  "#,Paint::blue("  "));
    println!(r#"  {:}  "#,Paint::blue("  █████╗ ██╗   ██╗████████╗ ██████╗ ███╗   ███╗ █████╗ ████████╗ █████╗ "));
    println!(r#"  {:}  "#,Paint::blue(" ██╔══██╗██║   ██║╚══██╔══╝██╔═══██╗████╗ ████║██╔══██╗╚══██╔══╝██╔══██╗ "));
    println!(r#"  {:}  "#,Paint::blue(" ███████║██║   ██║   ██║   ██║   ██║██╔████╔██║███████║   ██║   ███████║ "));
    println!(r#"  {:}  "#,Paint::blue(" ██╔══██║██║   ██║   ██║   ██║   ██║██║╚██╔╝██║██╔══██║   ██║   ██╔══██║ "));
    println!(r#"  {:}  "#,Paint::blue(" ██║  ██║╚██████╔╝   ██║   ╚██████╔╝██║ ╚═╝ ██║██║  ██║   ██║   ██║  ██║ "));
    println!(r#"  {:}  "#,Paint::blue(" ╚═╝  ╚═╝ ╚═════╝    ╚═╝    ╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝ "));
    // println!(r#"  {:}  "#,Paint::blue("   AUTOMATISATION SYSTEM"));
    // println!(r#"  ENVIRONMENTAL MONITORING  "#);
}



lazy_static! {
    pub static ref AUTOMATA_ROOT: PathBuf = {
        let path:PathBuf = env::var_os("AUTOMATA_ROOT")
            .unwrap_or_else(|| OsStr::new("/var/run/automata").to_os_string())
            .into();
        // if !path.exists() {
            // info!("init new store:{}",path.as_path().to_string_lossy());
            // fs::create_dir_all(path.as_path()).unwrap();
        // }
        path
    };
    pub static ref RUNDIR: PathBuf = {
       PathBuf::from("/var/run/automata")
    };
    // pub static ref MIO:TempDir =  {
        // Builder::new().prefix("automataio").tempdir()
    // };

    pub static ref LOCAL_DIR: PathBuf = AUTOMATA_ROOT.join("local");
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
    pub static ref CRATES_DIR: PathBuf = AUTOMATA_ROOT.join("shared/crates");
    pub static ref EXPERIMENT_DIR: PathBuf = AUTOMATA_ROOT.join("ex");
    pub static ref LOG_DIR: PathBuf = AUTOMATA_ROOT.join("logs");
    pub static ref LOCAL_CRATES_DIR: PathBuf = "local-crates".into();
    pub static ref SOURCE_CACHE_DIR: PathBuf = AUTOMATA_ROOT.join("cache").join("sources");
}


pub fn rootdir() -> PathBuf {
    AUTOMATA_ROOT.clone()
}
pub fn workdir() -> PathBuf {
    rootdir()
}
pub fn rundir() -> &'static Path {
    RUNDIR.as_path()
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
#[derive(Clone, Debug)]
pub struct Automata {
    pub path: PathBuf,
    // pub store:
}


impl Automata {

    pub fn new_from_path(path: &Path) -> Automata{
        // path
        let path = path.to_path_buf();
        // if !path.exists() {
            // DirBuilder::new().recursive(true).create("/tmp/foo/bar/baz").await?;
            // fs::DirBuilder::new()create_dir_all(path.join("channel")).unwrap();
            // fs::create_dir_all(path.join("sensor")).unwrap();
            // fs::create_dir_all(path.join("stream")).unwrap();
        // }
        Automata{
            path: path,
        }
    }
    pub fn new() -> Automata {
        let path = AUTOMATA_ROOT.clone();
        Automata::new_from_path(AUTOMATA_ROOT.as_path())
    }
    pub fn root(&self) -> Node{
        Node::new(self.path.as_path())
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
    let path = rundir();
    if !path.exists() {
        fs::DirBuilder::new().recursive(true).create(path).await?;
    }
    let path = path.join("/store");
    symlink(store_path,path.as_path()).await?;
    Ok(Node::new(path.as_path()))
}
