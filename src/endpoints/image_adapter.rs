use crate::core::image::Image;
use actix_web::HttpResponse;

impl From<Image> for HttpResponse {
    fn from(image: Image) -> Self {
        HttpResponse::Ok()
            .content_type(image.get_mime())
            .body(image.get_bytes().clone())
    }
}
