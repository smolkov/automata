//! LAR water quality analyzers
//!
//! [`log`]: https://lar.de/wqa
//!
//!
//!
//! ## What is wqa?
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
// #[macro_use]
// extern crate rust_embed;

pub mod local;
pub mod logger;
pub mod mio;
pub mod error;
pub mod config;
pub mod metrics;
pub mod emoji;
pub mod store;
pub mod state;
pub mod machine;
// pub mod api;
pub mod hid;
pub mod templates;
pub mod asset;
pub mod welcom;
use tempfile::{TempDir};
use tempfile::{Builder};


// pub use asset::Asset;
pub use config::GLobalConfig;
pub use store::Store;
pub use state::State;
pub use mio::Mio;
pub use templates::Template;

pub use machine::Machine;
use log::info;
use std::{
    fs,
    path::PathBuf,
};
use std::env;
use std::ffi::OsStr;

// const WELCOM_MD: &'static str = include_str!("assets/welcome.md");

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// pub mod api;




/// Entry point interface for interacting with Github API
#[derive(Clone, Debug)]
pub struct Wqa {
    pub path: PathBuf,
    // pub pathtable: PathTable;
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

impl Wqa {

    pub fn new(path: PathBuf) -> Result<Wqa>{
        if !path.exists() {
            fs::create_dir_all(path.join("channel")).unwrap();
            fs::create_dir_all(path.join("sensor")).unwrap();
            fs::create_dir_all(path.join("stream")).unwrap();
        }
        Ok(Wqa{
            path: WQADIR.clone()
        })
    }
    pub fn config(&self) -> GLobalConfig{
        GLobalConfig::new(self.clone())
    }
    // pub fn client(&self) -> Option<Client>{
        // self.address = address;
    // }
    pub fn store(&self) -> Store  {
        Store::new(self.clone())
    }
    // pub fn control() -> Control {

    // }
    pub fn mio(&self) -> Mio {
        Mio::new(self.clone())
    }
    pub fn state(&self) -> State{
        State::new(self.clone())
    }

    pub fn template(&self) -> Template {
        Template::new(self.clone())
    }

}
