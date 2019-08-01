use crate::mio;
use tide::{
    error::{ ResultExt },
    response, App, Context, EndpointResult,
};

use super::app::State;
use super::sensor;
use super::measure::*;

/// Endpoint:
///   POST /api/uv/sample/start
async fn handle_start_sample(_cx: Context<State>) -> EndpointResult<()> {
    mio::uv::sample_pump(true).await.unwrap();
    Ok(())
}
/// Endpoint:
///   POST /api/uv/sample/start
async fn handle_stop_sample(_cx: Context<State>) -> EndpointResult<()> {
    mio::uv::sample_pump(false).await.unwrap();
    Ok(())
}
/// Endpoint:
///   POST /api/uv/sample/start
async fn handle_open_sample_valve(cx: Context<State>) -> EndpointResult<()> {
    let num = cx.param(":num").client_err()?;
    mio::uv::open_sample_valve(num).await.unwrap();
    Ok(())
}
async fn handle_close_sample_valve(cx: Context<State>) -> EndpointResult<()> {
    let num= cx.param(":num").client_err()?;
    mio::uv::open_sample_valve(num).await.unwrap();
    Ok(())
}
async fn open_zeroflow_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::open_zeroflow_valve().await.unwrap();
    Ok(())
}
async fn close_zeroflow_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::close_zeroflow_valve().await.unwrap();
    Ok(())
}
async fn open_tic_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::open_tic_valve().await.unwrap();
    Ok(())
}
async fn close_tic_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::close_tic_valve().await.unwrap();
    Ok(())
}
async fn open_calibration_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::open_calibration_valve().await.unwrap();
    Ok(())
}
async fn close_calibration_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::close_calibration_valve().await.unwrap();
    Ok(())
}
async fn response_ndir1_value(_cx: Context<State>) -> EndpointResult {
    let val = mio::uv::get_ndir1_value().await.unwrap();
    Ok(response::json(val))
}
async fn response_ndir2_value(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::uv::get_ndir2_value().await.unwrap()))
}


/// Uv io state return all hardware parameter.
async fn get_uv_state(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(0))
}


async fn get_airflow_input(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::flow::get_airflow_input().await.unwrap()))
}

/// get airflow current value on in
async fn get_airflow_output(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::flow::get_airflow_input().await.unwrap()))
}

async fn get_humidity(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::flow::get_humidity().await.unwrap()))
}

async fn get_pressure(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::flow::get_humidity().await.unwrap()))
}

async fn handle_can_frame(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::flow::get_humidity().await.unwrap()))
}

pub fn setup_routes(mut app: App<State>) -> App<State> {

    app.at("/api/uv").nest(|api| {
        api.at("/state").get(get_uv_state);
        api.at("/sample/start").post(handle_start_sample);
        api.at("/sample/stop").post(handle_stop_sample);
        api.at("/valve/:num/open").post(handle_open_sample_valve);
        api.at("/valve/:num/close").post(handle_close_sample_valve);
        api.at("/valve/zeroflow/open").post(open_zeroflow_valve);
        api.at("/valve/zeroflow/close").post(close_zeroflow_valve);
        api.at("/valve/tic/open").post(open_tic_valve);
        api.at("/valve/tic/close").post(close_tic_valve);
        api.at("/valve/calibration/open").post(open_calibration_valve);
        api.at("/valve/calibration/close").post(close_calibration_valve);
        api.at("/ndir1").get(response_ndir1_value);
        api.at("/ndir2").get(response_ndir2_value);
        api.at("/airflow/input").get(get_airflow_input);
        api.at("/airflow/output").get(get_airflow_output);
        api.at("/humidity").get(get_humidity);
        api.at("/pressure").get(get_pressure);
        api.at("/device").get(response_device);
        api.at("/streams").get(response_stream_list).post(post_stream);
        api.at("/:stream").get(response_stream);
        api.at("/:stream/channels").get(response_stream_channels);
        // api.at("/:stream/:channel").get(response_stream_channel).post(post_stream_channel);
        api.at("/rules").get(response_rules);
        api.at("/:rule").get(response_rule).post(post_rule);
        api.at("/can/frame").post(handle_can_frame);
        // api.at("/lamp").get(get_signal);
        // api.at("/lamp/on").get(get_signal);
        // api.at("/lamp/off").get(get_signal);
        // api.at("/streams").get(stream)
        // api.at("/info").get(device::get_info);
    });
    app
}

#[cfg(test)]
mod tests {
    // #![feature(async_await)]
    // use http_service_mock::make_server;

    // use http_service;
    // use super::*;
    // use tide::{
    //     Server,
    //     error::{ StringError, ResultExt },
    //     response, App, Context, EndpointResult,
    //     querystring::ContextExt
    // };

    #[test]
    fn uvtest() {
        // let state = State::new();
        // let mut app      = tide::App::with_state(state);
        // app = setup_routes(app);
        // let mut server = make_server(app.into_http_service()).unwrap();

        // let req = http::Request::get("/add_one/3")
        //     .body(Body::empty())
        //     .unwrap();
        // let res = server.simulate(req).unwrap();
        // assert_eq!(res.status(), 200);
        // let body = block_on(res.into_body().into_vec()).unwrap();
        // assert_eq!(&*body, &*b"4");
    }

}
