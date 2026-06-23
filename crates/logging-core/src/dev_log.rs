use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::{
    event::{ParentLogEvent, LOG_SCHEMA_VERSION},
    field::LogFields,
    level::LogLevel,
    ndjson_writer::NdjsonWriter,
    path::{
        resolve_lane_id, resolve_log_root, resolve_log_run_id, resolve_log_scope, timestamp_now,
        DEV_LOG_DIR_ENV, LOG_ROOT_ENV,
    },
    source::LogSource,
};

pub const DEV_LOG_STREAM: &str = "dev-log";

pub struct DevLogger {
    source: LogSource,
    run_id: Option<String>,
    lane_id: Option<String>,
    target: DevLogTarget,
}

enum DevLogTarget {
    Scoped { writer: NdjsonWriter, scope: String },
    LegacyFile { directory: PathBuf },
}

impl DevLogger {
    pub fn from_env(source: LogSource) -> io::Result<Self> {
        let run_id = resolve_log_run_id();
        let lane_id = resolve_lane_id();
        let target = match legacy_dev_log_directory() {
            Some(directory) => DevLogTarget::LegacyFile {
                directory: ensure_directory(directory)?,
            },
            None => DevLogTarget::Scoped {
                writer: NdjsonWriter::new(resolve_log_root()?),
                scope: resolve_log_scope(),
            },
        };

        Ok(Self {
            source,
            run_id,
            lane_id,
            target,
        })
    }

    pub fn info(&self, message: &str, fields: LogFields) -> io::Result<PathBuf> {
        self.log(LogLevel::Info, message, fields)
    }

    pub fn warn(&self, message: &str, fields: LogFields) -> io::Result<PathBuf> {
        self.log(LogLevel::Warn, message, fields)
    }

    pub fn error(&self, message: &str, fields: LogFields) -> io::Result<PathBuf> {
        self.log(LogLevel::Error, message, fields)
    }

    pub fn debug(&self, message: &str, fields: LogFields) -> io::Result<PathBuf> {
        self.log(LogLevel::Debug, message, fields)
    }

    pub fn log(&self, level: LogLevel, message: &str, fields: LogFields) -> io::Result<PathBuf> {
        let timestamp = timestamp_now();
        let event = ParentLogEvent {
            schema_version: LOG_SCHEMA_VERSION,
            entry_id: create_log_id(&timestamp, &self.source, message),
            timestamp: timestamp.clone(),
            level,
            source: self.source.clone(),
            message: message.to_owned(),
            fields,
            run_id: self.run_id.clone(),
            lane_id: self.lane_id.clone(),
            command_id: None,
            correlation_id: None,
            file: None,
            line: None,
            column: None,
        };

        match &self.target {
            DevLogTarget::Scoped { writer, scope } => {
                writer.append_event(scope, DEV_LOG_STREAM, &event)
            }
            DevLogTarget::LegacyFile { directory } => append_legacy_event(directory, &event),
        }
    }
}

pub fn write_agent_info(
    source: LogSource,
    message: &str,
    fields: LogFields,
) -> io::Result<PathBuf> {
    DevLogger::from_env(source)?.info(message, fields)
}

fn legacy_dev_log_directory() -> Option<PathBuf> {
    if env::var_os(LOG_ROOT_ENV).is_some() {
        return None;
    }
    env::var_os(DEV_LOG_DIR_ENV)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn ensure_directory(directory: PathBuf) -> io::Result<PathBuf> {
    create_dir_all(&directory)?;
    Ok(directory)
}

fn create_log_id(timestamp: &str, source: &LogSource, message: &str) -> String {
    let digest = Sha256::digest(
        format!("{}:{}:{message}", source.compat_file_prefix(), timestamp).as_bytes(),
    );
    format!(
        "parent-log-{}-{}",
        timestamp
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect::<String>(),
        &format!("{digest:x}")[..8]
    )
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
