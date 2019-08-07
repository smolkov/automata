/// Monitor gear pump normally used for solution sampling.
///
use serde_derive::{Deserialize, Serialize};
use std::sync::RwLock;
use crate::error::*;
// use lpcan::can::{
// can0,
// Message,
// };
#[derive(Serialize,Deserialize, Clone, Debug, PartialEq)]
pub enum State {
    Stop,
    Runned,
    Persist,
}

impl Default for State {
  fn default() -> State {
      State::Stop
  }
}


use lazy_static::lazy_static;



lazy_static! {
    static ref CONDENSAT: RwLock<GearPump> = RwLock::new(GearPump::new());
    static ref GP1: RwLock<GearPump> = RwLock::new(GearPump::new());
    static ref GP2: RwLock<GearPump> = RwLock::new(GearPump::new());
    static ref GP3: RwLock<GearPump> = RwLock::new(GearPump::new());
    static ref GP4: RwLock<GearPump> = RwLock::new(GearPump::new());
    static ref GP5: RwLock<GearPump> = RwLock::new(GearPump::new());
    static ref GP6: RwLock<GearPump> = RwLock::new(GearPump::new());
}


async fn start_pump_simulation() -> Result<(),WqaError> {

}
async fn start_pump_simulation() -> Result<(),WqaError> {
}

#[derive(Serialize,Default, Deserialize, Clone, Debug, PartialEq)]
pub struct GearPump {
    pub hid: u64,
    pub state: State,
}

impl GearPump {
    pub fn new() -> GearPump {
        GearPump {
            state: State::default(),
        }
    }
    pub fn start(&mut self) {
        self.state = State::Runned;
    }
    pub fn stop(&mut self) {
        self.state = State::Runned;
    }
}





pub async fn gp1_start() -> Result<()> {
    Ok(())
}
pub async fn sample1_stop() -> Result<()> {
    Ok(())
}

pub async fn sample2_start() -> Result<()> {
    Ok(())
}
pub async fn sample2_stop() -> Result<()> {
    Ok(())
}
pub async fn sample3_start() -> Result<()> {
    Ok(())
}
pub async fn sample3_stop() -> Result<()> {
    Ok(())
}
pub async fn sample4_start() -> Result<()> {
    Ok(())
}
pub async fn sample4_stop() -> Result<()> {
    Ok(())
}
pub async fn sample5_start() -> Result<()> {
    Ok(())
}
pub async fn sample5_stop() -> Result<()> {
    Ok(())
}
pub async fn sample6_start() -> Result<()> {
    Ok(())
}
pub async fn sample6_stop() -> Result<()> {
    Ok(())
}

