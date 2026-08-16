#[path = "../support/test_invariants.rs"]
mod test_invariants;

use std::fs::{read, remove_file};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_core::enforcement_readiness::broad_os_adapter_readiness;
use ocentra_parent_agent_core::journal::ActivityJournal;
use ocentra_parent_agent_core::journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementAuditEvent;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths};
use crate::test_invariants::{require_json_decode, require_ok, require_some};
use crate::test_text::{optional_log_string, TestText};

const ENFORCEMENT_TEST_PATH_PREFIX: &str = "enforcement-tests";

#[tokio::test]
async fn enforcement_execute_records_audit_event_to_journal_and_store() {
    let paths = temp_paths(constants::enforcement::TEST_AUDIT_EVENT_ID);
    cleanup_paths(&paths);
    let event = build_enforcement_audit_report_with_paths(command(false), paths.clone()).await;
    let store = require_ok(
        ActivityStore::open(&paths.store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let status = require_ok(store.status(), constants::error::ACTIVITY_STORE_QUERIES);
    let journal_event_ids = journal_event_ids(&paths);
    let audit: EnforcementAuditEvent = require_json_decode(
        require_some(
            optional_log_string(&event.payload, constants::field::ENFORCEMENT_AUDIT_EVENT),
            constants::error::AGENT_EVENT_SERIALIZES,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
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
    assert_eq!(status.events_stored, 2);
    assert_eq!(audit.journal_sequence, Some("2".to_string()));
    assert_eq!(
        journal_event_ids,
        vec![
            TestText::from_display(format!(
                "{}{}",
                constants::enforcement::JOURNAL_BEFORE_ACTION_ID_PREFIX,
                constants::enforcement::TEST_AUDIT_EVENT_ID
            )),
            TestText::from_display(constants::enforcement::TEST_AUDIT_EVENT_ID)
        ]
    );
    #[cfg(windows)]
    {
        assert_eq!(
            event.payload.get(constants::field::ENFORCEMENT_STATUS),
            Some(&LogFieldValue::String(
                constants::enforcement::RESULT_NO_OP.to_string()
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
async fn enforcement_execute_reports_final_adapter_result_after_before_action_journal() {
    let paths = temp_paths(constants::enforcement::TEST_RESULT_ID);
    cleanup_paths(&paths);
    let event = build_enforcement_audit_report_with_paths(command(false), paths.clone()).await;
    let store = require_ok(
        ActivityStore::open(&paths.store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let summary = require_ok(
        store.recent_summary(2),
        constants::error::ACTIVITY_STORE_QUERIES,
    );
    cleanup_paths(&paths);

    assert_eq!(event.event, AgentEventName::AgentEnforcementAuditReported);
    assert_eq!(
        event.payload.get(constants::field::EVENTS_INGESTED),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        event.payload.get(constants::field::EVENTS_STORED),
        Some(&LogFieldValue::Number(2.0))
    );
    assert_eq!(
        summary.last_event_id,
        Some(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string())
    );
    assert_eq!(summary.returned, 2);

    #[cfg(windows)]
    {
        assert_eq!(
            event.payload.get(constants::field::ENFORCEMENT_STATUS),
            Some(&LogFieldValue::String(
                constants::enforcement::RESULT_NO_OP.to_string()
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
    assert_eq!(
        event
            .payload
            .get(constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE),
        Some(&LogFieldValue::String(
            constants::enforcement::ADAPTER_UNSUPPORTED_PLATFORM.to_string()
        ))
    );
}

#[tokio::test]
async fn enforcement_execute_rejects_missing_process_id_before_adapter_execution() {
    let paths = temp_paths(constants::enforcement::REJECTION_PROCESS_ID_REQUIRED);
    cleanup_paths(&paths);
    let mut command = command(false);
    command.payload = payload_without_process_id(false);
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
    for (suffix, target_type, expected_kind, readiness_id) in [
        (
            constants::enforcement::MODE_BLOCK_PROCESS,
            policy_constants::TARGET_TYPE_APP,
            constants::enforcement::ADAPTER_KIND_PROCESS_CONTROL,
            constants::enforcement::READINESS_ID_BROAD_APP_BLOCKING,
        ),
        (
            constants::enforcement::ADAPTER_KIND_NETWORK_CONTROL,
            policy_constants::TARGET_TYPE_DOMAIN,
            constants::enforcement::ADAPTER_KIND_NETWORK_CONTROL,
            constants::enforcement::READINESS_ID_NETWORK_DOMAIN_BLOCKING,
        ),
        (
            constants::enforcement::ADAPTER_KIND_MANAGED_BROWSER_CONTROL,
            policy_constants::TARGET_TYPE_SITE,
            constants::enforcement::ADAPTER_KIND_MANAGED_BROWSER_CONTROL,
            constants::enforcement::READINESS_ID_MANAGED_BROWSER_SERVICE_COMMAND,
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
        assert_unwired_adapter_readiness(&event, expected_kind, readiness_id);
    }
}

fn assert_unwired_adapter_readiness(
    event: &AgentEventEnvelope,
    expected_kind: impl std::fmt::Display,
    readiness_id: impl std::fmt::Display,
) {
    let expected_kind = TestText::from_display(expected_kind);
    let readiness_id = TestText::from_display(readiness_id);
    let action: ocentra_parent_agent_protocol::enforcement::EnforcementAction = require_json_decode(
        require_some(
            optional_log_string(&event.payload, constants::field::ENFORCEMENT_ACTION),
            constants::error::AGENT_EVENT_SERIALIZES,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let result: ocentra_parent_agent_protocol::enforcement::EnforcementResult = require_json_decode(
        require_some(
            optional_log_string(&event.payload, constants::field::ENFORCEMENT_RESULT),
            constants::error::AGENT_EVENT_SERIALIZES,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let readiness = require_some(
        broad_os_adapter_readiness(policy_constants::TEST_EVALUATED_AT)
            .entries
            .into_iter()
            .find(|entry| entry.readiness_id == readiness_id.to_string()),
        &readiness_id,
    );

    assert_eq!(
        TestText::from_display(action.adapter_kind.as_protocol_str()),
        expected_kind
    );
    assert_eq!(
        TestText::from_display(readiness.adapter_kind.as_protocol_str()),
        TestText::from_display(action.adapter_kind.as_protocol_str())
    );
    assert_eq!(
        TestText::from_display(readiness.readiness_state.as_protocol_str()),
        TestText::from_display(result.capability.capability_state.as_protocol_str())
    );
    assert_manual_or_unavailable_result(&result);
}

fn assert_manual_or_unavailable_result(
    result: &ocentra_parent_agent_protocol::enforcement::EnforcementResult,
) {
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

fn command_for_target(
    target_type: impl std::fmt::Display,
    suffix: impl std::fmt::Display,
) -> AgentCommandEnvelope {
    let target_type = target_type.to_string();
    let suffix = suffix.to_string();
    let mut command = command(false);
    command.message_id = suffix.to_string();
    command.payload = payload_for_target(&target_type, &suffix);
    command
}

fn payload(dry_run: bool) -> LogFields {
    payload_with_process_id(dry_run, true)
}

fn payload_without_process_id(dry_run: bool) -> LogFields {
    payload_with_process_id(dry_run, false)
}

fn payload_with_process_id(dry_run: bool, include_process_id: bool) -> LogFields {
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
    if include_process_id {
        fields.insert(
            constants::field::PROCESS_ID.to_string(),
            LogFieldValue::Number(f64::from(u32::MAX)),
        );
    }
    fields
}

fn payload_for_target(
    target_type: impl std::fmt::Display,
    suffix: impl std::fmt::Display,
) -> LogFields {
    let target_type = target_type.to_string();
    let suffix = suffix.to_string();
    let mut fields = payload_without_process_id(false);
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(target_type),
    );
    fields.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(suffix.clone()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(suffix),
    );
    fields
}

fn journal_event_ids(paths: &EnforcementJournalPaths) -> Vec<TestText> {
    let key_bytes = require_ok(read(&paths.key_path), constants::error::JOURNAL_READS);
    let key: [u8; JOURNAL_KEY_BYTES] =
        require_ok(key_bytes.try_into(), constants::error::JOURNAL_READS);
    let journal = require_ok(
        ActivityJournal::open(paths.journal_path.clone(), JournalKey::from_bytes(key)),
        constants::error::JOURNAL_OPENS,
    );
    require_ok(journal.lines(), constants::error::JOURNAL_READS)
        .into_iter()
        .map(|line| TestText::from_display(line.event_id))
        .collect()
}

fn temp_paths(suffix: impl std::fmt::Display) -> EnforcementJournalPaths {
    let suffix = suffix.to_string();
    let build_path = |role, extension| {
        let mut name = String::from(constants::journal::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(ENFORCEMENT_TEST_PATH_PREFIX);
        name.push(constants::delimiter::HYPHEN);
        name.push_str(&suffix);
        name.push(constants::delimiter::HYPHEN);
        name.push_str(role);
        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(extension);
        path
    };
    EnforcementJournalPaths {
        journal_path: build_path(
            constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX,
            constants::journal::FILE_EXTENSION,
        ),
        key_path: build_path(
            constants::activity_store::TEST_CAPTURE_KEY_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        store_path: build_path(
            constants::activity_store::TEST_STORE_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        timer_state_path: crate::enforcement_timer_state_path::EnforcementTimerStatePath(
            build_path(
                constants::enforcement::TIMER_STATE_ID_PREFIX,
                constants::activity_store::FILE_EXTENSION,
            ),
        ),
    }
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
