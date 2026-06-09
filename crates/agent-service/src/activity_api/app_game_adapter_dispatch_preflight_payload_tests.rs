use ocentra_parent_agent_protocol::{
    constants, AppGameAdapterDispatchPreflightReadModel, LogFieldValue,
    APP_GAME_ADAPTER_DISPATCH_DECISION_BLOCKED, APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY, APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE,
    APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE, APP_GAME_TEST_TIMESTAMP,
};

use super::app_game_adapter_dispatch_preflight_payload::{
    app_game_adapter_dispatch_preflight_payload, app_game_adapter_dispatch_preflight_read_model,
};

#[test]
fn app_game_adapter_dispatch_preflight_reports_one_scoped_dispatch_eligible_row() {
    let read_model = app_game_adapter_dispatch_preflight_read_model(APP_GAME_TEST_TIMESTAMP);
    let payload = app_game_adapter_dispatch_preflight_payload(&read_model);
    let decoded: AppGameAdapterDispatchPreflightReadModel = serde_json::from_str(string_payload(
        &payload,
        constants::field::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL,
    ))
    .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(decoded.returned, 8);
    assert_eq!(decoded.dispatch_eligible_count, 1);
    assert_eq!(decoded.blocked_before_dispatch_count, 7);
    assert_eq!(decoded.adapter_dispatch_eligible_count, 1);
    assert_eq!(decoded.adapter_dispatch_executed_claimed_count, 0);
    assert_eq!(
        decoded.host_capability_available_count
            + decoded.host_capability_not_detected_count
            + decoded.host_capability_not_applicable_count,
        decoded.returned
    );
    assert!(decoded.host_capability_available_count >= 4);
    assert!(decoded.host_capability_probe_ref_count >= 4);
    assert!(!decoded.broad_installed_app_blocking_claimed);
    assert!(!decoded.child_device_delivery_claimed);
    assert!(!decoded.platform_enforcement_claimed);
    assert!(!decoded.provider_delivery_claimed);
    assert!(!decoded.private_diagnostics_claimed);
    assert_eq!(
        decoded.rows[0].dispatch_preflight_state,
        APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE
    );
    assert_eq!(
        decoded.rows[0].dispatch_decision,
        APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE
    );
    assert_eq!(
        decoded.rows[0].dispatch_outcome_state,
        APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY
    );
    assert!(decoded.rows[0].adapter_dispatch_eligible);
    assert_eq!(
        decoded.rows[0].host_capability_state,
        APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
    );
    assert_eq!(
        decoded.rows[0].host_capability_evidence_refs,
        vec![
            constants::v08_supported_adapter_runtime_proof::REF_ADAPTER_CAPABILITY_STATE
                .to_string()
        ]
    );
    assert_eq!(
        decoded.rows[0].host_capability_probe_refs,
        vec![
            constants::v08_supported_adapter_runtime_proof::REF_WINDOWS_HOST_LOCAL_PROBE
                .to_string()
        ]
    );
    assert!(!decoded.rows[0].adapter_dispatch_executed_claimed);
    assert!(decoded
        .rows
        .iter()
        .skip(1)
        .all(|row| row.dispatch_decision == APP_GAME_ADAPTER_DISPATCH_DECISION_BLOCKED));
    assert!(decoded
        .rows
        .iter()
        .all(|row| !row.adapter_dispatch_executed_claimed));
}

fn string_payload<'a>(payload: &'a ocentra_parent_agent_protocol::LogFields, key: &str) -> &'a str {
    match payload.get(key) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
