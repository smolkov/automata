//! linux fs local storage.
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
// use super::mio::airflow::AirflowSetting;
use std::env;
use std::ffi::OsStr;
// use super::calibration::*;

// use std::collections::HashMap;

use super::Result;

lazy_static! {
    pub static ref ROOTDIR: PathBuf = {

        let path:PathBuf = env::var_os("WQAROOT")
            .unwrap_or_else(|| OsStr::new("/wqa").to_os_string())
            .into();
        if !path.exists() {
            info!("init new store:{}",path.as_path().to_string_lossy());
            fs::create_dir_all(path.as_path()).unwrap();
        }
        path
    };
    pub static ref MIO:TempDir =  {
            Builder::new().prefix("mio").tempdir().unwrap()
        };

    pub static ref LOCAL_DIR: PathBuf = WQADIR.join("local");
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
    pub static ref CRATES_DIR: PathBuf = WQADIR.join("shared/crates");
    pub static ref EXPERIMENT_DIR: PathBuf = WQADIR.join("ex");
    pub static ref LOG_DIR: PathBuf = WQADIR.join("logs");
    pub static ref LOCAL_CRATES_DIR: PathBuf = "local-crates".into();
    pub static ref SOURCE_CACHE_DIR: PathBuf = WQADIR.join("cache").join("sources");
}



pub fn rootdir() -> Result<PathBuf> {
    let path = ROOTDIR.clone();
    Ok(path)
}

pub fn createtmp() -> Result<PathBuf> {

}

//
