// use serde_derive::{Deserialize, Serialize};

mod solution;
mod adjustment;
mod channel;
mod stream;
mod measurement;
mod calibration;
mod signal;
mod indicators;
mod statistic;



pub use self::adjustment::*;
pub use self::solution::*;

// pub use self::calibration::*;
// pub use self::measurement::*;
// pub use self::channel::*
// pub use self::stream::*;

// pub struct Prepare;
// pub struct Analysis {
//     pub start: u64,
// }

// pub use solution::{Concentration,Solution};
// pub use signal::{Signal,SignalIter};
// pub use measurement::{Measurement,MeasurementIter};
// pub use calibration::{Adjustment,Calibration,Linear};



