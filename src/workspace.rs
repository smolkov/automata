use crate::error::*;
use failure::{format_err};
use regex::Regex;
use std::ffi::OsStr;
use std::str::FromStr;
use dirs;
use std::fs;
use std::path::PathBuf;


pub fn data_dir() -> Result<PathBuf> {
    let path = dirs::data_dir().ok_or_else(|| format_err!("Failed to find data directory"))?;
    let path = path.join("wqa");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create data directory")?;
    }
    Ok(path)
}

pub fn history_path() -> Result<PathBuf> {
    let path = data_dir()?;
    let path = path.join("history");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create history directory")?;
    }
    Ok(path)
}
pub fn measurements_dir() -> Result<PathBuf> {
    let mut path = history_path()?;
    let path = path.join("measurements");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create miasurements directory")?;
    }
    Ok(path)
}
pub fn signal_dir() -> Result<PathBuf> {
    let mut path = history_path()?;
    let path = path.join("signal");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create miasurements directory")?;
    }
    Ok(path)
}
pub fn module_dir() -> Result<PathBuf> {
    let path = data_dir()?;
    let path = path.join("modules");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create module directory")?;
    }
    Ok(path)
}

pub fn streams_dir() -> Result<PathBuf> {
    let path = data_dir()?.join("streams");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create streams directory")?;
    }
    Ok(path)
}

pub fn rules_dir() -> Result<PathBuf> {
    let path = data_dir()?.join("rules");
    if !path.exists() {
        fs::create_dir_all(&path).context("Failed to create rules directory")?;
    }
    Ok(path)
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct Workspace {
//     s: String,
// }

// impl Workspace {
//     #[inline]
//     pub fn db_path(&self) -> Result<PathBuf> {
//         Ok(paths::data_dir()?.join(self.s.to_string() + ".db"))
//     }

//     #[inline]
//     pub fn usage_human(&self) -> Result<String> {
//         let usage = self.usage()?;
//         Ok(bytesize::to_string(usage, false))
//     }

//     pub fn usage(&self) -> Result<u64> {
//         let blobs = BlobStorage::workspace(self)?;

//         let mut sum = fs::metadata(self.db_path()?)?.len();
//         for entry in fs::read_dir(blobs.path())? {
//             sum += fs::metadata(entry?.path())?.len();
//         }

//         Ok(sum)
//     }

//     pub fn delete(&self) -> Result<()> {
//         let blobs = BlobStorage::workspace(self)?;
//         fs::remove_dir_all(blobs.path())?;
//         fs::remove_file(self.db_path()?)?;
//         Ok(())
//     }
// }

// impl FromStr for Workspace {
//     type Err = Error;

//     fn from_str(s: &str) -> Result<Workspace> {
//         if s.is_empty() {
//             bail!("Workspace can't be empty")
//         }

//         lazy_static! {
//             static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9\._\-]*[a-zA-Z0-9])?$").unwrap();
//         }
//         if !RE.is_match(s) {
//             bail!("Workspace contains invalid characters")
//         }

//         Ok(Workspace {
//             s: s.into(),
//         })
//     }
// }

// use std::ops::Deref;
// impl Deref for Workspace {
//     type Target = String;

//     fn deref(&self) -> &String {
//         &self.s
//     }
// }

// pub fn list() -> Result<Vec<Workspace>> {
//     let mut workspaces = Vec::new();

//     for entry in fs::read_dir(paths::data_dir()?)? {
//         let entry = entry?;
//         let path = entry.path();
//         if path.is_dir() {
//             continue;
//         }
//         if path.extension() != Some(OsStr::new("db")) {
//             continue;
//         }

//         let name = match path.file_stem() {
//             Some(name) => name,
//             _ => continue,
//         };

//         let name = name.to_str()
//             .ok_or_else(|| format_err!("Workspace has invalid name: {:?}", name))?;

//         if let Ok(workspace) = Workspace::from_str(name) {
//             workspaces.push(workspace);
//         }
//     }

//     Ok(workspaces)
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_workspace() {
        // let x = Workspace::from_str("abc");
        // assert!(x.is_ok());
    }

}
