use wqa::api::app::State;
use tera;
// use tera::compile_templates;
use tide::{self, Context, EndpointResult, Error};

/// State to pass with context and will hold
/// the interface to the tera rendering engine
// Render some data into the 'tera-hello-world.html template in examples/templates directory
pub async fn index(ctx: Context<State>) -> EndpointResult {
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
// fn templates_create() -> tera::Tera{
    // let template_dir = format!("{}/examples/templates/*", env!("CARGO_MANIFEST_DIR"));
    // compile_templates!(&template_dir);
// }