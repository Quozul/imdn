use image::imageops::{resize, FilterType};
use image::{ImageFormat, RgbImage};
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::{Cursor, Read, Seek};
use thiserror::Error;

pub struct ThumbnailProcessor {
    pub requested_format: ImageFormat,
    pub largest_side: u32,
    pub cache_file: File,
}

impl ThumbnailProcessor {
    pub fn process_image(&mut self, bytes: &[u8]) -> Result<Vec<u8>, ReadThumbnailError> {
        let img = image::load_from_memory(bytes)
            .map_err(ReadThumbnailError::ImageError)?
            .into_rgb8();

        // Check the maximum size of the original image so we do not create an unnecessary big thumbnail
        let (width, height) = self.get_new_size(&img);
        let img = resize(&img, width, height, FilterType::Nearest);

        let mut cursor = Cursor::new(Vec::new());

        img.write_to(&mut cursor, self.requested_format)
            .map_err(ReadThumbnailError::ImageError)?;

        cursor
            .seek(io::SeekFrom::Start(0))
            .map_err(ReadThumbnailError::IoError)?;
        let mut buf = Vec::new();
        cursor.read_to_end(&mut buf).unwrap();

        // TODO: Async write to cache
        self.cache_file
            .write_all(&buf)
            .map_err(ReadThumbnailError::IoError)?;

        Ok(buf)
    }

    fn get_new_size(&self, img: &RgbImage) -> (u32, u32) {
        let (width, height) = img.dimensions();
        let scale = (self.largest_side as f32) / (width.max(height) as f32);
        if scale >= 1.0 {
            return (width, height);
        }
        (
            (width as f32 * scale).round() as u32,
            (height as f32 * scale).round() as u32,
        )
    }
}

#[derive(Error, Debug)]
pub enum ReadThumbnailError {
    #[error("{0}")]
    ImageError(image::ImageError),
    #[error("{0}")]
    IoError(io::Error),
}
