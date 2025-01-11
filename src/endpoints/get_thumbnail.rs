use crate::app_state::AppState;
use actix_web::{get, web, Responder};
use serde::Deserialize;

#[get("/api/thumbnail/{path:.*}")]
pub async fn get_thumbnail(
    data: web::Data<AppState>,
    file_name: web::Path<String>,
    web::Query(ImageParams { lte, format }): web::Query<ImageParams>,
) -> impl Responder {
    let format = format.unwrap_or(String::from("jpg"));
    let lte = lte.unwrap_or(512);

    data.thumbnail_service
        .read_thumbnail(file_name.as_str(), lte, format)
        .into_response()
        .await
}

#[derive(Deserialize)]
struct ImageParams {
    lte: Option<u32>,
    format: Option<String>,
}
