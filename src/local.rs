//! Local device linux fs
//! .
//! ├── /automata
//! │   ├── stats.ron
//! │   ├── automata.toml
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
//! //! .automata/
//! //!      streams/
//!             1/
//!               stream.ron
//!               1/
//!
//!      history/
//!      logs/
use serde::{Deserialize, Serialize};

// use crate::Result;
use failure::{Fallible};
// use regex::Regex;
// use dirs;
use std::fs;
// use walkdir::{WalkDir};
use log::info;
// use tempfile::{
//     TempDir,
//     Builder,
// };
// use analyzer::flow::*;
// use analyzer::*;
// use settings::ron::Config;
use std::{
    path::PathBuf,
};
// use super::mio::airflow::AirflowSetting;
use std::env;
use std::ffi::OsStr;
// use tempfile::Builder;
// use super::calibration::*;

// use std::collections::HashMap;

// use super::Result;




// use super::Hid;

// use http::status::StatusCode;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Draft,
    QuickTOCuv,
    QuickTOCultra,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataSheet {
    pub model: Model,
    pub producted: u64,
    pub updated: u64,
    pub wartung: u64
}




// pub fn createtmp() -> Result<PathBuf> {
// }

//

