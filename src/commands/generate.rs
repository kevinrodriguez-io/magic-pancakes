use crate::state::{
    layers_config::{LayerConfiguration, LayerItem, PickedLayerItem},
    nft_metadata::Metadata,
};
use crate::tools::mkdirp::mkdirp;
use anyhow::Result;
use image;
use image::imageops;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde_json::from_str;
use std::{borrow::Borrow, collections::HashMap};
use std::{error::Error, path::Path};
use std::{fs, path};

fn read_json_template(json_template_path: &String) -> Result<Metadata> {
    let json_template_str = fs::read_to_string(json_template_path)?;
    let result: Metadata = from_str(&json_template_str)?;
    Ok(result)
}

fn read_layers_config(layers_config_path: &String) -> Result<HashMap<String, LayerConfiguration>> {
    let layers_config_str = fs::read_to_string(layers_config_path).unwrap();
    let result = from_str(&layers_config_str)?;
    Ok(result)
}

fn pick_layers(
    layers_config: &HashMap<String, LayerConfiguration>,
) -> Result<HashMap<&String, PickedLayerItem>> {
    let mut rng = thread_rng();
    let mut results: HashMap<&String, PickedLayerItem> = HashMap::new();
    for (key, value) in layers_config.into_iter() {
        let dist = WeightedIndex::new(&value.weights).unwrap();
        let chosen_one = &value.items[dist.sample(&mut rng)];
        results.insert(
            key,
            PickedLayerItem {
                picked_layer_item: chosen_one.clone(),
                priority: value.priority,
            },
        );
    }
    Ok(results)
}

fn get_picked_layer_item_uri(layers_path: &String, layer_name: &String, item: &String) -> String {
    layers_path.clone() + "/" + &layer_name + "/" + &item
}

fn get_picked_layer_item_uris(
    layers_path: &String,
    picked_layers: HashMap<&String, PickedLayerItem>,
) -> Vec<LayerItem> {
    picked_layers
        .into_iter()
        .map(|(layer_name, value)| LayerItem {
            layer_name: layer_name.to_owned(),
            item: value.picked_layer_item.to_owned(),
            priority: value.priority,
            uri: get_picked_layer_item_uri(&layers_path, &layer_name, &value.picked_layer_item),
        })
        .collect()
}

fn build_image(
    output_path: &String,
    file_name_no_ext: &String,
    items: Vec<LayerItem>,
) -> Result<()> {
    let result = match items.len() {
        0 => Err(anyhow::anyhow!("No layers to build image from")),
        1 => {
            let item = &items[0];
            image::open(path::Path::new(&item.uri))?.save_with_format(
                output_path.to_owned() + "/" + file_name_no_ext + ".jpeg",
                image::ImageFormat::Jpeg,
            )?;
            Ok(())
        }
        _ => {
            let mut base_image = image::open(path::Path::new(&items[0].uri))?;
            for i in 1..items.len() {
                let current_layer = image::open(path::Path::new(&items[i].uri))?;
                imageops::overlay(&mut base_image, &current_layer, 0, 0);
            }
            base_image.save_with_format(
                output_path.to_owned() + "/" + file_name_no_ext + ".jpeg",
                image::ImageFormat::Jpeg,
            )?;
            Ok(())
        }
    };
    return result;
}

fn build_json(output_path: &String, file_name_no_ext: &String, json_template: &Metadata) {
    let mut output_json = json_template.clone();
    // output_json.id = file_name_no_ext.to_owned();
    // output_json.uri = output_path.to_owned() + "/" + file_name_no_ext + ".jpeg";
    let output_json_str = serde_json::to_string(&output_json).unwrap();
    let output_json_path = output_path.to_owned() + "/" + file_name_no_ext + ".json";
    fs::write(output_json_path, output_json_str).unwrap();
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
    match mkdirp(Path::new(output_path)) {
        Err(e) => panic!("Error mkdirp {}", e),
        _ => (),
    };
    for i in 0..*amount {
        let picked_layers = match pick_layers(&layers_config) {
            Ok(result) => result,
            Err(e) => panic!("Error picking layers: {}", e),
        };
        let picked_layer_items = get_picked_layer_item_uris(layers_path, picked_layers);
        let file_name_no_ext = i.to_string();
        match build_image(output_path, &file_name_no_ext, picked_layer_items) {
            Err(e) => panic!("Error building image: {}", e),
            _ => (),
        };
    }
}
