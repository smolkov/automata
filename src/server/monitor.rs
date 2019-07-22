pub use tide::{error::ResultExt, response, App, Context, EndpointResult};
use super::app::Repo;


// Device status
// pub async fn device_status(_cx: Context<AppState>) -> EndpointResult {
// Ok(response::json(DEVICE.read().unwrap().clone()))
// }
// Device serial number
// pub async fn device_get_serial(_cx: Context<AppState>) -> EndpointResult {
// Ok(response::json(DEVICE.read().unwrap().get_serial()))
// }
//
// pub async fn device_set_serial(_cx:Context<AppState>) -> EndpointResult<()> {
// let serial = cx.body_json().await.client_err()?;
// Ok(())
// }
