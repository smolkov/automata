//! Water quality analyzers state.
//!
//! [`log`]: https://lar-wqa.rs/wqa
//!
//!
//!
//! ## What is wqa?
//!
//! ## Client description?
//!
#![feature(async_await)]
#![feature(async_closure)]
#[allow(dead_code)]

#[macro_use]
extern crate lazy_static;


pub mod mio;
pub mod error;
pub mod config;
pub mod metrics;
pub mod emoji;
pub mod stream;
pub mod measurement;
pub mod rules;
pub mod store;
pub mod api;
