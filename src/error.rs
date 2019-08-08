#![allow(unused_variables)]
// use failure::{ResultExt};
use failure::{Fail};
use std::io;
use walkdir;
use serde_yaml;
use git2;
use settings::ConfigError;

#[cfg(feature = "canbus")]
use dbus::Error as DBusError;

// use serde::{Deserialize, Serialize};
// pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
// use tide::type

// use regex;


// pub use WqaError as Error;




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
pub enum WqaError {

    #[fail(display = "io error - {}",err)]
    IOError {err: io::Error },

    #[fail(display = "directory error - {}",err)]
    DirError {err: walkdir::Error },

    #[fail(display = "yaml error - {}",err)]
    BadYaml {err:serde_yaml::Error },

    #[fail(display = "git error - {}",err)]
    GitError {err: git2::Error },

    #[fail(display = "config error - {}",err)]
    ConfigError {err: ConfigError },

    #[cfg(feature = "candbus")]
    #[fail(display = "dbus  error - {}",err)]
    DBusError {err: DBusError },
    // #[fail(display = "tide responce err - {:?}",err)]
    // ResponceError{err: TideError},

    // #[fail(display = "io error - {}",serde_json)]
    // BadJson(serde_json::Error),
}

// macro_rules! app_error_from {
//   ($error: ty, $app_error: ident) => {
//     impl From<$error> for WqaError {
//       fn from(err: $error) -> WqaError {
//         WqaError::$app_error(err)
//       }
//     }
//   };
// }
impl From<ConfigError> for WqaError {
    fn from(kind:ConfigError) -> WqaError {
        WqaError::ConfigError{err: kind}
    }
}

impl From<git2::Error> for WqaError {
    fn from(kind:git2::Error) -> WqaError {
        WqaError::GitError{err: kind}
    }
}

impl From<io::Error> for WqaError {
    fn from(kind:io::Error) -> WqaError {
        WqaError::IOError{err: kind}
    }
}

impl From<walkdir::Error> for WqaError {
  fn from(kind: walkdir::Error) -> WqaError {
    WqaError::DirError{err:kind}
  }
}
impl From<serde_yaml::Error> for WqaError {
  fn from(kind: serde_yaml::Error) -> WqaError {
    WqaError::BadYaml{err:kind}
  }
}

#[cfg(feature = "candbus")]
impl From<DBusError> for WqaError {
    fn from(kind:DBusError) -> WqaError {
        WqaError::DBusError{err:kind}
    }
}

