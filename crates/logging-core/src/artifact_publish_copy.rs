use std::{
    fs::{File, OpenOptions},
    io::{self, copy},
    path::Path,
};

#[cfg(feature = "test-support")]
use std::io::Read;

use crate::artifact_publish_copy_owned::{
    copy_temporary_path, publish_owned_temporary, remove_failed_destination, remove_owned_temporary,
};

#[cfg(feature = "test-support")]
#[derive(Clone, Copy)]
pub(crate) enum FallbackPublishFault {
    Copy,
    Sync,
    Crash,
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
        FallbackPublishFault::Crash => leave_partial_owned_temporary(temporary, path),
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
    let owned_temporary = copy_temporary_path(path)?;
    remove_owned_temporary(&owned_temporary)?;
    let mut destination = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&owned_temporary)?;
    let result = copy_to_destination(&mut source, &mut destination)
        .and_then(|()| sync_destination(&destination));
    drop(destination);
    match result {
        Ok(()) => publish_owned_temporary(&owned_temporary, path),
        Err(error) => remove_failed_destination(&owned_temporary, error),
    }
}

#[cfg(feature = "test-support")]
fn leave_partial_owned_temporary(temporary: &Path, path: &Path) -> io::Result<()> {
    let owned_temporary = copy_temporary_path(path)?;
    remove_owned_temporary(&owned_temporary)?;
    let source = File::open(temporary)?;
    let mut destination = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(owned_temporary)?;
    copy(&mut source.take(1), &mut destination)?;
    destination.sync_all()?;
    Err(io::Error::other("injected artifact fallback process death"))
}
