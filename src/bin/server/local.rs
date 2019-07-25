use wqa;
use http::status::StatusCode;
use tide::{
    error::{ StringError, ResultExt },
    response, App, Context, EndpointResult,
    querystring::ContextExt
};
use super::app::State;




/// Device get status
pub async fn get_device(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(wqa::store::get_device().await.unwrap()))
}
/// Device get serial number
// pub async fn get_device_serial(_cx: Context<State>) -> EndpointResult {
    // Ok(response::json(wqa::store::get_local().await.unwrap().get_serial()))
// }
/// Divice set serial number
pub async fn set_device(mut cx:Context<State>) -> EndpointResult<()> {
    let device = cx.body_json().await.client_err()?;
    // wqa::store::et_local().await.unwrap().set_serial(serial);
    Ok(())
}
pub async fn get_streams_list(_cx: Context<State>) -> EndpointResult {
    // let app = cx.state();
    match wqa::store::get_stream_list().await {
        Ok(streams) => Ok(response::json (streams)),
        Err(_) => Err(StatusCode::NOT_FOUND.into()),
    }
}
pub async fn get_stream(cx: Context<State>) -> EndpointResult {
    let number = cx.param("number").client_err()?;
    let stream = wqa::store::get_stream(number).await.unwrap();
    Ok(response::json (stream))
}
pub async fn set_stream(mut cx: Context<State>) -> EndpointResult<()> {
    let stream = cx.body_json().await.client_err()?;
    wqa::store::set_stream(stream).await.unwrap();
    Ok(())
}
// pub async fn set_stream_name(_cx: Context<State>) -> EndpointResult<()> {
    // Ok(())
// }

pub async fn get_rules(_cx: Context<State>) -> EndpointResult {
    let rules = wqa::store::set_rules_list().await.unwrap();
    Ok(response::json(rules))
}
/// Get rule from id
///
pub async fn get_rule(cx:Context<State>) -> EndpointResult {
    let id = cx.param("id").client_err()?;
    let rule = wqa::store::get_rule(id).await.unwrap();
    Ok(response::json (rule))
}


/// Set rule
pub async fn set_rule(mut cx: Context<State>) -> EndpointResult<()> {
    let rule = cx.body_json().await.client_err()?;
    wqa::store::set_rule(rule).await.unwrap();
    Ok(())
}

pub fn setup_store(mut app: App<State>) -> App<State> {
    app.at("/api").nest(|api| {
        api.at("/device").get(get_device);
        api.at("/streams").get(get_streams_list);
        api.at("/stream/:number").get(get_stream).post(set_stream);
        api.at("/rules").get(get_rules);
        api.at("/rule/:id").get(get_rule).post(set_rule);
    });
    app
}
//
