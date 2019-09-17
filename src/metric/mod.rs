
use heim::Result;

use metrics_core::{Builder, Drain, Observe};
use metrics_runtime::{observers::PrometheusBuilder, Controller, Receiver};

mod cpu;
mod disk;
mod host;
mod memory;
mod gpio;


// use lazy_static::lazy_static;
use super::*;

pub async fn collect() -> Result<()> {
    futures::try_join!(
        cpu::collect(),
        disk::collect(),
        host::collect(),
        memory::collect(),
    )
    .map(|_| ())
}




async fn metrics(ctx: tide::Context<Automata>) -> tide::EndpointResult {
    match collect().await {
        Ok(..) => {
            let mut observer = PrometheusBuilder::new().build();
            ctx.state().controller.observe(&mut observer);

            Ok(tide::Response::new(tide::Body::from(observer.drain())))
        }
        Err(e) => {
            let body = tide::Body::from(format!("{}", e));
            let mut resp = tide::Response::new(body);
            *resp.status_mut() = tide::http::StatusCode::INTERNAL_SERVER_ERROR;
            Ok(resp)
        }
    }
}

pub fn routes(mut app: tide::App<Automata>) -> tide::App<Automata> {
    app.at("/metrics").nest(|api| {
        api.at("/").get(metrics);
    });
    app
}
