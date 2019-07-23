pub mod airflow;
pub mod humidity;
pub mod pressure;
pub mod temperatur;

use serde_derive::{Deserialize, Serialize};
use crate::systime;
use crate::error::*;
//
pub use humidity::Humidity;
pub use airflow::Airflow;
pub use pressure::Pressure;




/// Humidity api
pub async fn humidity() -> Result<Humidity> {
    Ok(Humidity::from_analog16(0))
}
pub async fn airflow_input()  {
}
pub async fn airflow_output() {
}
pub async fn pressure() -> Result<Pressure> {
    Ok(Pressure::from_analog16(0))
}
//
