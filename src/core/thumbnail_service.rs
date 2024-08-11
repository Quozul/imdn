use crate::core::image_service::{ImageService, ReadImageError};
use crate::core::readable_trait::ReadableTrait;
use crate::core::thumbnail::Thumbnail;
use image::ImageFormat;
use thiserror::Error;

pub struct ThumbnailService {
    image_service: ImageService,
    cache_directory: String,
}

impl ThumbnailService {
    pub fn new(image_service: ImageService, cache_directory: String) -> Self {
        Self {
            image_service,
            cache_directory,
        }
    }

    pub fn read_thumbnail(
        &self,
        requested_path: &str,
        lte: u32,
        format: String,
    ) -> Result<Box<dyn ReadableTrait>, ReadThumbnailError> {
        let format = ImageFormat::from_extension(format).unwrap_or(ImageFormat::Jpeg);
        let original_image = self
            .image_service
            .get_complete_path(requested_path)
            .map_err(ReadThumbnailError::from)?;

        Ok(Box::new(Thumbnail::new(
            requested_path.to_string(),
            original_image,
            format,
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
}

impl From<ReadImageError> for ReadThumbnailError {
    fn from(value: ReadImageError) -> Self {
        match value {
            ReadImageError::ForbiddenPath => Self::ForbiddenPath,
            ReadImageError::FileNotFound => Self::FileNotFound,
        }
    }
}
