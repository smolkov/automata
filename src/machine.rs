#[allow(dead_code)]
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

use hex::{FromHex, ToHex};
// use hexlify::Hex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Visitor, Unexpected, Error};

// use settings::ron::Config;
#[derive(Hash, PartialEq,Debug, Eq, PartialOrd, Ord, Clone)]
pub struct MachineId([u8; 16]);
use super::Wqa;

struct IdVisitor;


impl MachineId {
    pub fn read() -> Result<MachineId,io::Error> {
        let mut buf = String::with_capacity(33);
        File::open("/etc/machine-id")
        .and_then(|mut f| f.read_to_string(&mut buf))
        .and_then(|bytes| if bytes != 32 && bytes != 33  {
            return Err(io::Error::new(io::ErrorKind::Other,
                "Wrong length of /etc/machine-id"));
        } else {
            let vec: Vec<u8> = FromHex::from_hex(&buf[..32])
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            let mut ar = [0u8; 16];
            ar.copy_from_slice(&vec);
            Ok(MachineId(ar))
        })
    }
}

impl FromStr for MachineId {
    type Err = String;
    fn from_str(hash: &str) -> Result<MachineId,String> {
        if hash.len() != 32 {
            return Err(String::from(
                "MachineId must be exactly 32 hex chars"));
        }
        let vec: Vec<u8> = FromHex::from_hex(&hash[..])
            .map_err(|e| format!("MachineId invalid: {}", e))?;
        let mut val = [0u8; 16];
        val.copy_from_slice(&vec);
        Ok(MachineId(val))
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Draft,
    QuickTOCuv,
    QuickTOCultra,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataSheet {
    pub model: Model,
    pub producted: u64,
    pub updated: u64,
    pub wartung: u64
}

#[derive(Clone, Debug)]
pub struct Machine {
    wqa:Wqa,

}


impl Machine {
    pub fn new(wqa: Wqa)-> Machine {
        Machine{
            wqa: wqa,
        }
    }

    // pub fn get_machine_id(&self) -> Result<MachineId,io:Error> {
        // let id : MachineId = MachineId::read()?;
        // id
    // }
    // pub fn set_serial(&mut self,id:Hid) {
        // self.id = id;
    // }
    // pub async fn data_sheet(&self) -> Result<Device> {
    // let device = DataSheet::load_no_fallback(super::Local::root_dir()?.join("device.ron"))?;
    // Ok(device)
// }
    // MachineId
}



// pub async fn change_status(status:Status) -> Result<(),WqaError> {
    // let mut device = get_local().await?;
    // device.set_status(status);
    // save_local(device)
// }

// pub async fn save(device: &Device) -> Result<()> {
//     device.write(super::Local::root_dir()?.join("device.ron"))?;
//     Ok(())
// }

// impl fmt::Debug for MachineId {
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "MachineId({})", Hex(&self.0))
    // }
// }

impl fmt::Display for MachineId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.write_hex(f)
    }
}

impl<'a> Visitor<'a> for IdVisitor {
    type Value = MachineId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("bytes or string")
    }
    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
        where E: Error
    {
        if value.len() == 16 {
            let mut array = [0u8; 16];
            array.copy_from_slice(value);
            Ok(MachineId(array))
        } else {
            return Err(E::invalid_length(value.len(), &self));
        }
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: Error
    {
        if value.len() == 32 {
            let vec: Vec<u8> = FromHex::from_hex(value)
                .map_err(|_| E::invalid_value(
                    Unexpected::Str(value), &"hex coded string"))?;
            let mut val = [0u8; 16];
            val.copy_from_slice(&vec);
            Ok(MachineId(val))
        } else {
            return Err(E::invalid_length(value.len(), &self));
        }
    }
}

impl<'a> Deserialize<'a> for MachineId {
    fn deserialize<D>(deserializer: D) -> Result<MachineId, D::Error>
        where D: Deserializer<'a>
    {
        deserializer.deserialize_any(IdVisitor)
    }
}

impl Serialize for MachineId {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        if ser.is_human_readable() {
            ser.serialize_str(&self.to_string())
        } else {
            ser.serialize_bytes(&self.0)
        }
    }
}

pub async fn read_machine_id() -> Result<MachineId,io::Error> {
    MachineId::read()
}
