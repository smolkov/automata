// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::workspace as store;
// use http::status::StatusCode;

use crate::error::*;
use log::info;
// use lazy_static::lazy_static;
// use std::sync::{RwLock};
use wqa_settings::ron::Config;
use crate::systime;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Model {
    Draft,
    QuickTOCxy,
    QuickTOCuv,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Serial {
    pub id : String
}

impl Default for Serial {
    fn default() -> Serial {
        Serial{
            id: "dev".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Status {
    Off,
    Init,
    Stop,
    Wartung
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Device {
    pub model: Model,
    pub serial: String,
    pub producted: u64,
    pub status: Status,
    pub updated: u64,
}

impl Device {
    pub fn get_serial(&self) -> String {
        self.serial.clone()
    }
    pub fn set_serial(&mut self,serial:String) {
        self.serial = serial;
    }
    pub fn new(model: Model) -> Device {
        Device {
            model: model,
            serial: "dev".to_owned(),
            producted: 0,
            status: Status::Off,
            updated:0,
        }
    }
    pub fn set_status(&mut self, status: Status) {
        info!("status changed {:?} {:?}",self.status,status);
        self.status  = status;
        self.updated = systime::now_sec();
    }
}

impl Default for Device {
    fn default() -> Self {
        Device::new(Model::Draft)
    }
}


pub async fn get_local() -> Result<Device> {
    let device = Device::load_no_fallback(store::data_dir()?.join("device.ron"))?;
    Ok(device)
}

pub async fn save_local(device: Device) -> Result<()> {
    device.write(store::data_dir()?.join("device.ron"))?;
    Ok(())
}

pub async fn set_serial( serial: String ) -> Result<()> {
    let mut device = get_local().await?;
    device.set_serial(serial);
    device.write(store::data_dir()?.join("device.ron"))?;
    Ok(())
}

// pub async fn change_status(status:Status) -> Result<(),WqaError> {
    // let mut device = get_local().await?;
    // device.set_status(status);
    // save_local(device)
// }
