#![allow(unused_variables)]
use std::error;
use std::ffi;
use std::fmt;
use std::net;
use std::num;
use std::string;
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
pub enum Error {

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

    #[doc(hidden)]
    Other(Box<dyn error::Error + Send + 'static>),

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
impl From<ConfigError> for Error {
    fn from(kind:ConfigError) -> Error {
        Error::ConfigError{err: kind}
    }
}

impl From<git2::Error> for Error {
    fn from(kind:git2::Error) -> Error {
        Error::GitError{err: kind}
    }
}

impl From<async_io::Error> for Error {
    fn from(kind:async_io::Error) -> Error {
        Error::AsyncIOError{err: kind}
    }
}
impl From<serde_yaml::Error> for Error {
  fn from(kind: serde_yaml::Error) -> Error {
    Error::BadYaml{err:kind}
  }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Other(e) => Some(&**e),
            _ => None,
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Other(e) => fmt::Display::fmt(e, f),
            _ => f.write_str("Unknown error"),
        }
    }
}


impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<ffi::NulError> for Error {
    fn from(e: ffi::NulError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<ffi::IntoStringError> for Error {
    fn from(e: ffi::IntoStringError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<string::ParseError> for Error {
    fn from(e: string::ParseError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error::Other(Box::new(e))
    }
}

impl From<net::AddrParseError> for Error {
    fn from(e: net::AddrParseError) -> Self {
        Error::Other(Box::new(e))
    }
}

impl<T> From<Box<T>> for Error
where
    T: error::Error + Send + 'static,
{
    fn from(e: Box<T>) -> Self {
        Error::Other(e)
    }
}

// #[cfg(unix)]
// impl From<nix::Error> for Error {
    // fn from(e: nix::Error) -> Self {
        // Error::Other(Box::new(e))
    // }
// }



#[cfg(feature = "candbus")]
impl From<DBusError> for AutomataError {
    fn from(kind:DBusError) -> AutomataError {
        AutomataError::DBusError{err:kind}
    }
}

