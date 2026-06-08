use std::fs::remove_file;

use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    AppGameAdapterDispatchResultReadModel, LogFieldValue, LogFields, AGENT_PROTOCOL_SCHEMA_VERSION,
    APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_COMMAND_ID,
    APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_DEVICE_ID,
    APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_PORTAL_PEER,
    APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_SENT_AT, APP_GAME_PARENT_PLATFORM_WINDOWS,
};

use crate::enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths};

use super::app_game_adapter_dispatch_result_payload::{
    build_activity_app_game_adapter_dispatch_result_report,
    build_activity_app_game_adapter_dispatch_result_report_with_store_path,
};

#[tokio::test]
async fn app_game_adapter_dispatch_result_command_returns_typed_event() {
    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_COMMAND_ID.to_string(),
        sent_at: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_SENT_AT.to_string(),
        source: AgentPeer {
            peer_id: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_PORTAL_PEER.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_DEVICE_ID.to_string(),
            platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet,
        payload: Default::default(),
    };

    let event = build_activity_app_game_adapter_dispatch_result_report(command).await;

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameAdapterDispatchResultReadModelReported
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL)
            .is_some(),
        true
    );
}

#[tokio::test]
async fn app_game_adapter_dispatch_result_command_reads_latest_store_audit_evidence() {
    let paths = temp_paths(constants::enforcement::TEST_AUDIT_EVENT_ID);
    cleanup_paths(&paths);
    let enforcement_event =
        build_enforcement_audit_report_with_paths(enforcement_execute_command(), paths.clone())
            .await;
    let event = build_activity_app_game_adapter_dispatch_result_report_with_store_path(
        dispatch_result_command(),
        paths.store_path.clone(),
    )
    .await;
    cleanup_paths(&paths);

    assert_eq!(
        enforcement_event.event,
        AgentEventName::AgentEnforcementAuditReported
    );
    assert_eq!(
        event.event,
        AgentEventName::AgentActivityAppGameAdapterDispatchResultReadModelReported
    );
    let read_model = dispatch_result_read_model(&event);
    assert_eq!(read_model.adapter_execution_reported_count, 1);
    assert_eq!(read_model.adapter_execution_evidence_missing_count, 0);
    assert_eq!(read_model.adapter_dispatch_executed_claimed_count, 1);
    let accepted = read_model
        .rows
        .iter()
        .find(|row| row.adapter_dispatch_executed_claimed)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(
        accepted.dispatch_adapter_execution_result_id.as_deref(),
        Some(constants::enforcement::TEST_RESULT_ID)
    );
    assert_eq!(
        accepted
            .dispatch_adapter_execution_audit_event_id
            .as_deref(),
        Some(constants::enforcement::TEST_AUDIT_EVENT_ID)
    );

    #[cfg(windows)]
    assert_eq!(
        accepted.dispatch_adapter_execution_status.as_deref(),
        Some(constants::enforcement::RESULT_ACTUALLY_ENFORCED)
    );

    #[cfg(not(windows))]
    assert_eq!(
        accepted.dispatch_adapter_execution_status.as_deref(),
        Some(constants::enforcement::RESULT_UNAVAILABLE)
    );
}

fn dispatch_result_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_COMMAND_ID.to_string(),
        sent_at: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_SENT_AT.to_string(),
        source: AgentPeer {
            peer_id: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_PORTAL_PEER.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_DEVICE_ID.to_string(),
            platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet,
        payload: Default::default(),
    }
}

fn enforcement_execute_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_ACTION_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentEnforcementExecute,
        payload: enforcement_execute_payload(),
    }
}

fn enforcement_execute_payload() -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::POLICY_DECISION_ID.to_string(),
        LogFieldValue::String(policy_constants::TEST_DECISION_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_VERSION.to_string(),
        LogFieldValue::String(policy_constants::TEST_POLICY_VERSION.to_string()),
    );
    fields.insert(
        constants::field::POLICY_ACTION.to_string(),
        LogFieldValue::String(policy_constants::ACTION_BLOCK.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(policy_constants::TARGET_TYPE_PROCESS.to_string()),
    );
    fields.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_PROCESS_TARGET_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_PROCESS_TARGET_VALUE.to_string()),
    );
    fields.insert(
        constants::field::POLICY_DRY_RUN.to_string(),
        LogFieldValue::Boolean(false),
    );
    fields.insert(
        constants::field::POLICY_REASON_CODES.to_string(),
        LogFieldValue::String(policy_constants::TEST_REASON_PARENT_BLOCK.to_string()),
    );
    fields.insert(
        constants::field::POLICY_RULE_IDS.to_string(),
        LogFieldValue::String(policy_constants::TEST_BLOCK_RULE_ID.to_string()),
    );
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVIDENCE_ID.to_string()),
    );
    fields.insert(
        constants::field::REQUESTED_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVALUATED_AT.to_string()),
    );
    fields.insert(
        constants::field::EXPIRES_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EXPIRES_AT.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_ACTION_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_ACTION_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_RESULT_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    fields.insert(
        constants::field::ENFORCEMENT_TIMER_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_TIMER_EVENT_ID.to_string()),
    );
    fields.insert(
        constants::field::PROCESS_ID.to_string(),
        LogFieldValue::Number(f64::from(u32::MAX)),
    );
    fields
}

fn dispatch_result_read_model(
    event: &ocentra_parent_agent_protocol::AgentEventEnvelope,
) -> AppGameAdapterDispatchResultReadModel {
    let value = event
        .payload
        .get(constants::field::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL)
        .and_then(string_log_value)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    serde_json::from_str(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn string_log_value(value: &LogFieldValue) -> Option<&str> {
    match value {
        LogFieldValue::String(value) => Some(value.as_str()),
        _ => None,
    }
}

fn temp_paths(suffix: &str) -> EnforcementJournalPaths {
    EnforcementJournalPaths {
        journal_path: temp_path(
            suffix,
            constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX,
            constants::journal::FILE_EXTENSION,
        ),
        key_path: temp_path(
            suffix,
            constants::activity_store::TEST_CAPTURE_KEY_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        store_path: temp_path(
            suffix,
            constants::activity_store::TEST_STORE_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        timer_state_path: temp_path(
            suffix,
            constants::enforcement::TIMER_STATE_ID_PREFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
    }
}

fn temp_path(suffix: &str, role: &str, extension: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::journal::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(role);
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn cleanup_paths(paths: &EnforcementJournalPaths) {
    let _ = remove_file(&paths.journal_path);
    let _ = remove_file(&paths.key_path);
    let _ = remove_file(&paths.store_path);
    let _ = remove_file(&paths.timer_state_path);
    let mut wal_path = paths.store_path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = paths.store_path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
