
// use lazy_static::lazg_static;

use tide::{App};
// use async_std::prelude::*;
use super::*;
use super::error::Result;
use log::{warn,info};
use std::net::SocketAddr;
use std::env;
use super::config;

pub fn routes(mut app: App<Automata>) -> App<Automata> {
    info!("🔖️ setup routes");
    
    app
}

pub async fn server(_config: &config::ServerConfig) -> Result<()> {
    let host = env::var("HOST")
        .as_ref()
        .map(String::as_str)
        .unwrap_or("0.0.0.0")
        .to_string();

    let port = env::var("PORT")
        .as_ref()
        .map(String::as_str)
        .unwrap_or("8125")
        .to_string();

    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
// let file_path = automata::MIO.path().join("my-temporary-note.txt");
    // let mut file = File::create(file_path)?;
    // writeln!(file, "Brian was here. Briefly.")?;
    let path = workdir();
    let repo = automata(path.as_path()).await?;

    let mut app      = tide::App::with_state(repo);
    app.middleware(tide::middleware::RequestLogger::new());
    info!("🌩️   starting broker");

    warn!("⚠️   setup warning test");
    // app.middleware(tide::middleware::RequestLogger::new());
    // app.at("/").get(automata::templates::example_index);
    app = routes(app);
    app = metric::routes(app);
    app.run(addr).unwrap();
    Ok(())
}

// lazy_static! {

// }
