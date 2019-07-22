pub mod ndir;
pub use super::local as store;

use serde_derive::{Deserialize, Serialize};
use crate::systime;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RecState {
    Stop,
    Record,
    Done,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Record {
    pub start: u64,
    pub last: u64,
    pub signal: Vec<f64>,
    pub sigstat: Vec<u8>,
    pub state:RecState,
}

impl Default for Record {
    fn default() -> Self {
        Self {
            start: 0,
            last: 0,
            signal:Vec::new(),
            sigstat:Vec::new(),
            state: RecState::Stop,
        }
    }
}
impl Record {
    pub fn set_value(&mut self, value: f64) {
        if self.state == RecState::Record {
            self.signal.push(value);
            self.last = systime::now_sec();
        }
    }
    pub fn start(&mut self) {
        self.start = systime::now_sec();
        self.signal = Vec::new();
        self.sigstat = Vec::new();
        self.state = RecState::Record;
    }
    pub fn stop(&mut self) {
         self.state = match self.state {
            RecState::Record => RecState::Done,
            _ => RecState::Stop
        }
    }
}

