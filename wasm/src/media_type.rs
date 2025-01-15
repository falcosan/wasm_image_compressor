use image::ImageFormat;

#[derive(Clone, Copy)]
pub enum MediaType {
    Raster(ImageFormat),
}
impl MediaType {
    pub fn from_mime_type(mime_type: &str) -> Option<Self> {
        match ImageFormat::from_mime_type(mime_type) {
            Some(format) => Some(Self::Raster(format)),
            None => from_custom_mime_type(mime_type),
        }
    }
    pub fn guess_mime_type(format: ImageFormat) -> &'static str {
        match format {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Avif => "image/avif",
            ImageFormat::WebP => "image/webp",
            ImageFormat::Ico => "image/x-icon",
            _ => "application/octet-stream",
        }
    }
}
fn from_custom_mime_type(mime_type: &str) -> Option<MediaType> {
    match mime_type {
        "image/farbfeld" => Some(MediaType::Raster(ImageFormat::Farbfeld)),
        _ => None,
    }
}
