mod compression;
mod error;
mod media_type;

use compression::{convert_image_internal, parse_compression_factor};
use image::ImageFormat;
use js_sys::{Array, Uint8Array};
use media_type::MediaType;
use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, Url};

#[wasm_bindgen(js_name = convertImage)]
pub async fn convert_image(
    file_input: &JsValue,
    src_type: &str,
    target_type: &str,
    compression_factor: JsValue,
) -> Result<String, JsValue> {
    let compression = parse_compression_factor(&compression_factor);
    let output = convert_image_internal(file_input, src_type, target_type, compression).await?;
    let final_format = ImageFormat::from_mime_type(target_type).unwrap_or(ImageFormat::WebP);
    let mime_type = MediaType::guess_mime_type(final_format);
    let array = Uint8Array::from(output.as_slice());
    let blob_parts = Array::of1(&array);
    let blob_opts = BlobPropertyBag::new();
    blob_opts.set_type(mime_type);
    let blob = Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_opts)
        .map_err(|_| JsValue::from_str("Failed to create Blob"))?;
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|_| JsValue::from_str("Failed to create Blob URL"))?;
    Ok(url)
}

#[wasm_bindgen(js_name = convertImageAsUint8Array)]
pub async fn convert_image_as_uint8array(
    file_input: &JsValue,
    src_type: &str,
    target_type: &str,
    compression_factor: JsValue,
) -> Result<Uint8Array, JsValue> {
    let compression = parse_compression_factor(&compression_factor);
    let output = convert_image_internal(file_input, src_type, target_type, compression).await?;
    Ok(Uint8Array::from(output.as_slice()))
}
