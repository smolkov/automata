/// Infrarot gas sensors [Edinburgh] , [AIDE]
///
/// # Edinburgh
/// [Homepage]: https://edinburghsensors.com/
/// [Edinburgh]: https://edinburghsensors.com/products/oem-co2-sensor/gascard-ng
/// KEY FEATURES
/// On-board barometric pressure correction in the range 800mbar to 1150mbar.
/// Extensive temperature compensation.
/// Minimum operating voltage 7V and wide operating voltage range (7V to 30V).
/// True RS232 communications for control and data logging. Optional on-board LAN support.
/// # References
/// # GAS MEASUREMENT RANGE
/// Model	    CO2	CH4	CO
/// GasCard NG	-	0-5%	0-10%
/// GasCard NG	-	0-10%	0-30%
/// GasCard NG	0-2000 ppm	0-30%	0-100%
/// GasCard NG	0-3000 ppm	0-100%	-
/// CardCard NG	0-5000 ppm	-	-
/// GasCard NG	0-1%	-	-
/// GasCard NG	0-3%	-	-
/// GasCard NG	0-5%	-	-
/// GasCard NG	0-10%	-	-
/// GasCard NG	0-30%	-	-
/// GasCard NG	0-100%	-	-
/// Biogas	100%	100%	-
/// Accuracy	±2% of range ±<0.015% of range per mbar
/// Zero stability	±2% of range (over 12 months)
/// Response time	T90 = 10 seconds or programmable RC
/// Operating temperature	0-45ºC
/// Power requirements	24 V DC (7V-30V)
/// Warm-up time	1 minute (initial) 30 minutes (full specification)
/// Humidity	Measurements are unaffected by 0-95% relative humidity, non condensing
/// Output	Linear 4-20 mA, 0-20 mA (bit switch selectable) maximum load dependant on supply voltage
/// Please Note	Equipment is configured for one gas type at a time.
///
/// # Aide
/// [Homepage] http://www.analytische-instrumente.de
/// Digital infrared bench with high accuracy, stability and no moving parts plus ultra-sensitive electrochemical cell for NO-measurement.
/// Multi-channels IR-EC-System with automatically overflow-protection-system.
/// High resolution (16 bit ADC) and fast response.
/// Digital Interface RS 232, Analog Output.
/// Power supply 5 V DC/24 V DC.
///
/// Min. measuring range:	0 … 50 ppm CO2
/// 0 … 100 ppm NO (EC)
/// 0 … 2000 ppm NO (IR)
///
/// (Other gases and measuring ranges available)

///
///
///

use super::*;
use crate::Result;




pub async fn uart_data() -> Result<f64> {

}

pub async fn simulation(sensor:Sensor) -> Result<f64> {

}

pub async fn value(sensor: Sensor) -> Result<f64> {

}


pub fn directory(id:u64)-> Result<PathBuf> {
    let path = crate::local::rootdir()?.join(format!("ndir:{}/",id));
    if !path.exist() {
        fs::create_dir_all(path)?;
    }
    Ok(path)
}


pub async fn setup(sensor: Sensor) -> Result<()> {

}

pub async fn read() -> Result<Sensor> {
    let path = directory(id)?.join("/config");
    if !path.exist() {
        let sensor = Sensor::default();
        setting.write(path)?;
        Ok(setting)
    }
    Ok(0.0)
}
pub async fn save(sensor: Sensor) -> Result<()> {

}

pub fn reader(sensor: Sensor) -> impl Future<Output = Result<f64>> {

}

// pub async fn ndir1_average() -> Result<

pub async fn average(sensor: Sensor) -> Result<AverageSignal> {

    info!(" {}:{} calculate average signal {}",);
    let mut wtr = csv::Writer::from_path(directory()?.join("signal"))?;

    let read = reader(&sensor);
    let rec1 = Record { name: "Mark", place: "Melbourne", id: 56};
    let rec2 = Record { name: "Ashley", place: "Sydney", id: 64};
    let rec3 = Record { name: "Akshat", place: "Delhi", id: 98};

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;

    Ok(())
}

