/// Monitor gear pump normally used for solution sampling.
///
use serde_derive::{Deserialize, Serialize};
use std::sync::RwLock;
use crate::WqmError;
// use lpcan::can::{
// can0,
// Message,
// };

pub enum State {
    Runned,
    Stop,
}



use lazy_static::lazy_static;

lazy_static! {
    static ref GP: RwLock<GearPump> = RwLock::new(GearPump::new());
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GearPump {
    pub run: bool,
}

impl GearPump {
    pub fn new() -> GearPump {
        GearPump {
            run: false,
        }
    }
    pub fn start(&mut self) {
        self.run = true;
    }
    pub fn stop(&mut self) {
        self.run = false;
    }
}

pub struct Pumps {
    gp1: GearPump,
    gp2: GearPump,
    gp3: GearPump,
    gp4: GearPump,
    gp5: GearPump,
    gp6: GearPump,
}


pub async fn sample_start(pump:GearPump) -> Result<bool,WqmError> {

  Ok(true)
}

pub async fn sample_start_timeout(pump:GearPump,timeout:u64) -> Result<bool,WqmError> {

  Ok(true)
}

pub async fn sample_stop(pump:GearPump) -> Result<bool,WqmError> {

  Ok(true)
}
