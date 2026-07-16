use std::{
    env,
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadRecord;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadResponse;

#[path = "thread_lifecycle.rs"]
mod thread_lifecycle;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ParentAssistantThreadId(pub(crate) String);

#[derive(Clone, Debug)]
struct ParentAssistantThreadTimestamp(String);

#[derive(Clone, Debug)]
struct ParentAssistantThreadReason(String);

#[derive(Clone, Debug)]
struct ParentAssistantStoragePath(PathBuf);

impl ParentAssistantStoragePath {
    fn as_path(&self) -> &Path {
        &self.0
    }
}

pub(crate) fn thread_response_for_command(
    command: &ocentra_parent_agent_protocol::transport::AgentCommandEnvelope,
) -> ParentAssistantThreadResponse {
    thread_response_for_command_in_dir(command, thread_storage_dir().as_path())
}

pub(crate) fn record_message_for_thread(thread_id: ParentAssistantThreadId) {
    let _ = record_message_for_thread_in_dir(thread_id, thread_storage_dir().as_path());
}

pub(crate) fn thread_response_for_command_in_dir(
    command: &ocentra_parent_agent_protocol::transport::AgentCommandEnvelope,
    directory: &Path,
) -> ParentAssistantThreadResponse {
    thread_lifecycle::thread_response_for_command_in_dir_impl(command, directory)
}

fn record_message_for_thread_in_dir(
    thread_id: ParentAssistantThreadId,
    directory: &Path,
) -> Result<(), ()> {
    thread_lifecycle::record_message_for_thread_in_dir_impl(thread_id, directory)
}

fn load_threads(directory: &Path) -> Result<Vec<ParentAssistantThreadRecord>, ()> {
    let path = thread_storage_file(directory).0;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let body = read_to_string(path).map_err(|_error| ())?;
    serde_json::from_str(&body).map_err(|_error| ())
}

fn save_threads(directory: &Path, threads: &[ParentAssistantThreadRecord]) -> Result<(), ()> {
    create_dir_all(directory).map_err(|_error| ())?;
    let body = serde_json::to_string_pretty(threads).map_err(|_error| ())?;
    write(thread_storage_file(directory).0, body).map_err(|_error| ())
}

fn thread_storage_file(directory: &Path) -> ParentAssistantStoragePath {
    let mut path = PathBuf::from(directory);
    path.push(constants::parent_assistant::THREAD_STORAGE_FILE);
    ParentAssistantStoragePath(path)
}

fn thread_storage_dir() -> ParentAssistantStoragePath {
    let directory = env::var(constants::env_var::DEV_LOG_DIR)
        .unwrap_or_else(|_| constants::dev_log::DEFAULT_DIR.to_owned());
    let mut path = PathBuf::from(directory);
    path.push(constants::parent_assistant::THREAD_STORAGE_DIR);
    ParentAssistantStoragePath(path)
}
