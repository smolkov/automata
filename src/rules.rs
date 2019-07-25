use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration,SystemTime};
// use crate::error::*;
// use crate::workspace as store;
use analyzer::Statistic;
use std::path::PathBuf;



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
