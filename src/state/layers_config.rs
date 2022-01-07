use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerConfiguration {
    pub items: Vec<String>,
    pub weights: Vec<f64>,
    pub priority: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PickedLayerItem {
    pub priority: u32,
    #[serde(rename = "pickedLayerItem")]
    pub picked_layer_item: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerItem {
    #[serde(rename = "layerName")]
    pub layer_name: String,
    pub item: String,
    pub uri: String,
    pub priority: u32,
}
