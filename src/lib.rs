#![feature(async_await)]
#[allow(dead_code)]

// mod data;
pub mod error;
pub mod cli;
pub mod config;
pub mod metrics;

// mod templates;
mod monitor;

pub use monitor::*;
// pub use templates::*;
// pub use staticfile::*;
// mod graphql;
// pub mod logger;
// mod github;

// mod metrics;


// use github::*;
// pub use self::error::*;
// pub use self::cli::*;
// pub use self::graphql::*;



// pub fn hello() {
  // println!{"Automata,"}

//}
