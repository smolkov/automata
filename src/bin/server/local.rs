use crate::analyzer;
use http::status::StatusCode;
use tide::{
    error::{ StringError, ResultExt },
    response, App, Context, EndpointResult,
    querystring::ContextExt
};
use super::app::State;


/// Device get status
pub async fn get_device(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(analyzer::device::get_local().await.unwrap()))
}
/// Device get serial number
pub async fn get_device_serial(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(analyzer::device::get_local().await.unwrap().get_serial()))
}
/// Divice set serial number
pub async fn set_device_serial(mut cx:Context<State>) -> EndpointResult<()> {
    let serial = cx.body_json().await.client_err()?;
    analyzer::device::get_local().await.unwrap().set_serial(serial);
    Ok(())
}


pub async fn get_streams_list(_cx: Context<State>) -> EndpointResult {
    // let app = cx.state();
    match analyzer::stream::list().await {
        Ok(streams) => Ok(response::json (streams)),
        Err(_) => Err(StatusCode::NOT_FOUND.into()),
    }
}
pub async fn get_stream(cx: Context<State>) -> EndpointResult {
    let number = cx.param("number").client_err()?;
    let stream = analyzer::stream::get_stream(number).await.unwrap();
    Ok(response::json (stream))
}
pub async fn set_stream(mut cx: Context<State>) -> EndpointResult<()> {
    let stream = cx.body_json().await.client_err()?;
    analyzer::stream::set_stream(stream).await.unwrap();
    Ok(())
}
// pub async fn set_stream_name(_cx: Context<State>) -> EndpointResult<()> {
    // Ok(())
// }

pub async fn get_rules(_cx: Context<State>) -> EndpointResult {
    let rules = analyzer::rules::find_all().await.unwrap();
    Ok(response::json(rules))
}

pub async fn set_rule(mut cx: Context<State>) -> EndpointResult<()> {
    let rule = cx.body_json().await.client_err()?;
    analyzer::rules::set_rule(rule).await.unwrap();
    Ok(())
}


//
