use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogLevel,
    ParentAssistantActionConfirmResult, ParentAssistantBackendState, ParentAssistantProviderState,
    ParentAssistantProviderStatus, ParentAssistantRunCancelResult, ParentAssistantRunCancelState,
    ParentAssistantThreadResponse, ParentAssistantThreadState, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    fields::fields_from_pairs,
    parent_assistant_api::{
        build_parent_assistant_scaffold_event, thread_store::thread_response_for_command_in_dir,
    },
};

#[test]
fn parent_assistant_thread_create_returns_durable_service_state() {
    let event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantThreadCreate,
        fields_from_pairs(vec![(
            constants::parent_assistant::FIELD_THREAD_ID,
            LogFieldValue::String(constants::parent_assistant::DEFAULT_THREAD_ID.to_string()),
        )]),
    ));
    let response =
        thread_response_payload(&event.payload[constants::parent_assistant::FIELD_THREAD_RESPONSE]);

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantThreadUpdated
    );
    assert_eq!(event.severity, LogLevel::Info);
    assert_eq!(
        response.backend_state,
        ParentAssistantBackendState::DurableLocal
    );
    assert_eq!(
        response
            .active_thread
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .state,
        ParentAssistantThreadState::Open
    );
}

#[test]
fn parent_assistant_thread_list_reads_durable_local_store_after_create() {
    let directory = unique_temp_dir();
    let create = command(
        AgentCommandName::AgentParentAssistantThreadCreate,
        fields_from_pairs(vec![(
            constants::parent_assistant::FIELD_THREAD_ID,
            LogFieldValue::String(constants::parent_assistant::DEFAULT_THREAD_ID.to_string()),
        )]),
    );
    let list = command(
        AgentCommandName::AgentParentAssistantThreadList,
        Default::default(),
    );

    let created = thread_response_for_command_in_dir(&create, directory.clone());
    let listed = thread_response_for_command_in_dir(&list, directory.clone());

    remove_temp_dir(directory);

    assert_eq!(
        created.backend_state,
        ParentAssistantBackendState::DurableLocal
    );
    assert_eq!(
        listed.backend_state,
        ParentAssistantBackendState::DurableLocal
    );
    assert_eq!(listed.threads.len(), 1);
    assert_eq!(
        listed
            .active_thread
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .thread_id,
        constants::parent_assistant::DEFAULT_THREAD_ID
    );
}

#[test]
fn parent_assistant_provider_status_reports_local_runtime_and_api_boundary() {
    let event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantProviderStatusGet,
        Default::default(),
    ));
    let status =
        provider_status_payload(&event.payload[constants::parent_assistant::FIELD_PROVIDER_STATUS]);

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantProviderDegraded
    );
    assert_eq!(
        status.backend_state,
        ParentAssistantBackendState::RuntimeBacked
    );
    assert_eq!(
        status.provider_state,
        ParentAssistantProviderState::Unavailable
    );
    assert_eq!(status.queue_depth, 0);
    assert!(
        !status
            .api_provider_boundary
            .child_safety_or_enforcement_use_allowed
    );
}

#[test]
fn parent_assistant_run_cancel_reports_not_running_without_process_kill_claim() {
    let event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantRunCancel,
        fields_from_pairs(vec![(
            constants::parent_assistant::FIELD_RUN_ID,
            LogFieldValue::String(constants::parent_assistant::DEFAULT_RUN_ID.to_string()),
        )]),
    ));
    let result =
        run_cancel_payload(&event.payload[constants::parent_assistant::FIELD_RUN_CANCEL_RESULT]);

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantErrorReported
    );
    assert_eq!(
        result.cancel_state,
        ParentAssistantRunCancelState::NotRunning
    );
    assert_eq!(
        result.unavailable_reason.as_deref(),
        Some(constants::parent_assistant::RUN_NOT_RUNNING_REASON)
    );
}

#[test]
fn parent_assistant_action_confirm_requires_child_contract_without_enforcement() {
    let event = build_parent_assistant_scaffold_event(command(
        AgentCommandName::AgentParentAssistantActionConfirm,
        fields_from_pairs(vec![(
            constants::parent_assistant::FIELD_ACTION_INTENT_ID,
            LogFieldValue::String(
                constants::parent_assistant::DEFAULT_ACTION_INTENT_ID.to_string(),
            ),
        )]),
    ));
    let result = action_confirm_payload(
        &event.payload[constants::parent_assistant::FIELD_ACTION_CONFIRM_RESULT],
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentParentAssistantActionConfirmed
    );
    assert_eq!(
        result.backend_state,
        ParentAssistantBackendState::ContractRequired
    );
    assert!(result.requires_controller_lease);
    assert!(result.child_agent_contract_required);
    assert!(!result.enforcement_applied);
    assert!(!result.policy_written);
}

fn command(
    command_name: AgentCommandName,
    payload: ocentra_parent_agent_protocol::LogFields,
) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::parent_assistant::DEFAULT_MESSAGE_ID.to_string(),
        sent_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: command_name,
        payload,
    }
}

fn thread_response_payload(value: &LogFieldValue) -> ParentAssistantThreadResponse {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn provider_status_payload(value: &LogFieldValue) -> ParentAssistantProviderStatus {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn run_cancel_payload(value: &LogFieldValue) -> ParentAssistantRunCancelResult {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn action_confirm_payload(value: &LogFieldValue) -> ParentAssistantActionConfirmResult {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn unique_temp_dir() -> PathBuf {
    let mut name = constants::parent_assistant::THREAD_STORAGE_DIR.to_string();
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&nanos_now().to_string());
    let mut path = std::env::temp_dir();
    path.push(name);
    path
}

fn nanos_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
        .as_nanos()
}

fn remove_temp_dir(path: PathBuf) {
    let _ = fs::remove_dir_all(path);
}
