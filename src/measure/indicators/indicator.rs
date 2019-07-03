use serde_derive::{Deserialize, Serialize};



#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Indicator{
   pub value : f64,
   pub time: u64,
}


