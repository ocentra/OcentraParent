use std::{
    fs::{create_dir_all, OpenOptions},
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
        let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
        serde_json::to_writer(&mut file, event)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        file.write_all(b"\n")?;
        Ok(path)
    }
}
