use std::path::Path;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantBackendState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadRecord;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadResponse;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadState;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;

#[path = "thread_response_flow.rs"]
mod thread_response_flow;

use crate::time::timestamp_now;

use super::load_threads;
use super::save_threads;
use super::ParentAssistantThreadId;
use super::ParentAssistantThreadReason;
use super::ParentAssistantThreadTimestamp;

pub(super) fn thread_response_for_command_in_dir_impl(
    command: &AgentCommandEnvelope,
    directory: &Path,
) -> ParentAssistantThreadResponse {
    thread_response_flow::thread_response_for_command_in_dir_impl(command, directory)
}

pub(super) fn record_message_for_thread_in_dir_impl(
    thread_id: ParentAssistantThreadId,
    directory: &Path,
) -> Result<(), ()> {
    let now = ParentAssistantThreadTimestamp(timestamp_now());
    let mut threads = load_threads(directory)?;
    upsert_thread(
        &mut threads,
        thread_id,
        ParentAssistantThreadState::Open,
        &now,
        true,
    );
    save_threads(directory, &threads)
}

fn durable_response(
    active_thread: Option<ParentAssistantThreadRecord>,
    threads: Vec<ParentAssistantThreadRecord>,
    reason: ParentAssistantThreadReason,
) -> ParentAssistantThreadResponse {
    ParentAssistantThreadResponse {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::DurableLocal,
        active_thread,
        threads,
        reason: Some(reason.0),
    }
}

fn unavailable_response() -> ParentAssistantThreadResponse {
    ParentAssistantThreadResponse {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        backend_state: ParentAssistantBackendState::Unavailable,
        active_thread: None,
        threads: Vec::new(),
        reason: Some(
            ParentAssistantThreadReason(
                constants::parent_assistant::THREAD_STORAGE_UNAVAILABLE_REASON.to_string(),
            )
            .0,
        ),
    }
}

fn upsert_thread(
    threads: &mut Vec<ParentAssistantThreadRecord>,
    thread_id: ParentAssistantThreadId,
    state: ParentAssistantThreadState,
    now: &ParentAssistantThreadTimestamp,
    increment_message_count: bool,
) -> ParentAssistantThreadRecord {
    if let Some(existing) = threads
        .iter_mut()
        .find(|thread| thread.thread_id == thread_id.0.as_str())
    {
        existing.state = state;
        existing.backend_state = ParentAssistantBackendState::DurableLocal;
        existing.updated_at = now.0.clone();
        if increment_message_count {
            existing.message_count = existing.message_count.saturating_add(1);
        }
        return existing.clone();
    }

    let thread = ParentAssistantThreadRecord {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        thread_id: thread_id.0,
        title: constants::parent_assistant::THREAD_TITLE_DEFAULT.to_string(),
        state,
        backend_state: ParentAssistantBackendState::DurableLocal,
        created_at: now.0.clone(),
        updated_at: now.0.clone(),
        message_count: u32::from(increment_message_count),
    };
    threads.push(thread.clone());
    thread
}

fn thread_id_from_command(command: &AgentCommandEnvelope) -> ParentAssistantThreadId {
    match command
        .payload
        .get(constants::parent_assistant::FIELD_THREAD_ID)
    {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            ParentAssistantThreadId(value.trim().to_string())
        }
        _ => ParentAssistantThreadId(constants::parent_assistant::DEFAULT_THREAD_ID.to_string()),
    }
}

fn thread_reason(command: &AgentCommandEnvelope) -> ParentAssistantThreadReason {
    if command.command == AgentCommandName::AgentParentAssistantThreadArchive {
        return ParentAssistantThreadReason(
            constants::parent_assistant::THREAD_ARCHIVED_REASON.to_string(),
        );
    }
    ParentAssistantThreadReason(constants::parent_assistant::THREAD_DURABLE_REASON.to_string())
}
