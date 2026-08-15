use std::{
    fs::{remove_file, rename},
    io,
    path::{Path, PathBuf},
};

pub(crate) fn publish_owned_temporary(owned_temporary: &Path, path: &Path) -> io::Result<()> {
    if path.exists() {
        return remove_failed_destination(
            owned_temporary,
            io::Error::new(io::ErrorKind::AlreadyExists, "artifact path already exists"),
        );
    }
    match rename(owned_temporary, path) {
        Ok(()) => Ok(()),
        Err(error) => remove_failed_destination(owned_temporary, error),
    }
}

pub(crate) fn copy_temporary_path(path: &Path) -> io::Result<PathBuf> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "artifact has no file name"))?;
    Ok(path.with_file_name(format!(".{name}.copy.tmp")))
}

pub(crate) fn remove_owned_temporary(path: &Path) -> io::Result<()> {
    match remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

pub(crate) fn remove_failed_destination(
    path: &Path,
    publication_error: io::Error,
) -> io::Result<()> {
    match remove_file(path) {
        Ok(()) => Err(publication_error),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Err(publication_error),
        Err(cleanup_error) => Err(io::Error::new(
            cleanup_error.kind(),
            format!(
                "artifact fallback failed ({publication_error}) and partial destination cleanup failed ({cleanup_error})"
            ),
        )),
    }
}
