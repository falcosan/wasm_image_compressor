use image::ImageFormat;

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
}

fn from_custom_mime_type(mime_type: &str) -> Option<MediaType> {
    match mime_type {
        "image/farbfeld" => Some(MediaType::Raster(ImageFormat::Farbfeld)),
        _ => None,
    }
}
