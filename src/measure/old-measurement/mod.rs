// use serde_derive::{Deserialize, Serialize};

mod solution;
mod measurement;
mod calibration;
mod signal;

pub use solution::{Concentration,Solution};
pub use signal::{Signal,SignalIter};
pub use measurement::{Measurement,MeasurementIter};
pub use calibration::{Adjustment,Calibration,Linear};

