#![feature(async_await)]

use futures::executor::block_on;
use http_service::Body;
use http_service_mock::make_server;
use tide::{error::ResultExt, Context};



#[test]
fn mio_simulator() {
    // let mut app = tide::App::new();
    // app.at("/add_two/:one/:two/").get(add_two);
    // let mut server = make_server(app.into_http_service()).unwrap();

    // let req = http::Request::get("/add_two/1/2/")
    //     .body(Body::empty())
    //     .unwrap();
    // let res = server.simulate(req).unwrap();
    // assert_eq!(res.status(), 200);
    // let body = block_on(res.into_body().into_vec()).unwrap();
    // assert_eq!(&*body, &*b"3");

    // let req = http::Request::get("/add_two/-1/2/")
    //     .body(Body::empty())
    //     .unwrap();
    // let res = server.simulate(req).unwrap();
    // assert_eq!(res.status(), 200);
    // let body = block_on(res.into_body().into_vec()).unwrap();
    // assert_eq!(&*body, &*b"1");
    // let req = http::Request::get("/add_two/1")
    //     .body(Body::empty())
    //     .unwrap();
    // let res = server.simulate(req).unwrap();
    // assert_eq!(res.status(), 404);
}
