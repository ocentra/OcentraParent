use std::fs::remove_file;
use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;

use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultReadModel;
use ocentra_parent_agent_protocol::app_game_authority_classifier::APP_GAME_PARENT_PLATFORM_WINDOWS;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::enforcement_api::{build_enforcement_audit_report_with_paths, EnforcementJournalPaths};
use crate::test_invariants::{require_json_decode, require_some};
use crate::test_text::TestText;

use super::app_game_adapter_dispatch_execute_payload::build_activity_app_game_adapter_dispatch_execute_report_with_paths;
use super::app_game_adapter_dispatch_result_payload::{
    build_activity_app_game_adapter_dispatch_result_report,
    build_activity_app_game_adapter_dispatch_result_report_with_store_path, ActivityStorePath,
};

const APP_GAME_ADAPTER_DISPATCH_EXECUTE_TEST_COMMAND_ID: &TestStr =
    "app-game-adapter-dispatch-execute-command";
const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_COMMAND_ID: &TestStr =
    "app-game-adapter-dispatch-result-command";
const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_DEVICE_ID: &TestStr = "child-device";
const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_PORTAL_PEER: &TestStr = "portal-dev";
const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_SENT_AT: &TestStr = "2026-06-08T10:44:01Z";

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
    assert!(event
        .payload
        .get(constants::field::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL)
        .is_some());
}

#[tokio::test]
async fn app_game_adapter_dispatch_result_command_ignores_unowned_store_audit_evidence() {
    let paths = temp_paths(constants::enforcement::TEST_AUDIT_EVENT_ID);
    cleanup_paths(&paths);
    let enforcement_event =
        build_enforcement_audit_report_with_paths(enforcement_execute_command(), paths.clone())
            .await;
    let event = build_activity_app_game_adapter_dispatch_result_report_with_store_path(
        dispatch_result_command(),
        ActivityStorePath(paths.store_path.clone()),
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
    assert_eq!(read_model.adapter_execution_reported_count, 0);
    assert_eq!(read_model.adapter_execution_evidence_missing_count, 1);
    assert!(read_model.rows.iter().all(|row| {
        row.dispatch_adapter_execution_result_id.is_none()
            && row.dispatch_adapter_execution_audit_event_id.is_none()
            && !row.adapter_dispatch_executed_claimed
    }));
}

#[tokio::test]
async fn app_game_adapter_dispatch_readback_excludes_rejected_enforcement_audits() {
    let paths = temp_paths("rejected-enforcement-audit");
    cleanup_paths(&paths);
    let mut command = enforcement_execute_command();
    command.payload.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(policy_constants::TARGET_TYPE_DEVICE.to_string()),
    );
    command.payload.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
    );
    command.payload.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
    );
    let rejection = build_enforcement_audit_report_with_paths(command, paths.clone()).await;
    let event = build_activity_app_game_adapter_dispatch_result_report_with_store_path(
        dispatch_result_command(),
        ActivityStorePath(paths.store_path.clone()),
    )
    .await;
    cleanup_paths(&paths);

    assert_eq!(rejection.event, AgentEventName::AgentCommandRejected);
    let read_model = dispatch_result_read_model(&event);
    assert_eq!(read_model.adapter_execution_reported_count, 0);
    assert_eq!(read_model.adapter_execution_evidence_missing_count, 1);
    assert!(read_model.rows.iter().all(|row| {
        row.dispatch_adapter_execution_status.is_none()
            && row.dispatch_adapter_execution_audit_event_id.is_none()
            && !row.adapter_dispatch_executed_claimed
    }));
}

#[tokio::test]
async fn app_game_adapter_dispatch_readback_skips_newer_rejected_audit_for_typed_execution() {
    let paths = temp_paths("rejected-audit-after-execution");
    cleanup_paths(&paths);
    let accepted = build_activity_app_game_adapter_dispatch_execute_report_with_paths(
        dispatch_execute_command(),
        paths.clone(),
    )
    .await;
    let mut rejected_command = enforcement_execute_command();
    rejected_command.payload.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(policy_constants::TARGET_TYPE_DEVICE.to_string()),
    );
    rejected_command.payload.insert(
        constants::field::TARGET_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
    );
    rejected_command.payload.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_CHILD_DEVICE_ID.to_string()),
    );
    rejected_command.payload.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(format!("z{}", constants::enforcement::TEST_AUDIT_EVENT_ID)),
    );
    let rejected = build_enforcement_audit_report_with_paths(rejected_command, paths.clone()).await;
    let event = build_activity_app_game_adapter_dispatch_result_report_with_store_path(
        dispatch_result_command(),
        ActivityStorePath(paths.store_path.clone()),
    )
    .await;
    cleanup_paths(&paths);

    assert_eq!(
        accepted.event,
        AgentEventName::AgentActivityAppGameAdapterDispatchExecuted
    );
    assert_eq!(rejected.event, AgentEventName::AgentCommandRejected);
    let read_model = dispatch_result_read_model(&event);
    assert_eq!(read_model.adapter_execution_reported_count, 1);
    assert_eq!(read_model.adapter_execution_evidence_missing_count, 0);
    assert!(read_model.rows.iter().any(|row| {
        row.dispatch_adapter_execution_audit_event_id.as_deref()
            == Some(constants::enforcement::TEST_AUDIT_EVENT_ID)
    }));
}

#[tokio::test]
async fn app_game_adapter_dispatch_execute_command_runs_scoped_enforcement_and_readback() {
    let paths = temp_paths(APP_GAME_ADAPTER_DISPATCH_EXECUTE_TEST_COMMAND_ID);
    cleanup_paths(&paths);
    let execute_event = build_activity_app_game_adapter_dispatch_execute_report_with_paths(
        dispatch_execute_command(),
        paths.clone(),
    )
    .await;
    let readback_event = build_activity_app_game_adapter_dispatch_result_report_with_store_path(
        dispatch_result_command(),
        ActivityStorePath(paths.store_path.clone()),
    )
    .await;
    cleanup_paths(&paths);

    assert_eq!(
        execute_event.event,
        AgentEventName::AgentActivityAppGameAdapterDispatchExecuted
    );
    let execute_result = dispatch_execute_result(&execute_event);
    assert_eq!(
        execute_result
            .get(constants::field::EXECUTION_RESULT_ID)
            .and_then(|value| value.as_str()),
        Some(constants::enforcement::TEST_RESULT_ID)
    );
    assert_eq!(
        execute_result
            .get(constants::field::EXECUTION_AUDIT_EVENT_ID)
            .and_then(|value| value.as_str()),
        Some(constants::enforcement::TEST_AUDIT_EVENT_ID)
    );
    assert_eq!(
        execute_result
            .get(constants::field::BROAD_INSTALLED_APP_BLOCKING_CLAIMED)
            .and_then(|value| value.as_bool()),
        Some(false)
    );
    let read_model = dispatch_result_read_model(&readback_event);
    assert_eq!(read_model.adapter_execution_reported_count, 1);
    assert_eq!(read_model.adapter_execution_evidence_missing_count, 0);
}

#[tokio::test]
async fn app_game_adapter_dispatch_execute_rejects_non_windows_targets() {
    let paths = temp_paths(constants::enforcement::PLATFORM_LINUX);
    cleanup_paths(&paths);
    let mut command = dispatch_execute_command();
    command.target.platform = constants::enforcement::PLATFORM_LINUX.to_string();
    let event =
        build_activity_app_game_adapter_dispatch_execute_report_with_paths(command, paths.clone())
            .await;
    cleanup_paths(&paths);

    assert_eq!(event.event, AgentEventName::AgentCommandRejected);
    assert_eq!(
        event
            .payload
            .get(constants::field::REASON)
            .and_then(string_log_value),
        Some(TestText::from_display(
            constants::enforcement::REJECTION_UNSUPPORTED_CAPABILITY,
        ))
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

fn dispatch_execute_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: APP_GAME_ADAPTER_DISPATCH_EXECUTE_TEST_COMMAND_ID.to_string(),
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
        command: AgentCommandName::AgentActivityAppGameAdapterDispatchExecute,
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
    event: &ocentra_parent_agent_protocol::transport::AgentEventEnvelope,
) -> AppGameAdapterDispatchResultReadModel {
    let value = require_some(
        event
            .payload
            .get(constants::field::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL)
            .and_then(string_log_value),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    require_json_decode(value, constants::error::AGENT_EVENT_SERIALIZES)
}

fn dispatch_execute_result(
    event: &ocentra_parent_agent_protocol::transport::AgentEventEnvelope,
) -> serde_json::Value {
    let value = require_some(
        event
            .payload
            .get(constants::field::APP_GAME_ADAPTER_DISPATCH_EXECUTE_RESULT)
            .and_then(string_log_value),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    require_json_decode(value, constants::error::AGENT_EVENT_SERIALIZES)
}

fn string_log_value(value: &LogFieldValue) -> Option<TestText> {
    match value {
        LogFieldValue::String(value) => Some(TestText::from_display(value.as_str())),
        _ => None,
    }
}

fn temp_paths(suffix: impl std::fmt::Display) -> EnforcementJournalPaths {
    let suffix = suffix.to_string();
    EnforcementJournalPaths {
        journal_path: temp_path(
            &suffix,
            constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX,
            constants::journal::FILE_EXTENSION,
        ),
        key_path: temp_path(
            &suffix,
            constants::activity_store::TEST_CAPTURE_KEY_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        store_path: temp_path(
            &suffix,
            constants::activity_store::TEST_STORE_SUFFIX,
            constants::activity_store::FILE_EXTENSION,
        ),
        timer_state_path: crate::enforcement_timer_state_path::EnforcementTimerStatePath(
            temp_path(
                &suffix,
                constants::enforcement::TIMER_STATE_ID_PREFIX,
                constants::activity_store::FILE_EXTENSION,
            ),
        ),
    }
}

fn temp_path(
    suffix: impl std::fmt::Display,
    role: impl std::fmt::Display,
    extension: impl std::fmt::Display,
) -> TestPathBuf {
    let suffix = suffix.to_string();
    let role = role.to_string();
    let extension = extension.to_string();
    let mut name = TestString::from(constants::journal::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&suffix);
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&role);
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(extension);
    path
}

fn cleanup_paths(paths: &EnforcementJournalPaths) {
    let _ = remove_file(&paths.journal_path);
    let _ = remove_file(&paths.key_path);
    let _ = remove_file(&paths.store_path);
    let _ = remove_file(&paths.timer_state_path.0);
    let mut wal_path = paths.store_path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = paths.store_path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
