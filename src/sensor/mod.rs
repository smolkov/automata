/// Gas sensors [Edinburgh] [Aide]
///
/// # Edinburgh
/// [Homepage]: https://edinburghsensors.com/
/// [Edinburgh]: https://edinburghsensors.com/products/oem-co2-sensor/gascard-ng
/// KEY FEATURES
/// On-board barometric pressure correction in the range 800mbar to 1150mbar.
/// Extensive temperature compensation.
/// Minimum operating voltage 7V and wide operating voltage range (7V to 30V).
/// True RS232 communications for control and data logging. Optional on-board LAN support.
/// # References
/// # GAS MEASUREMENT RANGE
/// Model	    CO2	CH4	CO
/// GasCard NG	-	0-5%	0-10%
/// GasCard NG	-	0-10%	0-30%
/// GasCard NG	0-2000 ppm	0-30%	0-100%
/// GasCard NG	0-3000 ppm	0-100%	-
/// CardCard NG	0-5000 ppm	-	-
/// GasCard NG	0-1%	-	-
/// GasCard NG	0-3%	-	-
/// GasCard NG	0-5%	-	-
/// GasCard NG	0-10%	-	-
/// GasCard NG	0-30%	-	-
/// GasCard NG	0-100%	-	-
/// Biogas	100%	100%	-
/// Accuracy	±2% of range ±<0.015% of range per mbar
/// Zero stability	±2% of range (over 12 months)
/// Response time	T90 = 10 seconds or programmable RC
/// Operating temperature	0-45ºC
/// Power requirements	24 V DC (7V-30V)
/// Warm-up time	1 minute (initial) 30 minutes (full specification)
/// Humidity	Measurements are unaffected by 0-95% relative humidity, non condensing
/// Output	Linear 4-20 mA, 0-20 mA (bit switch selectable) maximum load dependant on supply voltage
/// Please Note	Equipment is configured for one gas type at a time.
///
/// # Aide
/// [Homepage] http://www.analytische-instrumente.de
/// Digital infrared bench with high accuracy, stability and no moving parts plus ultra-sensitive electrochemical cell for NO-measurement.
/// Multi-channels IR-EC-System with automatically overflow-protection-system.
/// High resolution (16 bit ADC) and fast response.
/// Digital Interface RS 232, Analog Output.
/// Power supply 5 V DC/24 V DC.
///
/// Min. measuring range:	0 … 50 ppm CO2
/// 0 … 100 ppm NO (EC)
/// 0 … 2000 ppm NO (IR)
///
/// (Other gases and measuring ranges available)

///
///
///

use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use settings::ron::Config;
use csv;
use std::{
    fs,
    path::PathBuf,
};
use crate::Result;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
/// time in mili seconds
    pub time:   u64,
/// Pereodic
    pub value:  u64,
/// state
    pub state: u8,
}



impl Default for Record {
    fn default() -> Self {
        Self {
            start:  0,
            period: 300,
            data:  Vec::new(),
        }
    }
}

pub struct Aide {

}


/// Sensor model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Simulation,
    Edinburgh500,
    Edinburgh3000,
    Aide50_150,
    No,
    Zirox
}


impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::Simulation => write!(f, "simulation"),
            Model::Edinburgh500 => write!(f, "simulation"),
            Model::Edinburgh3000 => write!(f, "simulation"),
            Model::Aide => write!(f, "simulation"),
            _ => write!(f, "not supported"),
        }
    }
}



pub type Range = std::ops::Range<f64>;

/// Serial sensor.


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sensor {
    pub id: u64,
    pub measurement: u64
    pub interval: u64,
    pub range: Range,
    pub model: Model,
    pub len : usize,
}

impl Default for Sensor {
    fn default() -> Self {
        Sensor::new(Model::Simulation)
    }
}

impl Sensor {
    pub fn  new(id:u64) -> Sensor  {
        Sensor {
            interval : 0.0,
            model: model,
            range: Range { start: 0.0, end: 1.0 }
        }
    }
}



impl Default for Sensor {
    fn default() -> Sensor {
        Sensor::new()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AverageSignal {
//sart time
    pub start:   u64,
// Pereodic
    pub meas_id:  u64,
// Sensor data
    pub value  :  f64,
}


impl Default for AverageSignal{
    fn default() -> Self {
        Self {
            start:   0,
            meas_id: 0,
            value:   0.0,
        }
    }
}

impl AverageSignal {
    fn new(meas_id:u64) -> AverageSignal {
         Self {
            start:   0,
            meas_id: 0,
            value:   0.0,
        }
    }

}
// Lazily initialize a singleton router,
// so we only end up with one routing thread per process.
// lazy_static! {
//     static ref ROUTER: Router = {
//         let (send, mut recv) = futures::channel::mpsc::unbounded();
//         let (waker, wakee) = ipc::channel().expect("Failed to create IPC channel");
//         thread::spawn(move || {
//             let mut receivers = IpcReceiverSet::new().expect("Failed to create receiver set");
//             let mut senders = HashMap::<u64, UnboundedSender<OpaqueIpcMessage>>::new();
//             let _ = receivers.add(wakee);
//             while let Ok(mut selections) = receivers.select() {
//                 for selection in selections.drain(..) {
//                     match selection {
//                         IpcSelectionResult::MessageReceived(id, msg) => if let Some(sender) = senders.get(&id) {
//                             let _ = sender.unbounded_send(msg);
//                         },
//                         IpcSelectionResult::ChannelClosed(id) => {
//                             senders.remove(&id);
//                         },
//                     }
//                 }
//                 if !recv.is_terminated() {
//                     while let Ok(Some((receiver, sender))) = recv.try_next() {
//                         if let Ok(id) = receivers.add_opaque(receiver) {
//                             senders.insert(id, sender);
// 			}
//                     }
//                 }
//             }
//         });
//         Router {
//             add_route: send,
//             wakeup: Mutex::new(waker),
//         }
//     };
// }

// lazy_static!{
    // static ref
// }

// pub trait NDirs{
//     type Error;
//     fn ndir1(&self) -> Result<NDir,Self::Error>;
//     fn ndir2(&self) -> Result<NDir,Self::Error>;
//     fn ndir1_fsr(&self) -> Result<f64,Self::Error>;
//     fn ndir2_fsr(&self) -> Result<f64,Self::Error>;
//     fn ndir1_set_range(&self,range:NdirRange);
//     fn ndir2_set_range(&self,range:NdirRange);
// }





// lazy_static!{

// }

pub fn get_directory(id:u64)-> Result<PathBuf> {
    let path = super::Local::root_dir()?.join(format!("sensor/{}",num));
    if !path.exist() {
        fs::create_dir_all(path)?;
    }
    Ok(path)
}

pub async fn read(id:u64) -> Result<Sensor> {
    let path = get_directory(id)?.join("/sensor.ron");
    if !path.exist() {
        let sensor = Sensor::default();
        setting.write(path)?;
        Ok(setting)
    }
    Ok(0.0)
}
fn entry_is_sensor(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().ends_with("sensor.ron")
}

pub async fn read_all() -> Result<Vec<Sensor>> {
    let mut streams: Vec<Stream> = Vec::new();
    for entry in WalkDir::new(path).into_iter()
        .filter_entry(|e| entry_is_stream(e))
        .filter_map(|e| e.ok()) {
        let file_type = entry.file_type();
        if file_type.is_file(){
            let cfg = entry.path();
            streams.push(Stream::load_no_fallback(cfg)?);
        }
    }
    Ok(streams)
}

pub async fn simulation_value() -> Result<f64> {

    Ok(0.0)
}


pub fn reader(sensor: &Sensor) -> impl Future<Output = Result<f64>> {

}

// pub async fn ndir1_average() -> Result<

pub async fn average_signal(sensor: Sensor) -> Result<AverageSignal> {

    info!(" {}:{} calculate average signal {}",);
    let mut wtr = csv::Writer::from_path(get_directory()?.join("last.csv");)?;
    let read = reader(&sensor);
    for 
    let rec1 = Record { name: "Mark", place: "Melbourne", id: 56};
    let rec2 = Record { name: "Ashley", place: "Sydney", id: 64};
    let rec3 = Record { name: "Akshat", place: "Delhi", id: 98};

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;

    Ok(())

}

pub struct NDir{
    sensor: Sensor,
    dir: TempDir,

}

pub struct NDir {

}

pub struct Sensors {
    wqa: Wqa,

}

impl Sensors {
    pub fn new(wqa : Wqa ) -> Sensors {
        Sensors {
            wqa:wqa,
        }
    }
    pub fn list(&self) -> Result<Vec<Sensor>> {

    }
    pub fn taken_dir(sensor: Sensor) -> {

    }
}

// pub async fn
// pub struct Frame {
    // pub data: Vec<u8>,
// }



// pub use self::uv::*;
// pub use self::flow::*;
// pub use self::sensor::*;

