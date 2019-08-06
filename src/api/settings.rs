use crate::{
    // error::*,
    store,
    // stream::Stream,
        // channel::Channel,
};


// use http::status::StatusCode;
use tide::{
    error::{ ResultExt },
    response, Context, EndpointResult,
    // querystring::ContextExt
};
use super::app::State;


// Setup route
pub fn setup_route(mut app: App<State>) -> App<State> {
    app.at("/api").nest(|api| {
        api.at("/device").get(response_device);
        api.at("/streams").get(response_stream_list).post(post_stream);
        api.at("/:stream").get(response_stream);
        api.at("/:stream/channels").get(response_stream_channels);
//         // api.at("/:stream/:channel").get(response_stream_channel).post(post_stream_channel);
        api.at("/rules").get(response_rules);
        api.at("/:rule").get(response_rule).post(post_rule);
    });
    app
}
