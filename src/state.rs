/// Wqa monitoring station state
use super::node::Node;
use failure::Fallible;

use settings::ron::Config;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Status {
    Unimplemented,
    Init,
    Wait,
    Wartung,
    Measurement,
    Calibration,
}

impl Default for Status {
    fn default() -> Status {
        Status::Unimplemented
    }
}

pub struct State{}

pub async fn status(node:Node<State>) -> Fallible<Status> {
    let status = Status::load_no_fallback(node.path())?;
    Ok(status)
}




//
// }

// #[derive(Clone,Debug)]
// pub struct State {
    // repo: WqRepo,

// }


// impl State
// {
    // pub fn new(wqa:WqRepo) -> State {
        // State {
            // repo : wqa ,
        // }
    // }
// }





