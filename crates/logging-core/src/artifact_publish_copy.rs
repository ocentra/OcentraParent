use std::{
    fs::{remove_file, File, OpenOptions},
    io::{self, copy},
    path::Path,
};

#[cfg(feature = "test-support")]
use std::io::Read;

#[cfg(feature = "test-support")]
#[derive(Clone, Copy)]
pub(crate) enum FallbackPublishFault {
    Copy,
    Sync,
}

pub(crate) fn copy_without_replacement(temporary: &Path, path: &Path) -> io::Result<()> {
    copy_without_replacement_using(
        temporary,
        path,
        |source, destination| copy(source, destination).map(|_| ()),
        File::sync_all,
    )
}

#[cfg(feature = "test-support")]
pub(crate) fn copy_without_replacement_with_fault(
    temporary: &Path,
    path: &Path,
    fault: FallbackPublishFault,
) -> io::Result<()> {
    match fault {
        FallbackPublishFault::Copy => copy_without_replacement_using(
            temporary,
            path,
            |source, destination| {
                copy(&mut source.take(1), destination)?;
                Err(io::Error::other("injected artifact fallback copy failure"))
            },
            File::sync_all,
        ),
        FallbackPublishFault::Sync => copy_without_replacement_using(
            temporary,
            path,
            |source, destination| copy(source, destination).map(|_| ()),
            |_destination| Err(io::Error::other("injected artifact fallback sync failure")),
        ),
    }
}

fn copy_without_replacement_using<C, S>(
    temporary: &Path,
    path: &Path,
    copy_to_destination: C,
    sync_destination: S,
) -> io::Result<()>
where
    C: FnOnce(&mut File, &mut File) -> io::Result<()>,
    S: FnOnce(&File) -> io::Result<()>,
{
    let mut source = File::open(temporary)?;
    let mut destination = OpenOptions::new().write(true).create_new(true).open(path)?;
    let result = copy_to_destination(&mut source, &mut destination)
        .and_then(|()| sync_destination(&destination));
    drop(destination);
    match result {
        Ok(()) => Ok(()),
        Err(error) => remove_failed_destination(path, error),
    }
}

fn remove_failed_destination(path: &Path, publication_error: io::Error) -> io::Result<()> {
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
