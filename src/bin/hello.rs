// use std::io::prelude::*;
// use std::net::TcpListener;

// use std::error::Error;
// use std::io::{self, prelude::*};
// use std::net::TcpListener;
// use log::info;
// use failure::{Fallible, ResultExt};





fn hello()  {
        println!(r#"      run it...                                "#);
        println!(r#"  __ _ _   _| |_ ___  _ __ ___   __ _| |_ __ _ "#);
        println!(r#" / _` | | | | __/ _ \| '_ ` _ \ / _` | __/ _` |"#);
        println!(r#"| (_| | |_| | || (_) | | | | | | (_| | || (_| |"#);
        println!(r#" \__,_|\__,_|\__\___/|_| |_| |_|\__,_|\__\__,_|"#);
        println!();
}

#[derive(structopt::StructOpt)]
struct Args {
    /// Port to listen on.
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "8080")]
    port: u16,
    /// Address to listen on.
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: String,
}


fn main() {
  hello();
}

// // #[paw::main]
// fn main(args: Args) -> Fallible<()> {
    // let listener = TcpListener::bind((&*args.address, args.port))?;
    // hello();
    // println!("listening on {}");
    // for stream in listener.incoming() {
        // stream?.write(b"hello world!")?;
    // }
    // Ok(())
// }
