/// Monitor gear pump normally used for solution sampling.
///
use serde_derive::{Deserialize, Serialize};
use std::sync::RwLock;
use crate::WqaError;
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
    pub state: Vec<bool>

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

pub struct SamplePump {
    gp : Vec<GearPump>,
    updated: u64,
}

impl Default for SamplePump {

}


pub async fn sample_start(stream:u8) -> Result<(),WqaError> {
    GP1.write().unwrap().start();
    Ok(())
}

pub async fn sample_start_timeout(pump:GearPump,timeout:u64) -> Result<bool,WqaError> {

  Ok(true)
}

pub async fn sample_stop(pump:GearPump) -> Result<bool,WqaError> {

  Ok(true)
}


