/// Pressure sensor
/// Anschlus `Analog:IN04`
///
///

// use crate::io;
use std::ops::Range;
use crate::{
    WqaError,
    systime
};

use tempfile::{
    tempdir_in,
    TempDir,
};
use serde_derive::{Deserialize, Serialize};

use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref IN : RwLock<Airflow> = {
        RwLock::new(Airflow::from_analog16(0))
    };
    static ref OUT : RwLock<Airflow> = {
        RwLock::new(Airflow::from_analog16(0))
    };
}


// static max:f64 = 0.5;
// static
// static max:f64 = 0.4;

/// für 0..60:   0.230197;
static A6:f32 = 0.003836617;
/// für 0..60:  -3.616438;
static A5:f32 = -0.06027397;
/// für 0..60:  22.36370;
static A4:f32 = 0.3727283;
/// für 0..60: -68.58285;
static A3:f32 = -1.1430475;
/// für 0..60: 110.3052;
static A2:f32 = 1.83842;
/// für 0..60: -84.19201;
static A1:f32 = -1.4032;
/// für 0..60:  23.49542;
static A0:f32 = 0.39159;


pub type FlowRange = Range<f32>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Airflow {
    pub updated: u64,
    pub fsr :    f32,
    pub broken:  bool,
}

impl Airflow {
    pub fn from_analog16(value:u16) -> Airflow{
        let mut signal = value as f32 / 4095.0 * 5.0;
        let broken = signal> 1.0;
        signal = (((((A6*signal +A5)*signal +A4) *signal +A3) *signal + A2) * signal + A1) *signal + A0;
        Airflow {
            updated:systime::now_sec(),
            fsr: signal,
            broken: broken,
        }
    }
    pub fn set_analog16(&mut self,value:u16) {
        let mut signal = value as f32 / 4095.0 * 5.0;
        let broken = signal> 1.0;
        signal = (((((A6*signal +A5)*signal +A4) *signal +A3) *signal + A2) * signal + A1) *signal + A0;
        self.fsr = signal;
        self.broken = broken;
        self.updated = systime::now_sec();
    }
}

// pub async fn warning_input() -> Option<Range> {
    // None
// }
// pub async fn critical_range_input() -> Option<Range> {
    // None
// }

// pub async fn setup_range_input(warn: Option<Range>, critical : Option<Range>) {

// }



pub async fn setup_range_output() {

}


pub async fn set_in_flow(value:u16) {
    IN.write().unwrap().set_analog16(value);
}
pub async fn get_in_flow() -> Airflow {
    IN.read().unwrap().clone()
}

pub async fn set_out_flow(value:u16) {
    OUT.write().unwrap().set_analog16(value);
}
pub async fn get_out_flow() -> Airflow {
    OUT.read().unwrap().clone()
}



// pub async fn airflow_input_value(input_read: ReadAnalogu16) -> Result<Airflow,MioError>{
//     let analog_value = input_read.await?;
//     Ok(Airflow::from_analog16(analog_value))
// }
// pub async fn airflow_output_value(output_read:ReadAnalogu16) -> Result<Airflow,MioError>{
//     let analog_value = output_read.await?;
//     Ok(Airflow::from_analog16(analog_value))
// }
