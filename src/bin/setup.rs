use std::io::prelude::*;
use std::net::TcpListener;

// use std::error::Error;
use std::io::{self};
// use std::net::TcpListener;
use log::info;
use failure::{Fallible};

/// To run this example do:
/// ```sh
/// $ cargo run --example args -- localhost 8080
/// ```
// With the "paw" feature enabled in structopt
// With the "paw" feature enabled in structopt

#[derive(structopt::StructOpt)]
struct Args {
    /// Address to listen on.
    #[structopt(short = "p", long = "path", default_value = ".")]
    address: String,
}

#[paw::main]
fn main(args: paw::Args) -> Fallible<()> {

    info!("✨ Setup automata ✨");
    let mut args = args.skip(1);

    let host = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the host argument is missing"))?;

    let port = args
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "the port argument is missing"))?
        .parse()?;

    let listener = TcpListener::bind((host.as_str(), port))?;
    println!("listening on {}", listener.local_addr()?);

    for stream in listener.incoming() {
        stream?.write(b"hello world!")?;
    }
    Ok(())
}
