use wqa::mio;
use tide::{
    error::{ StringError, ResultExt },
    response, App, Context, EndpointResult,
    querystring::ContextExt
};

use super::app::State;
/// airflow_input
async fn airflow_input(cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_airflow_input().await.unwrap()))
}

async fn airflow_output(cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_airflow_input().await.unwrap()))
}

async fn humidity(cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_humidity().await.unwrap()))
}

async fn pressure(cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_humidity().await.unwrap()))
}

async fn sample_pump(mut cx: Context<State>) -> EndpointResult<()> {
    let start = cx.body_json().await.client_err()?;
    mio::sample_pump(start).await.unwrap();
    Ok(())
}

async fn stream1_valve(mut cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream2_valve(mut cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream3_valve(mut cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream4_valve(mut cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream5_valve(mut cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream6_valve(mut cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn zeroflow_valve(mut cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::zeroflow_valve(state).await.unwrap();
    Ok(())
}
async fn tic_valve(mut cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::tic_valve(state).await.unwrap();
    Ok(())
}
async fn calibration_valve(mut cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::calibration_valve(state).await.unwrap();
    Ok(())
}
async fn get_ndir1(cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_ndir1_value().await.unwrap()))
}
async fn get_ndir2(cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_ndir2_value().await.unwrap()))
}

pub fn setup_routes(mut app: App<State>) -> App<State> {
    let get_airin     = airflow_input;
    let get_airout    = airflow_output;
    let get_humidity  = humidity;
    let get_pressure  = pressure;
    let sample_pump   = sample_pump;

    app.at("/uv").nest(|api| {
        api.at("/airflow/in").get(get_airin);
        api.at("/airflow/out").get(get_airout);
        api.at("/humidity").get(get_humidity);
        api.at("/pressure").get(get_pressure);
        api.at("/pump/sample/:start").post(sample_pump);
        api.at("/valve/sample1/:state").post(stream1_valve);
        api.at("/valve/sample2/:state").post(stream2_valve);
        api.at("/valve/sample3/:state").post(stream3_valve);
        api.at("/valve/sample4/:state").post(stream4_valve);
        api.at("/valve/sample5/:state").post(stream5_valve);
        api.at("/valve/sample6/:state").post(stream6_valve);
        api.at("/valve/zeroflow/:state").post(zeroflow_valve);
        api.at("/valve/tic/:state").post(tic_valve);
        api.at("/valve/calibration/:state").post(calibration_valve);
        api.at("/ndir1").get(get_ndir1);
        api.at("/ndir2").get(get_ndir2);
        // api.at("/lamp").get(get_signal);
        // api.at("/lamp/on").get(get_signal);
        // api.at("/lamp/off").get(get_signal);
        // api.at("/streams").get(stream)
        // api.at("/info").get(device::get_info);
    });
    app
}
