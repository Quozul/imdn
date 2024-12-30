use crate::core::image_service::ImageService;
use crate::core::thumbnail_service::ThumbnailService;
use crate::parse_cli::Cli;

#[derive(Clone)]
pub struct AppState {
    pub image_service: ImageService,
    pub thumbnail_service: ThumbnailService,
}

impl AppState {
    pub fn new(args: Cli) -> Self {
        let root = args.image_source;
        let cache = args.cache_directory;
        let image_service = ImageService::new(root);
        Self {
            image_service: image_service.clone(),
            thumbnail_service: ThumbnailService::new(image_service, cache),
        }
    }
}
