#![feature(async_await)]

use structopt::*;
use tide;
use automata;
use runtime;
use failure::{Fallible, ResultExt};
// use failure::{Fallible, ResultExt};



#[runtime::main]
async fn main() {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();

    let mut app = tide::App::new(automata::AppState::default());
    app.middleware(tide::middleware::RootLogger::new());
    app.at("/").get(async move |_| "Hello, world!");
    app.serve("127.0.0.1:8000").unwrap();
}

// mod automata;
// pub use tide::{error::ResultExt, response, App, Context, EndpointResult};


// async fn new_message(mut cx: Context<ApiState>) -> EndpointResult<String> {
//     let msg = cx.body_json().await.client_err()?;
//     Ok(cx.state().insert(msg).to_string())
// }

// async fn set_message(mut cx: Context<Station>) -> EndpointResult<()> {
//     let msg = cx.body_json().await.client_err()?;
//     let id = cx.param("id").client_err()?;

//     if cx.state().set(id, msg) {
//         Ok(())
//     } else {
//         Err(StatusCode::NOT_FOUND)?
//     }
// }

// async fn get_message(cx: Context<ApiState>) -> EndpointResult {
//     let id = cx.param("id").client_err()?;
//     if let Some(msg) = cx.state().get(id) {
//         Ok(response::json(msg))
//     } else {
//         Err(StatusCode::NOT_FOUND)?
//     }
// }






// async fn tide_server()  {

//     info!("✨ Tide server run ✨");

//     use log::LevelFilter;
//     use log4rs::append::console::ConsoleAppender;
//     use log4rs::config::{Appender, Config, Root};
//     //
//     let stdout = ConsoleAppender::builder().build();
//     let config = Config::builder()
//         .appender(Appender::builder().build("stdout", Box::new(stdout)))
//         .build(Root::builder().appender("stdout").build(LevelFilter::Info))
//         .unwrap();
//     // let _handle = log4rs::init_config(config).unwrap();
//     // let state = ApiState::init();
//     // let mut app = App::with_state(ApiState::default());
//     // app.config(app_config);
//     // app.at("/message").post(new_message);
//     // app.at("/message/:id").get(get_message).post(set_message);

//     // app.run("127.0.0.1:8000").unwrap();
// }


// #[paw::main]
// #[runtime::main]
// async fn main(args: paw::Args) {
//     for (i, arg) in args.enumerate() {
//         println!("#{:?}: {:?}", i, arg);
//     }
//     runtime::spawn(async move {
//         dbg!("hello");
//         tide_server().await;
//     })
//     .await;
// }

// #[cfg(unix)]
// mod signal {
//     use std::io;

//     use futures::compat::Future01CompatExt;
//     use futures::future::Future;
//     use tokio_signal::unix::{
//         libc::{SIGINT, SIGTERM},
//         Signal,
//     };

//     pub async fn quit() -> io::Result<impl Future<Output = ()>> {
//         let (int, term) = (Signal::new(SIGINT).compat(), Signal::new(SIGTERM).compat());
//         let (int, term) = futures::try_join!(int, term)?;
//         Ok(super::merge_select(super::first(int), super::first(term)))
//     }
// }

// #[cfg(windows)]
// mod signal {
//     use std::io;

//     use futures::compat::Future01CompatExt;
//     use futures::future::Future;
//     use tokio_signal::windows::Event;

//     pub async fn quit() -> io::Result<impl Future<Output = ()>> {
//         let (cc, cb) = (Event::ctrl_c().compat(), Event::ctrl_break().compat());
//         let (cc, cb) = futures::try_join!(cc, cb)?;
//         Ok(super::merge_select(super::first(cc), super::first(cb)))
//     }
// }

// fn first<S: Stream01>(s: S) -> impl Future<Output = ()>
// where
//     S::Error: Debug,
// {
//     s.compat().into_future().map(|(r, _)| {
//         r.unwrap().unwrap();
//     })
// }

// fn merge_select<A, B>(a: A, b: B) -> impl Future<Output = A::Output>
// where
//     A: Future + Unpin,
//     B: Future<Output = A::Output> + Unpin,
// {
//     future::select(a, b).map(|either| either.factor_first().0)
// }



// //
