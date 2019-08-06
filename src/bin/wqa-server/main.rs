#![feature(async_await)]
#![feature(async_closure)]

// #![allow(clippy::doc_markdown)]

// use structopt::*;
use wqa::error::*;
use wqa::api::{
    app,
    templates,
    routes,
};
//  use wqa::api::{
//     uv as analyzer,
    // app,
// };

// use wqa::mio;


// use tide::{Error};
// use crate::{
    // config::{
        // ServerConfig,
    // },
// };

// use runtime;
// use tide::{self, App, Context, EndpointResult, Error};
use tide;

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

    println!(r#"    "#);
    println!(r#"  ██╗      █████╗ ██████╗  "#);
    println!(r#"  ██║     ██╔══██╗██╔══██╗  "#);
    println!(r#"  ██║     ███████║██████╔╝  "#);
    println!(r#"  ██║     ██╔══██║██╔══██╗  "#);
    println!(r#"  ███████╗██║  ██║██║  ██║  "#);
    println!(r#"  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝  "#);
    println!(r#"   PROCESS WATER ANALYZER   "#);
}

// async fn collect(_cx: Context<()>) -> EndpointResult<http::Response<Body>> {
//     let mut buffer = BytesMut::with_capacity(16_384);

//     wqa::data::cpu::cpu(&mut buffer).await;
//     wqa::data::host::host(&mut buffer).await;
//     wqa::data::disk::disk(&mut buffer).await;
//     wqa::data::memory::memory(&mut buffer).await;

//     let resp = http::Response::builder()
//         .status(http::status::StatusCode::OK)
//         .body(Body::from(buffer))
//         .unwrap();
//     Ok(resp)
// }

#[runtime::main]
async fn main()  -> Result<()>{
    use log::{info,warn};
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};
    hello();
    info!("✨ run wqa backend ✨");
    wqa::config::setup();
    warn!("⚠️ setup warning test");
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    //  let template_dir = format!("{}/examples/templates/*", env!("CARGO_MANIFEST_DIR"));
    // TODO: load configuration.
    // let severConfig = wqa::config::ServerConfig::default();
    let state = app::State::new();
    // let repo = monitor::new_uv().await;
    let mut app      = tide::App::with_state(state);
    app.middleware(tide::middleware::RequestLogger::new());
    info!("🌩️   starting broker");
    // app.middleware(tide::middleware::RequestLogger::new());
    app.at("/").get(templates::index);
    app = routes::setup(app);
    app.run("127.0.0.1:8000")?;
    Ok(())
    // analyzer::store::setup(".").await?;
    //
    // wqa::server::`
}
