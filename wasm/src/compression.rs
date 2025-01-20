use crate::error::ConvertError;
use crate::media_type::MediaType;
use image::{DynamicImage, ImageFormat};
use js_sys::Uint8Array;
use pixlzr::{FilterType, Pixlzr};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Clone, Copy)]
pub(super) enum CompressionFactor {
    Value(f32),
    Skip,
}

pub fn parse_compression_factor(compression_factor: &JsValue) -> CompressionFactor {
    match compression_factor.as_f64() {
        Some(1.0) => CompressionFactor::Skip,
        Some(v) => CompressionFactor::Value(v as f32),
        None => CompressionFactor::Value(0.8),
    }
}

pub async fn convert_image_internal(
    file_input: &JsValue,
    src_type: &str,
    target_type: &str,
    compression_factor: CompressionFactor,
) -> Result<Vec<u8>, JsValue> {
    let file_data = match file_input {
        v if v.is_string() => fetch_image(&v.as_string().unwrap()).await?,
        v if v.is_instance_of::<Uint8Array>() => Uint8Array::new(v).to_vec(),
        _ => {
            return Err(JsValue::from_str(
                "Invalid input type. Must be a URL or Uint8Array.",
            ))
        }
    };

    let src_media_type = MediaType::from_mime_type(src_type);
    let img = load_image(&file_data, src_media_type)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;

    let src_format = ImageFormat::from_mime_type(src_type);
    let target_format = ImageFormat::from_mime_type(target_type);
    let processed_img = process_image(img, src_format, target_format);

    let output = parallel_write_image(
        &processed_img,
        target_format,
        compression_factor,
        &file_data,
    )
    .map_err(|e| JsValue::from_str(&format!("Error writing image: {}", e)))?;

    Ok(output)
}

async fn fetch_image(url: &str) -> Result<Vec<u8>, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or("No window available")?;
    let resp: Response = JsFuture::from(window.fetch_with_request(&request))
        .await?
        .dyn_into()?;
    let data = JsFuture::from(resp.array_buffer()?).await?;
    Ok(Uint8Array::new(&data).to_vec())
}

fn load_image(file: &[u8], source_type: Option<MediaType>) -> Result<DynamicImage, ConvertError> {
    match source_type {
        Some(MediaType::Raster(file_type)) => {
            image::load_from_memory_with_format(file, file_type).map_err(ConvertError::from)
        }
        None => image::load_from_memory(file)
            .map_err(|_| ConvertError::UnknownFileType("Failed to load image from memory".into())),
    }
}

fn process_image(
    img: DynamicImage,
    source_type: Option<ImageFormat>,
    target_type: Option<ImageFormat>,
) -> DynamicImage {
    let target = target_type.unwrap_or(ImageFormat::WebP);
    let img = if source_type == Some(ImageFormat::Hdr) {
        DynamicImage::ImageRgba8(img.to_rgba8())
    } else {
        img
    };
    match target {
        ImageFormat::Jpeg
        | ImageFormat::Qoi
        | ImageFormat::Farbfeld
        | ImageFormat::Pnm
        | ImageFormat::Tga => DynamicImage::ImageRgb8(img.to_rgb8()),
        ImageFormat::Ico => img.resize_exact(256, 256, image::imageops::FilterType::Lanczos3),
        ImageFormat::OpenExr => DynamicImage::ImageRgba32F(img.to_rgba32f()),
        _ => img,
    }
}

fn write_image(
    img: &DynamicImage,
    file_type: Option<ImageFormat>,
    compression_factor: CompressionFactor,
    original_data: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let target_type = file_type.unwrap_or(ImageFormat::WebP);
    let mut buffer = Vec::with_capacity(original_data.len());
    let final_img = match compression_factor {
        CompressionFactor::Value(compression) => {
            let mut pix = Pixlzr::from_image(img, 64, 64u32);
            pix.shrink_by(FilterType::Lanczos3, compression);
            pix.to_image(FilterType::Nearest)
        }
        CompressionFactor::Skip => img.clone(),
    };

    let format = match target_type {
        ImageFormat::Jpeg => ImageFormat::Jpeg,
        ImageFormat::Png => ImageFormat::Png,
        ImageFormat::WebP => ImageFormat::WebP,
        _ => ImageFormat::from(target_type),
    };

    final_img.write_to(&mut Cursor::new(&mut buffer), format)?;

    if buffer.len() > original_data.len() {
        Ok(original_data.to_vec())
    } else {
        Ok(buffer)
    }
}

fn parallel_write_image(
    img: &DynamicImage,
    file_type: Option<ImageFormat>,
    compression_factor: CompressionFactor,
    original_data: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    rayon::spawn_fifo(|| {});
    write_image(img, file_type, compression_factor, original_data)
}
