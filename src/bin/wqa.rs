//! wqa- water quality analyze
//! rewrited in runs.
//!
// use std::io::prelude::*;
// use std::net::TcpListener;
use async_log::{instrument, span};
use log::info;
fn hello()  {

    println!(r#" "#);

    println!(r#"   ##       #    #####     "#);
    println!(r#"   ##      ###   ##  ##    "#);
    println!(r#"   ##     ## ##  #####     "#);
    println!(r#"   ##    ## #### ##  ##    "#);
    println!(r#"   #######     ####   ##   "#);
    println!(r#"     -----------------     "#);
    println!(r#"   PROCESS WATER ANALYZER  "#);
    // println!()
    // println!(r#" "#);
    // println!(r#" "#);
    // println!(r#" ###   #   ### #####   #####   "#);
    // println!(r#"  ##   #   ## ##   ## ##   ##  "#);
    // println!(r#"   ## ### ##  ##   ## ##   ##  "#);
    // println!(r#"    #######    ######  ####### "#);
    // println!(r#"                   ##          "#);
    // println!(r#"                   #           "#);
    //     //       run it...                                "#);
        // println!(r#"  __ _ _   _| |_ ___  _ __ ___   __ _| |_ __ _ "#);
        // println!(r#" / _` | | | | __/ _ \| '_ ` _ \ / _` | __/ _` |"#);
        // println!(r#"| (_| | |_| | || (_) | | | | | | (_| | || (_| |"#);
        // println!(r#" \__,_|\__,_|\__\___/|_| |_| |_|\__,_|\__\__,_|"#);
        // println!();
}
// With the "paw" feature enabled in structopt
#[derive(structopt::StructOpt)]
struct Args {
    /// Address to listen on.
    #[structopt(short = "p", long = "path", default_value = ".")]
    path: String,
}
fn setup_logger() {
    let logger = env_logger::Builder::new()
        .filter(None, log::LevelFilter::Trace)
        .build();

    async_log::Logger::wrap(logger, || /* get the task id here */ 0)
        .start(log::LevelFilter::Trace)
        .unwrap();
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    setup_logger();
    hello();
    info!("✨ Init wqa directory {} ✨",args.path);

    span!("new level, depth={}", 1, {
        let x = "beep";
        info!("look at this value, x={}", x);

        span!("new level, depth={}", 2, {
            inner("boop");
        })
    });

    Ok(())
}

#[instrument]
fn inner(y: &str) {
    info!("another nice value, y={}", y);
}
