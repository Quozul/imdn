use crate::core::image_service::ReadImageError;
use crate::core::readable_trait::ReadableTrait;
use mime_guess::mime;
use std::path::PathBuf;

pub struct Image {
    original_bytes: Vec<u8>,
    mime_type: String,
}

impl ReadableTrait for Image {
    fn get_mime(&self) -> String {
        self.mime_type.clone()
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Box<(dyn std::error::Error)>> {
        Ok(self.original_bytes.clone())
    }
}

impl Image {
    pub fn from_path(path: PathBuf) -> Result<Self, ReadImageError> {
        let mime_type = get_format_from_path(&path);
        let original_bytes = std::fs::read(&path)?;
        Ok(Image {
            mime_type,
            original_bytes,
        })
    }

    pub fn from_bytes(requested_path: &str, original_bytes: Vec<u8>) -> Image {
        let mime_type = get_format_from_path(&PathBuf::from(requested_path));
        Image {
            mime_type,
            original_bytes,
        }
    }
}

fn get_format_from_path(path: &PathBuf) -> String {
    path.extension()
        .map(|ext| {
            let ext = ext.to_string_lossy().to_string();
            let ext = ext.as_str();
            mime_guess::from_ext(ext).first_or_octet_stream()
        })
        .unwrap_or(mime::APPLICATION_OCTET_STREAM)
        .to_string()
}
