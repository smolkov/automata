use crate::{
    AppState,
    device,
    Config

};
use runtime;
// use failure::{Fallible};

// use http::status::StatusCode;
// use failure::{Fallible, ResultExt};

use tide::{self, App, Context, EndpointResult, Error};


// Render some data into the 'tera-hello-world.html template in examples/templates directory
async fn index(ctx: Context<AppState>) -> EndpointResult {
    // Create the context for the template
    let mut context = tera::Context::new();
    context.insert("page_title", "Hello from Tera templating!");
    context.insert("points", &vec!["point1", "point2"]);

    // Render the variables into the template
    let s = ctx
        .state()
        .template
        .render("tera-hello-world.html", &context)
        .map_err(|err| {
            // Map the tera::Error into a Tide error
            let resp = http::Response::builder()
                .status(500)
                .body(err.description().into())
                .unwrap();
            Error::from(resp)
        })?;

    // Build normal response, putting the rendered string into bytes -> Body
    let resp = http::Response::builder()
        .header(http::header::CONTENT_TYPE, mime::TEXT_HTML.as_ref())
        .status(http::StatusCode::OK)
        .body(s.as_bytes().into())
        .expect("Failed to build response");
    Ok(resp)
}



/// run_server api
async fn run_server( cfg: ServerConfig ) {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    //  let template_dir = format!("{}/examples/templates/*", env!("CARGO_MANIFEST_DIR"));
    let state = AppState::new();
    // let repo = monitor::new_uv().await;
    let mut app = tide::App::with_state(state);
    app.middleware(tide::middleware::RequestLogger::new());
    app.at("/").get(index);
    app.at("/api").nest(|api| {
    api.at("/device").get(device::response_info);
    api.at("/device/serial").get( device::get_serial).post(device::set_serial);
    //   api.at("/info").get(device::get_info);
    });
    app.run("127.0.0.1:8000").unwrap();
}
