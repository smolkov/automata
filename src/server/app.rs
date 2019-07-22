// #![feature(async_await)]

// use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// use super::*;
// use super::*;
pub use tide::{error::ResultExt, response, App, Context, EndpointResult};
use tera::{self, compile_templates};


// Repo to pass with context and will hold
// the interface to the tera rendering engine
pub struct Repo{
    pub template: tera::Tera,
    contents: Mutex<Vec<Message>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Message {
    author: Option<String>,
    contents: String,
}


impl Repo {
    pub fn new() -> Self {
        let template_dir = format!("{}/assets/templates/*", env!("CARGO_MANIFEST_DIR"));
        // let template = compile_templates!(&template_dir);
        Repo{
            template: compile_templates!(&template_dir),
            contents : Mutex::new(Vec::new()),
        }
    }
   /// Message
    fn insert(&self, msg: Message) -> usize {
        let mut table = self.contents.lock().unwrap();
        table.push(msg);
        table.len() - 1
    }
    fn get(&self, id: usize) -> Option<Message> {
        self.contents.lock().unwrap().get(id).cloned()
    }

    fn set(&self, id: usize, msg: Message) -> bool {
        let mut table = self.contents.lock().unwrap();

        if let Some(old_msg) = table.get_mut(id) {
            *old_msg = msg;
            true
        } else {
            false
        }
    }
}
