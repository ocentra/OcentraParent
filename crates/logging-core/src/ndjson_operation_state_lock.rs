use std::{
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
};

pub(crate) fn with_stream_lock(
    path: &Path,
    action: impl FnOnce() -> io::Result<()>,
) -> io::Result<()> {
    let lock_path = stream_lock_path(path)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)?;
    file.lock()?;
    let result = action();
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

pub(crate) fn stream_lock_path(path: &Path) -> io::Result<PathBuf> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "NDJSON stream path has no file name",
            )
        })?;
    Ok(path.with_file_name(format!(".{name}.operations.lock")))
}
