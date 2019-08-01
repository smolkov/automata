use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration};
// use super::common::*;
use analyzer::Statistic;
use chrono::Utc;
// use super::stream::*;
// use crate::error::*;
// use super::store;
use super::channel::Channel;
use super::stream::Stream;

use lazy_static::lazy_static;
// use std::sync::RwLock;


lazy_static! {
    // static ref MEASUREMENTS:u64 = 0;
    // static ref EVALUATION_PROCESS: Evaluation
}

pub trait Evaluation {
    fn calc_statistic(&mut self, statistic:&Statistic) -> bool;
    fn get_acceptable(&self) -> bool;
    fn get_replication(&self) -> u8;
    fn get_cv(&self) -> f64;
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeasurementResult {
    pub timestamp: u64,
    pub value: u64,
    pub measure: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeasureStats {
    pub counter: u64,
}

impl Default for MeasureStats {
    fn default() -> Self {
        Self {
            counter: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnalyseResult {
    pub timestamp:   u64,
    pub value:       f64,
    pub counter:     u64,
    pub outlier:     bool,
}


impl AnalyseResult {
    pub fn new (counter:u64) -> Self{
        Self{
            timestamp:     Utc::now().timestamp() as u64,
            value:         0.0,
            counter:       counter,
            outlier:       false,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChannelResult {
    pub channel:    Channel,
    pub count:      u64,
    pub value:      f64,
}

impl ChannelResult {
    pub fn new(channel: Channel) -> Self {
       Self{
           channel: channel,
           value:   0.0,
           count:   0,
       }
    }
    // pub fn get_result(&self) -> Option<MeasurementResult> {
        // if !self.done {
            // None
        // }
        // MeasurementResult{
        //     timestamp:Utc::now().timestamp() as u64,
        //     value: self.result,
        //     measure: self.count,

        // }
    // }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StreamResult {
    pub done:     bool,
    pub stream:   Stream,
    pub count:    u64,
    pub channels: Vec<ChannelResult>,
}



impl Evaluation for StreamResult {
    fn calc_statistic(&mut self, statistic: &Statistic) -> bool {
        // for evaluate in self.channels.iter_mut() {
            // evaluate.atistic_calculation(statistic);
            // if !evaluate.acceptable() {
                // self.done = false;
            // }
        // }
        self.done
    }
    fn get_acceptable(&self) -> bool {
        self.done
    }
    fn get_replication(&self) -> u8 {
        let mut replicates = 0;
        // for ch in self.channels {
            // if ch.get_replication() > replicates {
                // replicates = ch.get_replication();
            // }
        // }
        replicates
    }
    fn get_cv(&self) -> f64 {
        let mut maxcv = 0.0;
        // for ch in self.channels {
            // if ch.get_cv() > maxcv{
                // maxcv= ch.get_cv();
            // }
        // }
        maxcv
    }
}


//
