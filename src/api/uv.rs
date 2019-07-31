use crate::mio;
use tide::{
    error::{ ResultExt },
    response, App, Context, EndpointResult,
};

use super::app::State;


async fn start_sample_pump(mut cx: Context<State>) -> EndpointResult<()> {
    mio::sample_pump(true).await.unwrap();
    Ok(())
}
async fn stop_sample_pump(mut cx: Context<State>) -> EndpointResult<()> {
    mio::sample_pump(false).await.unwrap();
    Ok(())
}


async fn open_sample_valve(cx: Context<State>) -> EndpointResult<()> {
    let sample = cx.param(":sample").client_err()?;
    mio::open_sample_valve(sample).await.unwrap();
    Ok(())
}
async fn close_sample_valve(cx: Context<State>) -> EndpointResult<()> {
    let sample = cx.param(":sample").client_err()?;
    mio::open_sample_valve(sample).await.unwrap();
    Ok(())
}
async fn open_zeroflow_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::open_zeroflow_valve().await.unwrap();
    Ok(())
}
async fn close_zeroflow_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::close_zeroflow_valve().await.unwrap();
    Ok(())
}
async fn open_tic_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::open_tic_valve().await.unwrap();
    Ok(())
}
async fn close_tic_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::close_tic_valve().await.unwrap();
    Ok(())
}
async fn open_calibration_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::open_calibration_valve().await.unwrap();
    Ok(())
}
async fn close_calibration_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::close_calibration_valve().await.unwrap();
    Ok(())
}
async fn get_ndir1_value(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_ndir1_value().await.unwrap()))
}
async fn get_ndir2_value(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_ndir2_value().await.unwrap()))
}


/// Uv io state return all hardware parameter.
async fn get_uv_state(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(0))
}

pub fn setup_routes(mut app: App<State>) -> App<State> {

    app.at("/uv").nest(|api| {
        api.at("/state").get(get_uv_state);
        api.at("/sample/start").post(start_sample_pump);
        api.at("/sample/stop").post(stop_sample_pump);
        api.at("/valve/{}/open").post(open_sample_valve);
        api.at("/valve/{}/close").post(close_sample_valve);
        api.at("/valve/zeroflow/open").post(open_zeroflow_valve);
        api.at("/valve/zeroflow/close").post(close_zeroflow_valve);
        api.at("/valve/tic/open").post(open_tic_valve);
        api.at("/valve/tic/close").post(close_tic_valve);
        api.at("/valve/calibration/open").post(open_calibration_valve);
        api.at("/valve/calibration/close").post(open_calibration_valve);
        api.at("/ndir1").get(get_ndir1_value);
        api.at("/ndir2").get(get_ndir2_value);
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
    #![feature(async_await)]
    use http_service_mock::make_server;

    use http_service;
    use super::*;
    use tide::{
        Server,
        error::{ StringError, ResultExt },
        response, App, Context, EndpointResult,
        querystring::ContextExt
    };

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
