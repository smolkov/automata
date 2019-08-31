use std::{
    error::Error,
    fmt, io,
    path::{PathBuf},
};
use toml;

use ron::{self, de::Error as DeError, ser::Error as SerError};
/// Error related to anything that manages/creates configurations as well as
/// "workspace"-related things.
#[derive(Debug)]
pub enum ConfigError {
    /// Forward to the `std::io::Error` error.
    File(io::Error),
    /// Errors related to serde's parsing of configuration files.
    Parser(DeError),
    /// Occurs if a value is ill-formed during serialization (like a poisoned mutex).
    Serializer(SerError),
    /// Related to the path of the file.
    Extension(PathBuf),
    /// Toml
    BadTomlData(toml::de::Error),
    SerializeTomlError(toml::ser::Error),

}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ConfigError::File(ref err) => write!(f, "{}", err),
            ConfigError::Parser(ref msg) => write!(f, "{}", msg),
            ConfigError::Serializer(ref msg) => write!(f, "{}", msg),
            ConfigError::Extension(ref path) => {
                let found = match path.extension() {
                    Some(extension) => format!("{:?}", extension),
                    None => "a directory.".to_string(),
                };

                write!(
                    f,
                    "{}: Invalid path extension, expected \"ron\", got {}.",
                    path.display().to_string(),
                    found,
                )
            },
            ConfigError::BadTomlData(ref err) => write!(f,"{}",err),
            ConfigError::SerializeTomlError(ref err) => write!(f,"{}",err),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> ConfigError {
        ConfigError::File(e)
    }
}

impl From<DeError> for ConfigError {
    fn from(e: DeError) -> Self {
        ConfigError::Parser(e)
    }
}

impl From<SerError> for ConfigError {
    fn from(e: SerError) -> Self {
        ConfigError::Serializer(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::BadTomlData(e)
    }
}
impl From<toml::ser::Error> for ConfigError {
    fn from(e: toml::ser::Error) -> Self {
        ConfigError::SerializeTomlError(e)
    }
}
impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::File(_) => "Project file error",
            ConfigError::Parser(_) => "Project parser error",
            ConfigError::Serializer(_) => "Project serializer error",
            ConfigError::Extension(_) => "Invalid extension or directory for a file",
            ConfigError::BadTomlData(_) => "Invalid toml data",
            ConfigError::SerializeTomlError(_) => "Invalid serialize to toml data",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ConfigError::File(ref err) => Some(err),
            _ => None,
        }
    }
}
