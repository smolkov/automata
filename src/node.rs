
use log::info;
use std::path::{PathBuf,Path};

use crate::templates::Template;

#[cfg(feature = "flame_it")]
extern crate flame;
#[cfg(feature = "flame_it")]
#[macro_use] extern crate flamer;




// pub mod data;
use lockfile::Lockfile;
use failure::Fallible;

#[cfg_attr(feature = "flame_it", flame)]
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
pub struct Node<T> {
    path: PathBuf,
    data: T,
    lock: Option<Lock>,
}

pub struct Root;

impl <R>Node<R> {
    pub fn root(path : &str) -> Node<Root> {
        Node{
            path:PathBuf::from(path),
            lock: None,
            data: Root{}
        }
    }
    pub fn lock(mut self) -> Fallible<Node<R>> {
        let node = Node {
            lock: Some(lock_path(&self.path.join("lock"))?),
            path: self.path,
            data: self.data

        };
        Ok(node)
    }
    pub fn templates(mut self) -> Node<Template> {
        Node {
            path: self.path.join("template"),
            lock: None,
            data:Template{},
        }
    }
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}
