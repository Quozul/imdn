use crate::core::image::Image;
use crate::parse_cli::ImageSource;
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::Client;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Clone)]
pub struct ImageService {
    image_provider: ImageProvider,
}

#[derive(Clone)]
enum ImageProvider {
    Local { root_path: PathBuf },
    S3 { client: Client, bucket: String },
}

impl ImageService {
    pub fn new(image_source: ImageSource) -> Result<Self, Box<dyn std::error::Error>> {
        let image_provider = match image_source {
            ImageSource::Local { root_path } => ImageProvider::Local { root_path },
            ImageSource::S3 {
                bucket,
                region,
                endpoint,
            } => {
                let client = get_s3_client(endpoint, region)?;
                ImageProvider::S3 { client, bucket }
            }
        };
        Ok(Self { image_provider })
    }

    pub async fn read_image(&self, requested_path: &str) -> Result<Image, ReadImageError> {
        match &self.image_provider {
            ImageProvider::Local { root_path } => {
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
            ImageProvider::S3 { client, bucket } => {
                let response = client
                    .get_object()
                    .bucket(bucket)
                    .key(requested_path)
                    .send()
                    .await
                    .map_err(|_| ReadImageError::S3)?;

                let byte_stream = response
                    .body
                    .collect()
                    .await
                    .map_err(|_| ReadImageError::S3)?;
                Ok(Image::from_bytes(&requested_path, byte_stream.to_vec()))
            }
        }
    }
}

fn get_s3_client(
    endpoint: Option<String>,
    region: String,
) -> Result<Client, Box<dyn std::error::Error>> {
    let key_id = std::env::var("AWS_ACCESS_KEY_ID")?;
    let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY")?;

    let cred = Credentials::new(key_id, secret_key, None, None, "loaded-from-custom-env");

    let mut s3_config = aws_sdk_s3::config::Builder::new()
        .credentials_provider(cred)
        .region(Region::new(region));

    if let Some(endpoint) = endpoint {
        s3_config = s3_config.endpoint_url(endpoint).force_path_style(true);
    }

    let client = Client::from_conf(s3_config.build());
    Ok(client)
}

#[derive(Error, Debug)]
pub enum ReadImageError {
    #[error("forbidden path")]
    ForbiddenPath,
    #[error("file not found")]
    FileNotFound,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("s3 communication error")]
    S3,
}
