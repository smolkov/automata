#![feature(async_await)]

use std::{
    net::ToSocketAddrs,
    sync::Arc,
    collections::hash_map::{HashMap, Entry},
};

use futures::{
    channel::mpsc,
    SinkExt,
    FutureExt,
    select,
};

use async_std::{
    io::BufReader,
    prelude::*,
    task,
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Hello, world!");
}
