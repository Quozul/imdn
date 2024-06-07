use std::error::Error;
use std::path::PathBuf;

use image::ImageFormat;
use image::imageops::{FilterType, resize};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub(crate) enum ThumbnailError {
    #[error("error while generating the thumbnail")]
    CannotGenerateThumbnail,
    #[error("cache not available")]
    CacheNotAvailable,
}

pub(crate) fn try_get_thumbnail(
    original_path: PathBuf,
    original_file_name: &str,
    format: ImageFormat,
    lte: u32,
) -> Result<Vec<u8>, ThumbnailError> {
    match try_read_from_cache(original_file_name, format, get_cache_key(lte)) {
        Ok(bytes) => Ok(bytes),
        Err(Some(output_cache_path)) => {
            match generate_thumbnail(&original_path, lte, format, output_cache_path) {
                Ok(bytes) => Ok(bytes),
                Err(e) => {
                    error!("Cannot generate thumbnail for {original_path:?}: {e}");
                    Err(ThumbnailError::CannotGenerateThumbnail)
                }
            }
        }
        _ => Err(ThumbnailError::CacheNotAvailable),
    }
}

fn get_cache_key(lte: u32) -> String {
    format!("thumb_lte{lte}")
}

fn try_get_cache_path(
    original_file_name: &str,
    new_format: ImageFormat,
    cache_key: String,
) -> Option<PathBuf> {
    if let Ok(cache_dir) = std::env::var("CACHE_DIR").map(PathBuf::from) {
        if let Some(new_extension) = new_format.extensions_str().first() {
            let new_file_name = format!("{}_{}.{}", original_file_name, cache_key, new_extension);
            Some(cache_dir.join(new_file_name))
        } else {
            None
        }
    } else {
        None
    }
}

fn try_read_from_cache(
    original_file_name: &str,
    new_format: ImageFormat,
    cache_key: String,
) -> Result<Vec<u8>, Option<PathBuf>> {
    if let Some(cached_path) = try_get_cache_path(original_file_name, new_format, cache_key) {
        if cached_path.exists() {
            std::fs::read(cached_path.clone()).map_err(|_| Some(cached_path))
        } else {
            Err(Some(cached_path))
        }
    } else {
        Err(None)
    }
}

fn generate_thumbnail(
    original_path: &PathBuf,
    max_size: u32,
    format: ImageFormat,
    output_path: PathBuf,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::open(original_path)?.into_rgb8();

    let (width, height) = img.dimensions();
    let scale = (max_size as f32).min((width as f32).max(height as f32))
        / (width as f32).max(height as f32);
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;

    let img = resize(&img, new_width, new_height, FilterType::Lanczos3);

    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    img.save_with_format(&output_path, format)?;
    Ok(std::fs::read(output_path)?)
}
