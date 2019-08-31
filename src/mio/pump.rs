/// Monitor gear pump normally used for solution sampling.
///
use serde::{self,Deserialize, Serialize};
use serde_json::{
    from_slice,
    to_vec
};
use async_std::fs;
// use async_std::io;
use log::info;
use crate::error::Result;
// use async_std::os::unix::fs::symlink;
use async_std::os::unix::net::UnixStream;
use async_std::prelude::*;
use async_std::task;
// use ron::{self};
use std::path::{PathBuf,Path};

use yansi::Paint;
use std::time::{
    // SystemTime,
    Duration,
 };


// use lazy_static::lazy_static;
#[derive(Serialize,Deserialize, Clone, Debug)]
pub enum State {
    Brocket,
    Stopped,
    Runned
}

#[derive(Serialize,Deserialize, Clone, Debug)]
pub enum Msg {
    Status(State)
}

#[derive(Serialize,Deserialize, Clone, Debug)]
pub enum Command {
    Stop,
    Run,
}



#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct Pump {
    path: PathBuf,
    pub state: State,
    pub uptime: u64,
    pub runtime: u64,
    pub label: String,
    // datagram: UnixDatagram,
}

impl Pump {

    /// Pump run time in seconds
    pub fn runtime(&self) -> u64 {
        self.runtime
    }
    pub fn state(&self) -> State {
        self.state.clone()
    }
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Working directory
pub fn workdir() -> PathBuf {
    super::workdir().join("pump")
}

/// Read current state
pub async fn state(pump:&Pump) -> Result<State> {
    let state = pump.path.join("state");
    let state:State = from_slice(fs::read(&state).await?.as_slice())?;
    Ok(state)
}
/// read pump state.
pub async fn pump(path:&Path) -> Result<Pump> {
    let state = path.join("state");
    let mut file = fs::File::open(state.as_path()).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let state: State = from_slice(&buf)?;
    let label = path.join("label");
    let label = fs::read_to_string(&label).await?;
    Ok(Pump{
        path: path.to_path_buf(),
        state:state,
        label: label,
        runtime: 0,
        uptime: 0,
    })
}

/// Send command to pump driver
pub async fn command(pump: &Pump,cmd:Command) -> Result<()> {
    let path = pump.path.join("event");
    let mut stream = UnixStream::connect(&path).await?;
    let buf = to_vec(&cmd)?;
    stream.write_all(buf.as_slice()).await?;
    Ok(())
    // let mut response = Vec::new();
    // stream.read_to_end(&mut response).await?;
    // let state:State = from_slice(response.as_slice())?;
    // Ok(state)
}

pub async fn start(pump :Pump ) -> Result<Pump> {
    info!("{}: turn {}", Paint::cyan(format!("{}",pump.path.as_path().to_str().unwrap())), Paint::green("start"));
    command(&pump,Command::Run).await?;
    Ok(pump)
    // let path = pump..directory().join("state");
    // let mut file = fs::File::create(path.as_path()).await?;
    // let state = serde_json::to_vec(&pump.start()).unwrap();
    // file.write_all(state.as_slice()).await?;
    // Ok(())
}

pub async fn stop(pump:Pump) -> Result<Pump> {
    info!("{}: turn {}", Paint::cyan(format!("{}",pump.path.as_path().to_str().unwrap())), Paint::green("start"));
    command(&pump,Command::Stop).await?;
    Ok(pump)
    // let path = pump.mio.directory().join("state");
    // let mut file = fs::File::create(path.as_path()).await?;
    // let state = serde_json::to_vec(&pump.stop()).unwrap();
    // file.write_all(state.as_slice()).await?;
    // file.sync_data().await?;
    // info!("pump[{}] turn {}", Paint::cyan(format!("{}", pump.mio.id)), Paint::cyan("stop"));
    // Ok(())
}

pub async fn sampling(pump:Pump,sec: u64) -> Result<Pump> {
    let pump = start(pump).await?;
    task::sleep(Duration::from_secs(sec)).await;
    Ok(stop(pump).await?)
}
