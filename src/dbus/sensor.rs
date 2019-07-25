/// Sensors: NDir1, NDir2
/// Anschlus:  `DoppelMotor::UART01`
/// Model:     `presurei877`
use serde_derive::{Deserialize, Serialize};

use crate::error::*;

/// Sensor model
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum NdirModel {
    Simulation,
    Edinburgh,
}

/// Record state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum RecState {
    Stop,
    Record,
    Done,
}

/// Record signal
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
            // self.last = systime::now_sec();
        }
    }
    pub fn start(&mut self) {
        // self.start = systime::now_sec();
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

/// Ndir sensor.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Sensor {
    updated: u64,
    fsr : f64,
    rec: Record,
    model: NdirModel,
}

/// FSR.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FSR {
    pub value:f64,
    pub brocked: bool,
}

impl FSR {
    pub fn new(value: f64) -> FSR {
        FSR{
            brocked: false,
            value: value,
        }
    }
}

impl Sensor {
    fn new() -> Sensor {
        Sensor {
            updated:0,
            fsr:0.0,
            rec:Record::default(),
            model: NdirModel::Simulation,
        }
    }
    pub fn start_record(&mut self) {
        self.rec.start();
    }
    pub fn stop_record(&mut self) {
        self.rec.stop();
    }
    pub fn set_fsr(&mut self,value:f64) {
        self.fsr = value;
        self.rec.set_value(self.fsr);
    }
    pub fn get_fsr(&self) -> f64 {
        self.fsr
    }
    pub fn get_updated(&self) -> u64 {
        self.updated
    }
}


pub async fn get_ndir1_value() -> Result<f64> {
    Ok(0.0)
}
pub async fn get_ndir2_value() -> Result<f64> {
    Ok(0.0)
}
