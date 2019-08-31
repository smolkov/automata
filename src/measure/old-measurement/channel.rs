use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
// use std::ops::{Index, IndexMut};
// use slab::{
//     Slab,
// };
// use slotmap;
// use 






#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Channel {
    pub idx:    u64,
    pub stream: u64,
    pub on:     bool,
    pub name:   String,
    pub unit:   String,
    pub min:    f64,
    pub max:    f64,

}

/// Channel 
impl Channel {
    pub fn new() -> Channel {
        Channel {
            stream: 0,
            on : false,
            name: "CH".to_owned(),
            unit:"mg/l".to_owned(),
            min: 0.0,
            max: 0.0,
        }
    }
}


