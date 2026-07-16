use std::primitive::str as TestStr;

use crate::test_invariants::{require_json_decode, require_log_string_field, require_some};
use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    AppGameAdapterExecutionReadinessReadModel, APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED,
    APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED, APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED,
    APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED,
    APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED, APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE,
    APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED, APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::{
    APP_GAME_PARENT_PLATFORM_ANDROID, APP_GAME_PARENT_PLATFORM_LINUX,
};
use ocentra_parent_agent_protocol::constants::{
    self, v08_supported_adapter_runtime_proof as proof,
};

use super::app_game_adapter_execution_readiness_payload::{
    app_game_adapter_execution_readiness_payload, app_game_adapter_execution_readiness_read_model,
    GeneratedAtText,
};

const APP_GAME_TEST_TIMESTAMP: &str = "2026-06-03T22:15:00Z";

#[test]
fn app_game_adapter_execution_readiness_payload_reports_supported_proof_without_claim_upgrades() {
    let read_model = app_game_adapter_execution_readiness_read_model(GeneratedAtText(
        APP_GAME_TEST_TIMESTAMP.to_string(),
    ));
    let payload = app_game_adapter_execution_readiness_payload(&read_model);
    let decoded = require_json_decode::<AppGameAdapterExecutionReadinessReadModel>(
        string_payload(
            &payload,
            constants::field::APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL,
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(decoded.returned, 8);
    assert_eq!(decoded.execution_allowed_count, 1);
    assert_eq!(decoded.blocked_before_execution_count, 7);
    assert_eq!(decoded.adapter_execution_claimed_count, 1);
    assert!(!decoded.broad_installed_app_blocking_claimed);
    assert!(!decoded.child_device_delivery_claimed);
    assert!(!decoded.platform_enforcement_claimed);
    assert!(!decoded.provider_delivery_claimed);
    assert!(!decoded.private_diagnostics_claimed);

    assert_host_capability_summary(&decoded);
    assert_scoped_timer_row(&decoded);
    assert_blocked_rows(&decoded);
    assert_broad_artifact_row(&decoded);
    assert_android_linux_rows_stay_blocked(&decoded);
}

fn assert_host_capability_summary(decoded: &AppGameAdapterExecutionReadinessReadModel) {
    assert_eq!(
        decoded.host_capability_available_count
            + decoded.host_capability_not_detected_count
            + decoded.host_capability_not_applicable_count,
        decoded.returned
    );
    assert!(decoded.host_capability_available_count >= 4);
    assert!(decoded.host_capability_probe_ref_count >= 4);
}

fn assert_scoped_timer_row(decoded: &AppGameAdapterExecutionReadinessReadModel) {
    assert_eq!(
        decoded.rows[0].adapter_execution_state,
        APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED
    );
    assert_eq!(
        decoded.rows[0].execution_decision,
        APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED
    );
    assert!(decoded.rows[0].adapter_execution_claimed);
    assert_eq!(
        decoded.rows[0].host_capability_state,
        APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
    );
    assert_eq!(
        decoded.rows[0].host_capability_evidence_refs,
        vec![proof::REF_ADAPTER_CAPABILITY_STATE.to_string()]
    );
    assert_eq!(
        decoded.rows[0].host_capability_probe_refs,
        vec![proof::REF_WINDOWS_HOST_LOCAL_PROBE.to_string()]
    );
    assert!(decoded
        .rows
        .iter()
        .filter(|row| row.adapter_execution_claimed)
        .all(|row| {
            row.host_capability_evidence_refs
                .contains(&proof::REF_ADAPTER_CAPABILITY_STATE.to_string())
        }));
}

fn assert_blocked_rows(decoded: &AppGameAdapterExecutionReadinessReadModel) {
    assert!(decoded
        .rows
        .iter()
        .skip(1)
        .all(|row| row.execution_decision == APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED));
    assert!(decoded
        .rows
        .iter()
        .skip(1)
        .all(|row| !row.adapter_execution_claimed));
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

fn assert_broad_artifact_row(decoded: &AppGameAdapterExecutionReadinessReadModel) {
    let broad_artifact = require_some(
        decoded
            .rows
            .iter()
            .find(|row| row.source_proof_entry_id == proof::ENTRY_ID_BROAD_APP_ARTIFACT_STATUS),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    assert_eq!(
        broad_artifact.evidence_refs,
        vec![
            proof::REF_WINDOWS_ADAPTER_ARTIFACT_GATE.to_string(),
            proof::REF_WINDOWS_ADAPTER_ARTIFACT_INGESTION.to_string(),
        ]
    );
    assert_eq!(
        broad_artifact.host_capability_evidence_refs,
        vec![proof::REF_ADAPTER_CAPABILITY_STATE.to_string()]
    );
    assert_eq!(
        broad_artifact.host_capability_probe_refs,
        vec![proof::REF_WINDOWS_HOST_LOCAL_PROBE.to_string()]
    );
    assert_eq!(
        broad_artifact.host_capability_state,
        APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
    );
    assert_eq!(
        broad_artifact.linked_proof_artifacts,
        vec![
            proof::ARTIFACT_WINDOWS_ADAPTER_CAPABILITY_PROOF.to_string(),
            proof::ARTIFACT_WINDOWS_ADAPTER_ARTIFACT_GATE.to_string(),
            proof::ARTIFACT_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF.to_string(),
        ]
    );
}

fn assert_android_linux_rows_stay_blocked(decoded: &AppGameAdapterExecutionReadinessReadModel) {
    assert!(decoded
        .rows
        .iter()
        .filter(|row| {
            row.platform == APP_GAME_PARENT_PLATFORM_ANDROID
                || row.platform == APP_GAME_PARENT_PLATFORM_LINUX
        })
        .all(
            |row| row.execution_decision == APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED
                && !row.adapter_execution_claimed
        ));
}

fn string_payload<'a>(
    payload: &'a ocentra_parent_agent_protocol::logging::LogFields,
    field_name: &TestStr,
) -> &'a TestStr {
    require_log_string_field(
        payload.get(field_name),
        constants::error::AGENT_EVENT_SERIALIZES,
    )
}
