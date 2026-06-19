use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::{
    event::ParentLogEvent,
    path::{DEV_LOG_DIR_ENV, LOG_ROOT_ENV},
    source::LogSource,
};

pub(crate) struct CompatDevLogWriter {
    directory: PathBuf,
}

impl CompatDevLogWriter {
    pub(crate) fn from_env() -> Option<io::Result<Self>> {
        if env::var_os(LOG_ROOT_ENV).is_some() {
            return None;
        }

        env::var_os(DEV_LOG_DIR_ENV)
            .filter(|value| !value.is_empty())
            .map(|directory| Self::new(PathBuf::from(directory)))
    }

    pub(crate) fn append_event(&self, event: &ParentLogEvent) -> io::Result<PathBuf> {
        append_legacy_event(&self.directory, event)
    }

    fn new(directory: PathBuf) -> io::Result<Self> {
        create_dir_all(&directory)?;
        Ok(Self { directory })
    }
}

pub fn resolve_compat_dev_log_path(source: &LogSource, timestamp: &str) -> io::Result<PathBuf> {
    match env::var_os(DEV_LOG_DIR_ENV) {
        Some(directory) => legacy_dev_log_path(PathBuf::from(directory), source, timestamp),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "OCENTRA_PARENT_DEV_LOG_DIR is not configured",
        )),
    }
}

fn append_legacy_event(directory: &Path, event: &ParentLogEvent) -> io::Result<PathBuf> {
    let path = legacy_dev_log_path(directory.to_path_buf(), &event.source, &event.timestamp)?;
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
    serde_json::to_writer(&mut file, event)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    file.write_all(b"\n")?;
    Ok(path)
}

fn legacy_dev_log_path(
    mut directory: PathBuf,
    source: &LogSource,
    timestamp: &str,
) -> io::Result<PathBuf> {
    create_dir_all(&directory)?;
    directory.push(format!(
        "{}-{}.ndjson",
        source.compat_file_prefix(),
        timestamp_day(timestamp)
    ));
    Ok(directory)
}

fn timestamp_day(timestamp: &str) -> String {
    timestamp.chars().take(10).collect()
}
