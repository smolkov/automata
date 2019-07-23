//! QuickTOCuv analyzer
//!
//! Kalibrierparameter
//!
//! Füllzeit Kalibrierlösung		???							Vorlaufzeit bis Lösung tatsächlich stabil bzw. t100 erreicht ist
//! Füllzeit Nullgas		???							Füllzeit Trägergas Detektor für Autokalibrierung
//! Signalabstand Kalibrierung		60							Messzeit für eine Signalaufnahme
//! Anzahl Messsignale pro Messung		3							Anzahl der Signalaufnahmen pro Messwiederholung
//! Messwiederholungen		5							Anzahl der Messwiederholungen pro Kalibrierlösung
//!
//! Messparameter – Online 		        PS1	PS2	PS3	PS4	PS5	PS6
//!
//! Messintervall								                    Wie definieren wir das für bis zu 6PS ?
//! Measurement delay     	        	0	0	0	0	0	0		startet die Messung Status M1 bsplw. Um irgendetwas anzutriggern
//! Füllzeit UV-Reaktor		           ???	???	???	???	???	???		beschreibt im Grunde die t90- oder t100 Zeit für Tendenzänderung auch kürzer
//! Zeit für Signalaufnahme	        	10	10	10	10	10	10		Messzeit für eine Signalaufnahme
//! Signalabstand Probenstrom            1	1	1	1	1	1		alle wieviel Sekunden wird ein Signal innerhalb der Signalaufzeichnung genommen
//! Anzahl Messsignale pro Messung		1	1	1	1	1	1		Anzahl Signalaufzeichnungen für eine Messwertbestimmung
//!
//!
//!
//! Service – Parameter
//!
//! Grenzwert relative Feuchte	[%]
//! Einstellpunkt Trägergasdruck	[mbar]	1200
//! Abweichung Trägergasdruck	[%]	10
//! Einstellpunkt Trägergasdurchfluss	[l/h]	5
//! Abweichung Trägergasdurchfluss	[%]	20
//!
//! Parameter Einzelmessung		Bereich	default
//!
//! Füllzeit Einzelmessung		60-1800	600
//! Messwiederholungen		1 – 10	5
//! Ausreisser		0-3	0
//! cV [%]		0-10	2
//!
//! restliche Parameter, wie ausgewählter Probenstrom
//!
//! Serviceparameter	default
//!
//! Volumenstrom Trägergas [l/h]	10		kann manuell gesetzt werden oder durch Kalibrierung automatisch
//! prozentuale Abweichung [%]	20		wird genutzt um Fehler bei Über- oder Unterschreiten zu generieren
//!
//! relative Feuchte [%]	50		Alarmwert für Feuchtefehler
//!
//! Druck (optional) [mbar]	0 (aus)		bei Wert 0 findet keine Auswertung statt
//! prozentual Abweichung [%]	0

pub struct UvMethod {
    pub calibration_filltime: u64,
    
}

use serde_derive::{Deserialize, Serialize};
use super::sensors::{Range,Scale};




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub humidity_range:Range,
    pub airflow_range:Range,
    pub pressure_range:Range,
    pub ndir1: Scale,
    pub ndir2: Scale,
    pub no:Scale,
    pub zirox:Scale,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServerConfig {
    pub address: String,
    pub port: u32,
}


impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:8000".to_owned(),
            port:8000,

        }
    }
}
