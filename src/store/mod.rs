//! Default structur
//! .
//! ├── /automata
//! │   ├── stats.ron
//! │   ├── automata.toml
//! │   ├──stream
//! │   │   └── 1
//! │   │   │   └── stream.ron
//! │   │       ├── 1
//! │   │       │   ├── channel.ron
//! │   ├── examples
//! │   │   ├── kaggle
//! │   │   │   └── schema-kaggle.toml
//! │   │   └── movies
//! │   │       ├── movies.csv
//! │   │       ├── README.md
//! │   │       └── schema-movies.toml
//! │   ├── LICENSE
//! │   ├── meilidb
//! │   │   ├── Cargo.toml
//! │   │   ├── examples
//! │   │   │   ├── create-database.rs
//! │   │   │   └── query-database.rs
//! │   │   └── src
//! │   │       ├── lib.rs
//! │   │       └── sort_by_attr.rs
//! │   ├── meilidb-core
//! │   │   ├── Cargo.toml
//! │   │   └── src
//! │   │       ├── automaton.rs
//! │   │       ├── criterion
//! │   │       │   ├── document_id.rs
//! │   │       │   ├── exact.rs
//! │   │       │   ├── mod.rs
//! │   │       │   ├── number_of_words.rs
//! │   │       │   ├── sum_of_typos.rs
//! │   │       │   ├── sum_of_words_attribute.rs
//! │   │       │   ├── sum_of_words_position.rs
//! │   │       │   └── words_proximity.rs
//! │   │       ├── distinct_map.rs
//! │   │       ├── lib.rs
//! │   │       ├── query_builder.rs
//! │   │       ├── reordered_attrs.rs
//! │   │       └── store.rs
//! //! .automata/
//! //!      streams/
//!             1/
//!               stream.ron
//!               1/
//!
//!      history/
//!      logs/
// use crate::Local;
// use failure::{format_err};
// use regex::Regex;
// use dirs;
// use std::fs;
// use walkdir::{WalkDir};
// use analyzer::Device;
// use analyzer::flow::*;
// use analyzer::*;
// use settings::ron::Config;
// use std::{
    // path::PathBuf,
// };
pub mod measurement;
pub mod airflow;
pub mod humidity;
pub mod pressure;
pub mod sensor;

pub mod rule;
pub mod stream;
pub mod channel;


// use crate::automata;

pub use crate::*;



// use self::measurement::MeasureStats;
// pub use self::rule;
// pub use self::stream;
// pub use self::channel;
// pub use self::machine_id;
// use super::mio::airflow::AirflowSetting;
// use super::calibration::*;

// use std::collections::HashMap;
// use analyzer::flow::MonitoringSetting;


// pub(crate) fn crate_source_dir(ex: &Experiment, tc: &Toolchain, krate: &Crate) -> PathBuf {
    // EXPERIMENT_DIR
        // .join(&ex.name)
        // .join("sources")
        // .join(tc.to_path_component())
        // .join(krate.id())
// }

// lazy_static! {
    // static ref HASHMAP: HashMap<u32, &'static str> = {
    //     let mut m = HashMap::new();
    //     m.insert(0, "foo");
    //     m.insert(1, "bar");
    //     m.insert(2, "baz");
    //     m
    // };
    // static ref WS:Workspace = Workspace::default();
    // static ref Func:
    // static ref COUNT: usize = HASHMAP.len();
    // static ref NUMBER: u32 = times_two(21);
    // static ref ROOTDIR: &'static str = ".";
// }





// #[derive(Debug, Clone)]
// pub struct Store {
//     // path: PathBuf,
//     automata : automata,

//     // list:
// }


// impl Store {
//     pub fn new(automata: automata) -> Store {
//         Store {
//             automata: automata,
//             // path: Local::root_dir().unwrap(),
//         }
//     }

// }

// pub async fn setup_root_directory() -> Result<()> {

    // Ok(())
// }

// pub async fn setup(path: &str) -> Result<()> {
//     let path = PathBuf::from(path);
//     let file = path.join("device.ron");
//     if !file.exists() {
//         Device::default().write(file)?;
//     }
//     let file = path.join("monitoring.ron");
//     if !file.exists() {
//         // FlowSetting::default().write(file)?;
//     }

    // let path = stream_dir()?.join("1");
    // if !path.exists() {

        // fs::create_dir_all(&path)?;
        // let st1 = Stream::default();
        // let ch1 = Channel::default();
        // stream_save(&st1).await?;
        // stream_channel_save(&st1,&ch1).await?
   // }
    // Ok(())
// }



// pub async fn get_flow_monitoring_settings() -> Result<MonitoringSetting> {
//     let settings = MonitoringSetting::load_no_fallback(Local::root_dir()?.join("flowmonitoring.ron"))?;
//     Ok(settings)
// }

// pub async fn set_flow_monitoring_settings(settings:MonitoringSetting) -> Result<()> {
//     settings.write(Local::root_dir()?.join("airflow.ron"))?;
//     Ok(())
// }

// pub async fn set_serial( serial: String ) -> Result<()> {
    // let mut device = get_().await?;
    // device.set_serial(serial);
    // device.write(Local::root_dir()?.join("device.ron"))?;
    // Ok(())
// }



// pub async fn measurement_evaluation_save() {}

// pub async fn stream_channel(stream:&Stream,channel:&Channel),



// pub async fn get_measure_stats() -> Result<MeasureStats> {
//     let path = Local::root_dir()?.join("mstats.ron");
//     let mstat = MeasureStats::load_no_fallback(path)?;
//     Ok(mstat)
// }


///next measurement
// pub async fn next_measurement() -> Result<MeasureStats> {
//     let path = Local::root_dir()?.join("mstat.ron");
//     let mut mstat = MeasureStats::load_no_fallback(path.clone())?;
//     mstat.counter +=1;
//     mstat.write(path)?;
//     Ok(mstat)
// }


// pub async fn

// pub async fn get_airflow_setting() -> Result<



// pub async fn airflow_input_set(airflow:Airflow) {

// }

// pub async fn

#[cfg(test)]
mod tests {
    use super::*;
    // use futures::executor::block_on;


    #[runtime::test]
    async fn test_valid_workspace() {
        // let path = Local::root_dir().unwrap();
        // println!("PATH:{:?}",path);
        // let first = stream::Stream::default();
        // stream::save(&first).await.unwrap();
        // let measrule = rule::Rule::new(1);
        // rule::save(measrule).await.unwrap();
        // let x = Workspace::from_str("abc");
        // assert!(x.is_ok());
    }

}



pub struct Store {
    node: Node,
}
