use async_std::fs;
use async_std::io;

use log::info;
use std::path::{PathBuf,Path};
use super::rootdir;
// use super::error::Result;

#[cfg(feature = "flame_it")]
extern crate flame;
#[cfg(feature = "flame_it")]
#[macro_use] extern crate flamer;

use serde::{Deserialize, Serialize};


// pub mod data;
use lockfile::Lockfile;
use failure::Fallible;

#[cfg_attr(feature = "flame_it", flame)]
#[derive(Debug)]
struct Lock{
    lock: Lockfile,
}

fn lock_path(lockfile:&Path) -> Fallible<Lock> {
    let lock = Lock{
        lock:Lockfile::create(lockfile)?,
    };
    info!("🔒 directory lock {:?}",lockfile);
    Ok(lock)
}
#[derive(Serialize, Deserialize,Debug)]
pub struct Node {
    pub path: PathBuf,
    #[serde(skip_serializing,skip_deserializing)]
    lock: Option<Lock>,
}

impl Node {
    pub fn root() -> Node {
        Node{
            path:rootdir(),
            lock: None,
        }
    }
    pub fn new(path : &Path) -> Node {
       Node{
           path: path.to_path_buf(),
           lock: None,
       }
    }
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
    pub fn lock(&self) -> Fallible<Node> {
        let node = Node {
            lock: Some(lock_path(&self.path.join("lock"))?),
            path: self.path.clone(),

        };
        Ok(node)
    }
}


pub async fn label(path: &Path) -> io::Result<String>{
    let label = path.join("label");
    fs::read_to_string(&label).await
}

