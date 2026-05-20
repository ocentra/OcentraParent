use std::{
    env,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::PathBuf,
};

use ocentra_parent_agent_protocol::{
    constants, DevLogEntry, LogFields, LogLevel, LogSource, LOG_SCHEMA_VERSION,
};

use crate::time::timestamp_now;

pub fn write_agent_info(message: &str, fields: LogFields) -> std::io::Result<()> {
    let timestamp = timestamp_now();
    let entry = DevLogEntry {
        schema_version: LOG_SCHEMA_VERSION,
        id: create_log_id(&timestamp),
        timestamp,
        level: LogLevel::Info,
        source: LogSource::AgentService,
        message: message.to_owned(),
        fields,
    };
    let path = dev_log_path(&entry.timestamp)?;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    let mut line = serde_json::to_string(&entry).expect(constants::error::DEV_LOG_SERIALIZES);
    line.push(constants::delimiter::NEWLINE);
    file.write_all(line.as_bytes())
}

fn create_log_id(timestamp: &str) -> String {
    let mut id = String::from(constants::dev_log::ID_PREFIX);
    id.extend(timestamp.chars().filter(char::is_ascii_alphanumeric));
    id
}

fn dev_log_path(timestamp: &str) -> std::io::Result<PathBuf> {
    let directory = env::var(constants::env_var::DEV_LOG_DIR)
        .unwrap_or_else(|_| constants::dev_log::DEFAULT_DIR.to_owned());
    let mut path = PathBuf::from(directory);
    create_dir_all(&path)?;
    path.push(file_name(timestamp));
    Ok(path)
}

fn file_name(timestamp: &str) -> String {
    let day = timestamp
        .chars()
        .take(constants::dev_log::DATE_CHARS)
        .collect::<String>();
    let mut name = String::new();
    name.push_str(constants::dev_log::AGENT_FILE_PREFIX);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&day);
    name.push(constants::delimiter::DOT);
    name.push_str(constants::dev_log::FILE_EXTENSION);
    name
}
