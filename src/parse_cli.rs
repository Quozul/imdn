use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "imdn")]
#[command(about = "An image delivery service", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub image_source: ImageSource,

    #[arg(short, long)]
    pub cache_directory: Option<PathBuf>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ImageSource {
    Local {
        root_path: PathBuf,
    },
    S3 {
        api_key: String,
        api_secret: String,
        bucket: String,
    },
}
