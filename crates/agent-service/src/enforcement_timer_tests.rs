use std::fs::{read_to_string, remove_file};

use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, EnforcementActiveTimerState,
    EnforcementAuditEvent, EnforcementTimerEvent, LogFieldValue, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths},
    enforcement_timer_api::build_enforcement_timer_report_with_paths,
};

#[tokio::test]
async fn timer_recovery_and_parent_cancel_use_persisted_active_state() {
    let paths = temp_paths(constants::enforcement::TEST_TIMER_STATE_ID);
    cleanup_paths(&paths);

    let execute_event =
        build_enforcement_audit_report_with_paths(execute_command(), paths.clone()).await;
    let stored_state = read_state(&paths);
    let recovered_event =
        build_enforcement_timer_report_with_paths(recover_command(), paths.clone()).await;
    let recovered_state = read_state(&paths);
    let cancel_event =
        build_enforcement_timer_report_with_paths(cancel_command(), paths.clone()).await;
    let state_after_cancel = read_to_string(&paths.timer_state_path);
    cleanup_paths(&paths);

    assert_eq!(
        execute_event.event,
        AgentEventName::AgentEnforcementAuditReported
    );
    assert_eq!(
        stored_state.timer_event.timer_event_kind.as_protocol_str(),
        constants::enforcement::TIMER_CREATED
    );
    assert_eq!(
        recovered_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert_eq!(
        recovered_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_RESTART_RECOVERED.to_string()
        ))
    );
    assert_eq!(
        recovered_state.action.action_id,
        constants::enforcement::TEST_ACTION_ID
    );
    assert_eq!(
        cancel_event.event,
        AgentEventName::AgentEnforcementTimerReported
    );
    assert_eq!(
        cancel_event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_CANCELLED.to_string()
        ))
    );
    assert!(state_after_cancel.is_err());

    let timer = payload_timer_event(&cancel_event.payload);
    let audit = payload_audit_event(&cancel_event.payload);
    assert_eq!(timer.action_id, constants::enforcement::TEST_ACTION_ID);
    assert_eq!(
        audit.audit_event_kind.as_protocol_str(),
        constants::enforcement::AUDIT_CANCELLED
    );
    assert_eq!(
        audit
            .parent_override
            .as_ref()
            .map(|reference| reference.action_reference_id.as_str()),
        Some(constants::enforcement::TEST_PARENT_ACTION_REFERENCE_ID)
    );
}

#[tokio::test]
async fn timer_recovery_reports_unavailable_when_active_state_is_missing() {
    let paths = temp_paths(constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED);
    cleanup_paths(&paths);
    let event = build_enforcement_timer_report_with_paths(recover_command(), paths.clone()).await;
    cleanup_paths(&paths);

    assert_eq!(event.event, AgentEventName::AgentEnforcementTimerReported);
    assert_eq!(
        event.payload.get(constants::field::AVAILABLE),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        event.payload.get(constants::field::REASON),
        Some(&LogFieldValue::String(
            constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND),
        Some(&LogFieldValue::String(
            constants::enforcement::TIMER_RECOVERY_NEEDED.to_string()
        ))
    );
}

fn execute_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_ACTION_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: portal_peer(),
        target: target(),
        command: AgentCommandName::AgentEnforcementExecute,
        payload: execute_payload(),
    }
}

fn recover_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_TIMER_EVENT_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: portal_peer(),
        target: target(),
        command: AgentCommandName::AgentEnforcementTimerRecover,
        payload: timer_payload(),
    }
}

fn cancel_command() -> AgentCommandEnvelope {
    let mut payload = timer_payload();
    payload.insert(
        constants::field::PARENT_ACTION_REFERENCE_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_PARENT_ACTION_REFERENCE_ID.to_string()),
    );
    payload.insert(
        constants::field::PARENT_ACTOR_ID.to_string(),
        LogFieldValue::String(policy_constants::TEST_PARENT_ACTOR_ID.to_string()),
    );
    payload.insert(
        constants::field::PARENT_ACTOR_ROLE.to_string(),
        LogFieldValue::String(policy_constants::ACTOR_ROLE_PARENT.to_string()),
    );
    payload.insert(
        constants::field::PARENT_ACTION_CREATED_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVALUATED_AT.to_string()),
    );

    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::enforcement::TEST_PARENT_ACTION_REFERENCE_ID.to_string(),
        sent_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source: portal_peer(),
        target: target(),
        command: AgentCommandName::AgentEnforcementOverrideCancel,
        payload,
    }
}

fn execute_payload() -> LogFields {
    let mut fields = timer_payload();
    fields.insert(
        constants::field::ENFORCEMENT_INTENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_INTENT_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_ACTION.to_string(),
        LogFieldValue::String(policy_constants::ACTION_ASK_PARENT.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(policy_constants::TARGET_TYPE_DEVICE.to_string()),
    );
    fields.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
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
        constants::field::EXPIRES_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EXPIRES_AT.to_string()),
    );
    fields
}

fn timer_payload() -> LogFields {
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
        constants::field::REQUESTED_AT.to_string(),
        LogFieldValue::String(policy_constants::TEST_EVALUATED_AT.to_string()),
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
    fields
}

fn portal_peer() -> AgentPeer {
    AgentPeer {
        peer_id: constants::peer::PORTAL_DEV.to_string(),
        role: AgentPeerRole::Portal,
    }
}

fn target() -> AgentMessageTarget {
    AgentMessageTarget {
        device_id: constants::enforcement::TEST_CHILD_DEVICE_ID.to_string(),
        platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
        route: AgentRoute::Localhost,
    }
}

fn read_state(paths: &EnforcementJournalPaths) -> EnforcementActiveTimerState {
    let text = read_to_string(&paths.timer_state_path).expect(constants::error::JOURNAL_READS);
    serde_json::from_str(&text).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn payload_timer_event(payload: &LogFields) -> EnforcementTimerEvent {
    payload_string(payload, constants::field::ENFORCEMENT_TIMER_EVENT)
        .and_then(|text| serde_json::from_str(text).ok())
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn payload_audit_event(payload: &LogFields) -> EnforcementAuditEvent {
    payload_string(payload, constants::field::ENFORCEMENT_AUDIT_EVENT)
        .and_then(|text| serde_json::from_str(text).ok())
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn payload_string<'a>(payload: &'a LogFields, field: &str) -> Option<&'a str> {
    match payload.get(field) {
        Some(LogFieldValue::String(value)) => Some(value.as_str()),
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
