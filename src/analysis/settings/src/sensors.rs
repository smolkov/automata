#[derive(Debug, Deserialize, Serialize)]
pub struct Airflow {
    pub injection_treshold :f64,
    pub volume_deviation: f64,
    pub volume_level:f64,
    pub ignore_error: bool,
}



#[derive(Debug, Deserialize, Serialize)]
pub struct Humidity {
   pub max : f64, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pressure{
   pub max : f64, 
}