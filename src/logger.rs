use log;
use femme;
use log::info;

pub fn setup() {
    info!("setup logger");
    femme::start(log::LevelFilter::Debug)?;

}




pub async fn message(mut cx: Context<()>) -> EndpointResult<()> {
  let log = await!(cx.body_string()).client_err()?;
  let parsed: Value = serde_json::from_str(&log).unwrap_or(json!({}));
  if parsed["upstream_uri"] != json!("/metrics") {
     println!("{}", log);
  }
  Ok(())
}

// pub async fn warning(mut cx: Context<()>) -> EndpointResult<()> {
//   let log = await!(cx.body_string()).client_err()?;
//   let parsed: Value = serde_json::from_str(&log).unwrap_or(json!({}));
//   if parsed["upstream_uri"] != json!("/metrics") {
    //  println!("{}", log);
//   }
//   Ok(())
// }
