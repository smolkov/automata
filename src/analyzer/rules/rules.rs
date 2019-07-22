use serde_derive::{Deserialize, Serialize};
use std::time::{Duration,SystemTime};
/// Channel sum parameter
///
// use std::{path::PathBuf,fmt};






/// Modus state.
#[derive(Clone, Deserialize, Serialize, PartialEq,Debug)]
pub enum Mode {
    Off,
    Interval(u64),
    Remote,
}



#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,
    Test,
}

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


impl Default for Rule {
    fn default() -> Self {

    }
}

impl Rule {
    pub fn new(id:u64) -> Rule {
        Self {
            id:id,
            online: false,
            prioritat: 0,
            name: "rule",
            statistic: Statistic::default(),
            inject: Inject::Signal,
            function: Function::None,
            output: Output::None,
        }
    }
}

pub async fn find_all() ->Result<Vec<Rule>,WqaError> {
    let path = store::rules_directory();
    let mut streams: Vec<Stream> = Vec::new();
    for entry in WalkDir::new(path).min_depth(1) {
        let entry = entry?;
        let metadate = entry.metadata()?;
        if metadate.is_dir(){
            match entry.file_name().to_str() {
                Some(sp) => {
                    let part: Vec<_> = sp.matches("rule").collect();
                    if part.len() > 0 {
                        streams.push(Stream::load_no_fallback(entry.into_path().join("config.ron"))?);
                    }
                },
                None => { }
            }
        }
    }
    Ok(streams)
}


pub async fn set_rule(rule: Rule) -> Result<(),WqaErrir> {
    
}


pub async fn set_stream() -> Result<(),WqaError> {

}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test_create() {
        File::create(&temp_file()).unwrap();
        let exercise = Exercise {
            path: PathBuf::from("example.rs"),
            mode: Mode::Test,
        };
        exercise.clean();
        assert!(!Path::new(&temp_file()).exists());
    }
}

//
