use crate::state::{layers_config::LayerConfiguration, nft_metadata::Metadata};
use crate::tools::mkdirp::mkdirp;
use anyhow::Result;
use serde_json::from_str;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn read_json_template(json_template_path: &String) -> Result<Metadata> {
    let json_template_str = fs::read_to_string(json_template_path).unwrap();
    let result: Metadata = from_str(&json_template_str)?;
    Ok(result)
}

fn read_layers_config(layers_config_path: &String) -> Result<HashMap<String, LayerConfiguration>> {
    let layers_config_str = fs::read_to_string(layers_config_path).unwrap();
    let result = from_str(&layers_config_str)?;
    Ok(result)
}

pub fn exec(
    amount: &u32,
    json_template_path: &String,
    layers_config_path: &String,
    layers_path: &String,
    output_path: &String,
    output_format: &String,
) {
    let json_template = match read_json_template(json_template_path) {
        Ok(result) => result,
        Err(e) => panic!("Error reading JSON Template: {}", e),
    };
    let layers_config = match read_layers_config(layers_config_path) {
        Ok(result) => result,
        Err(e) => panic!("Error reading layers config: {}", e),
    };
    let _ = match mkdirp(Path::new(output_path)) {
        Ok(result) => result,
        Err(e) => panic!("{}", e),
    };

}
