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


pub mod mio;
pub mod error;
pub mod config;
pub mod metrics;
pub mod emoji;
pub mod store;
pub mod api;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// pub mod api;

