#![feature(async_await)]

use automata::*;

pub mod command;
use command::Command;
// use std::path::PathBuf;
// use log::info;
use store::logger;

use async_std::io;
use async_std::prelude::*;
use async_std::task;
// use std::sync::Arc;
// use std::{thread, time};
// use quicli::prelude::*;


// use jsonrpc_core::{Error, IoHandler, Result};
// use std::io::prelude::*;
use structopt::StructOpt;

/// ✏ protocol options
#[derive(Debug,StructOpt)]
pub struct Opt {
    /// 📪  Working directory path.
    #[structopt(short = "", long = "path", default_value = ".")]
    path: String,

    /// 📪  Address IP or filepath
    #[structopt(short = "a", long = "address", default_value = "0.0.0.0")]
    address: String,

    /// Port to listen on.
    #[structopt(short = "p", long = "port", default_value = "9000")]
    port: u16,
}





/// 🧰  Edibnurgh sensor
#[derive(Debug, StructOpt)]
struct Cli {
    /// 🔧 Protocol
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub command: Command,
    // ⏱  Message interval in milliseconds
    // #[structopt(name = "interval", long = "interval", default_value = "1000")]
    // interval: u64,
    // ⦨  scale signal value
    // #[structopt(name = "scale", long = "scale", default_value = "1.0")]
    // scale: f64,
}

pub fn run_ipc(opt :Opt) -> io::Result<()> {
    // use jsonrpc_ipc_server::ServerBuilder;
    // use jsonrpc_ipc_server::jsonrpc_core::*;
    // let analog_node = can::AnalogNode::new(0x2);
    // let analog_node2 = can::AnalogNode::new(0x4);
    // let mut io  = IoHandler::new();
    // io.extend_with(can::NodeObject.to_delegate());
    // io.extend_with(analog_node.to_delegate());
    // io.extend_with(analog_node2.to_delegate());
	// io.extend_with(can::DMNode.to_delegate());
    // io.extend_with(can::DigitalNode.to_delegate());
    // io.extend_with(can::AOutsNode.to_delegate());
    // let builder = ServerBuilder::new(io);
    // let path = PathBuf::from(opt.path).join("ipc.socket");
	// let server = builder.start(path.to_str().unwrap())?;
	// server.wait();
    Ok(())
}

fn main() -> io::Result<()> {
    use atty::Stream;
    automata::hello();
    logger::setup_async();
    let args = Cli::from_args();
    if atty::is(Stream::Stdout) {
        println!("I'm a terminal");
    } else {
        println!("I'm not");
    }
    match args.command {
        // Command::Check(subopt) => {command::check::run(subopt)?;},
        _ => unreachable!(),
    };
    // info!("result : {:?}",res);
    Ok(())

    // let server = ServerBuilder::with_meta_extractor(io, |context: &RequestContext| {
		// Arc::new(Session::new(context.sender.clone()))
	// })
	// .session_stats(Stats)
	// .start("./test.ipc")
	// .expect("Unable to start RPC server");

	// server.wait();

    // let server = ServerBuilder::new(io)
		// .start(&SocketAddr::new(args.address.parse().unwrap(),args.port))
		// .expect("Unable to start RPC server");
	// let _ = server.wait();
}



//
