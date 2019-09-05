/// Monitor hardware caontrol system.
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration,SystemTime};
use log::{info,warn};
use lazy_static::lazy_static;
use serde_json::{from_slice,to_vec}
use failure::Fallible;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

use super::io;


#[derive(Serialize,Deserialize, Clone, Debug)]
pub enum Command {
    TurnOn,
    TurnOff,
}
pub struct State {
    pub lifetime: u64,
    pub on: bool,
}



pub struct Lamp {
    path: PathBuf,
}

impl Default for Lamp {
    fn default() -> Self {
        Self {
            uptime:SystemTime::now(),
            on: false,
            lifetime: Duration::from_secs(1),
        }
    }
}

impl Lamp {
    pub fn new(automata :automata) -> Lamp {
        automata.mio()
        Lamp{

        }
    }
    pub fn turn_on( &mut self) -> State{
         State{
            on:true,
            lifetime:0,

        }
    }
    pub fn turn_off(&mut self) -> State{
        State{
            on:true,
            lifetime:0,

        }
    }
    pub fn update_lifitime(&mut self){
        let now     = SystemTime::now();
        match now.duration_since(self.uptime) {
            Ok(uptime) if self.on => {
                self.lifetime += uptime;
                self.uptime    = SystemTime::now();
            },
            Ok(_)  => info!("UV Lamp turn off"),
            Err(e) => warn!("UV Lamp uptime:{:}",e),
        }
    }
    pub fn setup(&mut self,lifetime: u64) {
        self.lifetime = Duration::from_secs(lifetime);
    }
}

pub async fn workdir() -> PathBuf {
    let path = super::rootdir().join("/lamp/")
    if !path.exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(path.as_path())
            .await?;
        info!("{:} new creat", Paint::cyan("MIO:lamp"));
    }
    path
}

pub async fn send_command(lamp:&Lamp,cmd:Command) -> io::Result<()> {
    let path = lamp.path.join("cmd");
    let mut stream = UnixStream::connect(&path).await?;
    stream.write_all(to_vec(&cmd)?.as_slice()).await?;
    Ok(())
}

pub async fn state() -> io::Result<Lamp> {
    let path = workdir().join("state");
    let mut file = fs::File::open(path.as_path()).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let lamp: Lamp = from_slice(buf.as_slice())?;
    Ok(state)

}

// pub async fn command(lamp:&Lamp ) ->

pub async fn start(pump :&Pump ) -> io::Result<()> {
    let path = pump.mio.directory().join("state");
    let mut file = fs::File::create(path.as_path()).await?;
    let state    = serde_json::to_vec(&State::new().turn_on()).unwrap();
    file.write_all(state.as_slice()).await?;
    info!("UV-Lamp turn ON");
    Ok(())
}

pub async fn stop(pump:&Pump) -> Result<()> {
    let path = pump.mio.directory().join("state");
    let mut file = fs::File::create(path.as_path()).await?;
    let state    = serde_json::to_vec(&State::new().turn_on()).unwrap();
    file.write_all(state.as_slice()).await?;
    file.sync_data().await?;
    info!("UV-Lamp turn OFF");
    Ok(())
}

pub async fn sampling(pump:Pump,sec: u64) -> Result<()> {
    start(&pump).await?;
    task::sleep(Duration::from_secs(sec)).await;
    stop(&pump).await?;
    Ok(())
}
pub async fn set_lifetime(lifetime: u64) {
    UVLAMP.write().unwrap().set_lifetime(lifetime);
}

async fn status(lamp:Lamp) -> Result<Lamp,automataError> {

    Ok(Lamp::default())
}


// pub async fn turn_on() -> Result<(),automataError>{
//     UVLAMP.write().unwrap().turn_on();
//     Ok(())
// }

// pub async fn turn_off() -> Result<(),automataError>{
//     UVLAMP.write().unwrap().turn_off();
//     Ok(())
// }



// async fn get_lamp() -> EndpointResult {
    // let id: usize = cx.param("id").client_err()?;

    // if let Some(msg) = cx.app_data().messages().get(id) {
        // Ok(response::json(msg))
    // } else {
        // Err(StatusCode::NOT_FOUND)?
    // }
// }
