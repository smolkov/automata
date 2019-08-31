use lazy_static::lazy_static;
use async_std::io;
use async_std::prelude::*;

/// Monitor hardware caontrol system.
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration,SystemTime};
use log::{info,warn};
use lazy_static::lazy_static;
use crate::automataError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]



lazy_static!{
      static ref TOCUV: RwLock<TocUv> = RwLock::new(Lamp::default());
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lamp {
    pub uptime: SystemTime,
    pub lifetime: Duration,
    pub on: bool,
}


impl Default for Lamp {
    fn default() -> Self {
        Self {
            uptime:SystemTime::now(),
            on: false,
            lifetime: Duration::from_secs(1),
        }
    }
}

impl Lamp {

    pub fn turn_on( &mut self) {
        self.on     = true;
        self.uptime = SystemTime::now();
    }
    pub fn turn_off(&mut self) {
        self.update_lifitime();
        self.on     = false;
    }
    pub fn update_lifitime(&mut self){
        let now     = SystemTime::now();
        match now.duration_since(self.uptime) {
            Ok(uptime) if self.on => {
                self.lifetime += uptime;
                self.uptime    = SystemTime::now();
            },
            Ok(_)  => info!("UV Lamp turn off"),
            Err(e) => warn!("UV Lamp uptime:{:}",e),
        }
    }
    pub fn setup(&mut self,lifetime: u64) {
        self.lifetime = Duration::from_secs(lifetime);
    }
}


pub struct Lamp {
    pub uptime: SystemTime,
    pub lifetime: Duration,
    pub on: bool,
}


impl Default for Lamp {
    fn default() -> Self {
        Self {
            uptime:SystemTime::now(),
            on: false,
            lifetime: Duration::from_secs(1),
        }
    }
}




pub struct Closed;
pub struct Sample1;
pub struct Sample2;
pub struct Sample3;
pub struct Sample4;
pub struct Sample5;
pub struct Sample6;
pub struct Calibration;


pub struct UvSampler {

}



pub struct UvState {
    pub lamp: Lamp,
    pub pump : Pump,
    pub sample_valve:[bool;6]
    pub bypass: bool,
    pub zeroflow: bool
    pub fluid: bool,
    pub tic: bool,

}

pub struct Uv {
    node: Node,
}



impl TocUv {
    pub fn new(path : &str) -> QuickTocUv {
        QuickTocUv {
            lamp: Lamp::default(),
            sampler:Sampler::default(),
            bypass: false,
            zeroflow: false,
            fluid: false,
            tic: false,
        }
    }
}



pub async fn read(uv: &Uv) -> Result<UvState> {
   let path = sensor.node.path().join("state.ron");
    task::block_on(async {
        let mut file = File::open(&path).await?;
        let mut buffer:Vec<u8> = Vec::new();
        // Read a buffer from the file.
        let _n = file.read(&mut buffer).await?;
        let config: Config = from_bytes(buffer.as_slice())?;

        Ok(config)
    })
}





/// Sample pump start stop.
pub async fn sample_pump(start: bool) -> Result<()> {
    io::set_digital1_output(2,start).await
}
/// Open measurement sample N
pub async fn open_sample_valve(num:u8)  -> Result<()> {
    io::set_digital1_output(num,true).await
}
pub async fn close_sample_valve(num:u8)  -> Result<()> {
    io::set_digital1_output(num,true).await
}

pub async fn open_zeroflow_valve() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn close_zeroflow_valve() -> Result<()> {
    io::set_digital1_output(4,false).await
}
pub async fn open_tic_valve() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn close_tic_valve() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn open_calibration_valve() -> Result<()> {
    io::set_digital1_output(4,true).await
}
pub async fn close_calibration_valve( ) -> Result<()> {
    io::set_digital1_output(4,false).await
}

pub async fn is_fluid(_sample:u8) -> Result<bool> {
    io::get_digital2_input(2).await
}

