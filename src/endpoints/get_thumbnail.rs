use crate::app_state::AppState;
use crate::core::thumbnail_service::ReadThumbnailError;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[get("/api/thumbnail/{path:.*}")]
pub async fn get_thumbnail(
    data: web::Data<AppState>,
    file_name: web::Path<String>,
    web::Query(ImageParams { lte, format }): web::Query<ImageParams>,
) -> impl Responder {
    let format = format.unwrap_or(String::from("jpg"));
    let lte = lte.unwrap_or(512);

    match data
        .thumbnail_service
        .read_thumbnail(file_name.as_str(), lte, format)
        .await
    {
        Ok(thumbnail) => thumbnail.into(),
        Err(ReadThumbnailError::FileNotFound) => HttpResponse::NotFound().finish(),
        Err(ReadThumbnailError::ForbiddenPath) => HttpResponse::Forbidden().finish(),
        Err(ReadThumbnailError::Io(_)) => HttpResponse::InternalServerError().finish(),
        Err(ReadThumbnailError::S3) => HttpResponse::ServiceUnavailable().finish(),
    }
}

#[derive(Deserialize)]
struct ImageParams {
    lte: Option<u32>,
    format: Option<String>,
}
