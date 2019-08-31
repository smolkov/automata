// use crate::schema::articles;
// use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

// #[derive(Insertable, Deserialize, Debug, Clone)]
// #[table_name = "indicator"]
// pub struct NewIndicator {
    // pub stream: u64,
    // pub channel:u64,
    // pub count: u64,
    // pub value: f64,
// }

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Indicator {
    pub id : u64,
    pub count: u64,
    pub timestamp:u64,
    pub value: f64,
}


