use std::path::PathBuf;

use poem::{handler, IntoResponse};
use poem::http::StatusCode;
use poem::web::Path;
use tracing::error;

use crate::core::get_original_image_path::{get_original_image_path, ImageDeliveryError};

#[handler]
pub(crate) async fn get_image(Path(path): Path<String>) -> poem::Response {
    match get_original_image_path(path.as_str()) {
        Ok(original_path) => match read_original(original_path.clone()) {
            Ok(file) => {
                if let Some(mime) = get_mime(original_path) {
                    file.with_content_type(mime.to_string()).into_response()
                } else {
                    file.into_response()
                }
            }
            Err(err) => {
                error!("{err}");
                StatusCode::NOT_FOUND.into_response()
            }
        },
        Err(ImageDeliveryError::ForbiddenPath) => StatusCode::FORBIDDEN.into_response(),
        Err(ImageDeliveryError::UnknownRoot) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
        Err(ImageDeliveryError::FileNotFound) => StatusCode::NOT_FOUND.into_response(),
    }
}

fn read_original(original_path: PathBuf) -> std::io::Result<Vec<u8>> {
    std::fs::read(original_path)
}

fn get_mime(original_path: PathBuf) -> Option<String> {
    original_path.extension().map(|ext| {
        let ext = ext.to_string_lossy().to_string();
        let ext = ext.as_str();
        mime_guess::from_ext(ext)
            .first_or_octet_stream()
            .to_string()
    })
}
