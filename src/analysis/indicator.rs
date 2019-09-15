// use crate::schema::articles;
// use crate::schema::users;
// use chrono::NaiveDateTime;
// use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

// #[derive(Insertable, Deserialize, Debug, Clone)]
// #[table_name = "indicator"]
// pub struct NewIndicator {
    // pub stream: u64,
    // pub channel:u64,
    // pub count: u64,
    // pub value: f64,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Indicator {
    pub method : u64,
    pub measurement: u64,
    pub timestamp:u64,
    pub value: f64,
}



