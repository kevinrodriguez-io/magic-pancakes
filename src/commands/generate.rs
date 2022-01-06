use crate::state::{layers_config::LayerConfiguration, nft_metadata::Metadata};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs;

fn read_json_template(json_template_path: &String) -> Metadata {
    let json_template_str = fs::read_to_string(json_template_path).unwrap();
    let json_template: Metadata = from_str(&json_template_str).unwrap();
    return json_template;
}

fn read_layers_config(layers_config_path: &String) -> HashMap<String, LayerConfiguration> {
    let layers_config_str = fs::read_to_string(layers_config_path).unwrap();
    let layers_config: HashMap<String, LayerConfiguration> = from_str(&layers_config_str).unwrap();
    return layers_config;
}

pub fn exec(
    amount: &u32,
    json_template_path: &String,
    layers_config_path: &String,
    layers_path: &String,
    output_path: &String,
    output_format: &String,
) {
    let json_template = read_json_template(json_template_path);
}
