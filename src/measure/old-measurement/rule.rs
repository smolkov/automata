use serde_derive::{Deserialize, Serialize};
use std::time::{Duration,SystemTime};
/// Channel sum parameter
///
// use std::{path::PathBuf,fmt};

pub struct Procedure {
    pub statistic : Statistic,
    pub interval: u64,
    pub remote: bool,
    pub online: bool,
}

impl Default for Procedure{
    fn default() -> Self {
        Self {
            statistic: Statistic::default(),
            interval: 1800,
            remote: false,
            online: false,
        }
    }
}

pub enum Process {

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Stream {
    pub number: u32,
    pub idx: u64,
    pub name: String,
    pub description: String,
    pub activated: bool,
    pub delay: u64,
}


impl Default for Stream{
    fn default() -> Self {
        Self {
            number: 0,
            name: "unknown",
            description: "Default stream",
            activated: false,
            delay: 2,
            single: Procedure::default(),
            measurement: Procedure::default(),
            check: Procedure::default(),
            calibration: Procedure::default(),
        }
    }
}



impl Stream {
    pub fn get_number(&self) -> u32 {
        self.number
    }
}