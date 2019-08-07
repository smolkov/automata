//! Mio async api
//!
//!

pub mod dbus;
// pub mod uv;
pub mod airflow;
pub mod humidity;
pub mod pressure;

pub mod uv;
use super::Wqa;
pub use self::dbus as io;
pub mod uart;

// use std::pin::Pin;
// use std::sync::Mutex;
// use std::thread;

// Lazily initialize a singleton router,
// so we only end up with one routing thread per process.
// lazy_static! {
//     static ref ROUTER: Router = {
//         let (send, mut recv) = futures::channel::mpsc::unbounded();
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
//         }
//     }
//     }
//         });
//         Router {
//             add_route: send,
//             wakeup: Mutex::new(waker),
//         }
    // };
// pub use self::dbus as io;

// pub struct Frame {
    // pub data: Vec<u8>,
// }




// pub use self::uv::*;
// pub use self::flow::*;
// pub use self::sensor::*;


// pub struct Node {
    // id: u64,
// }


#[derive(Clone, Debug)]
pub struct Mio {
    wqa: Wqa,


}



impl Mio  {
    pub fn new( wqa : Wqa ) -> Mio {
        Mio {
            wqa: wqa,
        }
    }
    // pub fn lamp() -> Result<Lamp> {

    // }
}

