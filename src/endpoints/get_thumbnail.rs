use std::error::Error;
use std::path::PathBuf;
use std::thread::panicking;

use image::ImageFormat;
use image::imageops::FilterType;
use image::imageops::resize;
use poem::{handler, IntoResponse};
use poem::http::StatusCode;
use poem::web::{Path, Query};
use serde::Deserialize;
use thiserror::Error;
use tokio::fs;
use tracing::error;

use crate::core::get_original_image_path::{get_original_image_path, ImageDeliveryError};

#[derive(Error, Debug)]
pub enum ThumbnailError {
    #[error("error while generating the thumbnail")]
    CannotGenerateThumbnail,
    #[error("cache not available")]
    CacheNotAvailable,
}

#[handler]
pub(crate) async fn get_thumbnail(
    Path(file_name): Path<String>,
    Query(ImageParams { lte, format }): Query<ImageParams>,
) -> poem::Response {
    match get_original_image_path(file_name.as_str()) {
        Ok(original_path) => {
            let format = format
                .and_then(ImageFormat::from_extension)
                .unwrap_or(ImageFormat::Jpeg);
            let lte = lte.unwrap_or(512);

            match try_get_thumbnail(original_path, file_name, format, lte) {
                Ok(bytes) => bytes
                    .with_content_type(format.to_mime_type())
                    .into_response(),
                Err(ThumbnailError::CacheNotAvailable) => {
                    StatusCode::SERVICE_UNAVAILABLE.into_response()
                }
                Err(ThumbnailError::CannotGenerateThumbnail) => {
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(ImageDeliveryError::ForbiddenPath) => StatusCode::FORBIDDEN.into_response(),
        Err(ImageDeliveryError::UnknownRoot) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
        Err(ImageDeliveryError::FileNotFound) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Deserialize)]
struct ImageParams {
    lte: Option<u32>,
    format: Option<String>,
}

fn get_cache_key(lte: u32) -> String {
    format!("thumb_lte{lte}")
}

fn try_get_thumbnail(
    original_path: PathBuf,
    original_file_name: String,
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

fn try_get_cache_path(
    original_file_name: String,
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
    original_file_name: String,
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
