use ocentra_parent_agent_protocol::{
    constants, AppGameAdapterExecutionReadinessReadModel, LogFieldValue,
    APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED, APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED,
    APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED, APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
    APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED, APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED, APP_GAME_TEST_TIMESTAMP,
};

use super::app_game_adapter_execution_readiness_payload::{
    app_game_adapter_execution_readiness_payload, app_game_adapter_execution_readiness_read_model,
};

#[test]
fn app_game_adapter_execution_readiness_payload_reports_supported_proof_without_claim_upgrades() {
    let read_model = app_game_adapter_execution_readiness_read_model(APP_GAME_TEST_TIMESTAMP);
    let payload = app_game_adapter_execution_readiness_payload(&read_model);
    let decoded: AppGameAdapterExecutionReadinessReadModel = serde_json::from_str(string_payload(
        &payload,
        constants::field::APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL,
    ))
    .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(decoded.returned, 8);
    assert_eq!(decoded.execution_allowed_count, 1);
    assert_eq!(decoded.blocked_before_execution_count, 7);
    assert_eq!(decoded.adapter_execution_claimed_count, 1);
    assert!(!decoded.broad_installed_app_blocking_claimed);
    assert!(!decoded.child_device_delivery_claimed);
    assert!(!decoded.platform_enforcement_claimed);
    assert!(!decoded.provider_delivery_claimed);
    assert!(!decoded.private_diagnostics_claimed);
    assert_eq!(
        decoded.rows[0].adapter_execution_state,
        APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED
    );
    assert_eq!(
        decoded.rows[0].execution_decision,
        APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED
    );
    assert!(decoded.rows[0].adapter_execution_claimed);
    assert!(decoded
        .rows
        .iter()
        .skip(1)
        .all(|row| row.execution_decision == APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED));
    assert!(
        decoded
            .rows
            .iter()
            .any(|row| row.adapter_execution_state
                == APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED)
    );
    assert!(decoded
        .rows
        .iter()
        .any(|row| row.adapter_execution_state == APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE));
    assert!(decoded
        .rows
        .iter()
        .any(|row| row.adapter_execution_state == APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED));
    assert!(decoded
        .rows
        .iter()
        .any(|row| row.adapter_execution_state == APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED));
}

fn string_payload<'a>(payload: &'a ocentra_parent_agent_protocol::LogFields, key: &str) -> &'a str {
    match payload.get(key) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
