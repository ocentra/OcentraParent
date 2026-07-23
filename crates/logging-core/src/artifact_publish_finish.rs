use std::{fs::read, io, path::Path};

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
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => match read(path) {
            Ok(existing) => compare_existing(&existing, content).and_then(|_| sync(path)),
            Err(read_error) if read_error.kind() == io::ErrorKind::NotFound => Err(error),
            Err(read_error) => Err(read_error),
        },
        Err(error) => Err(error),
    }
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
