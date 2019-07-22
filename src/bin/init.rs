use std::io::prelude::*;
use std::net::TcpListener;
use log::info;
// With the "paw" feature enabled in structopt
#[derive(structopt::StructOpt)]
struct Args {
    /// Address to listen on.
    #[structopt(short = "p", long = "path", default_value = ".")]
    path: String,
}

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    info!("✨ Init wqa directory {} ✨",args.path);

    Ok(())
}
