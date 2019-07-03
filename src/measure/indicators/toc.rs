//! `TOC`
//! Wiki: https://en.wikipedia.org/wiki/Total_organic_carbon
//! 
//! Total organic carbon (TOC) is the amount of carbon found in an organic compound and is often used as a non-specific indicator of water quality or cleanliness of pharmaceutical manufacturing equipment.
//! TOC may also refer to the amount of organic carbon in soil, or in a geological formation, particularly the source rock for a petroleum play; 2% is a rough minimum.[1]
//! For marine surface sediments, average TOC content is 0.5% in the deep ocean, and 2% along the eastern margins.[2]
//! A typical analysis for total carbon (TC) measures both the total carbon present and the so-called "inorganic carbon" (IC),
//! the latter representing the content of dissolved carbon dioxide and carbonic acid salts. Subtracting the inorganic carbon from the total carbon yields TOC.
//! Another common variant of TOC analysis involves removing the IC portion first and then measuring the leftover carbon. 
//! This method involves purging an acidified sample with carbon-free air or nitrogen prior to measurement, and so is more accurately called non-purgeable organic carbon (NPOC).[3]
//! Measurement
//! Relationship of carbon-content categories. A variety of different terms are used to identify the different types of carbon present at different levels of detail.
//! * Total Carbon (TC) – all the carbon in the sample, including both inorganic and organic carbon
//! * Total Organic Carbon (TOC) – material derived from decaying vegetation, bacterial growth, and metabolic activities of living organisms or chemicals.
//! * Elemental Carbon (EC) – charcoal, coal, and soot. Resistant to analytical digestion and extraction, EC can be a fraction of either TIC or TOC depending on analytical approach.[9]
//! * Non-Purgeable Organic Carbon (NPOC) – commonly referred to as TOC; organic carbon remaining in an acidified sample after purging the sample with gas.
//! * Purgeable (volatile) Organic Compound (VOC) – organic carbon that has been removed from a neutral, or acidified sample by purging with an inert gas. These are the same compounds referred to as Volatile Organic Compounds (VOC) and usually determined by Purge and Trap Gas Chromatography.
//! * Dissolved Organic Carbon (DOC) – organic carbon remaining in a sample after filtering the sample, typically using a 0.45 micrometer filter.
//! * Suspended Organic Carbon – also called particulate organic carbon (POC); the carbon in particulate form that is too large to pass through a filter. 
use serde_derive::{Deserialize, Serialize};
// use super::channel::{Channel};

use std::time::{SystemTime};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TOC{
    time:    u64,
    value:   f64,
}





