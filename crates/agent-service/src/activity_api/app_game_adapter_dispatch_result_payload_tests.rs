use ocentra_parent_agent_protocol::{
    constants::v08_supported_adapter_runtime_proof, AppGameAdapterDispatchResultRow,
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
    APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_GENERATED_AT,
};

use super::app_game_adapter_dispatch_result_payload::app_game_adapter_dispatch_result_read_model;

#[test]
fn app_game_adapter_dispatch_result_keeps_only_scoped_timer_command_accepted() {
    let read_model = app_game_adapter_dispatch_result_read_model(
        APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_GENERATED_AT,
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

fn assert_scoped_accepted_row(rows: &[AppGameAdapterDispatchResultRow]) {
    let accepted = rows
        .iter()
        .find(|row| {
            row.dispatch_command_result_decision
                == APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED
        })
        .unwrap();
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
    assert!(!accepted.adapter_dispatch_executed_claimed);
    assert!(accepted.manual_proof_requirements.is_empty());
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
                && !row.adapter_dispatch_command_result_claimed
                && !row.service_local_execution_audit_claimed
                && !row.adapter_dispatch_executed_claimed
                && !row.manual_proof_requirements.is_empty()
        }));
}
