use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerConfiguration {
    pub items: Vec<String>,
    pub weights: Vec<f64>,
    pub priority: i64,
}