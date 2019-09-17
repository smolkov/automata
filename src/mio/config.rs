// Config
//

use glob::glob;
use std::collections::{BTreeSet, HashMap};
use std::path::Path;
use std::fmt;
use std::str::FromStr;
use sysfs_gpio;
use toml;
use serde::{self,Deserialize};
use async_std::prelude::*;
use async_std::io;
use async_std::fs;
use async_std::task;
use crate::error::Result;

const DEFAULT_SYMLINK_ROOT: &str = "/var/run/mio";

#[derive(Debug, PartialEq, Clone)]
pub struct Direction(pub sysfs_gpio::Direction);

#[derive(Deserialize, Debug)]
#[serde(remote = "sysfs_gpio::Direction")]
pub enum DirectionDef {
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
}

impl From<sysfs_gpio::Direction> for Direction {
    fn from(e: sysfs_gpio::Direction) -> Self {
        Direction(e)
    }
}




#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Protocol {
    #[serde(rename = "gpio")]
    GPIO,
    #[serde(rename = "digital")]
    Digital,
    #[serde(rename = "analog")]
    Analog,
    #[serde(rename = "uart")]
    Uart,
}

fn default_protocol() -> Protocol {
    Protocol::GPIO
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PinConfig {
    pub num: u64,
    #[serde(default = "default_direction")]
    #[serde(with = "DirectionDef")]
    pub direction: sysfs_gpio::Direction,
    #[serde(default)]
    pub names: BTreeSet<String>,
    #[serde(default = "bool_true")]
    pub export: bool,
    #[serde(default)]
    pub active_low: bool,
    pub user: Option<String>,
    pub group: Option<String>,
    pub mode: Option<u32>,
    pub protocol: Protocol,
}

fn default_direction() -> sysfs_gpio::Direction {
    sysfs_gpio::Direction::In
}

fn bool_true() -> bool {
    true
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MioConfig {
    pub pins: Vec<PinConfig>,
    #[serde(default)]
    pub config: SysConfig,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SysConfig {
    pub symlink_root: Option<String>,
}

impl PinConfig {
    /// Get the `sysfs_gpio::Pin` to go along with this config`
    pub fn get_pin(&self) -> sysfs_gpio::Pin {
        sysfs_gpio::Pin::new(self.num)
    }
}



impl MioConfig {
    /// Validate invariants on the config that cannot easily be done earlier
    ///
    /// Currently, this just checks that there are no duplicated names between
    /// different pins in the config
    fn validate(&self) -> Result<()> {
        // let mut all_names: HashMap<&str, &MioConfig> = HashMap::new();
        // for pin in &self.pins {
        //     for name in &pin.names {
        //         if let Some(other_pin) = all_names.get(&name[..]) {
        //             Err(format!( "Pins {} and {} share duplicate name '{}'", pin.num, other_pin.num, name).as_str())

        //         }
        //         all_names.insert(&name[..], pin);
        //     }
        // }
//
        Ok(())
    }



    /// Get the pin with the provided name if present in this configuration
    pub fn get_pin(&self, name: &str) -> Option<&PinConfig> {
        // first, try to find pin by name
        if let Some(pin) = self.pins.iter().find(|p| p.names.contains(name)) {
            return Some(pin);
        }

        // Try to parse the name as a 64-bit integer and match against that
        match name.parse::<u64>() {
            Ok(pin_num) => self.pins.iter().find(|p| p.num == pin_num),
            Err(_) => None,
        }
    }

    /// Get a reference to all the pins in this config
    pub fn get_pins(&self) -> &[PinConfig] {
        &self.pins[..]
    }

    /// Get the symlink root specified in the config (or the default)
    pub fn get_symlink_root(&self) -> &str {
        match self.config.symlink_root {
            Some(ref root) => &root,
            None => DEFAULT_SYMLINK_ROOT,
        }
    }

    /// Merge other into self (takes ownership of other)
    ///
    /// If in conflict, the other GPIO config takes priority.
    pub fn update(&mut self, other: MioConfig) -> Result<()> {
        if let Some(symlink_root) = other.config.symlink_root {
            self.config.symlink_root = Some(symlink_root);
        }
        for other_pin in other.pins {
            // determine the case we are dealing with
            let existing = match self.pins.iter_mut().find(|p| p.num == other_pin.num) {
                Some(pin) => {
                    pin.names.extend(other_pin.names.clone());
                    pin.direction = other_pin.direction;
                    pin.export = other_pin.export;
                    pin.active_low = other_pin.active_low;
                    true
                }
                None => false,
            };

            if !existing {
                self.pins.push(other_pin);
            }
        }

        // validate the resulting structure
        self.validate()
    }
}


/// Load a GPIO config from the specified path
pub async fn read(path: &Path) -> Result<MioConfig> {
    let contents = fs::read_to_string(path).await?;
    // fs.read_to_string(&mut contents).?;
    let config: MioConfig = toml::from_str(&contents)?;
    config.validate()?;

    Ok(config)
}

    /// Load a GPIO Config from the system
    ///
    /// This function will load the GPIO configuration from standard system
    /// locations as well as from the additional configs passed in via the
    /// `configs` parameter.  Each parameter is expected to be a path to a
    /// config file in disk.
    ///
    /// Under the covers, this function will attempt to discover configuration
    /// files in the following standard locations in order:
    ///
    /// - `/etc/gpio.toml`
    /// - `/etc/gpio.d/*.toml`
    /// - `configs` (parameter)
    ///
    /// Each config file found in these locations will be loaded and then they
    /// will be pulled together to form a unified configuration via the
    /// `combine` method.
    pub async fn all(configs: &[String]) -> Result<MioConfig> {
        let mut config_instances: Vec<MioConfig> = Vec::new();

        // check /etc/gpio.toml
        if fs::metadata("/etc/gpio.toml").await.is_ok() {
            config_instances.push(read(Path::new("/etc/gpio.toml")).await?);
        }
        // /etc/gpio.d/*.toml
        // for fragment in glob("/etc/gpio.d/*.toml").unwrap().filter_map(Result::ok) {
            // config_instances.push(Self::from_file(fragment)?);
        // }

        // additional from command-line
        for fragment in configs {
            config_instances.push(read(Path::new(fragment)).await?);
        }
        let mut cfg = config_instances.remove(0);
        for higher_priority_cfg in config_instances {
            cfg.update(higher_priority_cfg)?;
        }
        // if config_instances.is_empty() {
        //     Err(Error::NoConfigFound)
        // } else {
        //     let mut cfg = config_instances.remove(0);
        //     for higher_priority_cfg in config_instances {
        //         cfg.update(higher_priority_cfg)?;
        //     }
        // }
        Ok(cfg)
    }

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeSet;
    use std::iter::FromIterator;
    use std::str::FromStr;
    use sysfs_gpio::Direction as D;
}
