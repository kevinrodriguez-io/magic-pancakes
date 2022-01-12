use std::{fs::File, io::BufWriter, path::Path};

use image::{
    jpeg::JpegEncoder,
    png::{CompressionType, FilterType, PngEncoder},
    DynamicImage, GenericImageView, ImageEncoder, ImageResult,
};

pub fn save_jpeg_with_quality(image: &DynamicImage, path: &Path, quality: u8) -> ImageResult<()> {
    let out = &mut BufWriter::new(File::create(path)?);
    JpegEncoder::new_with_quality(out, quality).write_image(
        image.as_bytes(),
        image.width(),
        image.height(),
        image.color(),
    )
}

pub fn save_png_with_quality(
    image: &DynamicImage,
    path: &Path,
    compression_type: CompressionType,
    filter_type: FilterType,
) -> ImageResult<()> {
    let out = &mut BufWriter::new(File::create(path)?);
    PngEncoder::new_with_quality(out, compression_type, filter_type).write_image(
        image.as_bytes(),
        image.width(),
        image.height(),
        image.color(),
    )
}
