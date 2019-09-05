use serde_derive::{Deserialize, Serialize};
use super::{
    Solution,
    MeasurementIter
};

/// Linear
/// An indicator calibration and adjust
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Linear {
    pub slope : f64,
    pub intercept: f64,
    pub min: f64,
    pub max: f64,
}

/// Polygon calibration
pub type Polygon = Vec<Linear>;

/// Calibration
/// An indicator calibration and adjust
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum  Adjustment{
    None,
    Lineal(Linear),
    Polygon(Polygon),
}


impl Linear {
    pub fn new(slope:f64, intercept:f64) -> Linear {
        Self {
            slope : slope,
            intercept : intercept,
            min: 0.0, 
            max: 500.0,
        } 
    }
}

impl Default for Linear {
    fn default() -> Self {
        Self{
            slope : 1.0,
            intercept : 0.0,
            min: 0.0,
            max: 500.0,
        }
    }
}



#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Point {
    pub sol :Solution,
    pub cal: MeasurementIter,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Calibration {
    pub time: u64,
    pub adjustment: Adjustment,
    pub points: Vec<Point>,
}


// /// Calibration parameter
// #[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
// pub struct CalibrationConfig {
//     pub interval: Duration,
// /// Statistic  replicates,outliers...usw
//     pub statistic: Statistic,
// ///Füllzeit Kalibrierlösung[s]
//     pub filltime: Duration,
// /// Autocalibration 
//     pub autocal: bool,
// /// Autocalibration 
//     pub solution: Vec<Solution>,
// }

// impl Default for CalibrationSetting {
//     fn default()-> Self {
//         Self{
//             interval:  Duration::from_secs(14400),
//             statistic: Statistic::default(),
//             filltime:  Duration::from_secs(10),
//             autocal:   false,
//             solution:  vec![
//                 Solution::new(),
//             ]
//         }
//     }
// }

// #[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
// pub enum CalibrationMutation {
//     Setting(CalibrationSetting),
//     Interval(Duration),
//     Statistic(Statistic),
//     FillTime(Duration),
//     Autocal(bool),
//     Solutions(Vec<Solution>),
//     Solution(usize,Solution),
//     SolutionAdd(Solution),
//     SolutionRemove(usize)
// }