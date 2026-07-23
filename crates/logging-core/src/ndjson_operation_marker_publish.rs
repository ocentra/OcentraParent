use std::{
    fs::{remove_file, File, OpenOptions},
    io::{self, Write},
    path::Path,
};

use crate::{
    artifact_publish_lock::{remove_temporary, temporary_path},
    ndjson_operation_marker_publish_state::{accept_or_repair_existing, publish_marker},
};

pub(crate) fn write_marker(path: &Path, content: &str) -> io::Result<()> {
    write_marker_using(
        path,
        content,
        |marker, content| marker.write_all(content),
        File::sync_all,
    )
}

#[cfg(feature = "test-support")]
pub(crate) fn write_marker_with_fault(
    path: &Path,
    content: &str,
    fault: MarkerWriteFault,
) -> io::Result<()> {
    match fault {
        MarkerWriteFault::Write => write_marker_using(
            path,
            content,
            |marker, content| {
                marker.write_all(&content[..1])?;
                Err(io::Error::other("injected operation marker write failure"))
            },
            File::sync_all,
        ),
        MarkerWriteFault::Sync => write_marker_using(
            path,
            content,
            |marker, content| marker.write_all(content),
            |_marker| Err(io::Error::other("injected operation marker sync failure")),
        ),
    }
}

#[cfg(feature = "test-support")]
#[derive(Clone, Copy)]
pub(crate) enum MarkerWriteFault {
    Write,
    Sync,
}

fn write_marker_using<W, S>(
    path: &Path,
    content: &str,
    write_content: W,
    sync_marker: S,
) -> io::Result<()>
where
    W: FnOnce(&mut File, &[u8]) -> io::Result<()>,
    S: FnOnce(&File) -> io::Result<()>,
{
    if accept_or_repair_existing(path, content.as_bytes())? {
        return Ok(());
    }
    let temporary = temporary_path(path)?;
    remove_temporary(&temporary)?;
    let mut marker = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    let result = write_content(&mut marker, content.as_bytes()).and_then(|_| sync_marker(&marker));
    drop(marker);
    match result {
        Ok(()) => publish_marker(&temporary, path, content.as_bytes()),
        Err(error) => remove_failed_marker(&temporary, error),
    }
}

pub(crate) fn remove_failed_marker(path: &Path, marker_error: io::Error) -> io::Result<()> {
    match remove_file(path) {
        Ok(()) => Err(marker_error),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Err(marker_error),
        Err(cleanup_error) => Err(io::Error::new(
            cleanup_error.kind(),
            format!(
                "operation marker failed ({marker_error}); partial marker cleanup failed ({cleanup_error})"
            ),
        )),
    }
}
