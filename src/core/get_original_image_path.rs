use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ImageDeliveryError {
    #[error("forbidden path")]
    ForbiddenPath,
    #[error("unknown root")]
    UnknownRoot,
    #[error("file not found")]
    FileNotFound,
}

pub(crate) fn get_original_image_path(path: &str) -> Result<PathBuf, ImageDeliveryError> {
    if let Ok(root_dir) = std::env::var("ROOT_DIR") {
        let path = PathBuf::from(root_dir).join(path);

        if path
            .components()
            .any(|x| x == std::path::Component::ParentDir)
        {
            Err(ImageDeliveryError::ForbiddenPath)
        } else if path.exists() {
            Ok(path)
        } else {
            Err(ImageDeliveryError::FileNotFound)
        }
    } else {
        Err(ImageDeliveryError::UnknownRoot)
    }
}
