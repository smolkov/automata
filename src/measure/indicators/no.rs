//! 
//! `NOx`
//! Wiki: https://de.wikipedia.org/wiki/Stickoxide
//! 
//! Stickoxide (auch Stickstoffoxide[1]) ist ein Sammelbegriff für zahlreiche gasförmige Oxide des Stickstoffs. 
//! Stickstoffmonoxid (NO) und Stickstoffdioxid (NO2) werden als NOx zusammengefasst. 
//! In NOx-Gemischen höherer Konzentration kommen auch die Spezies N2O3 und N2O4 vor. 
//! Diese Gemische werden als nitrose Gase bezeichnet,[2] vor allem im Bereich Arbeitsschutz. 
//! In der Luftchemie werden NOx und andere Stickoxide mit einer Oxidationsstufe von 2 oder mehr, darunter auch Säuren und organische Spezies, zu NOy zusammengefasst.
//! [3] Für Distickstoffmonoxid (N2O) siehe Lachgas.


use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NOxItem {
    time: u64,
    value: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NOx{
    value: f64,
}


impl  NOx{
    pub fn new(value:f64) -> Self {
        Self {value}
    }
}


