use structopt::StructOpt;
// With the "paw" feature enabled in structopt
#[derive(structopt::StructOpt)]
struct Args {
    /// Port to listen on.
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "8080")]
    port: u16,
    /// Address to listen on.
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: String,
}


//run comand
// #[derive(StructOpt)]
// enum Cmd {
    // #[structopt(name = "migration", about = "Run database migrations")]
    // Migration(migration::Opt),
    // #[structopt(name = "run", about = "Start running the bot")]
    // Run(run::Opt),
    // #[structopt(name = "setup", about = "")]
    // Setup(setup::Opt),
    // #[structopt(name = "twitter-list-sync", about = "")]
    // TwitterListSync(twitter_list_sync::Opt),
    // #[structopt(name = "twitter-login", about = "")]
    // TwitterLogin(twitter_login::Opt),
// }


mod exercise;
mod run;
mod verify;


pub use self::exercise::*;
pub use self::run::*;
pub use self::verify::*;
//
