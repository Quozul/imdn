use crate::core::image::Image;
use crate::core::readable_trait::ReadableTrait;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Clone)]
pub struct ImageService {
    root: PathBuf,
}

impl ImageService {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn read_image(
        &self,
        requested_path: &str,
    ) -> Result<Box<dyn ReadableTrait>, ReadImageError> {
        let complete_path = self.get_complete_path(requested_path)?;
        Ok(Box::new(Image::new(complete_path)))
    }

    pub fn get_complete_path(&self, path: &str) -> Result<PathBuf, ReadImageError> {
        let path = PathBuf::from(&self.root).join(path);

        if path
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
        }
    }
}

#[derive(Error, Debug)]
pub enum ReadImageError {
    #[error("forbidden path")]
    ForbiddenPath,
    #[error("file not found")]
    FileNotFound,
}
