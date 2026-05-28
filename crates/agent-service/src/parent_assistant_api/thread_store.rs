use std::{
    env,
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName, LogFieldValue,
    ParentAssistantBackendState, ParentAssistantThreadRecord, ParentAssistantThreadResponse,
    ParentAssistantThreadState,
};

use crate::time::timestamp_now;

pub(crate) fn thread_response_for_command(
    command: &AgentCommandEnvelope,
) -> ParentAssistantThreadResponse {
    thread_response_for_command_in_dir(command, thread_storage_dir())
}

pub(crate) fn record_message_for_thread(thread_id: &str) {
    let _ = record_message_for_thread_in_dir(thread_id, thread_storage_dir());
}

#[cfg(test)]
pub(crate) fn thread_response_for_command_in_dir(
    command: &AgentCommandEnvelope,
    directory: PathBuf,
) -> ParentAssistantThreadResponse {
    thread_response_for_command_in_dir_impl(command, directory)
}

#[cfg(not(test))]
fn thread_response_for_command_in_dir(
    command: &AgentCommandEnvelope,
    directory: PathBuf,
) -> ParentAssistantThreadResponse {
    thread_response_for_command_in_dir_impl(command, directory)
}

fn thread_response_for_command_in_dir_impl(
    command: &AgentCommandEnvelope,
    directory: PathBuf,
) -> ParentAssistantThreadResponse {
    let thread_id = thread_id_from_command(command);
    let now = timestamp_now();
    let mut threads = match load_threads(&directory) {
        Ok(threads) => threads,
        Err(()) => return unavailable_response(),
    };

    let active_thread = match command.command {
        AgentCommandName::AgentParentAssistantThreadList => threads
            .iter()
            .find(|thread| thread.state == ParentAssistantThreadState::Open)
            .cloned(),
        AgentCommandName::AgentParentAssistantThreadArchive => {
            let thread = upsert_thread(
                &mut threads,
                thread_id,
                ParentAssistantThreadState::Archived,
                &now,
                false,
            );
            if save_threads(&directory, &threads).is_err() {
                return unavailable_response();
            }
            Some(thread)
        }
        _ => {
            let thread = upsert_thread(
                &mut threads,
                thread_id,
                ParentAssistantThreadState::Open,
                &now,
                false,
            );
            if save_threads(&directory, &threads).is_err() {
                return unavailable_response();
            }
            Some(thread)
        }
    };

    durable_response(active_thread, threads, thread_reason(command))
}

fn record_message_for_thread_in_dir(thread_id: &str, directory: PathBuf) -> Result<(), ()> {
    record_message_for_thread_in_dir_impl(thread_id, directory)
}

fn record_message_for_thread_in_dir_impl(thread_id: &str, directory: PathBuf) -> Result<(), ()> {
    let now = timestamp_now();
    let mut threads = load_threads(&directory)?;
    upsert_thread(
        &mut threads,
        thread_id.to_string(),
        ParentAssistantThreadState::Open,
        &now,
        true,
    );
    save_threads(&directory, &threads)
}

fn durable_response(
    active_thread: Option<ParentAssistantThreadRecord>,
    threads: Vec<ParentAssistantThreadRecord>,
    reason: &'static str,
) -> ParentAssistantThreadResponse {
    ParentAssistantThreadResponse {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::DurableLocal,
        active_thread,
        threads,
        reason: Some(reason.to_string()),
    }
}

fn unavailable_response() -> ParentAssistantThreadResponse {
    ParentAssistantThreadResponse {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::Unavailable,
        active_thread: None,
        threads: Vec::new(),
        reason: Some(constants::parent_assistant::THREAD_STORAGE_UNAVAILABLE_REASON.to_string()),
    }
}

fn upsert_thread(
    threads: &mut Vec<ParentAssistantThreadRecord>,
    thread_id: String,
    state: ParentAssistantThreadState,
    now: &str,
    increment_message_count: bool,
) -> ParentAssistantThreadRecord {
    if let Some(existing) = threads
        .iter_mut()
        .find(|thread| thread.thread_id == thread_id)
    {
        existing.state = state;
        existing.backend_state = ParentAssistantBackendState::DurableLocal;
        existing.updated_at = now.to_string();
        if increment_message_count {
            existing.message_count = existing.message_count.saturating_add(1);
        }
        return existing.clone();
    }

    let thread = ParentAssistantThreadRecord {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        thread_id,
        title: constants::parent_assistant::THREAD_TITLE_DEFAULT.to_string(),
        state,
        backend_state: ParentAssistantBackendState::DurableLocal,
        created_at: now.to_string(),
        updated_at: now.to_string(),
        message_count: u32::from(increment_message_count),
    };
    threads.push(thread.clone());
    thread
}

fn load_threads(directory: &Path) -> Result<Vec<ParentAssistantThreadRecord>, ()> {
    let path = thread_storage_file(directory);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let body = read_to_string(path).map_err(|_| ())?;
    serde_json::from_str(&body).map_err(|_| ())
}

fn save_threads(directory: &Path, threads: &[ParentAssistantThreadRecord]) -> Result<(), ()> {
    create_dir_all(directory).map_err(|_| ())?;
    let body = serde_json::to_string_pretty(threads).map_err(|_| ())?;
    write(thread_storage_file(directory), body).map_err(|_| ())
}

fn thread_storage_file(directory: &Path) -> PathBuf {
    let mut path = PathBuf::from(directory);
    path.push(constants::parent_assistant::THREAD_STORAGE_FILE);
    path
}

fn thread_storage_dir() -> PathBuf {
    let directory = env::var(constants::env_var::DEV_LOG_DIR)
        .unwrap_or_else(|_| constants::dev_log::DEFAULT_DIR.to_owned());
    let mut path = PathBuf::from(directory);
    path.push(constants::parent_assistant::THREAD_STORAGE_DIR);
    path
}

fn thread_id_from_command(command: &AgentCommandEnvelope) -> String {
    string_payload_field(command, constants::parent_assistant::FIELD_THREAD_ID)
        .unwrap_or_else(|| constants::parent_assistant::DEFAULT_THREAD_ID.to_string())
}

fn thread_reason(command: &AgentCommandEnvelope) -> &'static str {
    if command.command == AgentCommandName::AgentParentAssistantThreadArchive {
        return constants::parent_assistant::THREAD_ARCHIVED_REASON;
    }
    constants::parent_assistant::THREAD_DURABLE_REASON
}

fn string_payload_field(command: &AgentCommandEnvelope, key: &str) -> Option<String> {
    match command.payload.get(key) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Some(value.trim().to_string())
        }
        _ => None,
    }
}
