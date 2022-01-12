use crate::{
    state::{
        layers_config::{LayerConfiguration, LayerItem, PickedLayerItem},
        nft_metadata::{Attribute, Metadata},
    },
    tools::{
        image::{save_jpeg_with_quality, save_png_with_quality},
        mkdirp::mkdirp,
    },
};
use anyhow::Result;
use image::{
    imageops,
    png::{CompressionType, FilterType},
    ImageFormat,
};
use pbr::ProgressBar;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde_json::from_str;
use std::collections::HashMap;
use std::path::Path;
use std::{fs, path, sync::mpsc::channel};
use threadpool::ThreadPool;

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
    layers_config: HashMap<String, LayerConfiguration>,
) -> Result<HashMap<String, PickedLayerItem>> {
    let mut rng = thread_rng();
    let mut results: HashMap<String, PickedLayerItem> = HashMap::new();
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
    picked_layers: HashMap<String, PickedLayerItem>,
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
    quality: u8,
    compression_type: CompressionType,
    filter_type: FilterType,
) -> Result<String> {
    let mut items_sorted = items.clone();
    items_sorted.sort_by(|a, b| a.priority.cmp(&b.priority));

    let format = match output_format.as_str() {
        "png" => ImageFormat::Png,
        "jpg" => ImageFormat::Jpeg,
        "jpeg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::WebP,
        _ => ImageFormat::Png,
    };
    let result = match items_sorted.len() {
        0 => Err(anyhow::anyhow!("No layers to build image from")),
        1 => {
            let item = &items_sorted[0];
            let path = format!("{}/{}.{}", output_path, file_name_no_ext, output_format);
            image::open(path::Path::new(&item.uri))?.save_with_format(path.clone(), format)?;
            Ok(path)
        }
        _ => {
            let base_path = path::Path::new(&items_sorted[0].uri);
            let mut base_image = image::open(base_path)?;
            for i in 1..items_sorted.len() {
                let item_path = path::Path::new(&items_sorted[i].uri);
                let current_layer = image::open(item_path)?;
                imageops::overlay(&mut base_image, &current_layer, 0, 0);
            }
            let path = format!("{}/{}.{}", output_path, file_name_no_ext, output_format);
            match format {
                ImageFormat::Jpeg => {
                    save_jpeg_with_quality(&base_image, Path::new(&path), quality)?
                }
                ImageFormat::Png => save_png_with_quality(
                    &base_image,
                    Path::new(&path),
                    compression_type,
                    filter_type,
                )?,
                _ => base_image.save_with_format(Path::new(&path), format)?,
            }
            Ok(path)
        }
    };
    result
}

fn build_json(
    output_path: &String,
    file_name_no_ext: &String,
    json_template: Metadata,
    items: Vec<LayerItem>,
    output_format: &String,
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
    output_json.image = format!("{}.{}", file_name_no_ext, output_format);
    let output_json_str = serde_json::to_string(&output_json)?;
    let output_json_path = format!("{}/{}.json", output_path, file_name_no_ext);
    fs::write(output_json_path.clone(), output_json_str)?;
    Ok((output_json, output_json_path))
}

fn generate_item(
    layers_config: HashMap<String, LayerConfiguration>,
    json_template: Metadata,
    layers_path: &String,
    file_name_no_ext: &String,
    output_path: &String,
    output_format: &String,
    quality: u8,
    compression_type: CompressionType,
    filter_type: FilterType,
) -> Result<(Metadata, String, String)> {
    let picked_layers = pick_layers(layers_config)?;
    let picked_layer_items = get_picked_layer_item_uris(layers_path, picked_layers);

    // This thing here takes like 6s on a good CPU, how can we improve this?
    let image_path = build_image(
        output_path,
        &file_name_no_ext,
        picked_layer_items.clone(),
        &output_format,
        quality,
        compression_type,
        filter_type,
    )?;

    let (metadata, json_path) = build_json(
        output_path,
        &file_name_no_ext,
        json_template,
        picked_layer_items,
        &output_format,
    )?;

    Ok((metadata, image_path, json_path))
}

pub fn exec(
    amount: u32,
    json_template_path: String,
    layers_config_path: String,
    layers_path: String,
    output_path: String,
    output_format: String,
    unparsed_threads: Option<String>,
    unparsed_jpeg_quality: Option<u8>,
    unparsed_png_compression_type: Option<String>,
    unparsed_png_filter_type: Option<String>,
) {
    if (output_format != "jpg" && output_format != "jpeg") && !unparsed_jpeg_quality.is_none() {
        panic!("Quality is only supported for jpeg output");
    } else if output_format != "png" && !unparsed_png_compression_type.is_none()
        || !unparsed_png_filter_type.is_none()
    {
        panic!("Compression/Filter is only supported for png output");
    }

    let quality = match unparsed_jpeg_quality {
        Some(quality) => quality,
        None => 90,
    };
    let compression = match unparsed_png_compression_type {
        Some(compression) => match compression.as_str() {
            "best" => CompressionType::Best,
            "default" => CompressionType::Default,
            "fast" => CompressionType::Fast,
            "huffman" => CompressionType::Huffman,
            "rle" => CompressionType::Rle,
            _ => panic!("Unsupported compression type"),
        },
        None => CompressionType::default(),
    };
    let filter = match unparsed_png_filter_type {
        Some(filter) => match filter.as_str() {
            "avg" => FilterType::Avg,
            "nofilter" => FilterType::NoFilter,
            "paeth" => FilterType::Paeth,
            "sub" => FilterType::Sub,
            "up" => FilterType::Up,
            _ => panic!("Unsupported filter type"),
        },
        None => FilterType::NoFilter,
    };
    let threads = match unparsed_threads {
        Some(threads) => match threads.parse::<usize>() {
            Ok(threads) => {
                println!("🧵 Using {} threads", threads);
                threads
            }
            Err(_) => panic!("Unable to parse threads argument"),
        },
        None => num_cpus::get(),
    };

    println!("⛓ Reading dependencies...");
    let json_template = match read_json_template(&json_template_path) {
        Ok(result) => result,
        Err(e) => panic!("Error reading JSON Template: {}", e),
    };
    let layers_config = match read_layers_config(&layers_config_path) {
        Ok(result) => result,
        Err(e) => panic!("Error reading layers config: {}", e),
    };
    match mkdirp(Path::new(&output_path)) {
        Err(e) => panic!("Error mkdirp {}", e),
        _ => (),
    };

    println!("🪡 Building Thread Pool...");
    let pool = ThreadPool::new(threads);
    let (tx, rx) = channel();

    println!("🚀 Ready to go!");
    let mut pb = ProgressBar::new(amount as u64);
    pb.message("Building json/image pairs");

    for i in 0..amount {
        /*
            TODO: Find a more memory-efficient way of doing this.
            the good news is that it is super atomic,
            but it duplicates a lot of read-only values for each job.
        */
        let layers_config = layers_config.clone();
        let json_template = json_template.clone();
        let layers_path = layers_path.clone();
        let output_path = output_path.clone();
        let output_format = output_format.clone();
        let quality = quality.clone();
        let compression = compression.clone();
        let filter = filter.clone();

        let tx = tx.clone();
        pool.execute(move || {
            match tx.send(generate_item(
                layers_config,
                json_template,
                &layers_path,
                &i.to_string(),
                &output_path,
                &output_format,
                quality,
                compression,
                filter,
            )) {
                Ok(_) => (),
                Err(e) => panic!("Error sending to channel: {}", e),
            }
        });
    }
    rx.iter().for_each(|result| match result {
        Ok((_metadata, _image_path, _json_path)) => {
            pb.inc();
        }
        Err(e) => panic!("Error generating item: {}", e),
    });
}
