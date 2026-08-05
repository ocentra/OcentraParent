use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::{
    AppGameAdapterDispatchResultRow, APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_MISSING,
    APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_REPORTED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID,
    APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF,
    APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_ACTION_MODE,
    APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND,
    APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_EVENT,
    APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID,
};
use ocentra_parent_agent_protocol::constants::{self, v08_supported_adapter_runtime_proof};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::test_invariants::require_some;

use super::app_game_adapter_dispatch_result_payload::{
    app_game_adapter_dispatch_execution_evidence, app_game_adapter_dispatch_result_read_model,
    AppGameAdapterDispatchExecutionEvidence, DispatchExecutionStatusText,
};
use super::app_game_adapter_execution_readiness_payload::GeneratedAtText;

const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_GENERATED_AT: &str = "2026-06-08T10:44:00Z";

#[test]
fn app_game_adapter_dispatch_result_keeps_only_scoped_timer_command_accepted() {
    let read_model = app_game_adapter_dispatch_result_read_model_with_execution(
        GeneratedAtText(APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_GENERATED_AT.to_string()),
        None,
    );

    assert_eq!(
        read_model.read_model_id,
        APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID
    );
    assert_eq!(read_model.returned, 8);
    assert_eq!(read_model.command_accepted_count, 1);
    assert_eq!(read_model.blocked_before_command_count, 7);
    assert_eq!(read_model.execution_audit_recorded_count, 1);
    assert_eq!(read_model.blocked_before_execution_audit_count, 7);
    assert_eq!(read_model.adapter_execution_reported_count, 0);
    assert_eq!(read_model.adapter_execution_evidence_missing_count, 1);
    assert_eq!(read_model.blocked_before_adapter_execution_count, 7);
    assert_eq!(read_model.adapter_dispatch_command_result_claimed_count, 1);
    assert_eq!(read_model.service_local_execution_audit_claimed_count, 1);
    assert_eq!(read_model.adapter_dispatch_executed_claimed_count, 0);
    assert!(!read_model.broad_installed_app_blocking_claimed);
    assert!(!read_model.child_device_delivery_claimed);
    assert!(!read_model.platform_enforcement_claimed);
    assert!(!read_model.provider_delivery_claimed);
    assert!(!read_model.private_diagnostics_claimed);

    assert_scoped_accepted_row(&read_model.rows);
    assert_blocked_rows(&read_model.rows);
}

#[test]
fn app_game_adapter_dispatch_result_parses_enforcement_audit_payload_evidence() {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_RESULT_ID.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_STATUS.to_string(),
        LogFieldValue::String(constants::enforcement::RESULT_ACTUALLY_ENFORCED.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE.to_string(),
        LogFieldValue::String(constants::enforcement::ADAPTER_PROCESS_ALREADY_EXITED.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    payload.insert(
        constants::field::SOURCE_READ_MODEL_ID.to_string(),
        LogFieldValue::String(APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID.to_string()),
    );

    let evidence = require_some(
        app_game_adapter_dispatch_execution_evidence(&payload),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(evidence.result_id, constants::enforcement::TEST_RESULT_ID);
    assert_eq!(
        evidence.status_text.0,
        constants::enforcement::RESULT_ACTUALLY_ENFORCED
    );
    assert_eq!(
        evidence.adapter_result_code,
        constants::enforcement::ADAPTER_PROCESS_ALREADY_EXITED
    );
    assert_eq!(
        evidence.audit_event_id,
        constants::enforcement::TEST_AUDIT_EVENT_ID
    );
}

#[test]
fn app_game_adapter_dispatch_result_rejects_unowned_execution_audit_evidence() {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_RESULT_ID.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_STATUS.to_string(),
        LogFieldValue::String(constants::enforcement::RESULT_ACTUALLY_ENFORCED.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE.to_string(),
        LogFieldValue::String(constants::enforcement::ADAPTER_PROCESS_ALREADY_EXITED.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    payload.insert(
        constants::field::SOURCE_READ_MODEL_ID.to_string(),
        LogFieldValue::String("timer-enforcement-read-model".to_string()),
    );

    assert!(app_game_adapter_dispatch_execution_evidence(&payload).is_none());
}

#[test]
fn app_game_adapter_dispatch_result_keeps_typed_executed_failure_evidence() {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::ENFORCEMENT_RESULT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_RESULT_ID.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_STATUS.to_string(),
        LogFieldValue::String(constants::enforcement::RESULT_FAILED.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE.to_string(),
        LogFieldValue::String(constants::enforcement::ADAPTER_FAILED.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT_ID.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    payload.insert(
        constants::field::ENFORCEMENT_AUDIT_EVENT.to_string(),
        LogFieldValue::String(constants::enforcement::TEST_AUDIT_EVENT_ID.to_string()),
    );
    payload.insert(
        constants::field::SOURCE_READ_MODEL_ID.to_string(),
        LogFieldValue::String(APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID.to_string()),
    );

    let evidence = require_some(
        app_game_adapter_dispatch_execution_evidence(&payload),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(
        evidence.status_text.0,
        constants::enforcement::RESULT_FAILED
    );
    assert_eq!(
        evidence.adapter_result_code,
        constants::enforcement::ADAPTER_FAILED
    );
}

#[test]
fn app_game_adapter_dispatch_result_attaches_scoped_execution_evidence_only() {
    let evidence = AppGameAdapterDispatchExecutionEvidence {
        result_id: constants::enforcement::TEST_RESULT_ID.to_string(),
        status_text: DispatchExecutionStatusText(
            constants::enforcement::RESULT_ACTUALLY_ENFORCED.to_string(),
        ),
        adapter_result_code: constants::enforcement::ADAPTER_PROCESS_ALREADY_EXITED.to_string(),
        audit_event_id: constants::enforcement::TEST_AUDIT_EVENT_ID.to_string(),
    };
    let read_model = app_game_adapter_dispatch_result_read_model_with_execution(
        GeneratedAtText(APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_GENERATED_AT.to_string()),
        Some(&evidence),
    );

    assert_eq!(read_model.adapter_execution_reported_count, 1);
    assert_eq!(read_model.adapter_execution_evidence_missing_count, 0);
    assert_eq!(read_model.blocked_before_adapter_execution_count, 7);
    assert_eq!(read_model.adapter_dispatch_executed_claimed_count, 1);
    assert!(read_model.platform_enforcement_claimed);

    let accepted = scoped_accepted_row(&read_model.rows);
    assert_eq!(
        accepted.dispatch_adapter_execution_decision,
        APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_REPORTED
    );
    assert_eq!(
        accepted.dispatch_adapter_execution_result_id.as_deref(),
        Some(constants::enforcement::TEST_RESULT_ID)
    );
    assert_eq!(
        accepted.dispatch_adapter_execution_status.as_deref(),
        Some(constants::enforcement::RESULT_ACTUALLY_ENFORCED)
    );
    assert_eq!(
        accepted
            .dispatch_adapter_execution_adapter_result_code
            .as_deref(),
        Some(constants::enforcement::ADAPTER_PROCESS_ALREADY_EXITED)
    );
    assert_eq!(
        accepted
            .dispatch_adapter_execution_audit_event_id
            .as_deref(),
        Some(constants::enforcement::TEST_AUDIT_EVENT_ID)
    );
    assert!(accepted.adapter_dispatch_executed_claimed);
    assert!(accepted.platform_enforcement_claimed);
    assert_blocked_rows(&read_model.rows);
}

fn app_game_adapter_dispatch_result_read_model_with_execution(
    generated_at: GeneratedAtText,
    execution_evidence: Option<&AppGameAdapterDispatchExecutionEvidence>,
) -> ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultReadModel{
    app_game_adapter_dispatch_result_read_model(generated_at, execution_evidence)
}

fn assert_scoped_accepted_row(rows: &[AppGameAdapterDispatchResultRow]) {
    let accepted = scoped_accepted_row(rows);
    assert_eq!(
        accepted.source_proof_entry_id,
        v08_supported_adapter_runtime_proof::ENTRY_ID_APP_GAME_TIMER
    );
    assert_eq!(
        accepted.enforcement_command_name.as_deref(),
        Some(APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND)
    );
    assert_eq!(
        accepted.enforcement_event_name.as_deref(),
        Some(APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_EVENT)
    );
    assert_eq!(
        accepted.enforcement_action_mode.as_deref(),
        Some(APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_ACTION_MODE)
    );
    assert!(accepted.adapter_dispatch_command_result_claimed);
    assert_eq!(
        accepted.dispatch_execution_audit_decision,
        APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED
    );
    assert_eq!(
        accepted.dispatch_execution_audit_id.as_deref(),
        Some(APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID)
    );
    assert_eq!(
        accepted.dispatch_execution_audit_refs,
        vec![APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF.to_string()]
    );
    assert!(accepted.service_local_execution_audit_claimed);
    assert_eq!(
        accepted.dispatch_adapter_execution_decision,
        APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_MISSING
    );
    assert!(accepted.dispatch_adapter_execution_result_id.is_none());
    assert!(accepted.dispatch_adapter_execution_status.is_none());
    assert!(accepted
        .dispatch_adapter_execution_adapter_result_code
        .is_none());
    assert!(accepted.dispatch_adapter_execution_audit_event_id.is_none());
    assert!(accepted.dispatch_adapter_execution_refs.is_empty());
    assert!(!accepted.adapter_dispatch_executed_claimed);
    assert!(accepted.manual_proof_requirements.is_empty());
}

fn scoped_accepted_row(
    rows: &[AppGameAdapterDispatchResultRow],
) -> &AppGameAdapterDispatchResultRow {
    require_some(
        rows.iter().find(|row| {
            row.dispatch_command_result_decision
                == APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED
        }),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}

fn assert_blocked_rows(rows: &[AppGameAdapterDispatchResultRow]) {
    assert!(rows
        .iter()
        .filter(|row| {
            row.dispatch_command_result_decision
                == APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED
        })
        .all(|row| {
            row.enforcement_command_name.is_none()
                && row.enforcement_event_name.is_none()
                && row.enforcement_action_mode.is_none()
                && row.dispatch_command_result_id.is_none()
                && row.dispatch_execution_audit_decision
                    == APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED
                && row.dispatch_execution_audit_id.is_none()
                && row.dispatch_execution_audit_refs.is_empty()
                && row.dispatch_adapter_execution_decision
                    == APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_BLOCKED
                && row.dispatch_adapter_execution_result_id.is_none()
                && row.dispatch_adapter_execution_status.is_none()
                && row.dispatch_adapter_execution_adapter_result_code.is_none()
                && row.dispatch_adapter_execution_audit_event_id.is_none()
                && row.dispatch_adapter_execution_refs.is_empty()
                && !row.adapter_dispatch_command_result_claimed
                && !row.service_local_execution_audit_claimed
                && !row.adapter_dispatch_executed_claimed
                && !row.manual_proof_requirements.is_empty()
        }));
}
