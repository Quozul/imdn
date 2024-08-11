use std::fs::OpenOptions;
use std::io;
use std::io::{Cursor, Read, Seek, Write};
use std::path::PathBuf;

pub trait SeekableWriter: Write + Seek + Read {}

impl<T> SeekableWriter for T where T: Write + Seek + Read {}

impl dyn SeekableWriter {
    pub fn read_all_bytes(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.seek(io::SeekFrom::Start(0))?; // Rewind the writer to the beginning
        self.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

pub fn create_seekable_writer() -> Box<dyn SeekableWriter> {
    let buffer: Vec<u8> = vec![];
    let cursor = Cursor::new(buffer);
    Box::new(cursor)
}

/// Creates a new seekable writer from a file or fallback to a writer in memory
pub fn create_seekable_writer_from_path(path: PathBuf) -> Box<dyn SeekableWriter> {
    if let Some(parent) = path.parent() {
        if !parent.exists() && std::fs::create_dir_all(parent).is_err() {
            return create_seekable_writer();
        }
    }

    match OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)
    {
        Ok(file) => Box::new(file),
        Err(_) => create_seekable_writer(),
    }
}
