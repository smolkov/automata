// #![feature(async_await)]

// use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// use super::*;
pub use tide::{error::ResultExt, response, App, Context, EndpointResult};

#[derive(Default)]
pub struct AppState{
    contents: Mutex<Vec<Message>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    author: Option<String>,
    contents: String,
}


impl AppState {


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


