/// RestAPI

pub mod app;
pub mod routes;
pub mod templates;
pub mod client;


use crate::error::*;
use tide::{response::IntoResponse, Response};
use http::Response as HttpResponse;
use http::StatusCode;
use http_service::Body;


impl IntoResponse for WqaError {
    fn into_response(self) -> Response {
        empty(StatusCode::INTERNAL_SERVER_ERROR)
        // match self {
        //     UserError(d) => json(StatusCode::OK, d),
        //     BadRequest(o) => {
        //         if let Some(d) = o {
        //             json(StatusCode::BAD_REQUEST, d)
        //         } else {
        //             empty(StatusCode::BAD_REQUEST)
        //         }
        //     }
        //     UnprocessableEntity(o) => {
        //         if let Some(d) = o {
        //             json(StatusCode::UNPROCESSABLE_ENTITY, d)
        //         } else {
        //             empty(StatusCode::UNPROCESSABLE_ENTITY)
        //         }
        //     }
        //     TooManyRequests {
        //         retry_after_secs: r,
        //     } => {
        //         if let Some(t) = r {
        //             HttpResponse::builder()
        //                 .status(StatusCode::TOO_MANY_REQUESTS)
        //                 .header("Retry-After", t.to_string())
        //                 .body(Body::empty())
        //                 .unwrap()
        //         } else {
        //             empty(StatusCode::TOO_MANY_REQUESTS)
        //         }
        //     }
        //     Unauthorized => empty(StatusCode::UNAUTHORIZED),
        //     Forbidden => empty(StatusCode::FORBIDDEN),
        //     NotFound => empty(StatusCode::NOT_FOUND),
        //     BadGateway => empty(StatusCode::BAD_GATEWAY),
        //     GatewayTimeout => empty(StatusCode::GATEWAY_TIMEOUT),
        //     Internal { error: e } => {
        //         log::error!("{:?}", e);
        //         empty(StatusCode::INTERNAL_SERVER_ERROR)
        //     }
        //     Unknown => ,
        // }
    }
}

fn empty(code: StatusCode) -> Response {
    HttpResponse::builder()
        .status(code)
        .body(Body::empty())
        .unwrap()
}

// fn json<T: Send + serde::Serialize>(code: StatusCode, body: T) -> Response {
//     let mut response = tide::response::json(body);
//     *response.status_mut() = code;
//     response
// }
// impl From<TideError> for WqaError {
    // fn from(kind:TideError) -> WqaError {
        // WqaError::ResponceError{err:kind}
    // }
// }
// app_error_from!(git2::Error, GitError);
// app_error_from!(io::Error, IO);
// app_error_from!(serde_json::Error, BadJson);
// app_error_from!(regex::Error, Regex);
// app_error_from!(gitlab::Error, GitlabApiError);


//
