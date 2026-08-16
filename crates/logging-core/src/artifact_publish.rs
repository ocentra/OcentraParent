use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

use crate::{
    artifact_publish_finish::{compare_existing, finish_publication, read_immutable},
    artifact_publish_lock::{remove_temporary, temporary_path},
    artifact_publish_platform::{publish_temporary, sync_parent},
};

#[cfg(feature = "test-support")]
use crate::artifact_publish_platform::publish_temporary_with_fallback;

pub(crate) fn publish_immutable(path: &Path, content: &[u8]) -> io::Result<()> {
    publish_immutable_using(path, content, publish_temporary, sync_parent)
}

#[cfg(feature = "test-support")]
pub(crate) fn publish_immutable_with_fallback(
    path: &Path,
    content: &[u8],
    force_fallback: bool,
) -> io::Result<()> {
    publish_immutable_using(
        path,
        content,
        |temporary, path| publish_temporary_with_fallback(temporary, path, force_fallback),
        sync_parent,
    )
}

#[cfg(feature = "test-support")]
pub(crate) fn publish_immutable_with_fallback_fault(
    path: &Path,
    content: &[u8],
    fault: crate::artifact_publish_copy::FallbackPublishFault,
) -> io::Result<()> {
    publish_immutable_using(
        path,
        content,
        |temporary, path| {
            crate::artifact_publish_copy::copy_without_replacement_with_fault(
                temporary, path, fault,
            )
        },
        sync_parent,
    )
}

#[cfg(feature = "test-support")]
pub(crate) fn publish_immutable_with_link_error(
    path: &Path,
    content: &[u8],
    kind: io::ErrorKind,
) -> io::Result<()> {
    publish_immutable_using(
        path,
        content,
        |temporary, path| {
            crate::artifact_publish_platform::publish_temporary_with_link_error(
                temporary, path, kind,
            )
        },
        sync_parent,
    )
}

#[cfg(feature = "test-support")]
pub(crate) fn publish_immutable_with_parent_sync_fault(
    path: &Path,
    content: &[u8],
) -> io::Result<()> {
    publish_immutable_using(path, content, publish_temporary, |_path| {
        Err(io::Error::other("injected parent directory sync failure"))
    })
}

#[cfg(feature = "test-support")]
pub(crate) fn publish_immutable_with_stale_temporary(
    path: &Path,
    content: &[u8],
) -> io::Result<()> {
    let temporary = temporary_path(path)?;
    std::fs::write(&temporary, b"stale temporary")?;
    publish_new_using_temporary(path, content, &temporary, publish_temporary, sync_parent)
}

fn publish_immutable_using<P, S>(path: &Path, content: &[u8], publish: P, sync: S) -> io::Result<()>
where
    P: FnOnce(&Path, &Path) -> io::Result<()>,
    S: FnOnce(&Path) -> io::Result<()>,
{
    match read_immutable(path) {
        Ok(existing) => {
            compare_existing(&existing, content)?;
            remove_temporary(&temporary_path(path)?)?;
            sync(path)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            publish_new(path, content, publish, sync)
        }
        Err(error) => Err(error),
    }
}

fn publish_new<P, S>(path: &Path, content: &[u8], publish: P, sync: S) -> io::Result<()>
where
    P: FnOnce(&Path, &Path) -> io::Result<()>,
    S: FnOnce(&Path) -> io::Result<()>,
{
    let temporary = temporary_path(path)?;
    publish_new_using_temporary(path, content, &temporary, publish, sync)
}

fn publish_new_using_temporary<P, S>(
    path: &Path,
    content: &[u8],
    temporary: &Path,
    publish: P,
    sync: S,
) -> io::Result<()>
where
    P: FnOnce(&Path, &Path) -> io::Result<()>,
    S: FnOnce(&Path) -> io::Result<()>,
{
    if let Err(error) = write_temporary_recovering_stale(temporary, content) {
        let _ = remove_temporary(temporary);
        return Err(error);
    }
    let result = publish(temporary, path);
    let cleanup = remove_temporary(temporary);
    finish_publication(result, cleanup, path, content, sync)
}

fn write_temporary_recovering_stale(path: &Path, content: &[u8]) -> io::Result<()> {
    match write_temporary(path, content) {
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            remove_temporary(path)?;
            write_temporary(path, content)
        }
        result => result,
    }
}

fn write_temporary(path: &Path, content: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(content)?;
    file.sync_all()
}
