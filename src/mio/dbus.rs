use crate::error::*;

// use once_cell::sync::OnceCell;
// use serde_json::Value;
// use std::num::ParseIntError;
// use std::str::FromStr;
// use futures::prelude::*;
// use super::mio::CanBus;
// use lazy_static::lazy_static;
use dbus;
use dbus::Error as DBusError;
use dbus::{BusType, Connection, Message};

// use std::{
    // sync::{Mutex},
//     pin::Pin,
// };
use lazy_static::lazy_static;
use std::sync::RwLock;
lazy_static! {

    // static ref CONN : Connection = Connection::get_private(BusType::System).unwrap();
    // static ref DATA : RwLock<Vec<f32>> = {
        // RwLock::new(Vec::new())
    // };
}

// pub async fn send_message(m : Connection) -> Message {
    // self.conn.send_with_reply_and_block(m, 2000).unwrap()
// }

// pub struct CanDBus{
    // conn: Connection,
// }


// impl CanDBus {
    // fn new_for_bus(bus:BusType)
// }

pub fn send_message(m: Message) -> Result<Message> {
    let conn = Connection::get_private(BusType::System).unwrap();
    conn.send_with_reply_and_block(m,2000)
}


fn dig18out_set(num:u8,val:bool) ->  Result<()> {
    send_message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append1(num).append1(val));
    Ok(())
}


fn get_ain01() -> Result<u16> {
    set.conn = Connectio
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn1").unwrap());
        r.get1().unwrap()
    }
    fn get_ain02(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn2").unwrap());
        r.get1().unwrap()
    }
    fn get_ain03(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn3").unwrap());
        r.get1().unwrap()
    }
    fn get_ain04(&mut self) -> u16 {
       let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn4").unwrap());
       r.get1().unwrap()
    }
    fn get_ain05(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetIn5").unwrap());
        r.get1().unwrap()
    }
    fn get_aout(&mut self) -> u16{
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetOut").unwrap());
        r.get1().unwrap()
    }
    fn set_aout(&mut self, val: u16) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "SetOut").unwrap().append1(val));
    }
    fn get_temp01(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp1").unwrap());
        r.get1().unwrap()
    }
    fn get_temp02(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp1").unwrap());
        r.get1().unwrap()
    }
     fn get_temp03(&mut self) -> u16 {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetTemp3").unwrap());
        r.get1().unwrap()
    }
    ///com.lar.nodes.Digital16
    fn get_dig18in(&mut self,num:u8) -> bool {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(num));
        bool::from(r.get1().unwrap())
    }
    fn get_dig18out(&mut self,num:u8) -> bool  {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital1", "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(num));
        bool::from(r.get1().unwrap())
    }
    fn set_dig18out(&mut self,num:u8,val:bool) {
        // let outdig = self.get_dig18out();
    }
     ///com.lar.nodes.Digital16
    fn get_dig19in(&mut self,num:u8) -> bool {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "GetDigitalIn").unwrap().append1(num));
        bool::from(r.get1().unwrap())
    }
    fn get_dig19out(&mut self,num:u8) -> bool  {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "GetDigitalOut").unwrap().append1(num));
        bool::from(r.get1().unwrap())
    }
    fn set_dig19out(&mut self,num:u8,val:bool) {
        // let outdig = self.get_dig18out();
       self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Digital2", "com.lar.nodes.Digital16", "SetDigitalOut").unwrap().append1(num).append1(val));
    }
    ///Uart.data
    fn get_uart01(&mut self) -> Vec<u8>  {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart1").unwrap());
        r.get1().unwrap()
    }
    fn get_uart02(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetUart2").unwrap());
        r.get1().unwrap()
    }
    fn get_uart03(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
        r.get1().unwrap()
    }
    fn get_uart04(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
        r.get1().unwrap()
    }
    fn get_uart05(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
        r.get1().unwrap()
    }
    fn get_uart06(&mut self) -> Vec<u8> {
        let r = self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap());
        r.get1().unwrap()
    }
fn set_uart01(&mut self, data: Vec<u8>) {
    self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
}
    fn set_uart02(&mut self, data: Vec<u8>) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Analog1", "com.lar.nodes.Analog1", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn set_uart03(&mut self, data: Vec<u8>) {
       self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn set_uart04(&mut self, data: Vec<u8>) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor1", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn set_uart05(&mut self, data: Vec<u8>) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn set_uart06(&mut self, data: Vec<u8>) {
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(String::from_utf8(data).unwrap()));
    }
    fn setup_uart03(&mut self, baut: u16){
       self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
    }
    fn setup_uart04(&mut self, baut: u16){
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
    }
    fn setup_uart05(&mut self, baut: u16){
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
    }
    fn setup_uart06(&mut self, baut: u16){
        self.message(Message::new_method_call( "com.lar.service.can", "/com/lar/nodes/Doppelmotor2", "com.lar.nodes.Doppelmotor3", "GetDigitalIn").unwrap().append1(baut));
    }
