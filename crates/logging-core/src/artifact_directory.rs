use std::{
    fs::{create_dir, symlink_metadata},
    io,
    path::{Path, PathBuf},
};

use crate::artifact_publish_platform::sync_parent;

pub(crate) fn create_durable_directory_hierarchy(directory: &Path) -> io::Result<()> {
    let mut missing = Vec::new();
    let mut current = directory;
    loop {
        match symlink_metadata(current) {
            Ok(metadata) if metadata.is_dir() => break,
            Ok(_) => return Err(non_directory_error()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                missing.push(current.to_path_buf());
                current = parent_directory(current)?;
            }
            Err(error) => return Err(error),
        }
    }
    sync_missing_directories(missing)
}

fn parent_directory(path: &Path) -> io::Result<&Path> {
    path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "artifact directory has no existing ancestor",
        )
    })
}

fn non_directory_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::AlreadyExists,
        "artifact directory path is not a directory",
    )
}

fn sync_missing_directories(missing: Vec<PathBuf>) -> io::Result<()> {
    for path in missing.into_iter().rev() {
        create_and_sync_directory(&path)?;
    }
    Ok(())
}

pub(crate) fn create_and_sync_directory(directory: &Path) -> io::Result<()> {
    match create_dir(directory) {
        Ok(()) => {}
        Err(error)
            if error.kind() == io::ErrorKind::AlreadyExists
                && symlink_metadata(directory)?.is_dir() => {}
        Err(error) => return Err(error),
    }
    sync_parent(directory)
}
