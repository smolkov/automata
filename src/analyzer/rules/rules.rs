use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration,SystemTime};
use super::{
    Statistic,
};
use crate::error::*;
use crate::workspace as store;

use wqa_settings::ron::Config;
use std::{
    path::PathBuf,
    fs::{create_dir_all},
};

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



#[derive(Deserialize, Serialize,Debug)]
pub struct Rule {
    pub id:         u64,
    pub online:     bool,
    pub prioritat:  u8,
    pub name:       String,
    pub statistic:  Statistic,
    pub inject:     Inject,
    pub function:   Function,
    pub output:     Output,
}

#[derive(Deserialize, Serialize,Debug)]
pub struct RuleList {
    pub rules: Vec<Rule>,
}

impl Default for Rule {
    fn default() -> Self {
        Rule::new(1)
    }
}

impl Rule {
    pub fn new(id:u64) -> Rule {
        Self {
            id:id,
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

impl RuleList {
    pub fn new() -> RuleList {
        RuleList {
            rules: Vec::new(),
        }
    }
    pub fn push(&mut self, rule:Rule) {
        self.rules.push(rule);
    }
}

pub async fn find_all() ->Result<Vec<Rule>> {
    let path = store::rules_dir()?;
    let mut rules = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let metadate = entry.metadata()?;
        if metadate.is_dir(){
            rules.push(Rule::load_no_fallback(entry.into_path().join("rule.ron"))?);
        }
    }
    Ok(rules)
}

pub fn directory(id:u64) -> Result<PathBuf> {
    let path = store::rules_dir()?.join(format!("{}/",id));
    if !path.exists() {
        create_dir_all(path.as_path());
    }
    Ok(path)
}


pub async fn set_rule(rule: Rule) -> Result<()> {
    let path = directory(rule.id)?;
    rule.write(path.join("rule.ron"))?;
    Ok(())
}

pub async fn get_rule(id:u64) -> Result<Rule> {
    let rule = Rule::load_no_fallback(store::rules_dir()?.join(format!("{}/",id)).join("rule.ron"))?;
    Ok(rule)
}

// pub async fn set_stream() -> Result<(),WqaError> {
// }

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::path::Path;

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
