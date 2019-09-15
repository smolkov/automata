#[allow(dead_code)]
use std::fmt;
use std::fs;
// use std::io::{Read};
use std::str::FromStr;
use failure::{Fallible};

use hex::{FromHex, ToHex};
// use hexlify::Hex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Visitor, Unexpected, Error};

// use settings::ron::Config;
#[derive(Hash, PartialEq,Debug, Eq, PartialOrd, Ord, Clone)]
pub struct MachineId([u8; 16]);

struct IdVisitor;

impl MachineId {
    pub fn read() -> Fallible<MachineId> {
        let mut buf = String::with_capacity(33);
        let content = fs::read_to_string("/etc/machine-id")?;
        let vec: Vec<u8> = FromHex::from_hex(&buf[..32])?;
        let mut ar = [0u8; 16];
        ar.copy_from_slice(&vec);
        Ok(MachineId(ar))
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

// pub async fn read_machine_id() -> Fallible<MachineId> {
    // let mid = MachineId::read()?;
    // Ok(mid);
// }
