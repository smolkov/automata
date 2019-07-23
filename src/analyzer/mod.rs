// pub mod calibration;
pub mod solution;
pub mod adjustment;
pub mod integration;
pub mod statistic;
pub mod stream;
pub mod device;
pub mod rules;
pub mod sensors;
pub mod monitoring;
pub mod measurement;
pub mod calibration;

pub use device::{
    Model,
    Device,
};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub device: Option<Device>,
}

impl Default for State {
    fn default()->Self {
        Self {
            // status: None,
            device: None,
        }
    }
}
