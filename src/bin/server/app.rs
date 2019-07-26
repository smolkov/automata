// #![feature(async_await)]

// use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// use super::*;
// use super::*;
pub use tide::{error::ResultExt, response, App, Context, EndpointResult};
use tera::{self, compile_templates};

// use staticfile::StaticFile;
// State to pass with context and will hold
// the interface to the tera rendering engine
pub struct State{
    pub template: tera::Tera,
   
    // pub staticfs: StaticFile,
    contents: Mutex<Vec<Message>>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Message {
    author: Option<String>,
    contents: String,
}


impl State {
    pub fn new() -> Self {
        use log::info;
        info!("template directory {}/assets/templates/*",env!("CARGO_MANIFEST_DIR"));

        let template_dir = format!("{}/assets/templates/*", env!("CARGO_MANIFEST_DIR"));
        // let template = compile_templates!(&template_dir);
        State{
            template: compile_templates!(&template_dir),
            // staticfs: StaticFile::new("/."),
            contents : Mutex::new(Vec::new()),
        }
    }
   /// Insert messagr
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
