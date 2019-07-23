#![feature(async_await)]
#![feature(async_closure)]

#![allow(clippy::doc_markdown)]

use structopt::*;
use wqa::error::*;
use wqa;
use wqa::analyzer;

mod local;
mod app;
mod templates;
// use tide::{Error};
// use crate::{
    // config::{
        // ServerConfig,
    // },
// };

use app::State;
// use runtime;
use tide::{self, App, Context, EndpointResult, Error};

// impl WqaError {
//     pub fn response_500(self) -> Error {
//         let resp = http::Response::builder()
//                 .status(500)
//                 .body("response".to_owned().into())
//                 .unwrap();
//         Error::from(resp)
//     }
// }


fn hello()  {
        println!(r#"      run it...                                "#);
        println!(r#"  __ _ _   _| |_ ___  _ __ ___   __ _| |_ __ _ "#);
        println!(r#" / _` | | | | __/ _ \| '_ ` _ \ / _` | __/ _` |"#);
        println!(r#"| (_| | |_| | || (_) | | | | | | (_| | || (_| |"#);
        println!(r#" \__,_|\__,_|\__\___/|_| |_| |_|\__,_|\__\__,_|"#);
        println!();
}


#[runtime::main]
async fn main() -> Result<()> {
    use log::info;
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};
    info!("✨ run wqa server ✨");
    wqa::config::setup();
    hello();

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    //  let template_dir = format!("{}/examples/templates/*", env!("CARGO_MANIFEST_DIR"));
    /// TODO: load configuration.
    // let severConfig = wqa::config::ServerConfig::default();
    let state = State::new();
    // let repo = monitor::new_uv().await;
    let mut app = tide::App::with_state(state);
    app.middleware(tide::middleware::RequestLogger::new());
    app.at("/").get(templates::index);
    app.at("/api").nest(|api| {
        api.at("/device").get(local::get_device);
        api.at("/device/serial").get(local::get_device_serial).post(local::set_device_serial);
        api.at("/streams").get(local::get_streams_list);
        api.at("/stream/:number").get(local::get_stream).post(local::set_stream);
        api.at("/rules").get(local::get_rules);
    // api.at("/streams").get(stream)
    //   api.at("/info").get(device::get_info);
    });

    app.run("127.0.0.1:8000")?;
    Ok(())
    // analyzer::store::setup(".").await?;
    //
    // wqa::server::`
}
