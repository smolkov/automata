/// Can dbus api.
///
/// ## Analog interface
///
/// ## DoppelMotor interface
///
/// ## Digital interface
///
/// ## Analog extension interface

use crate::error::*;
use analyzer::uv::*;
// use once_cell::sync::OnceCell;
// use serde_json::Value;
// use std::num::ParseIntError;
// use std::str::FromStr;
// use futures::prelude::*;
// use super::mio::CanBus;
// use lazy_static::lazy_static;
use dbus;
// use dbus::Error as DBusError;
use dbus::{BusType, Connection, Message};

// use std::{
    // sync::{Mutex},
//     pin::Pin,
// };
use lazy_static::lazy_static;
// use std::sync::{Mutex,Arc};
// use once_cell::sync::OnceCell;


lazy_static! {

    // static ref NEW : impl Future<Output = Result<Connection>> =
    // static ref DATA : RwLock<Vec<f32>> = {
        // RwLock::new(Vec::new())
    // };
}

// pub async fn send_message(m : Connection) -> Message {
    // self.conn.send_with_reply_and_block(m, 2000).unwrap()
// }
pub struct UvDBus {
    pub state:State,
    // conn: Connection,

}

// static CONNECTION: OnceCell<Connection> = OnceCell::new();



// impl Control for UvDBus {
//     type Error = Error;

//     fn status(&self) -> State{

//         self.state.clone()
//     }
//     fn lamp(&mut self,on:bool) -> Result<()> {
//         Ok(())
//     }
//     fn sample_pump(&mut self,start:bool) ->  Result<()> {
//         Ok(())
//     }
//     fn open_sample_valve(&mut self,sample: u8) ->  Result<()> {
//         Ok(())
//     }

//     fn close_sample_valve(&mut self,sample: u8) ->  Result<()> {
//          Ok(())
//     }
//     fn open_calibration_valve(&mut self) ->  Result<()> {
//           Ok(())
//     }
//     fn close_calibration_valve(&mut self) ->  Result<()> {
//           Ok(())
//     }
//     fn open_bypass(&mut self,open:bool)  -> Result<()> {
//           Ok(())
//     }
//     fn zeroflow(&mut self,open:bool) -> Result<()> {
//           Ok(())
//     }
//     fn tic(&mut self, open: bool) -> Result<()> {
//           Ok(())
//     }
//     fn fluid(&self,sample:u8) ->  Result<bool> {
//         Ok(false)
//     }
// }

pub struct CanDBus{
    conn: Connection,
}


impl CanDBus {
    fn new_for_bus(bus:BusType) -> CanDBus{
        let conn = Connection::get_private(BusType::System).unwrap();
        CanDBus {
            conn: conn
        }
    }
}


// pub fn setup_connection(conn:Connection) ->Result<()> {
    // CONNECTION.set(conn);
    // Ok(())
// }



pub fn send_message(m: Message) -> Result<Message> {
    let conn =  Connection::get_private(BusType::System)?;
    let r = conn.send_with_reply_and_block(m,2000)?;
    Ok(r)
}


/// get analog node ID:0x2 input01
pub async fn get_analog1_input01() -> Result<u16> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn1").unwrap())?;
    Ok(r.get1().unwrap())
}
/// get analog node ID:0x2 input02
pub async fn get_analog1_input02() ->  Result<u16> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn2").unwrap())?;
    Ok(r.get1().unwrap())
}
/// get analog node ID:0x2 input03
pub async fn get_analog1_input03() ->  Result<u16> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn3").unwrap())?;
    Ok(r.get1().unwrap())
}
/// get analog node ID:0x2 input04
pub async fn get_analog1_input04() ->  Result<u16> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn4").unwrap())?;
    Ok(r.get1().unwrap())
}
/// get analog node ID:0x2 input05
pub async fn get_analog1_input05() ->  Result<u16> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn5").unwrap())?;
    Ok(r.get1().unwrap())
}
pub async fn get_analog1_output01() ->  Result<u16>{
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetOut").unwrap())?;
    Ok(r.get1().unwrap())
}
pub async fn set_analog1_output(val: u16) -> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "SetOut").unwrap().append1(val))?;
    Ok(())
}
pub async fn get_analog1_temp01() -> Result<u16> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp1").unwrap())?;
   Ok(r.get1().unwrap())
}
pub async fn get_analog1_temp02() -> Result<u16> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp1").unwrap())?;
    Ok(r.get1().unwrap())
}
pub async fn get_analog1_temp03() -> Result<u16> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp3").unwrap())?;
    Ok(r.get1().unwrap())
}
///get analog node ID:0x2 UART01 Data
pub async fn get_analog1_uart01() ->Result<Vec<u8>>  {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart1").unwrap())?;
    Ok(r.get1().unwrap())
}
///get analog node ID:0x2 UART02 Data
pub async fn get_analog1_uart02() -> Result<Vec<u8>> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart2").unwrap())?;
    Ok(r.get1().unwrap())
}
///set analog node ID:0x2 UART01
pub async fn set_analog1_uart01(data: Vec<u8>) -> Result<()> {
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "SetUart1").unwrap().append1(String::from_utf8(data).unwrap()))?;
    Ok(())
}
///set analog node ID:0x2 UART02
pub async fn set_analog1_uart02(data: Vec<u8>) -> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "SetUart2").unwrap().append1(String::from_utf8(data).unwrap()))?;
    Ok(())
}

/// get digital1 node ID:0x18 inputN com.lar.nodes.Digital16
pub async fn get_digital1_input(num:u8) -> Result<bool> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(num))?;
    Ok(bool::from(r.get1().unwrap()))
}
/// get digital1 node ID:0x18 outputN com.lar.nodes.Digital16
pub async fn get_digital1_output(num:u8) -> Result<bool>  {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(num))?;
    Ok(bool::from(r.get1().unwrap()))
}
/// set digital1 node ID:0x18 outputN valur DBUS interface com.lar.nodes.Digital16
pub async fn set_digital1_output(num:u8,val:bool) ->  Result<()> {
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append2(num,val))?;
    Ok(())
}
/// get digital node2 ID:0x19 inputN DBus: /com/lar/nodes/Digital2 com.lar.nodes.Digital16
pub async fn get_digital2_input(num:u8) -> Result<bool> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(num))?;
    Ok(bool::from(r.get1().unwrap()))
}
/// get digital node2 ID:0x19 otputN
pub async fn get_digital2_output(num:u8) -> Result<bool>  {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(num))?;
    Ok(r.get1().unwrap())
}
/// set digital node2 ID:0x19 outputN set val
pub async fn set_digital2_output(num:u8,val:bool) -> Result<()> {
    // let outdig = self.get_dig18out();
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append2(num,val))?;
    Ok(())
}

pub async fn get_motor1_uart01() -> Result<Vec<u8>> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetUart1").unwrap())?;
    Ok(r.get1().unwrap())
}
pub async fn get_motor1_uart02() -> Result<Vec<u8>> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetUart2").unwrap())?;
    Ok(r.get1().unwrap())
}
pub async fn get_motor2_uart01() -> Result<Vec<u8>> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetUart1").unwrap())?;
    Ok(r.get1().unwrap())
}
pub async fn get_motor2_uart02() -> Result<Vec<u8>> {
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetUart2").unwrap())?;
    Ok(r.get1().unwrap())
}
pub async fn set_motor1_uart01(data: Vec<u8>) -> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "SetUart1").unwrap().append1(String::from_utf8(data).unwrap()))?;
    Ok(())
}
pub async fn set_motor1_uart02(data: Vec<u8>) -> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "SetUart2").unwrap().append1(String::from_utf8(data).unwrap()))?;
    Ok(())
}
pub async fn set_motor2_uart01(data: Vec<u8>)-> Result<()> {
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "SetUart1").unwrap().append1(String::from_utf8(data).unwrap()))?;
    Ok(())
}
pub async fn set_motor2_uart02(data: Vec<u8>) -> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "SetUart2").unwrap().append1(String::from_utf8(data).unwrap()))?;
        Ok(())
}
pub async fn set_motor1_bautrate01(baut: u32)-> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "SetBautrate1").unwrap().append1(baut))?;
    Ok(())
}
pub async fn setup_motor1_bautrate02(baut: u32)-> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "SetBautrate2").unwrap().append1(baut))?;
    Ok(())
}
pub async fn set_motor2_bautrate01(baut: u32)-> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "SetBautrate1").unwrap().append1(baut))?;
    Ok(())
}
pub async fn set_motor2_bautrate02(baut: u32)-> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "SetBautrate2").unwrap().append1(baut))?;
    Ok(())
}

pub async fn get_analogext_count() ->  Result<u32>{
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analogext", "com.lar.nodes.Analogext", "GetNout").unwrap())?;
    Ok(r.get1().unwrap())
}

pub async fn get_analogext_output(num:u8) ->  Result<u16>{
    let r = send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analogext", "com.lar.nodes.Analogext", "GetValue").unwrap().append1(num))?;
    Ok(r.get1().unwrap())
}
pub async fn set_analogext_output(num:u8,val: u16) -> Result<()>{
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analogext", "com.lar.nodes.Analogext", "SetValue").unwrap().append2(num,val))?;
    Ok(())
}
