/// Wqa monitoring station state
use serde::{Deserialize, Serialize};
use super::Wqa;





#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Status {
    None,
    Init,
    Wait,
    Wartung,
    Measurement,
    Calibration,
}


#[derive(Clone,Debug)]
pub struct State {
    wqa: Wqa,

}

impl State
{
    pub fn new(wqa:Wqa) -> State {
        State {
            wqa : wqa ,
        }
    }
}



