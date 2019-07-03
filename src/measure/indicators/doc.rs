use serde_derive::{Deserialize, Serialize};
/// Dissolved Organic Carbon (DOC) – organic carbon remaining in a sample after filtering the sample, typically using a 0.45 micrometer filter.
 #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DOC{
    value: f64,
}