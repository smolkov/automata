#![feature(async_await)]
#![feature(async_closure)]
#[allow(dead_code)]

/// mod data;
pub mod error;
pub mod common;
pub mod analyzer;
pub mod server;
// pub mod cli;
pub mod config;
pub mod metrics;
pub mod emoji;

// mod templates;
// pub mod monitor;
// pub use app::AppState;
// pub use templates::*;
// pub use staticfile::*;
// mod graphql;
// pub mod logger;
// mod github;

// mod metrics;

// use github::*;
pub use self::error::*;
pub use self::common::*;
// pub use self::cli::*;
// pub use self::graphql::*;
