use std::{
    fs::{hard_link, File, OpenOptions},
    io::{self, copy},
    path::Path,
};

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
    match hard_link(temporary, path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Err(error),
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::Unsupported | io::ErrorKind::CrossesDevices
            ) =>
        {
            copy_without_replacement(temporary, path)
        }
        Err(error) => Err(error),
    }
}

fn copy_without_replacement(temporary: &Path, path: &Path) -> io::Result<()> {
    let mut source = File::open(temporary)?;
    let mut destination = OpenOptions::new().write(true).create_new(true).open(path)?;
    copy(&mut source, &mut destination)?;
    destination.sync_all()
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
#[cfg(not(unix))]
pub(crate) fn sync_parent(_path: &Path) -> io::Result<()> {
    Ok(())
}
