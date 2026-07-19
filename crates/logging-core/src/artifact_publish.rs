use std::{
    fs::{read, OpenOptions},
    io::{self, Write},
    path::Path,
};

use crate::{
    artifact_publish_lock::{remove_temporary, temporary_path},
    artifact_publish_platform::{publish_temporary, sync_parent},
};

pub(crate) fn publish_immutable(path: &Path, content: &[u8]) -> io::Result<()> {
    publish_immutable_with_fallback(path, content, false)
}

pub(crate) fn publish_immutable_with_fallback(
    path: &Path,
    content: &[u8],
    force_fallback: bool,
) -> io::Result<()> {
    match read(path) {
        Ok(existing) => compare_existing(&existing, content),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            publish_new(path, content, force_fallback)
        }
        Err(error) => Err(error),
    }
}

fn publish_new(path: &Path, content: &[u8], force_fallback: bool) -> io::Result<()> {
    let temporary = temporary_path(path)?;
    if let Err(error) = write_temporary(&temporary, content) {
        let _ = remove_temporary(&temporary);
        return Err(error);
    }
    let result = if force_fallback {
        crate::artifact_publish_platform::publish_temporary_with_fallback(&temporary, path, true)
    } else {
        publish_temporary(&temporary, path)
    };
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
