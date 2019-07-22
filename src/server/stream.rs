use crate::{
    analyzer::stream,
};
use http::status::StatusCode;
use tide::{
    error::{ StringError, ResultExt },
    response, App, Context, EndpointResult,
    querystring::ContextExt
};
use super::app::Repo;

pub async fn list_streams(_cx: Context<Repo>) -> EndpointResult {
    // let app = cx.state();
    match stream::find_all().await {
        Ok(streams) => Ok(response::json (streams)),
        Err(_) => Err(StatusCode::NOT_FOUND.into()),
    }
}

