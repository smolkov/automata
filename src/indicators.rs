// use crate::schema::articles;
// use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "indicators"]
pub struct NewIndicator {
    pub stream: u32,
    pub channel:u32,
    pub count: u64,
    pub value: f64,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Indicator {
    pub id : u64,
    pub stream: u32,
    pub channel: u32,
    pub count: u64,
    pub value: f64
}


