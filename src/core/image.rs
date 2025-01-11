use crate::core::image_service::ReadImageError;
use mime_guess::mime;
use std::path::{Path, PathBuf};

pub struct Image {
    original_bytes: Vec<u8>,
    mime_type: String,
}

impl Image {
    pub(crate) fn get_mime(&self) -> String {
        self.mime_type.clone()
    }

    pub(crate) fn get_bytes(&self) -> &Vec<u8> {
        &self.original_bytes
    }

    pub async fn from_path(path: PathBuf) -> Result<Self, ReadImageError> {
        let mime_type = get_format_from_path(&path);
        let original_bytes = tokio::fs::read(&path).await?;
        Ok(Image {
            original_bytes,
            mime_type,
        })
    }

    pub fn from_bytes(requested_path: &str, original_bytes: Vec<u8>) -> Image {
        let mime_type = get_format_from_path(&PathBuf::from(requested_path));
        Image {
            original_bytes,
            mime_type,
        }
    }
}

fn get_format_from_path(path: &Path) -> String {
    path.extension()
        .map_or(mime::APPLICATION_OCTET_STREAM, |ext| {
            let ext = ext.to_string_lossy().to_string();
            let ext = ext.as_str();
            mime_guess::from_ext(ext).first_or_octet_stream()
        })
        .to_string()
}
