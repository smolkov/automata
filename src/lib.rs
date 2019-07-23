#![feature(async_await)]
#![feature(async_closure)]
#[allow(dead_code)]

/// mod data;
pub mod error;
pub mod common;
pub mod workspace;
pub mod analyzer;
// pub mod cli;
pub mod config;
pub mod metrics;
pub mod emoji;
pub use self::common::*;

// pub use self::error::*;
