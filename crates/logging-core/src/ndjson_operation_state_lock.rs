use std::{fs::OpenOptions, io, path::Path};

pub(crate) fn with_stream_lock(
    path: &Path,
    action: impl FnOnce() -> io::Result<()>,
) -> io::Result<()> {
    let file = match OpenOptions::new().read(true).write(true).open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return action(),
        Err(error) => return Err(error),
    };
    file.lock()?;
    let result = action();
    let unlock_result = file.unlock();
    result.and(unlock_result)
}
