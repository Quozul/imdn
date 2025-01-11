use crate::core::image::Image;
use crate::core::image_service::{ImageService, ReadImageError};
use crate::core::seekable_writer::create_seekable_writer_from_path;
use crate::core::thumbnail_processor::ThumbnailProcessor;
use actix_web::HttpResponse;
use image::ImageFormat;
use std::path::PathBuf;

pub struct Thumbnail {
    image_service: ImageService,
    requested_format: ImageFormat,
    largest_side: u32,
    original_file_name: String,
    cache_directory: Option<PathBuf>,
}

impl Thumbnail {
    pub fn get_mime(&self) -> String {
        self.requested_format.to_mime_type().to_string()
    }

    pub async fn get_bytes(&self) -> anyhow::Result<Vec<u8>> {
        match self.try_read_from_cache(self.largest_side).await {
            Ok(bytes) => Ok(bytes),
            Err(Some(writer)) => {
                let image = self.get_original_image().await?;
                let mut thumbnail_processor = ThumbnailProcessor {
                    largest_side: self.largest_side,
                    requested_format: self.requested_format,
                    cache_file: writer.into_std().await,
                };
                let result_bytes = actix_web::web::block(move || {
                    let bytes = image.get_bytes();
                    thumbnail_processor.process_image(bytes)
                })
                .await??;
                Ok(result_bytes)
            }
            Err(None) => {
                anyhow::bail!("Thumbnail could not be read from cache")
            }
        }
    }

    pub async fn into_response(self) -> HttpResponse {
        match self.get_bytes().await {
            Ok(body) => HttpResponse::Ok().content_type(self.get_mime()).body(body),
            Err(_) => HttpResponse::InternalServerError().json(
                crate::endpoints::error_code::ErrorCode::new("internal.server.error"),
            ),
        }
    }

    pub fn new(
        original_file_name: String,
        requested_format: ImageFormat,
        lte: u32,
        cache_directory: Option<PathBuf>,
        image_service: ImageService,
    ) -> Self {
        Thumbnail {
            image_service,
            original_file_name,
            requested_format,
            largest_side: lte,
            cache_directory,
        }
    }

    async fn get_original_image(&self) -> Result<Image, ReadImageError> {
        self.image_service
            .read_image(&self.original_file_name)
            .await
    }

    fn try_get_cache_path(&self, cache_key: String) -> Option<PathBuf> {
        match (
            self.requested_format.extensions_str().first(),
            &self.cache_directory,
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

    async fn try_read_from_cache(&self, lte: u32) -> Result<Vec<u8>, Option<tokio::fs::File>> {
        let cache_key = format!("thumb_lte{lte}");
        if let Some(cached_path) = self.try_get_cache_path(cache_key) {
            if cached_path.exists() {
                tokio::fs::read(&cached_path)
                    .await
                    .ok()
                    .ok_or(create_seekable_writer_from_path(&cached_path).await.ok())
            } else {
                Err(create_seekable_writer_from_path(&cached_path).await.ok())
            }
        } else {
            Err(None)
        }
    }
}
