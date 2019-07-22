// pub mod calibration;
pub mod local;
pub mod solution;
pub mod adjustment;
pub mod integration;
pub mod statistic;
pub mod stream;
pub mod channel;
pub mod device;
pub mod status;
pub mod rules;
pub mod sensors;
pub mod monitoring;
pub use crate::WqaError;
pub use local as store;

pub use device::{
    Model,
    Device,
};

pub use status::{
    Status
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub status: Option<Status>,
    pub device: Option<Device>,
}

impl Default for State {
    fn default()->Self {
        Self {
            status: None,
            device: None,
        }
    }
}

pub async fn state() -> Result<State,WqaError> {
    Ok(State::default())
}
pub async fn setup(model: device::Model) -> Result<(),WqaError> {
    Ok(())
}

//
