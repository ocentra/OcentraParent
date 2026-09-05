use crate::{
    AgentCommandName, AgentEventName, AppGamePlatformProofStatusReadModel,
    AppGamePlatformProofStatusRow, APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE,
    APP_GAME_ADAPTER_PRODUCT_NATIVE_APP, APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME,
    APP_GAME_PARENT_PLATFORM_WINDOWS, APP_GAME_PLATFORM_AUTHORITY_SCOPED_EXECUTION_ONLY,
    APP_GAME_PLATFORM_GAP_BROAD_BLOCKING, APP_GAME_PLATFORM_GAP_CHILD_DELIVERY,
    APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT, APP_GAME_PLATFORM_PROOF_SCOPED_WINDOWS_EXECUTION,
    APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID, APP_GAME_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{
    AppGameWindowsLocalPolicyEvidence, AppGameWindowsLocalPolicyEvidenceError,
    AppGameWindowsLocalPolicyEvidenceGap, AppGameWindowsLocalPolicyEvidenceState,
    APP_GAME_WINDOWS_LOCAL_POLICY_MAX_RULE_COUNT,
};

#[test]
fn app_game_platform_proof_status_command_and_event_names_are_stable() {
    assert_eq!(
        serde_json::to_value(AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet)
            .expect_value(crate::constants::error::AGENT_EVENT_SERIALIZES),
        "agent.activity.app-game.platform-proof-status.read-model.get"
    );
    assert_eq!(
        serde_json::to_value(
            AgentEventName::AgentActivityAppGamePlatformProofStatusReadModelReported
        )
        .expect_value(crate::constants::error::AGENT_EVENT_SERIALIZES),
        "agent.activity.app-game.platform-proof-status.read-model.reported"
    );
}

#[test]
fn app_game_platform_proof_status_serializes_without_enforcement_claims() {
    let read_model = AppGamePlatformProofStatusReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID.to_string(),
        generated_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec!["v0-8-supported-adapter-runtime-proof".to_string()],
        custody_label: "app-game-platform-proof-status".to_string(),
        capability_status: "app-game-platform-proof-status-partial".to_string(),
        returned: 1,
        host_visible_count: 1,
        host_not_detected_count: 0,
        local_runtime_not_applicable_count: 0,
        enforcement_ready_count: 0,
        open_gap_count: 3,
        adapter_dispatch_claimed: false,
        broad_installed_app_blocking_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        child_device_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows: vec![platform_row()],
    };

    let reparsed = serde_json::from_value::<AppGamePlatformProofStatusReadModel>(
        serde_json::to_value(read_model)
            .expect_value(crate::constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect_value("platform proof status read model reparses");

    assert_eq!(
        reparsed.read_model_id,
        APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL_ID
    );
    assert_eq!(reparsed.enforcement_ready_count, 0);
    assert!(!reparsed.platform_enforcement_claimed);
    assert_eq!(reparsed.rows[0].platform, APP_GAME_PARENT_PLATFORM_WINDOWS);
    assert_eq!(
        reparsed.rows[0].windows_local_policy_evidence,
        Some(local_policy_evidence())
    );
    assert_eq!(
        reparsed.rows[0].validate_windows_local_policy_evidence(),
        Ok(())
    );
}

#[test]
fn windows_local_policy_evidence_rejects_unknown_fields_and_invalid_nonclaims() {
    let mut encoded = serde_json::to_value(local_policy_evidence())
        .expect_value(crate::constants::error::AGENT_EVENT_SERIALIZES);
    encoded["rawRuleXml"] = serde_json::json!("private");
    let unknown = serde_json::from_value::<AppGameWindowsLocalPolicyEvidence>(encoded)
        .expect_err_value("local-policy evidence must reject unknown fields");
    assert_eq!(unknown.classify(), serde_json::error::Category::Data);

    let mut evidence = local_policy_evidence();
    evidence.platform_enforcement_claimed = true;
    assert_eq!(
        evidence.validate(),
        Err(AppGameWindowsLocalPolicyEvidenceError::UnsupportedClaim)
    );

    let mut evidence = local_policy_evidence();
    evidence.identifiers_redacted = false;
    assert_eq!(
        evidence.validate(),
        Err(AppGameWindowsLocalPolicyEvidenceError::IdentifiersNotRedacted)
    );

    let mut evidence = local_policy_evidence();
    evidence.proof_refs = vec![
        "app-game-windows-local-policy-evidence:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
            .to_string(),
    ];
    assert_eq!(
        evidence.validate(),
        Err(AppGameWindowsLocalPolicyEvidenceError::InvalidProofReference)
    );
}

#[test]
fn windows_local_policy_evidence_rejects_impossible_counts_and_platforms() {
    let mut evidence = local_policy_evidence();
    evidence.app_locker_rule_count = APP_GAME_WINDOWS_LOCAL_POLICY_MAX_RULE_COUNT + 1;
    assert_eq!(
        evidence.validate(),
        Err(AppGameWindowsLocalPolicyEvidenceError::InvalidAppLockerCounts)
    );

    let mut row = platform_row();
    row.platform = "Linux".to_string();
    assert_eq!(
        row.validate_windows_local_policy_evidence(),
        Err(AppGameWindowsLocalPolicyEvidenceError::PlatformMismatch)
    );

    let mut evidence = local_policy_evidence();
    evidence.device_guard_query_succeeded = false;
    evidence.device_guard_configured = false;
    evidence.device_guard_running = false;
    evidence.state = AppGameWindowsLocalPolicyEvidenceState::Partial;
    assert_eq!(
        evidence.validate(),
        Err(AppGameWindowsLocalPolicyEvidenceError::MissingRequiredGap)
    );
}

fn platform_row() -> AppGamePlatformProofStatusRow {
    AppGamePlatformProofStatusRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: "app-game-platform-proof-status-windows".to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        proof_state: APP_GAME_PLATFORM_PROOF_SCOPED_WINDOWS_EXECUTION.to_string(),
        authority_state: APP_GAME_PLATFORM_AUTHORITY_SCOPED_EXECUTION_ONLY.to_string(),
        host_capability_state: APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE.to_string(),
        host_capability_evidence_refs: vec!["adapter-capability-state-ref".to_string()],
        host_capability_probe_refs: vec!["windows-host-local-probe-ref".to_string()],
        linux_docker_host_preflight: None,
        windows_local_policy_evidence: Some(local_policy_evidence()),
        product_meanings: vec![
            APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
            APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
        ],
        proof_refs: vec!["app-game-session-evidence-ref".to_string()],
        open_gaps: vec![
            APP_GAME_PLATFORM_GAP_BROAD_BLOCKING.to_string(),
            APP_GAME_PLATFORM_GAP_PLATFORM_ENFORCEMENT.to_string(),
            APP_GAME_PLATFORM_GAP_CHILD_DELIVERY.to_string(),
        ],
        adapter_dispatch_claimed: false,
        broad_installed_app_blocking_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        child_device_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: crate::policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}

fn local_policy_evidence() -> AppGameWindowsLocalPolicyEvidence {
    AppGameWindowsLocalPolicyEvidence {
        schema_version: APP_GAME_SCHEMA_VERSION,
        state: AppGameWindowsLocalPolicyEvidenceState::Ready,
        observed_at: "2026-09-02T00:00:00Z".to_string(),
        probe_supported: true,
        app_id_service_query_succeeded: true,
        app_id_service_present: true,
        app_id_service_running: true,
        app_locker_policy_readable: true,
        app_locker_collection_count: 4,
        app_locker_rule_count: 12,
        device_guard_query_succeeded: true,
        device_guard_configured: true,
        device_guard_running: true,
        app_control_configured: true,
        app_control_audit_only: true,
        app_control_policy_reports_enforced: false,
        identifiers_redacted: true,
        proof_refs: vec![
            "app-game-windows-local-policy-evidence:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .to_string(),
        ],
        open_gaps: vec![
            AppGameWindowsLocalPolicyEvidenceGap::BroadBlockingNotProved,
            AppGameWindowsLocalPolicyEvidenceGap::SystemAllowlistNotProved,
            AppGameWindowsLocalPolicyEvidenceGap::RollbackNotProved,
            AppGameWindowsLocalPolicyEvidenceGap::AuditCustodyNotProved,
        ],
        adapter_dispatch_claimed: false,
        broad_installed_app_blocking_claimed: false,
        platform_enforcement_claimed: false,
        rollback_claimed: false,
        audit_custody_claimed: false,
        provider_delivery_claimed: false,
        child_device_delivery_claimed: false,
        private_diagnostics_claimed: false,
    }
}
