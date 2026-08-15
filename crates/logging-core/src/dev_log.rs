use std::{
    env,
    fs::create_dir_all,
    io,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::{
    event::{ParentLogEvent, LOG_SCHEMA_VERSION},
    field::LogFields,
    level::LogLevel,
    ndjson_writer::{append_record, NdjsonWriter},
    path::{
        resolve_lane_id, resolve_log_root, resolve_log_run_id, resolve_log_scope, timestamp_now,
        DEV_LOG_DIR_ENV, LOG_ROOT_ENV,
    },
    redaction::redact_fields,
    source::LogSource,
};
use ocentra_schema::logging_contracts::{LogEntryId, LogLaneId, LogRunId};

pub const DEV_LOG_STREAM: &str = "dev-log";

pub struct DevLogger {
    source: LogSource,
    run_id: Option<String>,
    lane_id: Option<String>,
    target: DevLogTarget,
}

enum DevLogTarget {
    Scoped { writer: NdjsonWriter, scope: String },
    CompatFile { directory: PathBuf },
}

impl DevLogger {
    pub fn from_env(source: LogSource) -> io::Result<Self> {
        let run_id = resolve_log_run_id();
        let lane_id = resolve_lane_id();
        let target = match compat_dev_log_directory() {
            Some(directory) => DevLogTarget::CompatFile {
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
        let fields = LogFields::from(fields.into_inner());
        let timestamp = timestamp_now();
        let event = ParentLogEvent {
            schema_version: LOG_SCHEMA_VERSION,
            entry_id: create_log_id(&timestamp, &self.source, message)?,
            timestamp: timestamp.clone(),
            level,
            source: self.source.clone(),
            message: message.to_owned(),
            fields: redact_fields(&fields),
            run_id: self.run_id.clone().and_then(LogRunId::parse),
            lane_id: self.lane_id.clone().and_then(LogLaneId::parse),
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
            DevLogTarget::CompatFile { directory } => append_compat_event(directory, &event),
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

fn compat_dev_log_directory() -> Option<PathBuf> {
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

fn create_log_id(timestamp: &str, source: &LogSource, message: &str) -> io::Result<LogEntryId> {
    let nonce = random_nonce()?;
    let digest = Sha256::digest(
        format!(
            "{}:{timestamp}:{nonce}:{message}",
            source.compat_file_prefix()
        )
        .as_bytes(),
    );
    LogEntryId::parse(format!(
        "parent-log-{}-{}",
        timestamp
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect::<String>(),
        &format!("{digest:x}")[..12]
    ))
    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "generated log id is invalid"))
}

fn random_nonce() -> io::Result<String> {
    let mut bytes = [0_u8; 16];
    getrandom::fill(&mut bytes)
        .map_err(|error| io::Error::other(format!("random nonce generation failed: {error}")))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn append_compat_event(directory: &Path, event: &ParentLogEvent) -> io::Result<PathBuf> {
    let path = compat_dev_log_path(directory.to_path_buf(), &event.source, &event.timestamp)?;
    let mut record = serde_json::to_vec(event)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    record.push(b'\n');
    append_record(&path, &record)?;
    Ok(path)
}

fn compat_dev_log_path(
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
