use crate::{
    error::*,
    stream::*,
    store,
};

use http::status::StatusCode;
use tide::{Error,
    error::{ ResultExt },
    response, App, Context, EndpointResult,
    // querystring::ContextExt
};
use super::app::State;




/// Device get status
pub async fn device_get_info(_cx: Context<State>) -> EndpointResult {
    let device = store::device_get().await?;
    Ok(response::json(device))
}
/// Device get serial number
// pub async fn get_device_serial(_cx: Context<State>) -> EndpointResult {
    // Ok(response::json(store::get_local().await.unwrap().get_serial()))
// }
/// Divice set serial number
// pub async fn device_(mut cx:Context<State>) -> EndpointResult<()> {
    // let device = cx.body_json().await.client_err()?;
    // store::et_local().await.unwrap().set_serial(serial);
    // Ok(())
// }
async fn stream_get_list(_cx: Context<State>) -> EndpointResult {
    // let app = cx.state();
    let list = store::stream_get_list().await?;
    Ok(response::json (list))
}

async fn get_stream(cx: Context<State>) -> EndpointResult {
    let stream:u64 = cx.param("stream").client_err()?;
    let stream = store::stream_get_from_id(stream).await?;
    Ok(response::json(stream))
}

async fn set_stream(mut cx: Context<State>) -> EndpointResult<()> {
    let stream:Stream = cx.body_json().await.client_err()?;
    store::stream_save(&stream).await?;
    Ok(())
}


/// Channel settings api
async fn stream_channel_get_list(_cx: Context<State>) -> EndpointResult {
    // let app = cx.state();
    let list = store::stream__list().await?;
    Ok(response::json(list))
}
/// set_stream
// pub async fn set_stream_name(_cx: Context<State>) -> EndpointResult<()> {
    // Ok(())
// }

pub async fn get_rules(_cx: Context<State>) -> EndpointResult {
    let rules = store::rule_get_list().await.unwrap();
    Ok(response::json(rules))
}
/// Get rule from id
///
pub async fn get_rule(cx:Context<State>) -> EndpointResult {
    let stream:u64 = cx.param("rule").client_err()?;
    let rule = store::rule_get_id(id).await.unwrap();
    Ok(response::json (rule))
}


/// Set rule
pub async fn set_rule(mut cx: Context<State>) -> EndpointResult<()> {
    let rule = cx.body_json().await.client_err()?;
    store::rule_save(rule).await.unwrap();
    Ok(())
}
/// Setup route
pub fn setup_route(mut app: App<State>) -> App<State> {
    app.at("/api").nest(|api| {
        api.at("/device").get(device);
        api.at("/streams").get(stream_get_list).post(set_stream);
        api.at("/:stream").get(get_stream);
        api.at("/:stream/channels").get(get_channels_list)
        api.at("/:stream/:channel").get(get_channel);
        api.at("/rules").get(get_rules);
        api.at("/:rule").get(get_rule).post(set_rule);
    });
    app
}
