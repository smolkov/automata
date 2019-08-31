#![allow(unused_variables)]
// use failure::{ResultExt};
use failure::{Fail};
// use std::io;
// use walkdir;
use serde_yaml;
use git2;
use store::ConfigError;
use async_std::io as async_io;
#[cfg(feature = "candbus")]
use dbus::Error as DBusError;

// pub type Result<T> = async_std::io::Result<T,async_std::Error + Send + Sync>;
// use serde::{Deserialize, Serialize};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
// use tide::type
// pub type Result<T> = std::result::Result<T, Error>;
// use regex;


// pub use AutomataError as Error;




// use std::string::FromUtf8Error;
// #[derive(Debug)]
// pub enum AppError {
//   IO(io::Error),
//   UserError(String),
//   RuntimeError(String),
//   InternalError(&'static str),
//   ClockError(SystemTimeError),
//   GitError(git2::Error),
//   Regex(regex::Error),
//   GitlabApiError(gitlab::Error),
// }
// use mut_guard::*;

#[derive(Fail, Debug)]
pub enum AutomataError {

    #[fail(display = "async io error - {}",err)]
    AsyncIOError {err: async_io::Error },

    #[fail(display = "yaml error - {}",err)]
    BadYaml {err:serde_yaml::Error },

    #[fail(display = "git error - {}",err)]
    GitError {err: git2::Error },

    #[fail(display = "config error - {}",err)]
    ConfigError {err: ConfigError },

    #[cfg(feature = "candbus")]
    #[fail(display = "dbus  error - {}",err)]
    DBusError {err: DBusError },

    #[fail(display = "IO device not found - {:}",msg)]
    IoNotFound{msg:String},

    // #[fail(display = "io error - {}",serde_json)]
    // BadJson(serde_json::Error),
}



// macro_rules! app_error_from {
//   ($error: ty, $app_error: ident) => {
//     impl From<$error> for AutomataError {
//       fn from(err: $error) -> AutomataError {
//         AutomataError::$app_error(err)
//       }
//     }
//   };
// }
impl From<ConfigError> for AutomataError {
    fn from(kind:ConfigError) -> AutomataError {
        AutomataError::ConfigError{err: kind}
    }
}

impl From<git2::Error> for AutomataError {
    fn from(kind:git2::Error) -> AutomataError {
        AutomataError::GitError{err: kind}
    }
}

impl From<async_io::Error> for AutomataError {
    fn from(kind:async_io::Error) -> AutomataError {
        AutomataError::AsyncIOError{err: kind}
    }
}
impl From<serde_yaml::Error> for AutomataError {
  fn from(kind: serde_yaml::Error) -> AutomataError {
    AutomataError::BadYaml{err:kind}
  }
}

#[cfg(feature = "candbus")]
impl From<DBusError> for AutomataError {
    fn from(kind:DBusError) -> AutomataError {
        AutomataError::DBusError{err:kind}
    }
}

