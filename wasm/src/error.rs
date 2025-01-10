use image::ImageError;

#[derive(thiserror::Error, Debug)]
pub enum ConvertError {
    #[error("Unknown file type: {0}")]
    UnknownFileType(String),
    #[error("Image library error: {0}")]
    LibError(#[from] ImageError),
    #[error("Parsing error: {0}")]
    ParseError(#[from] serde_json::Error),
}
