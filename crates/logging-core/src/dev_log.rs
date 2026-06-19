use std::{env, io, path::PathBuf};

use sha2::{Digest, Sha256};

use crate::{
    compat_dev_log::CompatDevLogWriter,
    event::{ParentLogEvent, LOG_SCHEMA_VERSION},
    field::LogFields,
    level::LogLevel,
    ndjson_writer::NdjsonWriter,
    path::{
        resolve_log_root, resolve_log_scope, timestamp_now, CODEX_LANE_ID_ENV, CODEX_RUN_ID_ENV,
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
    LegacyCompat(CompatDevLogWriter),
}

impl DevLogger {
    pub fn from_env(source: LogSource) -> io::Result<Self> {
        let run_id = env_value(CODEX_RUN_ID_ENV);
        let lane_id = env_value(CODEX_LANE_ID_ENV);
        let target = match CompatDevLogWriter::from_env() {
            Some(writer) => DevLogTarget::LegacyCompat(writer?),
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
            id: create_log_id(&timestamp, &self.source, message),
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
            DevLogTarget::LegacyCompat(writer) => writer.append_event(&event),
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

fn env_value(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
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
