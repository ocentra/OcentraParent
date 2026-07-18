use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use serde::Serialize;

use crate::path::{date_stamp_now, sanitize_segment};

pub struct NdjsonWriter {
    root: PathBuf,
}

impl NdjsonWriter {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn append_event<T: Serialize>(
        &self,
        scope: &str,
        stream: &str,
        event: &T,
    ) -> io::Result<PathBuf> {
        let scope = sanitize_segment(scope)?;
        let stream = sanitize_segment(stream)?;
        let directory = self.root.join(scope).join("ndjson").join(stream);
        create_dir_all(&directory)?;
        let path = directory.join(format!("{}.ndjson", date_stamp_now()));
        let mut record = serde_json::to_vec(event)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        record.push(b'\n');
        append_record(&path, &record)?;
        Ok(path)
    }
}

pub fn append_record(path: &std::path::Path, record: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(path)?;
    lock_and_append(&mut file, record)
}

fn lock_and_append(file: &mut File, record: &[u8]) -> io::Result<()> {
    file.lock()?;
    let result = file.write_all(record).and_then(|_| file.sync_data());
    let unlock_result = file.unlock();
    result.and(unlock_result)
}
