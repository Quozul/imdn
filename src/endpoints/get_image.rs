use crate::app_state::AppState;
use crate::core::image_service::ReadImageError;
use crate::endpoints::error_code::ErrorCode;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/image/{path:.*}")]
pub async fn get_image(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    match data.image_service.read_image(path.as_str()) {
        Ok(cdn_image) => cdn_image.into(),
        Err(ReadImageError::FileNotFound) => {
            HttpResponse::NotFound().json(ErrorCode::new("not.found"))
        }
        Err(ReadImageError::ForbiddenPath) => {
            HttpResponse::Forbidden().json(ErrorCode::new("forbidden"))
        }
    }
}
