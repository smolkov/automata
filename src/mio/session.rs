use dbus::{BusType, ConnMsgs, Connection};

use crate::errror::*;


#[derive(Debug)]
pub struct DBusSession {
    connection: Connection,
}

impl DBusSession {
    pub fn create_session(path: Option<&str>) -> Result<CreateSession, Box<Error>> {
        let rule = {
            if let Some(path) = path {
                format!("{},path='{}'", BLUEZ_MATCH, path)
            } else {
                String::from(BLUEZ_MATCH)
            }
        };

        let c = try!(Connection::get_private(BusType::System));
        c.add_match(rule.as_str())?;
        Ok(BluetoothSession::new(c))
    }

    fn new(connection: Connection) -> BluetoothSession {
        BluetoothSession {
            connection: connection,
        }
    }

    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }

    pub fn incoming(&self, timeout_ms: u32) -> ConnMsgs<&Connection> {
        self.connection.incoming(timeout_ms)
    }
}
