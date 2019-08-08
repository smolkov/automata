/// Pressure sensor
/// Anschlus:  `Analog:IN04`
/// Model:     `presurei877`
///
///
use super::node::Node;
use failure::Fallible;

use settings::ron::Config;
use serde::{Deserialize, Serialize};
// use crate::error::*;

lazy_static::lazy_static! {
    static ref INTERVAL:u64 = 2000;
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pressure {
    pub val: f32,
    pub brocken: bool,
}

impl Default for Pressure {
    fn default() -> Pressure {
        Pressure {
            val: 0.0,
            brocken: true,
        }

    }
}

pub struct Config {
    pub warn_level: f32,
    pub crit_level: f32,
    pub interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            warn_level:  300.0,
            crit_level:  600.0,
            interval:   0,
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
// use lazy_static::lazy_static;
// use std::sync::RwLock;
// lazy_static! {
//     static ref PRESSURE : RwLock<Pressure> = {
//         RwLock::new(Pressure::from_analog16(0))
//     };
//     static ref DATA : RwLock<Vec<f32>> = {
//         RwLock::new(Vec::new())
//     };
// }

/// Presure value model
///
/// fsr - full scale range
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct Pressure {
    // pub value: f32,
    // pub broken: bool,
    // pub pressure_warn_level:  f32,
    // pub pressure_crit_level:  f32,
    // pub monitorin_interval:   u64,
// }



impl Pressure {
    pub fn from_analog16(&mut self,value: u16)  {
        let voltage =  value as f32 / 4096.0 * 5.0;
        self.from_voltage(voltage);
    }
    pub fn from_voltage(&mut self, voltage: f32){
        self.brocken = voltage< 1.0 * 4.0 / 5.0;
        self.val = ((voltage - 1.0)  / (5.0 - 1.0))*1000.0;
    }
}

pub async fn is_pressure(path: &str) -> bool {
    let path = PathBuf::from(path).join("PRESSURE");
    let test = if path.exists() {
        true
    } else {
        false
    }
}



pub async fn last_value(path : &Path) -> Pressure {
   let status = Status::load_no_fallback(node.path())?;
   Ok(status)
}


pub async fn read_config(path: &Path) -> Fallible<Config> {
    let config: Config::load_no_fallback(path.join("config"))
}




// pub async fn setup() -> Result<(),MioError> {
    // Ok(())
// }
//
// pub async fn pressure_value() -> Result<Pressure, MioError> {
    // let analog_value  = io::analog_input16(0x4).await?;
    // Ok(Pressure::from_analog16(analog_value))
// }
//
