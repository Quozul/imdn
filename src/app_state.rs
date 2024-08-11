use crate::core::image_service::ImageService;
use crate::core::thumbnail_service::ThumbnailService;

pub struct AppState {
    pub image_service: ImageService,
    pub thumbnail_service: ThumbnailService,
}

impl Default for AppState {
    fn default() -> Self {
        let root = std::env::var("ROOT_DIR").unwrap(); // TODO: Get from CLI
        let cache = std::env::var("CACHE_DIR").unwrap(); // TODO: If not provided, use mktemp
        let image_service = ImageService::new(root);
        Self {
            image_service: image_service.clone(),
            thumbnail_service: ThumbnailService::new(image_service, cache),
        }
    }
}
