//! Environmental indicators 
//! Links:
//!     * https://en.wikipedia.org/wiki/Water_quality#Environmental_indicators}
//!     * https://en.wikipedia.org/wiki/Analysis_of_water_chemistry
//!     * https://en.wikipedia.org/wiki/Analytical_chemistry
//!     * https://en.wikipedia.org/wiki/Chemical_industry
//! Physical indicators
//!     * Water temperature
//!     * Specific conductance or electrical conductance (EC) or conductivity
//!     * Total suspended solids (TSS)
//!     * Transparency or turbidity
//!     * Total dissolved solids (TDS)
//!     * Odour of water
//!     * Color of water
//!     * Taste of water
//! Chemical indicators
//!     * pH
//!     * Biochemical oxygen demand (BOD)
//!     * Chemical oxygen demand (COD)
//!     * Dissolved oxygen (DO)
//!     * Total hardness (TH)
//!     * Heavy metals
//!     * Nitrate
//!     * Orthophosphates
//!     * Pesticides
//!     * Surfactants
//! Biological indicators
//!     Ephemeroptera
//!     Plecoptera
//!     Mollusca
//!     Trichoptera
//!     Escherichia coli (E. coli)
//!     Coliform bacteria
//!     Pimephales promelas (fathead minnow)
//!     Americamysis bahia (Mysid shrimp)
//! sea urchin
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
//! * Total Inorganic Carbon (TIC) – often referred to as inorganic carbon (IC), carbonate, bicarbonate, and dissolved carbon dioxide (CO2).
//! * Total Organic Carbon (TOC) – material derived from decaying vegetation, bacterial growth, and metabolic activities of living organisms or chemicals.
//! * Elemental Carbon (EC) – charcoal, coal, and soot. Resistant to analytical digestion and extraction, EC can be a fraction of either TIC or TOC depending on analytical approach.[9]
//! * Non-Purgeable Organic Carbon (NPOC) – commonly referred to as TOC; organic carbon remaining in an acidified sample after purging the sample with gas.
//! * Purgeable (volatile) Organic Compound (VOC) – organic carbon that has been removed from a neutral, or acidified sample by purging with an inert gas. These are the same compounds referred to as Volatile Organic Compounds (VOC) and usually determined by Purge and Trap Gas Chromatography.
//! * Dissolved Organic Carbon (DOC) – organic carbon remaining in a sample after filtering the sample, typically using a 0.45 micrometer filter.
//! * Suspended Organic Carbon – also called particulate organic carbon (POC); the carbon in particulate form that is too large to pass through a filter. 
//! 

//! TOC- Detection and quantification
//! Accurate detection and quantification are the most vital components of the TOC analysis process. Conductivity and non-dispersive infrared (NDIR) are the two common detection methods used in modern TOC analyzers.
//! * Conductivity
//!   There are two types of conductivity detectors, direct and membrane. 
//!   Direct conductivity provides an all-encompassing approach of measuring CO2.
//!   This detection method uses no carrier gas, is good at the parts per billion (ppb) ranges, 
//!   but has a very limited analytical range. Membrane conductivity relies upon the filtering of the CO2 prior to measuring it with a conductivity cell.
//!   Both methods analyze sample conductivity before and after oxidization, attributing this differential measurement to the TOC of the sample. 
//!   During the sample oxidization phase, CO2 (directly related to the TOC in the sample) and other gases are formed. The dissolved CO2 forms a weak acid,
//!   thereby changing the conductivity of the original sample proportionately to the TOC in the sample. Conductivity analyses assume that only CO2 is present within the solution.
//!   As long as this holds true, then the TOC calculation by this differential measurement is valid. However, depending on the chemical
//!   species present in the sample and their individual products of oxidation, they may present either a positive or a negative interference to the actual TOC value, 
//!   resulting in analytical error. Some of the interfering chemical species include Cl−, HCO3−, SO32−, SO2−, ClO2−, and H+. 
//!   Small changes in pH and temperature fluctuations also contribute to inaccuracy. Membrane conductivity analyzers have improved upon the direct conductivity approach by incorporating 
//!   the use of hydrophobic gas permeation membranes to allow a more “selective” passage of the dissolved CO2 gas and nothing else. This provides a more precise and accurate measurement 
//!   of the organics that were converted to CO2.[16]
//! 
//! * Non-dispersive infrared (NDIR)
//!   The non-dispersive infrared analysis (NDIR) method offers the only practical interference-free method for detecting CO2 in TOC analysis. 
//!   The principal advantage of using NDIR is that it directly and specifically measures the CO2 generated by oxidation of the organic carbon in the oxidation reactor, 
//!   rather than relying on a measurement of a secondary, corrected effect, such as used in conductivity measurements.
//!   Plot of atmospheric transmittance in part of IR region showing CO2 absorbing wavelengths
//!   A traditional NDIR detector relies upon flow-through-cell technology, where the oxidation product flows into and out of the detector continuously. 
//!   A region of absorption of infrared light specific to CO2, usually around 4.26 µm (2350 cm−1), is measured over time as the gas flows through the detector. 
//!   A second reference measurement that is non-specific to CO2 is also taken[clarification needed] and the differential result correlates to the CO2 concentration in the detector at that moment. 
//!   As the gas continues to flow into and out of the detector cell the sum of the measurements results in a peak that is integrated and correlated to the total CO2 concentration in the sample aliquot.
//!   A new advance of NDIR technology is Static Pressurized Concentration (SPC). 
//!   The exit valve of the NDIR is closed to allow the detector to become pressurized. Once the gases in the detector have reached equilibrium,
//!   the concentration of the CO2 is analyzed. This pressurization of the sample gas stream in the NDIR, a patented technique, allows for increased sensitivity 
//!   and precision by measuring the entirety of the oxidation products of the sample in one reading, compared to flow-through cell technology. The output signal
//!   is proportional to the concentration of CO2 in the carrier gas, from the oxidation of the sample aliquot. UV/ Persulfate oxidation combined with NDIR detection provides
//!   good oxidation of organics, low instrument maintenance, good precision at ppb levels, relatively fast sample analysis time and easily accommodates multiple applications,
//!   including purified water (PW), water for injection (WFI), CIP, drinking water and ultra-pure water analyses. 
 
pub enum TotalCarbon {
    TOC,
    TC,
    TIC,
    EC

}

pub enum Indicator {
    TotalCarbon(TotalCarbon),
    COD,
    NO,

}

pub mod tc;
pub mod toc;
pub mod tic;
pub mod no;
pub mod cod;
pub mod npoc;
pub mod poc;
pub mod voc;
pub mod ec;
pub mod doc;