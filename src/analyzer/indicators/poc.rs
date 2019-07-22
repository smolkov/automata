/// Suspended Organic Carbon – also called particulate organic carbon (POC); the carbon in particulate form that is too large to pass through a filter.

use serde_derive::{Deserialize, Serialize};
// use super::channel::{Channel};
use std::time::{SystemTime};




 #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct POC{
    value: f64,
}