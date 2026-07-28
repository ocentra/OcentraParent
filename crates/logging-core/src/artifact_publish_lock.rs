use std::{
    fs::{remove_file, OpenOptions},
    io,
    path::{Path, PathBuf},
};

pub(crate) fn with_publish_lock<T>(
    path: &Path,
    operation: impl FnOnce() -> io::Result<T>,
) -> io::Result<T> {
    let name = file_name(path)?;
    let lock_path = path.with_file_name(format!(".{name}.lock"));
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)?;
    lock.lock()?;
    let result = operation();
    let unlock = lock.unlock();
    match result {
        Ok(value) => unlock.map(|()| value),
        Err(error) => {
            let _ = unlock;
            Err(error)
        }
    }
}
pub(crate) fn temporary_path(path: &Path) -> io::Result<PathBuf> {
    let name = file_name(path)?;
    Ok(path.with_file_name(format!(".{name}.tmp")))
}
pub(crate) fn remove_temporary(path: &Path) -> io::Result<()> {
    match remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}
fn file_name(path: &Path) -> io::Result<&str> {
    path.file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "artifact path has no file name",
            )
        })
}
