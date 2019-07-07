#![feature(async_await)]

use structopt::*;
use tide;
use wqm_uv::*;
use runtime;
use failure::{Fallible};

pub use tide::{error::ResultExt, response, App, Context, EndpointResult};
use http::status::StatusCode;
// use failure::{Fallible, ResultExt};



#[runtime::main]
async fn main() {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    let repo = monitor::new_uv().await;
    let mut app = tide::App::new(repo);
    app.middleware(tide::middleware::RootLogger::new());
    app.at("/api").nest(|api| {
      api.at("/model").get(async move |_| "QuickTOCuv");
      api.at("/info").get(device::get_info);
    });
    app.serve("127.0.0.1:8000").unwrap();
}

