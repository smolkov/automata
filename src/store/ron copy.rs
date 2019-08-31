//! Loads RON files into a structure for easy / statically typed usage.

use std::{
    path::{Path},
};

use log::error;
use ron::{self};
use serde::{Deserialize, Serialize};
use super::ConfigError;

/// Trait implemented by the `config!` macro.
pub trait Config
where
    Self: Sized,
{
    /// Loads a configuration structure from a file.
    /// Defaults if the file fails in any way.
    fn load<P: AsRef<Path>>(path: P) -> Self;

    /// Loads a configuration structure from a file.
    fn load_no_fallback<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError>;

    /// Loads configuration structure from raw bytes.
    fn load_bytes(bytes: &[u8]) -> Result<Self, ConfigError>;

    /// Writes a configuration structure to a file.
    fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError>;
}

impl<T> Config for T
where
    T: for<'a> Deserialize<'a> + Serialize + Default,
{
    fn load<P: AsRef<Path>>(path: P) -> Self {
        Self::load_no_fallback(path.as_ref()).unwrap_or_else(|e| {
            if let Some(path) = path.as_ref().to_str() {
                error!("Failed to load config file '{}': {}", path, e);
            } else {
                error!("Failed to load config: {}", e);
            }

            Self::default()
        })
    }

    fn load_no_fallback<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        use std::{fs::File, io::Read};

        let path = path.as_ref();

        let content = {
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            buffer
        };

        if path.extension().and_then(std::ffi::OsStr::to_str) == Some("ron") {
            Self::load_bytes(&content)
        } else {
            Err(ConfigError::Extension(path.to_path_buf()))
        }
    }

    fn load_bytes(bytes: &[u8]) -> Result<Self, ConfigError> {
        let mut de = ron::de::Deserializer::from_bytes(bytes)?;
        let val = T::deserialize(&mut de)?;
        de.end()?;

        Ok(val)
    }

    fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        use ron::ser::to_string_pretty;
        use std::{fs::File, io::Write};

        let s = to_string_pretty(self, Default::default())?;
        File::create(path)?.write_all(s.as_bytes())?;

        Ok(())
    }
}



