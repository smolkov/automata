use directories::ProjectDirs;
use log::{debug, error, info, warn};
use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};


use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use lazy_static::lazy_static;
use std::sync::RwLock;

use super::{Autosampler, Device, WQError};

lazy_static! {
    // pub static ref SETTINGS: RwLock<Settings> = RwLock::new(Settings::new());
    // pub static ref STREAM1: RwLock<Stream> = RwLock::new(Stream::new(1));
    // pub static ref STREAM2: RwLock<Stream> = RwLock::new(Stream::new(2));
    // pub static ref STREAM3: RwLock<Stream> = RwLock::new(Stream::new(3));
    // pub static ref STREAM4: RwLock<Stream> = RwLock::new(Stream::new(4));
    // pub static ref STREAM5: RwLock<Stream> = RwLock::new(Stream::new(5));
    // pub static ref STREAM6: RwLock<Stream> = RwLock::new(Stream::new(6));
}

const TLD: &str = "com";
const SLD: &str = "larag";
const STORE: &str = "storage.yaml";



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Collection {
    pub device:      Device,
    // pub methods:     Vec<Methods>,
    pub autosampler: Autosampler,
}



// impl Collection {
//     pub fn new() -> Self {
//         // let mut id = Identificator::new(1);
//         Self {
//             device: Device::default(),
//             methods: vec![
//                MethodSetting::new_tc(&mut id),
//             ],
//             autosampler: Autosampler::default(),
//         }
//     }
// }

/// Storage
#[derive(Debug)]
pub struct Storage {
    path: PathBuf,
    project_dirs: ProjectDirs,
    pub current: Settings,
    last: Settings,
}

impl Storage {
    pub fn new() -> Storage {
        let project_dirs = ProjectDirs::from(TLD, SLD, env!("CARGO_PKG_NAME"))
            .expect("Couldn't find project dirs for this platform");
        let config_dir = project_dirs.config_dir();

        create_dir_all(&config_dir).unwrap();
        let path = config_dir.join(STORE).to_owned();
        let last = match File::open(&path) {
            Ok(mut file) => {
                let mut yaml_str = String::new();
                file.read_to_string(&mut yaml_str).unwrap();
                let settings: Settings = match serde_yaml::from_str(&yaml_str) {
                    Ok(value) => value,
                    Err(error) => {
                        warn!("Unable to parse cache file, {}", error);
                        Settings::new()
                    }
                };
                settings
            }
            Err(error) => {
                warn!(
                    "Unable to open settings file, directory: {:#?} error: {}",
                    &config_dir, error
                );
                Settings::new()
            }
        };

        Storage {
            project_dirs,
            path,
            current: last.clone(),
            last,
        }
    }
    pub fn save(&mut self) {
        if self.last == self.current {
            // unchanged => don't save
            println!("Unchanged");
            return;
        }
        match File::create(&self.path) {
            Ok(mut file) => match serde_yaml::to_string(&self.current) {
                Ok(yaml_str) => {
                    let data = yaml_str.as_bytes();
                    match file.write_all(data) {
                        Ok(_) => {
                            self.last = self.current.clone();
                            debug!("saved config: {:?}", self.current);
                        }
                        Err(error) => {
                            info!(
                                "Error while writing to cache file: {:#?}, error: {:#?}",
                                &self.path, error
                            );
                        }
                    }
                },
                Err(err) => {
                    error!("couldn't serialize configuration: {}", err);
                    println!("couldn't serialize configuration: {}", err);
                }
            }, 
            Err(err) => {
                error!("couldn't create configuration file: {}", err);
                println!("couldn't create configuration file: {}", err);
            }
        }
    }
    pub fn current_mut(&mut self) -> &mut Settings {
        &mut self.current
    }
    pub fn config_dir(&mut self) -> &Path {
        self.project_dirs.config_dir()
    }
//    pub fn get_method(&self, num: usize) -> Result<Stream, WQError> {
        // if let Some(st) = self.current.streams.get(stream) {
            // Ok(st.clone())
        // } else {
            // Err(WQError::NoStream { stream })
        // }
    // }

    // pub fn get_stream(&self, stream: usize) -> Result<Stream, WQError> {
    //     if let Some(st) = self.current.streams.get(stream) {
    //         Ok(st.clone())
    //     } else {
    //         Err(WQError::NoStream { stream })
    //     }
    // }
}


// pub fn get_stream1(stream: usize) -> Result<Stream, WQError> {
//     let storage = STORAGE.read().expect("bug: Lock was poisoned");
//     if let Some(st) = storage.current.streams.get(stream) {
//         Ok(st.clone())
//     } else {
//         Err(WQError::NoStream { stream })
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Serial;

    #[test]
    fn test_create_config() {
        let mut store = Storage::new();
        assert!(store.current.streams[0].calibrate.interval.as_secs() == 24400);
        store.current.device.serial = Serial {
            id: "QU1967667".to_owned(),
            build: 1,
        };
        // for st in &mut store.streams {
        // st.current.description = "Test change description".to_owned();
        // st.save();
        // }
        // println!("STORE:{:?}",store);
        store.save();

        // let project_dirs = ProjectDirs::from(TLD, SLD, env!("CARGO_PKG_NAME"))
        //     .expect("Couldn't find project dirs for this platform");

        // assert_eq!("rust-lang/rust.vim".to_owned(), plug.get_plug_path());
    }
}


pub enum StorageMutation {
    Stream(Stream),
    SingleSetting(usizem,Single)
    Autosampler(Autosampler),
    Device(Autosampler),
}