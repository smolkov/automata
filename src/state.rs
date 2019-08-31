/// automata monitoring station state
// use super::node::Node;
// use failure::Fallible;
use std::path::PathBuf;
// use store::ron::Config;
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

pub struct State {
    repo : PathBuf,
    status : Status,
    // controller: Controller,
}

// pub async fn status(node:Node<State>) -> Fallible<Status> {
    // let status = Status::load_no_fallback(node.path())?;
    // Ok(status)
// }


// pub struct device()


// pub async fn generate(state:State) {

// }


//
// }

// #[derive(Clone,Debug)]
// pub struct State {
    // repo: WqRepo,

// }


// impl State
// {
    // pub fn new(automata:WqRepo) -> State {
        // State {
            // repo : automata ,
        // }
    // }
// }





