use crate::core::image::Image;
use actix_web::HttpResponse;

impl From<Image> for HttpResponse {
    fn from(image: Image) -> Self {
        match image.get_bytes() {
            Ok(body) => HttpResponse::Ok().content_type(image.get_mime()).body(body),
            Err(_) => HttpResponse::InternalServerError().json(
                crate::endpoints::error_code::ErrorCode::new("internal.server.error"),
            ),
        }
    }
}
