								
// Messparameter – Online 		PS1	PS2	PS3	PS4	PS5	PS6		
									
// Messintervall									Wie definieren wir das für bis zu 6PS ?
// Measurement delay		0	0	0	0	0	0		startet die Messung Status M1 bsplw. Um irgendetwas anzutriggern
// Füllzeit UV-Reaktor		???	???	???	???	???	???		beschreibt im Grunde die t90- oder t100 Zeit für Tendenzänderung auch kürzer
// Zeit für Signalaufnahme		10	10	10	10	10	10		Messzeit für eine Signalaufnahme
// Signalabstand Probenstrom 1		1	1	1	1	1	1		alle wieviel Sekunden wird ein Signal innerhalb der Signalaufzeichnung genommen
// Anzahl Messsignale pro Messung		1	1	1	1	1	1		Anzahl Signalaufzeichnungen für eine Messwertbestimmung
									
									
									
	
// restliche Parameter, wie ausgewählter Probenstrom			

// 
// Serviceparameter	default		
			// 
// Volumenstrom Trägergas [l/h]	10		kann manuell gesetzt werden oder durch Kalibrierung automatisch
// prozentuale Abweichung [%]	20		wird genutzt um Fehler bei Über- oder Unterschreiten zu generieren
			// 
// relative Feuchte [%]	50		Alarmwert für Feuchtefehler
			// 
// Druck (optional) [mbar]	0 (aus)		bei Wert 0 findet keine Auswertung statt
// prozentual Abweichung [%]	0		


// Kalibrierparameter									

// Füllzeit Kalibrierlösung		???							Vorlaufzeit bis Lösung tatsächlich stabil bzw. t100 erreicht ist
// Füllzeit Nullgas		???							Füllzeit Trägergas Detektor für Autokalibrierung
// Signalabstand Kalibrierung		60							Messzeit für eine Signalaufnahme
// Anzahl Messsignale pro Messung		3							Anzahl der Signalaufnahmen pro Messwiederholung
// Messwiederholungen		5							Anzahl der Messwiederholungen pro Kalibrierlösung
// 									
// Service – Parameter									
// 									
// Grenzwert relative Feuchte	[%]								
// Einstellpunkt Trägergasdruck	[mbar]	1200							
// Abweichung Trägergasdruck	[%]	10							
// Einstellpunkt Trägergasdurchfluss	[l/h]	5							
// Abweichung Trägergasdurchfluss	[%]	20							
// 

pub struct UvToc {

}


pub struct Humidity {

}


pub struct UvSettings{
    pub serial: String,
/// Kalibrierparameter									
/// Füllzeit Kalibrierlösung		???							Vorlaufzeit bis Lösung tatsächlich stabil bzw. t100 erreicht ist
    pub kalibrationlosung_fullzeit: u64,
/// Füllzeit Nullgas		???							Füllzeit Trägergas Detektor für Autokalibrierung
    pub fullzeit_nullgas: u64,
/// Signalabstand Kalibrierung		60							Messzeit für eine Signalaufnahme
    pub signalaufnahmezeit: u64,

}


pub struct Service {
// Service – Parameter									
    pub relative_feuchte: f32,
// Einstellpunkt Trägergasdruck	[mbar]	120,0							
    pub tragegasdruck: f32 ,
// Abweichung Trägergasdruck	[%]	10							
    pub abweichungsdeuck: f32,
// Einstellpunkt Trägergasdurchfluss	[l/h]	5							
    pub volumenstrom: f32,
// Abweichung Trägergasdurchfluss	[%]	20						
    pub abweichung_tragegasdurchfluss: f32,
}


pub struct Measurement {
// 	Messparameter – Online 		PS1	PS2		
//  measurement_delay_time	Measurement delay [s]		0 – 300	0 – 300		startet die Messung Status M1 bsplw. Um irgendetwas anzutriggern

//  reactor_delay_time	Füllzeit UV-Reaktor [s]		1 – 1800	1 – 1800		beschreibt im Grunde die t90- oder t100 Zeit für Tendenzänderung auch kürzer
// 	TIC delay		?	?		
//  signal_count_time	Zeit für Signalaufnahme [s]		10 – 60	10 – 60		Messzeit für eine Signalaufnahme
//  signal_count	Anzahl Signalaufnahmen pro Messung		1-5	1-5		Anzahl Signalaufzeichnungen für eine Messwertbestimmung
						
// 	Gleitender Mittelwert		Off / 1 – 20	Off / 1 – 20		Mittelwert aus der aktuellen Messung + der letzten [1;2;3;4;5;10;15;20] Werte
// 	Prozentuale Abweichung [%]		5 – 500	5 – 500		wird eine prozentuale Abweichung des neuen Messwertes In bezug auf den Mittelwert der letzten Messungen überschritten Wird der neue Wert angezeigt, ansonsten geht er in die Mittelwertbildung ein

}



pub struct UvToc {
    pub name: String,
}