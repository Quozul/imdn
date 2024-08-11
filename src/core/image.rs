use crate::core::readable_trait::ReadableTrait;
use mime_guess::mime;
use std::path::PathBuf;

pub struct Image {
    pub path: PathBuf,
}

impl ReadableTrait for Image {
    fn get_mime(&self) -> String {
        self.path
            .extension()
            .map(|ext| {
                let ext = ext.to_string_lossy().to_string();
                let ext = ext.as_str();
                mime_guess::from_ext(ext).first_or_octet_stream()
            })
            .unwrap_or(mime::APPLICATION_OCTET_STREAM)
            .to_string()
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Box<(dyn std::error::Error)>> {
        Ok(std::fs::read(&self.path)?)
    }
}

impl Image {
    pub fn new(path: PathBuf) -> Self {
        Image { path }
    }
}
