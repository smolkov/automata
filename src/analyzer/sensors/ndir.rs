/// Sensors: NDir1, NDir2
/// Anschlus:  `DoppelMotor::UART01`
/// Model:     `presurei877`

use failure::{Fail};
use std::io;
use crate::{
    // WqaError,
    systime,
};
use super::Record;


use lazy_static::lazy_static;
use std::sync::RwLock;
lazy_static! {
    static ref NDIR1: RwLock<Sensor> = {
        RwLock::new(Sensor::new())
    };
    static ref NDIR1: 
}







// use std::string::FromUtf8Error;

// use mut_guard::*;

#[derive(Fail, Debug)]
pub enum SensorError {
    #[fail(display = "io error - {}",err)]
    IOError {err: io::Error },

}

impl From<io::Error> for SensorError {
    fn from(kind:io::Error) -> SensorError {
        SensorError::IOError{err: kind}
    }
}

struct NdirSensor {
    updated: u64,
    fsr : f64,
    rec: Record,
    model: NdirModel,
}


impl Sensor {
    fn new() -> Sensor {
        Sensor {
            updated:0,
            fsr:0.0,
            rec:Record::default(),
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
        self.updated = systime::now_sec();
        self.rec.set_value(self.fsr);
    }
    pub fn get_fsr(&self) -> f64 {
        self.fsr
    }
    pub fn get_updated(&self) -> u64 {
        self.updated
    }
}
//

pub async fn ndir1_set_fsr(value:f64){
    NDIR1.write().unwrap().set_fsr(value)
}

pub async fn ndir1_get_fsr() -> f64  {
    NDIR1.read().unwrap().get_fsr()
}

pub async fn ndir1_start_record() {
    NDIR1.write().unwrap().start_record()
}
pub async fn ndir1_stop_record() {
    NDIR1.write().unwrap().stop_record()
}

pub async fn ndir1_get_updated() -> u64  {
    NDIR1.read().unwrap().get_updated()
}


