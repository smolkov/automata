/// Valve
///
use serde::{self,Deserialize, Serialize};
use serde_json::{
    from_slice,
    to_vec
};
use crate::error::Result;
use super::Mio;
use async_std::fs;
use async_std::io;
use log::info;
use chrono::Utc;
// use async_std::os::unix::fs::symlink;
use async_std::os::unix::net::UnixDatagram;
use async_std::prelude::*;
use async_std::task;
// use ron::{self};
use crate::error::Result;

use std::time::{
    // SystemTime,
    Duration,
 };

    use yansi::Paint;

/// Valve state
#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct State {
    pub updated: u64,
    pub on: bool,
}

#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct Valve
{
    path: PathBuf,
    pub open: bool,
    pub updated: u64,
}


impl Valve {
    /// Open valve
    pub fn open(&self) -> State {
        State {
            on      : true,
            updated : Utc::now().timestamp_millis() as u64,
        }
   }
   /// Close valve
    pub fn close(&self) -> State{
         State {
            on      : false,
            updated : Utc::now().timestamp_millis() as u64,
        }
    }
}

pub async fn valve(path:&Path) -> Result<Valve> {

    Ok(Valve{
        path: path.to_path_buf(),
        open: FALSE,
        updated: 0,
    }
}

// pub async fn valves(path:&Path) -> Re

/// read valve state
pub async fn state(valve:&Valve) -> io::Result<State> {
    let path = valve.mio.directory().join("state");
    let mut file = fs::File::open(path.as_path()).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let state: State = from_slice(&buf)?;
    Ok(state)

}
/// open valve
pub async fn open(valve:&Valve) -> io::Result<()> {
    let path = pump.mio.directory().join("state");
    let mut file = fs::File::create(path.as_path()).await?;
    let state = serde_json::to_vec(&pump.start()).unwrap();
    file.write_all(state.as_slice()).await?;
    info!("valve[{}] turn {}", Paint::cyan(format!("{}", id)), Paint::cyan("open"));
    Ok(())
}
/// close valve
pub async fn close(valve:&Valve) -> Result<()> {
    let path = valve.mio.directory().join("state");
    let mut file = fs::File::create(path.as_path()).await?;
    let state = serde_json::to_vec(&pump.stop()).unwrap();
    file.write_all(state.as_slice()).await?;
    file.sync_data().await?;
    info!("valve[{}] turn {}", Paint::cyan(format!("{}", id)), Paint::cyan("close"));
    Ok(())
}

