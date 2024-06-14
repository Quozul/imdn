use actix_web::{get, HttpResponse, Responder, web};
use image::ImageFormat;
use serde::Deserialize;

use crate::core::get_original_image_path::{get_original_image_path, ImageDeliveryError};
use crate::core::try_get_thumbnail::{ThumbnailError, try_get_thumbnail};

#[get("/api/thumbnail/{path:.*}")]
pub(crate) async fn get_thumbnail(
    file_name: web::Path<String>,
    web::Query(ImageParams { lte, format }): web::Query<ImageParams>,
) -> impl Responder {
    match get_original_image_path(file_name.as_str()) {
        Ok(original_path) => {
            let format = format
                .and_then(ImageFormat::from_extension)
                .unwrap_or(ImageFormat::Jpeg);
            let lte = lte.unwrap_or(512);

            match try_get_thumbnail(original_path, file_name.as_str(), format, lte) {
                Ok(bytes) => HttpResponse::Ok()
                    .content_type(format.to_mime_type())
                    .body(bytes),
                Err(ThumbnailError::CacheNotAvailable) => {
                    HttpResponse::ServiceUnavailable().finish()
                }
                Err(ThumbnailError::CannotGenerateThumbnail) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(ImageDeliveryError::ForbiddenPath) => HttpResponse::Forbidden().finish(),
        Err(ImageDeliveryError::UnknownRoot) => HttpResponse::ServiceUnavailable().finish(),
        Err(ImageDeliveryError::FileNotFound) => HttpResponse::NotFound().finish(),
    }
}

#[derive(Deserialize)]
struct ImageParams {
    lte: Option<u32>,
    format: Option<String>,
}
