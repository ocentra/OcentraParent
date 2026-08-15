use std::{
    fs::{read, symlink_metadata},
    io,
    path::Path,
};

pub(crate) fn finish_publication<S>(
    result: io::Result<()>,
    cleanup: io::Result<()>,
    path: &Path,
    content: &[u8],
    sync: S,
) -> io::Result<()>
where
    S: FnOnce(&Path) -> io::Result<()>,
{
    match result {
        Ok(()) => cleanup.and_then(|_| sync(path)),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => match read_immutable(path) {
            Ok(existing) => compare_existing(&existing, content).and_then(|_| sync(path)),
            Err(read_error) if read_error.kind() == io::ErrorKind::NotFound => Err(error),
            Err(read_error) => Err(read_error),
        },
        Err(error) => Err(error),
    }
}

pub(crate) fn read_immutable(path: &Path) -> io::Result<Vec<u8>> {
    let metadata = symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "artifact leaf must not be a symlink",
        ));
    }
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "artifact leaf must be a regular file",
        ));
    }
    read(path)
}

pub(crate) fn compare_existing(existing: &[u8], content: &[u8]) -> io::Result<()> {
    if existing == content {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "artifact path already contains different content",
        ))
    }
}
