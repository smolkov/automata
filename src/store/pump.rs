/// Monitor gear pump normally used for solution sampling.
///
use serde_derive::{Deserialize, Serialize};
use std::sync::RwLock;
use failure::Fallible;
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
pub struct Pump {
    id: u64
}



#[derive(Serialize,Default, Deserialize, Clone, Debug, PartialEq)]
pub struct GearPump {
    pump: Pump,
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
pub async fn directory(pump:Pump) -> Result<> {
    let path = rootdir()?.join(format!("/pump:{}/",pump.id));
    Ok(path)
}



pub async fn start(pump:Pump) -> Result<()> {
    Ok(())
}
pub async fn stop(pump:Pump) -> Result<()> {
    Ok(())
}


