// use wqa::mio;
// use hyper::Client;

use analyzer::{
    Airflow,
    Humidity,
    Pressure,
};
use crate::Result;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue,ACCEPT};



pub struct ApiClient {
    pub address: String,
}

impl ApiClient {
    pub fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let value = HeaderValue::from_str("application/json").unwrap();
        headers.insert(ACCEPT, value);
        headers
    }
    /// Get airflow_input
    pub async fn get_airflow_input(&self) -> Result<Airflow> {
        let airflow:Airflow = reqwest::Client::new()
            .get(format!("{}/airflow/input",self.address).as_str())
            .send()?
            .json()?;
        Ok(airflow)
    }
    /// Get airflow_output
    pub async fn get_airflow_output(&self) -> Result<Airflow>{
        let airflow:Airflow = reqwest::Client::new()
            .get(format!("{}/airflow/output",self.address).as_str())
            .send()?
            .json()?;
        Ok(airflow)
    }
    /// Get humidity
    pub async fn get_humidity(&self) -> Result<Humidity> {
        let humidity:Humidity = reqwest::Client::new()
            .get(format!("{}/humidity",self.address).as_str())
            .send()?
            .json()?;
        Ok(humidity)
    }

    /// Get pressure
    pub async fn get_pressure(&self) -> Result<Pressure>  {
        let pressure:Pressure = reqwest::Client::new()
            .get(format!("{}/pressure",self.address).as_str())
            .send()?
            .json()?;
        Ok(pressure)
    }

    /// Sample pump start
    pub async fn sample_pump_start(&self) -> Result<()> {
        reqwest::Client::new()
            .get(format!("{}/gp1/start",self.address).as_str())
            .send()?
            .json()?;
        Ok(())
    }

    /// Sample pump stop
    pub async fn sample_pump_stop(&self) -> Result<()> {
        reqwest::Client::new()
            .get(format!("{}/gp1/stop",self.address).as_str())
            .send()?
            .json()?;
        Ok(())
    }



    pub async fn open_sample1(&self) -> Result<()>  {
        reqwest::Client::new()
            .post(format!("{}/sample1",self.address).as_str())
            .send()?
            .json()?;
        Ok(())
    }
    pub async fn close_sample(&self) -> Result<()>  {
      reqwest::Client::new()
        .post(format!("{}/sample_close",self.address).as_str())
        .send()?
        .json()?;
        Ok(())
    }

    pub async fn open_calibration(&self) -> Result<()>  {
        reqwest::Client::new()
        .post(format!("{}/calibration",self.address).as_str())
        .send()?
        .json()?;
        Ok(())
    }

}

// async fn zeroflow_valve(cx: Context<State>) -> EndpointResult<()> {
//     let state:bool = cx.param(":state").client_err()?;
//     mio::zeroflow_valve(state).await.unwrap();
//     Ok(())
// }
// async fn tic_valve(cx: Context<State>) -> EndpointResult<()> {
//     let state:bool = cx.param(":state").client_err()?;
//     mio::tic_valve(state).await.unwrap();
//     Ok(())
// }
// async fn calibration_valve(cx: Context<State>) -> EndpointResult<()> {
//     let state:bool = cx.param(":state").client_err()?;
//     mio::calibration_valve(state).await.unwrap();
//     Ok(())
// }
// async fn get_ndir1(_cx: Context<State>) -> EndpointResult {
//     Ok(response::json(mio::get_ndir1_value().await.unwrap()))
// }
// async fn get_ndir2(_cx: Context<State>) -> EndpointResult {
//     Ok(response::json(mio::get_ndir2_value().await.unwrap()))
// }

// pub fn setup_routes(mut app: App<State>) -> App<State> {

//     app.at("/uv").nest(|api| {
//         api.at("/airflow/input").get(airflow_input);
//         api.at("/airflow/out").get(airflow_output);
//         api.at("/humidity").get(humidity);
//         api.at("/pressure").get(pressure);
//         api.at("/sample/:start").post(sample_pump);
//         api.at("/valve/1/:open").post(stream1_valve);
//         api.at("/valve/2/:open").post(stream2_valve);
//         api.at("/valve/3/:open").post(stream3_valve);
//         api.at("/valve/4/:open").post(stream4_valve);
//         api.at("/valve/5/:open").post(stream5_valve);
//         api.at("/valve/6/:open").post(stream6_valve);
//         api.at("/valve/zeroflow/:state").post(zeroflow_valve);
//         api.at("/valve/tic/:state").post(tic_valve);
//         api.at("/valve/calibration/:state").post(calibration_valve);
//         api.at("/ndir1").get(get_ndir1);
//         api.at("/ndir2").get(get_ndir2);
//         // api.at("/lamp").get(get_signal);
//         // api.at("/lamp/on").get(get_signal);
//         // api.at("/lamp/off").get(get_signal);
//         // api.at("/streams").get(stream)
//         // api.at("/info").get(device::get_info);
//     });
//     app
// }

pub async fn get_digital1_output(num:u8) -> Result<bool>{
   let dout :bool = reqwest::Client::new()
        .get(format!("https://127.0.0.1/can/digital1/output/{}",num).as_str())
        .send()?
        .json()?;
    Ok(dout)
}
pub async fn get_digital2_output(num:u8) -> Result<bool>{
   let dout :bool = reqwest::Client::new()
        .get(format!("https://127.0.0.1/can/digital2/output/{}",num).as_str())
        .send()?
        .json()?;
    Ok(dout)
}

pub async fn set_digital1_output(num:u8,val:bool) -> Result<bool>{
   let dout :bool = reqwest::Client::new()
        .post(format!("https://127.0.0.1/can/digital1/output/{}/{}",num,val).as_str())
        .send()?
        .json()?;
    Ok(dout)
}
pub async fn set_digital2_output(num:u8,val:bool) -> Result<bool>{
   let dout :bool = reqwest::Client::new()
        .post(format!("https://127.0.0.1/can/digital2/output/{}/{}",num,val).as_str())
        .send()?
        .json()?;
    Ok(dout)
}

pub async fn get_digital1_input(num:u8) -> Result<bool>{
   let dout :bool = reqwest::Client::new()
        .get(format!("https://127.0.0.1/can/digital1/input/{}",num).as_str())
        .send()?
        .json()?;
    Ok(dout)
}
pub async fn get_digital2_input(num:u8) -> Result<bool>{
   let dout :bool = reqwest::Client::new()
        .get(format!("https://127.0.0.1/can/digital2/input/{}",num).as_str())
        .send()?
        .json()?;
    Ok(dout)
}
