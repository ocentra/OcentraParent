use ocentra_parent_agent_protocol::{
    constants::v08_supported_adapter_runtime_proof,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED,
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
    assert_eq!(read_model.adapter_dispatch_command_result_claimed_count, 1);
    assert_eq!(read_model.adapter_dispatch_executed_claimed_count, 0);
    assert!(!read_model.broad_installed_app_blocking_claimed);
    assert!(!read_model.child_device_delivery_claimed);
    assert!(!read_model.platform_enforcement_claimed);
    assert!(!read_model.provider_delivery_claimed);
    assert!(!read_model.private_diagnostics_claimed);

    let accepted = read_model
        .rows
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
    assert!(!accepted.adapter_dispatch_executed_claimed);
    assert!(accepted.manual_proof_requirements.is_empty());

    assert!(read_model
        .rows
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
                && !row.adapter_dispatch_command_result_claimed
                && !row.adapter_dispatch_executed_claimed
                && !row.manual_proof_requirements.is_empty()
        }));
}
