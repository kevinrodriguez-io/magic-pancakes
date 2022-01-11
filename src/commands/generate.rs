use crate::state::{
    layers_config::{LayerConfiguration, LayerItem, PickedLayerItem},
    nft_metadata::{Attribute, Metadata},
};
use crate::tools::mkdirp::mkdirp;
use anyhow::Result;
use image;
use image::imageops;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde_json::from_str;
use std::path::Path;
use std::{collections::HashMap, sync::atomic::AtomicU32};
use std::{fs, path, sync::Arc};

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
    format!("{}/{}/{}.png", layers_path, layer_name, item)
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
    output_format: &String,
) -> Result<String> {
    let format = match output_format.as_str() {
        "png" => image::ImageFormat::Png,
        "jpg" => image::ImageFormat::Jpeg,
        "webp" => image::ImageFormat::WebP,
        _ => image::ImageFormat::Png,
    };
    let result = match items.len() {
        0 => Err(anyhow::anyhow!("No layers to build image from")),
        1 => {
            let item = &items[0];
            let path = output_path.to_owned() + "/" + file_name_no_ext + "." + output_format;
            image::open(path::Path::new(&item.uri))?.save_with_format(path.clone(), format)?;
            Ok(path)
        }
        _ => {
            let base_path = path::Path::new(&items[0].uri);
            let mut base_image = image::open(base_path)?;
            for i in 1..items.len() {
                let item_path = path::Path::new(&items[i].uri);
                let current_layer = image::open(item_path)?;
                imageops::overlay(&mut base_image, &current_layer, 0, 0);
            }
            let path = output_path.to_owned() + "/" + file_name_no_ext + "." + output_format;
            base_image.save_with_format(path.clone(), format)?;
            Ok(path)
        }
    };
    result
}

fn build_json(
    output_path: &String,
    file_name_no_ext: &String,
    json_template: &Metadata,
    items: Vec<LayerItem>,
) -> Result<(Metadata, String)> {
    let mut output_json = json_template.clone();
    output_json.name = output_json.name + " - " + format!("{:0>4}", file_name_no_ext).as_str();
    output_json.attributes = items
        .into_iter()
        .map(|item| Attribute {
            trait_type: item.layer_name,
            value: item.item,
        })
        .collect();
    output_json.image = file_name_no_ext.to_owned() + ".jpeg";
    let output_json_str = serde_json::to_string(&output_json)?;
    let output_json_path = output_path.to_owned() + "/" + file_name_no_ext + ".json";
    fs::write(output_json_path.clone(), output_json_str)?;
    Ok((output_json, output_json_path))
}

fn generate_item(
    layers_config: &HashMap<String, LayerConfiguration>,
    layers_path: &String,
    file_name_no_ext: &String,
    output_path: &String,
    json_template: &Metadata,
    output_format: &String,
) -> Result<(Metadata, String, String)> {
    let picked_layers = pick_layers(layers_config)?;
    let picked_layer_items = get_picked_layer_item_uris(layers_path, picked_layers);
    let image_path = build_image(
        output_path,
        &file_name_no_ext,
        picked_layer_items.clone(),
        &output_format,
    )?;
    let (metadata, json_path) = build_json(
        output_path,
        &file_name_no_ext,
        json_template,
        picked_layer_items,
    )?;
    Ok((metadata, image_path, json_path))
}

pub fn exec(
    amount: &u32,
    json_template_path: &String,
    layers_config_path: &String,
    layers_path: &String,
    output_path: &String,
    output_format: &String,
    unparsed_threads: &Option<String>,
) {
    let threads = match unparsed_threads {
        Some(threads) => match threads.parse::<usize>() {
            Ok(threads) => threads,
            Err(_) => {
                eprintln!("Could not parse threads argument");
                std::process::exit(1);
            }
        },
        None => num_cpus::get(),
    };
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
    let completed: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
    for i in 0..*amount {
        println!("🛠 Generating item {}", i);
        match generate_item(
            &layers_config,
            layers_path,
            &i.to_string(),
            output_path,
            &json_template,
            &output_format,
        ) {
            Err(e) => panic!("❌ Error building item {}: {}", i, e),
            _ => println!("✅ Generated item {}", i),
        }
    }
}
