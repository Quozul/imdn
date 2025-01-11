use std::io;
use std::path::Path;
use tokio::fs::File;

/// Creates a new seekable writer from a file or fallback to a writer in memory
pub async fn create_seekable_writer_from_path(path: &Path) -> io::Result<File> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }

    tokio::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)
        .await
}
