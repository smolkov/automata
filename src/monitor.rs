// #![feature(async_await)]

// use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use super::*;
// use super::*;
pub use tide::{error::ResultExt, response, App, Context, EndpointResult};




#[derive(Default,Clone)]
pub struct Monitor{
    contents: Mutex<Vec<Message>>,
    // pub template: tera::Tera,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct Message {
    author: Option<String>,
    contents: String,
}


impl Monitor {
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




pub async fn monitor_system_info(cx:Context<Monitor>) -> String {
  "none".to_owned()
}

pub fn new_uv_monitor() -> Monitor {
    let template_dir = format!("{}/assets/templates/*", env!("CARGO_MANIFEST_DIR"));
    // let template = compile_templates!(&template_dir);
    Monitor{
      contents:Mutex::new(Vec::new()),
    }
}
//
