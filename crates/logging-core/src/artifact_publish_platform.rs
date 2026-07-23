use std::{fs::hard_link, io, path::Path};

use crate::artifact_publish_copy::copy_without_replacement;

pub(crate) fn publish_temporary(temporary: &Path, path: &Path) -> io::Result<()> {
    publish_temporary_with_fallback(temporary, path, false)
}

pub(crate) fn publish_temporary_with_fallback(
    temporary: &Path,
    path: &Path,
    force_fallback: bool,
) -> io::Result<()> {
    if force_fallback {
        return copy_without_replacement(temporary, path);
    }
    publish_temporary_using(temporary, path, |source, target| hard_link(source, target))
}

fn publish_temporary_using<H>(temporary: &Path, path: &Path, link: H) -> io::Result<()>
where
    H: FnOnce(&Path, &Path) -> io::Result<()>,
{
    match link(temporary, path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Err(error),
        Err(_) => copy_without_replacement(temporary, path),
    }
}

#[cfg(feature = "test-support")]
pub(crate) fn publish_temporary_with_link_error(
    temporary: &Path,
    path: &Path,
    kind: io::ErrorKind,
) -> io::Result<()> {
    publish_temporary_using(temporary, path, |_temporary, _path| {
        Err(io::Error::new(kind, "injected hard-link failure"))
    })
}

#[cfg(unix)]
pub(crate) fn sync_parent(path: &Path) -> io::Result<()> {
    use std::fs::File;

    File::open(
        path.parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "artifact has no parent"))?,
    )?
    .sync_all()
}

#[cfg(windows)]
pub(crate) fn sync_parent(path: &Path) -> io::Result<()> {
    use std::{fs::OpenOptions, os::windows::fs::OpenOptionsExt};

    const FILE_FLAG_WRITE_THROUGH: u32 = 0x8000_0000;
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(FILE_FLAG_WRITE_THROUGH)
        .open(path)?
        .sync_all()
}

#[cfg(not(any(unix, windows)))]
pub(crate) fn sync_parent(_path: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "parent-directory durability is unsupported on this platform",
    ))
}
