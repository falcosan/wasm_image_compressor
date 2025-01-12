use error::ConvertError;
use image::ImageFormat;
use js_sys::Uint8Array;
use media_type::MediaType;
use pixlzr::{FilterType, Pixlzr};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

mod error;
mod media_type;

fn load_image(
    file: &[u8],
    source_type: Option<MediaType>,
) -> Result<image::DynamicImage, ConvertError> {
    match source_type {
        Some(MediaType::Raster(file_type)) => {
            Ok(image::load_from_memory_with_format(file, file_type)?)
        }
        None => Ok(image::load_from_memory(file).map_err(|_| {
            ConvertError::UnknownFileType("Failed to load image from memory".into())
        })?),
    }
}

fn write_image(
    img: &image::DynamicImage,
    file_type: Option<ImageFormat>,
    compression_factor: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let target_type = file_type.unwrap_or(ImageFormat::WebP);
    let mut pix = Pixlzr::from_image(img, 64, 64);
    pix.shrink_by(FilterType::Lanczos3, compression_factor);
    let compressed_img = pix.to_image(FilterType::Nearest);
    let mut buffer = Vec::with_capacity(8192);
    compressed_img.write_to(&mut Cursor::new(&mut buffer), target_type)?;
    Ok(buffer)
}

fn process_image(
    img: image::DynamicImage,
    source_type: Option<ImageFormat>,
    target_type: Option<ImageFormat>,
) -> image::DynamicImage {
    let t = target_type.unwrap_or(ImageFormat::WebP);
    let i = match source_type {
        Some(ImageFormat::Hdr) => image::DynamicImage::ImageRgba8(img.to_rgba8()),
        _ => img,
    };
    match t {
        ImageFormat::Jpeg
        | ImageFormat::Qoi
        | ImageFormat::Farbfeld
        | ImageFormat::Pnm
        | ImageFormat::Tga => image::DynamicImage::ImageRgb8(i.to_rgb8()),
        ImageFormat::Ico => i.resize(256, 256, image::imageops::FilterType::Lanczos3),
        ImageFormat::OpenExr => image::DynamicImage::ImageRgba32F(i.to_rgba32f()),
        _ => i,
    }
}

#[wasm_bindgen(js_name = convertImage)]
pub fn convert_image(
    file: &Uint8Array,
    src_type: &str,
    target_type: &str,
    compression: f32,
    cb: &js_sys::Function,
) -> Result<Uint8Array, JsValue> {
    let this = JsValue::NULL;
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(10.0),
        &JsValue::from_str("Starting conversion"),
    );
    let file = file.to_vec();
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(35.0),
        &JsValue::from_str("Loading image"),
    );
    let img = load_image(&file, MediaType::from_mime_type(src_type))
        .map_err(|_| JsValue::from_str("unknown file type"))?;
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(50.0),
        &JsValue::from_str("Processing image"),
    );
    let img = process_image(
        img,
        ImageFormat::from_mime_type(src_type),
        ImageFormat::from_mime_type(target_type),
    );
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(70.0),
        &JsValue::from_str("Converting image"),
    );
    let output = write_image(&img, ImageFormat::from_mime_type(target_type), compression)
        .map_err(|_| JsValue::from_str("error writing image"))?;
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(100.0),
        &JsValue::from_str("Conversion complete"),
    );
    Ok(Uint8Array::from(output.as_slice()))
}
