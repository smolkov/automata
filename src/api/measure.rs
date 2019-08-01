use crate::{
    // error::*,
    measure::{
        store,
        stream::Stream,
        // channel::Channel,
    }
};


// use http::status::StatusCode;
use tide::{
    error::{ ResultExt },
    response, App, Context, EndpointResult,
    // querystring::ContextExt
};
use super::app::State;




/// Device get status
pub async fn response_device(_cx: Context<State>) -> EndpointResult {
    let device = store::get_device().await.unwrap();
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
pub async fn response_stream_list(_cx: Context<State>) -> EndpointResult {
    // let app = cx.state();
    let list = store::get_streams().await.unwrap();
    Ok(response::json (list))
}

pub async fn response_stream(cx: Context<State>) -> EndpointResult {
    let stream:u64 = cx.param("stream").client_err()?;
    let stream = store::get_stream_from_id(stream).await.unwrap();
    Ok(response::json(stream))
}

pub async fn post_stream(mut cx: Context<State>) -> EndpointResult<()> {
    let stream:Stream = cx.body_json().await.client_err()?;
    store::set_stream(&stream).await.unwrap();
    Ok(())
}


/// Channel settings api
pub async fn response_stream_channels(cx: Context<State>) -> EndpointResult {
    let stream:u64 = cx.param("stream").client_err()?;
    // let app = cx.state();
    let list = store::get_stream_channels(stream).await.unwrap();
    Ok(response::json(list))
}
// async fn response_stream_channel(cx: Context<State>) -> EndpointResult {
//     let stream:u64 = cx.param("stream").client_err()?;
//     let channel:u64 = cx.param("channel").client_err()?;
//     let channel = store::get_channel_from_ids(stream,channel).await.unwrap();
//     Ok(list)
// set_stream
// pub async fn set_stream_name(_cx: Context<State>) -> EndpointResult<()> {
    // Ok(())
// }
// async fn post_stream_channel(cx: Context<State>) -> EndpointResult {
    // let stream:u64 = cx.param("stream").client_err()?;
    // let channel:u64 = cx.param("channel").client_err()?;

/// set_stream
// pub async fn set_stream_name(_cx: Context<State>) -> EndpointResult<()> {
    // Ok(())
// }
pub async fn response_rules(_cx: Context<State>) -> EndpointResult {
    let rules = store::get_rules().await.unwrap();
    Ok(response::json(rules))
}
/// Get rule from id
///
pub async fn response_rule(cx:Context<State>) -> EndpointResult {
    let rule:u64 = cx.param("rule").client_err()?;
    let rule = store::get_rule_from_id(rule).await.unwrap();
    Ok(response::json (rule))
}


/// Set rule
pub async fn post_rule(mut cx: Context<State>) -> EndpointResult<()> {
    let rule = cx.body_json().await.client_err()?;
    store::set_rule(rule).await.unwrap();
    Ok(())
}

// Setup route
// pub fn setup_route(mut app: App<State>) -> App<State> {
//     app.at("/api").nest(|api| {
//         api.at("/device").get(response_device);
//         api.at("/streams").get(response_stream_list).post(post_stream);
//         api.at("/:stream").get(response_stream);
//         api.at("/:stream/channels").get(response_stream_channels);
//         // api.at("/:stream/:channel").get(response_stream_channel).post(post_stream_channel);
//         api.at("/rules").get(response_rules);
//         api.at("/:rule").get(response_rule).post(post_rule);
//     });
//     app
// }
