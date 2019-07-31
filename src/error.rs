#![allow(unused_variables)]
// use failure::{ResultExt};
use failure::{Fail};
use std::io;
use serde::{Deserialize, Serialize};
use walkdir;
use serde_yaml;
use git2;
use settings::ConfigError;
use dbus::Error as DBusError;
use tide::error::Error as TideError;
// use tide::type

// use regex;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum ErrLevel {
    Warning,
    Critical,
}
#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum ErrState {
    Ok,
    Waiting,
    Came,
}
use WqaError as Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct WqaErr {
    pub level: ErrLevel,
    pub description: String,
    pub changed: u64,
    pub state: ErrState,
    pub number: u16,
}



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
impl From<DBusError> for WqaError {
    fn from(kind:DBusError) -> WqaError {
        WqaError::DBusError{err:kind}
    }
}
// impl From<TideError> for WqaError {
    // fn from(kind:TideError) -> WqaError {
        // WqaError::ResponceError{err:kind}
    // }
// }
// app_error_from!(git2::Error, GitError);
// app_error_from!(io::Error, IO);
// app_error_from!(serde_json::Error, BadJson);
// app_error_from!(regex::Error, Regex);
// app_error_from!(gitlab::Error, GitlabApiError);


//
