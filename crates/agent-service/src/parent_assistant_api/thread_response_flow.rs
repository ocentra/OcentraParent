use std::path::Path;

use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadRecord;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadResponse;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadState;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;

use crate::time::timestamp_now;

use super::durable_response;
use super::load_threads;
use super::save_threads;
use super::thread_id_from_command;
use super::thread_reason;
use super::unavailable_response;
use super::upsert_thread;
use super::ParentAssistantThreadTimestamp;

pub(super) fn thread_response_for_command_in_dir_impl(
    command: &AgentCommandEnvelope,
    directory: &Path,
) -> ParentAssistantThreadResponse {
    let thread_id = thread_id_from_command(command);
    let now = ParentAssistantThreadTimestamp(timestamp_now());
    let mut threads = match load_threads(directory) {
        Ok(threads) => threads,
        Err(()) => return unavailable_response(),
    };
    let active_thread =
        active_thread_for_command(command, &mut threads, thread_id, &now, directory);

    durable_response(active_thread, threads, thread_reason(command))
}

fn active_thread_for_command(
    command: &AgentCommandEnvelope,
    threads: &mut Vec<ParentAssistantThreadRecord>,
    thread_id: super::ParentAssistantThreadId,
    now: &ParentAssistantThreadTimestamp,
    directory: &Path,
) -> Option<ParentAssistantThreadRecord> {
    match command.command {
        AgentCommandName::AgentParentAssistantThreadList => open_thread_for_list(threads),
        AgentCommandName::AgentParentAssistantThreadArchive => {
            let thread = upsert_thread(
                threads,
                thread_id,
                ParentAssistantThreadState::Archived,
                now,
                false,
            );
            save_thread_update(directory, threads, thread)
        }
        _ => {
            let thread = upsert_thread(
                threads,
                thread_id,
                ParentAssistantThreadState::Open,
                now,
                false,
            );
            save_thread_update(directory, threads, thread)
        }
    }
}

fn open_thread_for_list(
    threads: &[ParentAssistantThreadRecord],
) -> Option<ParentAssistantThreadRecord> {
    threads
        .iter()
        .find(|thread| thread.state == ParentAssistantThreadState::Open)
        .cloned()
}

fn save_thread_update(
    directory: &Path,
    threads: &[ParentAssistantThreadRecord],
    thread: ParentAssistantThreadRecord,
) -> Option<ParentAssistantThreadRecord> {
    if save_threads(directory, threads).is_err() {
        return None;
    }

    Some(thread)
}
