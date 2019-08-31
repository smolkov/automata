/// Ultra configuration file 
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Airflow {
    pub injection_treshold :f64,
    pub volume_deviation: f64,
    pub volume_level:f64,
}


use serde_derive::{Deserialize, Serialize};




pub struct UltraConfig {
    
}