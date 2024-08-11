use crate::core::readable_trait::ReadableTrait;
use crate::endpoints::error_code::ErrorCode;
use actix_web::HttpResponse;

impl From<Box<dyn ReadableTrait>> for HttpResponse {
    fn from(readable: Box<dyn ReadableTrait>) -> Self {
        match readable.get_bytes() {
            Ok(body) => HttpResponse::Ok()
                .content_type(readable.get_mime())
                .body(body),
            Err(_) => {
                HttpResponse::InternalServerError().json(ErrorCode::new("internal.server.error"))
            }
        }
    }
}
