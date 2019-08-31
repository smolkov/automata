use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration,SystemTime};
use failure::Fallible;
use analyzer::Statistic;
use std::path::PathBuf;
use settings::ron::Config;

use std::fs;
use walkdir::{WalkDir};
/// Modus state.
#[derive(Clone, Deserialize, Serialize, PartialEq,Debug)]
pub enum Mode {
    Off,
    Interval(u64),
    Remote,
}




///ExerciseList
#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Exercise {
    pub path: PathBuf,
    pub mode: Mode,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Remote {
    pub state: bool
}

#[derive(Deserialize,Serialize,Debug)]
pub enum Inject {
    Signal,
    Interval(u64),
    Planed(),
    Remote(Remote),
}
#[derive(Deserialize,Serialize,Debug)]
pub enum Function {
    None,
    Exercise(Exercise),
}

#[derive(Deserialize,Serialize,Debug)]
pub enum Output {
    None,
    Logger(),
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Config {
    pub online:     bool,
    pub prioritat:  u8,
    pub name:       String,
    pub statistic:  Statistic,
    pub inject:     Inject,
    pub function:   Function,
    pub output:     Output,
}


#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Rule {
    pub path: PathBuf,
}



impl Default for Rule {
    fn default() -> Self {
        Rule::new(1)
    }
}

impl Config {
    pub fn new() -> Rule {
        Self {
            online: false,
            prioritat: 0,
            name: format!("{}-rule",id),
            statistic: Statistic::default(),
            inject: Inject::Signal,
            function: Function::None,
            output: Output::None,
        }
    }
}

pub async fn read(path:Path) -> io::Result<Rule>{
    Rule{
        path: path,
    }
}


#[derive(Deserialize, Serialize,Debug)]
pub struct Rules {
    node: Node,
}


impl Rules {
    pub fn new(node: Node) -> Rules {
        Rules {
            node:node,
        }
    }
}
pub fn workdir() -> PathBuf {
    super::workdir().join("/rules/")
}

pub async fn rule(path: PathBuf) -> io::Result<Rule> {

}
/// 🗓 read all rules
pub async fn read_all( rules: &Rules) -> Result<Vec<Rule>> {

}


/// Get streams work directory
pub fn get_directory() -> Fallible<PathBuf> {
    let path = crate::local::rootdir()?;
    let path = path.join("rule");
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(path)
}

pub async fn read_all() ->Fallible<Vec<Rule>> {
    let path = get_directory()?;
    let mut rules = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let metadate = entry.metadata()?;
        if metadate.is_file(){
            rules.push(Rule::load_no_fallback(entry.into_path())?);
        }
    }
    Ok(rules)
}

pub async fn save(rule: Rule) -> Fallible<()> {
    let path = get_directory()?.join(format!("/{}-rule.ron",rule.id));
    rule.write(path.join("rule.ron"))?;
    Ok(())
}

pub async fn read(path: PathBuf) -> io::Result<Rule> {

}
    let rule = Rule::load_no_fallback(get_directory()?.join(format!("/{}-rule.ron",id)))?;
    Ok(rule)
}

// pub async fn set_stream() -> Result<(),automataError> {
// }

#[cfg(test)]
mod test {
    // use super::*;
    // use std::fs::File;
    // use std::path::Path;

    #[test]
    fn test_find_all() {

        // File::create(&temp_file()).unwrap();
        // let exercise = Exercise {
            // path: PathBuf::from("example.rs"),
            // mode: Mode::Test,
        // };
        // exercise.clean();
        // assert!(!Path::new(&temp_file()).exists());
    }
}

//
