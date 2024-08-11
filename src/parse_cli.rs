use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser, Clone)]
#[command(name = "imdn")]
#[command(about = "An image delivery service", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub root_directory: PathBuf,

    #[arg(short, long)]
    pub cache_directory: Option<PathBuf>,
}
