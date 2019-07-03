#![feature(async_await)]

use structopt::*;
use tide;
use tocuv::*;
use runtime;
use failure::{Fallible, ResultExt};
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
    let monitor = new_uv_monitor();
    let mut app = tide::App::new(monitor);
    app.middleware(tide::middleware::RootLogger::new());
    app.at("/").get(async move |_| "QuickTOCuv!");
    app.at("/info").get(monitor_system_info);
    app.serve("127.0.0.1:8000").unwrap();
}

