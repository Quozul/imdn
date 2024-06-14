use std::path::PathBuf;

use actix_web::{get, HttpResponse, Responder, web};
use actix_web::http::header::ContentType;
use tracing::error;

use crate::core::get_original_image_path::{get_original_image_path, ImageDeliveryError};

#[get("/api/image/{path:.*}")]
pub(crate) async fn get_image(path: web::Path<String>) -> impl Responder {
    match get_original_image_path(path.as_str()) {
        Ok(original_path) => match read_original(original_path.clone()) {
            Ok(file) => {
                if let Some(mime) = get_mime(original_path) {
                    HttpResponse::Ok().content_type(mime.to_string()).body(file)
                } else {
                    HttpResponse::Ok()
                        .content_type(ContentType::octet_stream())
                        .body(file)
                }
            }
            Err(err) => {
                error!("{err}");
                HttpResponse::NotFound().finish()
            }
        },
        Err(ImageDeliveryError::ForbiddenPath) => HttpResponse::Forbidden().finish(),
        Err(ImageDeliveryError::UnknownRoot) => HttpResponse::ServiceUnavailable().finish(),
        Err(ImageDeliveryError::FileNotFound) => HttpResponse::NotFound().finish(),
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
