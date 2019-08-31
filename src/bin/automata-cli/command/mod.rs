pub mod check;
pub mod init;


use structopt::StructOpt;

/// 📣 The various kinds of commands that `cantorpc` can execute.
#[derive(Debug, StructOpt)]
pub enum Command {
    /// 🔧 init automata directory
    #[structopt(name = "init", about = "📠  checkt CAN and nodes")]
    Init(init::Opt),
    /// 📠  checkt CAN and nodes.
    #[structopt(name = "check", about = "📠  checkt CAN and nodes")]
    Check(check::Opt),
   //  ⥄‍♀ run ipc communication mod
    // #[structopt(name = "ipc", about = "⥄‍♀ run ipc named socket server")]
    // Ipc(Opt),
   //  ⥄‍♀ run pipe mod.
    // #[structopt(name = "pipe", about = " ⥄‍♀ run in out pipe")]
    // Pipe,
   //  🔂‍♀ run tcp server.
    // #[structopt(name = "tcp", about = " 🔂‍♀ run tcp socket server")]
    // TCP(Opt),
   //  🔂‍♀ run udp server.
    // #[structopt(name = "udp", about = "🔂‍♀ run udp socket server")]
    // UDP(Opt),
}



pub async fn run (command: Command ) {

}
