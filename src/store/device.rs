use serde::{Deserialize, Serialize};
// use super::Hid;
use crate::Result;
use settings::ron::Config;
// use http::status::StatusCode;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Draft,
    QuickTOCxy,
    QuickTOCuv,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Status {
    None,
    Init,
    Wait,
    Wartung,
    Measurement,
    Calibration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    pub id: u64,
    pub model: Model,
    pub producted: u64,
    pub status: Status,
    pub updated: u64,
}

impl Device {
    pub fn get_id(&self) -> u64 {
        self.id
    }
    // pub fn set_serial(&mut self,id:Hid) {
        // self.id = id;
    // }
    pub fn new(model: Model) -> Device {
        Device {
            model: model,
            id: 0,
            producted: 0,
            status: Status::None,
            updated:0,
        }
    }
    pub fn set_status(&mut self, status: Status) {
        self.status  = status;
    }
}

impl Default for Device {
    fn default() -> Self {
        Device::new(Model::Draft)
    }
}



// pub async fn change_status(status:Status) -> Result<(),WqaError> {
    // let mut device = get_local().await?;
    // device.set_status(status);
    // save_local(device)
// }



pub async fn read() -> Result<Device> {
    let device = Device::load_no_fallback(super::data_dir()?.join("device.ron"))?;
    Ok(device)
}

pub async fn save(device: &Device) -> Result<()> {
    device.write(super::data_dir()?.join("device.ron"))?;
    Ok(())
}
