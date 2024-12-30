use crate::core::image::Image;
use crate::core::readable_trait::ReadableTrait;
use crate::core::seekable_writer::{
    create_seekable_writer, create_seekable_writer_from_path, SeekableWriter,
};
use image::imageops::{resize, FilterType};
use image::{ImageFormat, RgbImage};
use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub struct Thumbnail {
    original_image: Image,
    requested_format: ImageFormat,
    largest_side: u32,
    original_file_name: String,
    cache_directory: Option<PathBuf>,
}

impl ReadableTrait for Thumbnail {
    fn get_mime(&self) -> String {
        self.requested_format.to_mime_type().to_string()
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let img = image::load_from_memory(&self.original_image.get_bytes()?)
            .map_err(ReadThumbnailError::ImageError)?
            .into_rgb8();

        // Check the maximum size of the original image so we do not create an unnecessary big thumbnail
        let (width, height) = self.get_new_size(&img);
        let lte = width.max(height);

        match self.try_read_from_cache(lte) {
            Ok(bytes) => Ok(bytes),
            Err(mut writer) => {
                let img = resize(&img, width, height, FilterType::Lanczos3);
                img.write_to(&mut writer, self.requested_format)
                    .map_err(ReadThumbnailError::ImageError)?;
                Ok(writer
                    .read_all_bytes()
                    .map_err(ReadThumbnailError::IoError)?)
            }
        }
    }
}

impl Thumbnail {
    pub fn new(
        original_file_name: String,
        requested_format: ImageFormat,
        original_image: Image,
        lte: u32,
        cache_directory: Option<PathBuf>,
    ) -> Self {
        Thumbnail {
            original_file_name,
            requested_format,
            original_image,
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
        match (
            self.requested_format.extensions_str().first(),
            self.cache_directory.clone(),
        ) {
            (Some(new_extension), Some(cache_directory)) => {
                let new_file_name = format!(
                    "{}_{}.{}",
                    self.original_file_name, cache_key, new_extension
                );
                Some(cache_directory.join(new_file_name))
            }
            _ => None,
        }
    }

    fn try_read_from_cache(&self, lte: u32) -> Result<Vec<u8>, Box<dyn SeekableWriter>> {
        let cache_key = format!("thumb_lte{lte}");
        self.try_get_cache_path(cache_key)
            .map(|cached_path| {
                if cached_path.exists() {
                    std::fs::read(&cached_path)
                        .ok()
                        .ok_or(create_seekable_writer_from_path(cached_path))
                } else {
                    Err(create_seekable_writer_from_path(cached_path))
                }
            })
            .unwrap_or(Err(create_seekable_writer()))
    }
}

#[derive(Error, Debug)]
pub enum ReadThumbnailError {
    #[error("{0}")]
    ImageError(image::ImageError),
    #[error("{0}")]
    IoError(io::Error),
}
