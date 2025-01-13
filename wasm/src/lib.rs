use error::ConvertError;
use image::{DynamicImage, ImageFormat};
use js_sys::{Array, Function, Uint8Array};
use media_type::MediaType;
use pixlzr::{FilterType, Pixlzr};
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, Request, RequestInit, RequestMode, Response, Url};

mod error;
mod media_type;

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

fn write_image(
    img: &DynamicImage,
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
    img: DynamicImage,
    source_type: Option<ImageFormat>,
    target_type: Option<ImageFormat>,
) -> DynamicImage {
    let t = target_type.unwrap_or(ImageFormat::WebP);
    let i = if source_type == Some(ImageFormat::Hdr) {
        DynamicImage::ImageRgba8(img.to_rgba8())
    } else {
        img
    };
    match t {
        ImageFormat::Jpeg
        | ImageFormat::Qoi
        | ImageFormat::Farbfeld
        | ImageFormat::Pnm
        | ImageFormat::Tga => DynamicImage::ImageRgb8(i.to_rgb8()),
        ImageFormat::Ico => i.resize(256, 256, image::imageops::FilterType::Lanczos3),
        ImageFormat::OpenExr => DynamicImage::ImageRgba32F(i.to_rgba32f()),
        _ => i,
    }
}

#[wasm_bindgen(js_name = convertImage)]
pub async fn convert_image(
    file_input: &JsValue,
    src_type: &str,
    target_type: &str,
    compression: f32,
    cb: &Function,
) -> Result<JsValue, JsValue> {
    let this = JsValue::NULL;
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(10.0),
        &JsValue::from_str("Starting conversion"),
    );
    let file_data = if file_input.is_string() {
        let _ = cb.call2(
            &this,
            &JsValue::from_f64(35.0),
            &JsValue::from_str("Fetching image"),
        );
        fetch_image(&file_input.as_string().unwrap()).await?
    } else if file_input.is_instance_of::<Uint8Array>() {
        Uint8Array::new(file_input).to_vec()
    } else {
        return Err(JsValue::from_str(
            "Invalid input type. Must be a URL or Uint8Array.",
        ));
    };
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(50.0),
        &JsValue::from_str("Loading image"),
    );
    let img = load_image(&file_data, MediaType::from_mime_type(src_type))
        .map_err(|_| JsValue::from_str("Unknown file type"))?;
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(60.0),
        &JsValue::from_str("Processing image"),
    );
    let img = process_image(
        img,
        ImageFormat::from_mime_type(src_type),
        ImageFormat::from_mime_type(target_type),
    );
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(80.0),
        &JsValue::from_str("Converting image"),
    );
    let output = write_image(&img, ImageFormat::from_mime_type(target_type), compression)
        .map_err(|_| JsValue::from_str("Error writing image"))?;
    let final_format = ImageFormat::from_mime_type(target_type).unwrap_or(ImageFormat::WebP);
    let mime_type = MediaType::guess_mime_type(final_format);
    let array = Uint8Array::from(output.as_slice());
    let blob_parts = Array::new();
    blob_parts.push(&array);
    let blob_opts = BlobPropertyBag::new();
    blob_opts.set_type(mime_type);
    let blob = Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_opts)
        .map_err(|_| JsValue::from_str("Failed to create Blob"))?;
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|_| JsValue::from_str("Failed to create Blob URL"))?;
    let _ = cb.call2(
        &this,
        &JsValue::from_f64(100.0),
        &JsValue::from_str("Conversion complete"),
    );
    Ok(JsValue::from_str(&url))
}
