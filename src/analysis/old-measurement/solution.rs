use serde_derive::{Deserialize, Serialize};
// use std::time::{Duration};

/// Concentation - Volumenkonzentration
/// WIKI: https://de.wikipedia.org/wiki/Volumenkonzentration
/// Die Gehaltsgröße Volumenkonzentration wird in der Regel nur dann benutzt, wenn die Reinstoffe vor dem Mischvorgang und die Mischphase denselben Aggregatzustand haben,
/// in der Praxis also vor allem bei Gasgemischen und bei Mischungen von Flüssigkeiten (Untergruppe der Lösungen).
/// Die Volumenkonzentration σi ist definiert als Wert des Quotienten aus dem Volumen Vi einer betrachteten Mischungskomponente i und dem Gesamtvolumen V der Mischphase:[1][2]
/// `sigma= V/Vi
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Concentration {
    pub mixing: f64,
    pub total: f64,
}

impl Concentration {
    pub fn quotient (&self) -> f64 {
        self.total/self.mixing
    }
}

///Solution standard
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct  Solution {
    pub volume: f64,
    pub concentration: Vec<Concentration>,
}


impl Default for Solution {
    fn default() -> Self {
        Self {
            volume: 2000.0,
            concentration:Vec::new() ,
        }
    }
}
