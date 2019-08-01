//! # Server api sensors for monitoring flow
//!
//! ## Airflow
//!
//! ## Humidity
//!
//! ## Pressure
//!





use crate::mio;
use tide::{
    // error::{ ResultExt },
    response, App, Context, EndpointResult,
};

use super::app::State;

// use lazy_static::lazy_static;

/// get airflow current value on in
async fn get_airflow_input(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::flow::get_airflow_input().await.unwrap()))
}

/// get airflow current value on in
pub async fn get_airflow_output(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::flow::get_airflow_input().await.unwrap()))
}

pub async fn get_humidity(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::flow::get_humidity().await.unwrap()))
}

pub async fn get_pressure(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::flow::get_humidity().await.unwrap()))
}

// pub fn setup_routes(mut app: App<State>) -> App<State> {

    // app.at("/api").nest(|api| {

        // api.at("/lamp").get(get_signal);
        // api.at("/lamp/on").get(get_signal);
        // api.at("/lamp/off").get(get_signal);
        // api.at("/streams").get(stream)
        // api.at("/info").get(device::get_info);
    // });
    // app
// }

#[cfg(test)]
mod tests {
    // #![feature(async_await)]
    // use http_service_mock::make_server;

    // use http_service;
    // use super::*;
    // use tide::{
        // Server,
        // error::{ StringError, ResultExt },
        // response, App, Context, EndpointResult,
        // querystring::ContextExt
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
