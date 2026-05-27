use std::fs::{read_to_string, remove_file};

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, policy_constants, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths};

#[tokio::test]
async fn enforcement_execute_records_audit_event_to_journal_and_store() {
    let paths = temp_paths(constants::enforcement::TEST_AUDIT_EVENT_ID);
    cleanup_paths(&paths);
    let event = build_enforcement_audit_report_with_paths(command(false), paths.clone()).await;
    let store =
        ActivityStore::open(&paths.store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    let status = store
        .status()
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let journal_text = read_to_string(&paths.journal_path).expect(constants::error::JOURNAL_READS);
    cleanup_paths(&paths);

    assert_eq!(event.event, AgentEventName::AgentEnforcementAuditReported);
    assert_eq!(
        event.payload.get(constants::field::DATABASE_READY),
        Some(&LogFieldValue::Boolean(true))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::ENFORCEMENT_JOURNAL_EVENT_ID),
        Some(&LogFieldValue::String(
            constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()
        ))
    );
    assert_eq!(status.events_stored, 1);
    assert!(!journal_text.contains(policy_constants::TEST_DECISION_ID));

    #[cfg(windows)]
    {
        assert_eq!(
            event.payload.get(constants::field::ENFORCEMENT_STATUS),
            Some(&LogFieldValue::String(
                constants::enforcement::RESULT_ACTUALLY_ENFORCED.to_string()
            ))
        );
        assert_eq!(
            event
                .payload
                .get(constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE),
            Some(&LogFieldValue::String(
                constants::enforcement::ADAPTER_PROCESS_ALREADY_EXITED.to_string()
            ))
        );
    }

    #[cfg(not(windows))]
    {
        assert_eq!(
            event.payload.get(constants::field::ENFORCEMENT_STATUS),
            Some(&LogFieldValue::String(
                constants::enforcement::RESULT_UNAVAILABLE.to_string()
            ))
        );
        assert_eq!(
            event
                .payload
                .get(constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE),
            Some(&LogFieldValue::String(
                constants::enforcement::ADAPTER_UNSUPPORTED_PLATFORM.to_string()
            ))
        );
    }
}

#[tokio::test]
async fn enforcement_execute_rejects_missing_process_id_before_adapter_execution() {
    let paths = temp_paths(constants::enforcement::REJECTION_PROCESS_ID_REQUIRED);
    cleanup_paths(&paths);
    let mut command = command(false);
    command.payload.remove(constants::field::PROCESS_ID);
    let event = build_enforcement_audit_report_with_paths(command, paths.clone()).await;
    cleanup_paths(&paths);

    #[cfg(windows)]
    {
        assert_eq!(event.event, AgentEventName::AgentCommandRejected);
        assert_eq!(
            event.payload.get(constants::field::REASON),
            Some(&LogFieldValue::String(
                constants::enforcement::REJECTION_PROCESS_ID_REQUIRED.to_string()
            ))
        );
    }

    #[cfg(not(windows))]
    {
        assert_eq!(event.event, AgentEventName::AgentEnforcementAuditReported);
        assert_eq!(
            event.payload.get(constants::field::ENFORCEMENT_STATUS),
            Some(&LogFieldValue::String(
                constants::enforcement::RESULT_UNAVAILABLE.to_string()
            ))
        );
    }
}

#[tokio::test]
async fn dry_run_enforcement_execute_journals_without_adapter_request() {
    let paths = temp_paths(constants::enforcement::ADAPTER_DRY_RUN_NO_ACTION);
    cleanup_paths(&paths);
    let event = build_enforcement_audit_report_with_paths(command(true), paths.clone()).await;
    cleanup_paths(&paths);

    assert_eq!(event.event, AgentEventName::AgentEnforcementAuditReported);
    assert_eq!(
        event.payload.get(constants::field::ENFORCEMENT_STATUS),
        Some(&LogFieldValue::String(
            constants::enforcement::RESULT_WOULD_ENFORCE.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE),
        Some(&LogFieldValue::String(
            constants::enforcement::ADAPTER_DRY_RUN_NO_ACTION.to_string()
        ))
    );
}

#[tokio::test]
async fn enforcement_execute_reports_manual_required_service_states_for_unwired_adapters() {
    for (suffix, target_type, expected_kind) in [
        (
            constants::enforcement::MODE_BLOCK_PROCESS,
            policy_constants::TARGET_TYPE_APP,
            constants::enforcement::ADAPTER_KIND_PROCESS_CONTROL,
        ),
        (
            constants::enforcement::ADAPTER_KIND_NETWORK_CONTROL,
            policy_constants::TARGET_TYPE_DOMAIN,
            constants::enforcement::ADAPTER_KIND_NETWORK_CONTROL,
        ),
        (
            constants::enforcement::ADAPTER_KIND_MANAGED_BROWSER_CONTROL,
            policy_constants::TARGET_TYPE_SITE,
            constants::enforcement::ADAPTER_KIND_MANAGED_BROWSER_CONTROL,
        ),
    ] {
        let paths = temp_paths(suffix);
        cleanup_paths(&paths);
        let event = build_enforcement_audit_report_with_paths(
            command_for_target(target_type, suffix),
            paths.clone(),
        )
        .await;
        cleanup_paths(&paths);

        assert_eq!(event.event, AgentEventName::AgentEnforcementAuditReported);
        assert_eq!(
            event.payload.get(constants::field::ENFORCEMENT_STATUS),
            Some(&LogFieldValue::String(
                constants::enforcement::RESULT_UNAVAILABLE.to_string()
            ))
        );
        let action = payload_string(&event.payload, constants::field::ENFORCEMENT_ACTION)
            .and_then(|text| {
                serde_json::from_str::<ocentra_parent_agent_protocol::EnforcementAction>(text).ok()
            })
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let result = payload_string(&event.payload, constants::field::ENFORCEMENT_RESULT)
            .and_then(|text| {
                serde_json::from_str::<ocentra_parent_agent_protocol::EnforcementResult>(text).ok()
            })
            .expect(constants::error::AGENT_EVENT_SERIALIZES);

        assert_eq!(action.adapter_kind.as_protocol_str(), expected_kind);
        #[cfg(windows)]
        {
            assert_eq!(
                result.capability.capability_state.as_protocol_str(),
                constants::enforcement::CAPABILITY_MANUAL_REQUIRED
            );
            assert_eq!(
                result
                    .unavailable_status
                    .as_ref()
                    .map(|status| status.unavailable_reason.as_protocol_str()),
                Some(constants::enforcement::UNAVAILABLE_MANUAL_REQUIRED)
            );
        }
        #[cfg(not(windows))]
        assert_eq!(
            result.capability.capability_state.as_protocol_str(),
            constants::enforcement::CAPABILITY_UNAVAILABLE
        );
    }
}

fn command(dry_run: bool) -> AgentCommandEnvelope {
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
        payload: payload(dry_run),
    }
}

fn command_for_target(target_type: &str, suffix: &str) -> AgentCommandEnvelope {
    let mut command = command(false);
    command.message_id = suffix.to_string();
    command.payload = payload_for_target(target_type, suffix);
    command
}

fn payload(dry_run: bool) -> LogFields {
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
        LogFieldValue::Boolean(dry_run),
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

fn payload_for_target(target_type: &str, suffix: &str) -> LogFields {
    let mut fields = payload(false);
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(target_type.to_string()),
    );
    fields.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(suffix.to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(suffix.to_string()),
    );
    fields.remove(constants::field::PROCESS_ID);
    fields
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
    for index in 1..=3 {
        let mut rotated_path = paths.journal_path.clone();
        let mut extension = index.to_string();
        extension.push(constants::delimiter::DOT);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
    let mut wal_path = paths.store_path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = paths.store_path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
