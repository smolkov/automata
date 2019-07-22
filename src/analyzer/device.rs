// use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::store;
// use http::status::StatusCode;

use super::WqaError;
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

#[derive(Serialize, Deserialize)]
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
        device.status = status;
        self.updated = systime::now_sec();
    }
}

impl Default for Device {
    fn default() -> Self {
        Self {
            model: Model::Draft,
            serial: "draft".to_owned(),
            producted:0,
        }
    }
}


pub async fn get_local() -> Result<Device,WqaError> {
    let device = Device::load_no_fallback(store::root_directory().join("device.ron"))?;
    Ok(device)
}
pub async fn save_local(device: Device) -> Result<(), WqaError> {
    device.write(store::root_directory().join("device.ron"))?;
    Ok(())
}
// pub async fn from_git() -> Result<Device,WqaError> {
//     Ok(Device::default())
// }

pub async fn set_serial( serial: String ) -> Result<(),WqaError> {
    let mut device = get_local().await?;
    device.set_serial(serial);
    device.write(store::root_directory().join("device.ron"))?;
    Ok(())
}


pub async fn change_status(status:Status) -> Result<(),WqaError>{
    let mut device = get_local().await?;
    device.set_status(status);
    save_local(device)
}


//
