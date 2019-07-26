use wqa;
use http::status::StatusCode;
use tide::{
    error::{ ResultExt },
    response, App, Context, EndpointResult,
    // querystring::ContextExt
};
use super::app::State;




/// Device get status
pub async fn device(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(wqa::store::device_get().await.unwrap()))
}
/// Device get serial number
// pub async fn get_device_serial(_cx: Context<State>) -> EndpointResult {
    // Ok(response::json(wqa::store::get_local().await.unwrap().get_serial()))
// }
/// Divice set serial number
// pub async fn device_(mut cx:Context<State>) -> EndpointResult<()> {
    // let device = cx.body_json().await.client_err()?;
    // wqa::store::et_local().await.unwrap().set_serial(serial);
    // Ok(())
// }
pub async fn stream_get_list(_cx: Context<State>) -> EndpointResult {
    // let app = cx.state();
    match wqa::store::stream_get_list().await {
        Ok(streams) => Ok(response::json (streams)),
        Err(_) => Err(StatusCode::NOT_FOUND.into()),
    }
}
pub async fn stream_get(cx: Context<State>) -> EndpointResult {
    let id:u64 = cx.param("id").client_err()?;
    let stream = wqa::store::stream_get_id(id).await.unwrap();
    Ok(response::json (stream))
}
pub async fn stream_save(mut cx: Context<State>) -> EndpointResult<()> {
    let stream = cx.body_json().await.client_err()?;
    wqa::store::stream_save(stream).await.unwrap();
    Ok(())
}
// pub async fn set_stream_name(_cx: Context<State>) -> EndpointResult<()> {
    // Ok(())
// }

pub async fn get_rules(_cx: Context<State>) -> EndpointResult {
    let rules = wqa::store::rule_get_list().await.unwrap();
    Ok(response::json(rules))
}
/// Get rule from id
///
pub async fn get_rule(cx:Context<State>) -> EndpointResult {
    let id = cx.param("id").client_err()?;
    let rule = wqa::store::rule_get_id(id).await.unwrap();
    Ok(response::json (rule))
}


/// Set rule
pub async fn set_rule(mut cx: Context<State>) -> EndpointResult<()> {
    let rule = cx.body_json().await.client_err()?;
    wqa::store::rule_save(rule).await.unwrap();
    Ok(())
}

pub fn setup_route(mut app: App<State>) -> App<State> {
    app.at("/api").nest(|api| {
        api.at("/device").get(device);
        api.at("/streams").get(stream_get_list);
        api.at("/stream/:number").get(stream_get).post(stream_save);
        api.at("/rules").get(get_rules);
        api.at("/rule/:id").get(get_rule).post(set_rule);
    });
    app
}
//
