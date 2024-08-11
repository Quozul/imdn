use crate::core::readable_trait::ReadableTrait;
use image::imageops::{resize, FilterType};
use image::{ImageFormat, RgbImage};
use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub struct Thumbnail {
    original_path: PathBuf,
    format: ImageFormat,
    largest_side: u32,
    original_file_name: String,
    cache_directory: String,
}

impl ReadableTrait for Thumbnail {
    fn get_mime(&self) -> String {
        self.format.to_mime_type().to_string()
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let img = image::open(&self.original_path)
            .map_err(ReadThumbnailError::ImageError)?
            .into_rgb8();

        // Check the maximum size of the original image so we do not create an unnecessary big thumbnail
        let (width, height) = self.get_new_size(&img);
        let lte = width.max(height);

        match self.try_read_from_cache(lte) {
            Ok(bytes) => Ok(bytes),
            Err(Some(output_path)) => {
                if let Some(parent) = output_path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent).map_err(ReadThumbnailError::IoError)?;
                    }
                }

                let img = resize(&img, width, height, FilterType::Lanczos3);
                img.save_with_format(&output_path, self.format)
                    .map_err(ReadThumbnailError::ImageError)?;
                Ok(std::fs::read(output_path).map_err(ReadThumbnailError::IoError)?)
            }
            Err(None) => Err(Box::new(ReadThumbnailError::Oops)),
        }
    }
}

impl Thumbnail {
    pub fn new(
        original_file_name: String,
        original_path: PathBuf,
        format: ImageFormat,
        lte: u32,
        cache_directory: String,
    ) -> Self {
        Thumbnail {
            original_file_name,
            original_path,
            format,
            largest_side: lte,
            cache_directory,
        }
    }

    fn get_new_size(&self, img: &RgbImage) -> (u32, u32) {
        let (width, height) = img.dimensions();
        let scale = (self.largest_side as f32).min((width as f32).max(height as f32))
            / (width as f32).max(height as f32);
        let new_width = (width as f32 * scale) as u32;
        let new_height = (height as f32 * scale) as u32;

        (new_width, new_height)
    }

    fn try_get_cache_path(&self, cache_key: String) -> Option<PathBuf> {
        self.format.extensions_str().first().map(|new_extension| {
            let new_file_name = format!(
                "{}_{}.{}",
                self.original_file_name, cache_key, new_extension
            );
            PathBuf::from(&self.cache_directory).join(new_file_name)
        })
    }

    fn try_read_from_cache(&self, lte: u32) -> Result<Vec<u8>, Option<PathBuf>> {
        let cache_key = format!("thumb_lte{lte}");
        self.try_get_cache_path(cache_key)
            .map(|cached_path| {
                if cached_path.exists() {
                    std::fs::read(&cached_path).ok().ok_or(Some(cached_path))
                } else {
                    Err(Some(cached_path))
                }
            })
            .unwrap_or(Err(None))
    }
}

#[derive(Error, Debug)]
pub enum ReadThumbnailError {
    #[error("oops")]
    Oops,
    #[error("{0}")]
    ImageError(image::ImageError),
    #[error("{0}")]
    IoError(io::Error),
}
