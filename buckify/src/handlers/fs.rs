use axum::body::Bytes;
use std::{
    fs::{create_dir_all, metadata, File},
    io::{Error, ErrorKind, Write},
    path::Path,
};

pub struct FileData {
    pub path: String,
    pub size: u64,
}

pub fn create(path: String, bytes: Bytes) -> Result<FileData, Error> {
    let path = Path::new(&path);
    let parent = path.parent().ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid path: {}", path.display()),
        )
    })?;

    if !parent.exists() {
        create_dir_all(parent)?;
    }

    if path.exists() {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            format!("File {} already exists. Skipping write.", path.display()),
        ));
    }

    let mut file = File::create(path)?;
    file.write_all(&bytes)?;

    let size: u64 = metadata(path)?.len();
    println!("{}", size);

    // Check if the file exists after writing
    if path.exists() {
        Ok(FileData {
            path: path.display().to_string(),
            size,
        })
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("Failed to retrieve file once uploaded: {}", path.display()),
        ))
    }
}
