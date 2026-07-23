use std::{
    fs::{read, OpenOptions},
    io::{self, Write},
    path::Path,
};

use crate::{
    artifact_publish_lock::{remove_temporary, temporary_path},
    artifact_publish_platform::{publish_temporary, sync_parent},
};

#[cfg(feature = "test-support")]
use crate::artifact_publish_platform::publish_temporary_with_fallback;

pub(crate) fn publish_immutable(path: &Path, content: &[u8]) -> io::Result<()> {
    publish_immutable_using(path, content, publish_temporary)
}

#[cfg(feature = "test-support")]
pub(crate) fn publish_immutable_with_fallback(
    path: &Path,
    content: &[u8],
    force_fallback: bool,
) -> io::Result<()> {
    publish_immutable_using(path, content, |temporary, path| {
        publish_temporary_with_fallback(temporary, path, force_fallback)
    })
}

#[cfg(feature = "test-support")]
pub(crate) fn publish_immutable_with_fallback_fault(
    path: &Path,
    content: &[u8],
    fault: crate::artifact_publish_copy::FallbackPublishFault,
) -> io::Result<()> {
    publish_immutable_using(path, content, |temporary, path| {
        crate::artifact_publish_copy::copy_without_replacement_with_fault(temporary, path, fault)
    })
}

fn publish_immutable_using<P>(path: &Path, content: &[u8], publish: P) -> io::Result<()>
where
    P: FnOnce(&Path, &Path) -> io::Result<()>,
{
    match read(path) {
        Ok(existing) => compare_existing(&existing, content),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            publish_new(path, content, publish)
        }
        Err(error) => Err(error),
    }
}

fn publish_new<P>(path: &Path, content: &[u8], publish: P) -> io::Result<()>
where
    P: FnOnce(&Path, &Path) -> io::Result<()>,
{
    let temporary = temporary_path(path)?;
    if let Err(error) = write_temporary(&temporary, content) {
        let _ = remove_temporary(&temporary);
        return Err(error);
    }
    let result = publish(&temporary, path);
    let cleanup = remove_temporary(&temporary);
    finish_publication(result, cleanup, path, content)
}

fn finish_publication(
    result: io::Result<()>,
    cleanup: io::Result<()>,
    path: &Path,
    content: &[u8],
) -> io::Result<()> {
    match result {
        Ok(()) => cleanup.and_then(|_| sync_parent(path)),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            compare_existing(&read(path)?, content)
        }
        Err(error) => Err(error),
    }
}

fn compare_existing(existing: &[u8], content: &[u8]) -> io::Result<()> {
    if existing == content {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "artifact path already contains different content",
        ))
    }
}

fn write_temporary(path: &Path, content: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(content)?;
    file.sync_all()
}
