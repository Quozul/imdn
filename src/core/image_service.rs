use crate::core::image::Image;
use crate::parse_cli::ImageSource;
use thiserror::Error;

#[derive(Clone)]
pub struct ImageService {
    image_source: ImageSource,
}

impl ImageService {
    pub fn new(root: ImageSource) -> Self {
        Self { image_source: root }
    }

    pub fn read_image(&self, requested_path: &str) -> Result<Image, ReadImageError> {
        match &self.image_source {
            ImageSource::Local { root_path } => {
                let path = root_path.join(requested_path);

                let complete_path = if path
                    .components()
                    .any(|x| x == std::path::Component::ParentDir)
                {
                    Err(ReadImageError::ForbiddenPath)
                } else if !path.exists() {
                    Err(ReadImageError::FileNotFound)
                } else if !path.is_file() {
                    Err(ReadImageError::ForbiddenPath)
                } else {
                    Ok(path)
                }?;

                Ok(Image::from_path(complete_path)?)
            }
            ImageSource::S3 { .. } => {
                panic!("S3 image service not supported");
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ReadImageError {
    #[error("forbidden path")]
    ForbiddenPath,
    #[error("file not found")]
    FileNotFound,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
