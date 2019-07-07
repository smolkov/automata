
use serde::{Deserialize, Serialize};
pub use tide::{error::ResultExt, response, App, Context, EndpointResult};
use super::monitor::*;
use http::status::StatusCode;


pub async fn get_info(mut cx : Context<Repo>) -> Result<String,StatusCode>{
  Ok("none".to_owned())
}
