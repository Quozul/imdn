use crate::core::image_service::{ImageService, ReadImageError};
use crate::core::readable_trait::ReadableTrait;
use crate::core::thumbnail::Thumbnail;
use image::ImageFormat;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Clone)]
pub struct ThumbnailService {
    image_service: ImageService,
    cache_directory: Option<PathBuf>,
}

impl ThumbnailService {
    pub fn new(image_service: ImageService, cache_directory: Option<PathBuf>) -> Self {
        Self {
            image_service,
            cache_directory,
        }
    }

    pub fn read_thumbnail(
        &self,
        requested_path: &str,
        lte: u32,
        requested_extension: String,
    ) -> Result<Box<dyn ReadableTrait>, ReadThumbnailError> {
        let format = ImageFormat::from_extension(requested_extension).unwrap_or(ImageFormat::Jpeg);
        let original_image = self.image_service.read_image(requested_path)?;

        Ok(Box::new(Thumbnail::new(
            requested_path.to_string(),
            format,
            original_image,
            lte,
            self.cache_directory.clone(),
        )))
    }
}

#[derive(Error, Debug)]
pub enum ReadThumbnailError {
    #[error("forbidden path")]
    ForbiddenPath,
    #[error("file not found")]
    FileNotFound,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<ReadImageError> for ReadThumbnailError {
    fn from(value: ReadImageError) -> Self {
        match value {
            ReadImageError::ForbiddenPath => Self::ForbiddenPath,
            ReadImageError::FileNotFound => Self::FileNotFound,
            ReadImageError::Io(err) => Self::Io(err),
        }
    }
}
