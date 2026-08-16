use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::AgentCommandName;
use ocentra_parent_agent_protocol::AgentEventName;
use ocentra_parent_agent_protocol::AppGameAdapterExecutionReadinessReadModel;
use ocentra_parent_agent_protocol::AppGameAdapterExecutionReadinessRow;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_PRODUCT_NATIVE_APP;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME;
use ocentra_parent_agent_protocol::APP_GAME_SCHEMA_VERSION;

#[test]
fn app_game_adapter_execution_readiness_command_and_event_names_are_stable() {
    assert_eq!(
        serde_json::to_value(
            AgentCommandName::AgentActivityAppGameAdapterExecutionReadinessReadModelGet
        )
        .expect_value(ocentra_parent_agent_protocol::constants::error::AGENT_EVENT_SERIALIZES),
        "agent.activity.app-game.adapter-execution-readiness.read-model.get"
    );
    assert_eq!(
        serde_json::to_value(
            AgentEventName::AgentActivityAppGameAdapterExecutionReadinessReadModelReported
        )
        .expect_value(ocentra_parent_agent_protocol::constants::error::AGENT_EVENT_SERIALIZES),
        "agent.activity.app-game.adapter-execution-readiness.read-model.reported"
    );
}

#[test]
fn app_game_adapter_execution_readiness_serializes_no_claim_upgrades() {
    let read_model = AppGameAdapterExecutionReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID.to_string(),
        generated_at: ocentra_parent_agent_protocol::policy_constants::TEST_EVALUATED_AT
            .to_string(),
        source_read_model_ids: vec!["v0-8-supported-adapter-runtime-proof".to_string()],
        custody_label: "supported-adapter-runtime-proof".to_string(),
        capability_status: "app-game-adapter-execution-partial".to_string(),
        returned: 1,
        execution_allowed_count: 1,
        blocked_before_execution_count: 0,
        adapter_execution_claimed_count: 1,
        host_capability_available_count: 1,
        host_capability_not_detected_count: 0,
        host_capability_not_applicable_count: 0,
        host_capability_probe_ref_count: 1,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows: vec![AppGameAdapterExecutionReadinessRow {
            schema_version: APP_GAME_SCHEMA_VERSION,
            row_id: "app-game-adapter-execution-windows-app-game-owned-process-time-limit"
                .to_string(),
            source_proof_entry_id: "windows-app-game-owned-process-time-limit".to_string(),
            platform: "windows".to_string(),
            product_meanings: vec![
                APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
                APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
            ],
            adapter_capability: "app-game-owned-process-time-limit".to_string(),
            adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED.to_string(),
            execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED.to_string(),
            runtime_boundary: "windows-app-game-owned-process-time-limit".to_string(),
            target_identity_state: "process-session-evidence-backed".to_string(),
            rollback_reference_state: "timer-recovery-backed".to_string(),
            audit_reference_state: "audit-reference-backed".to_string(),
            evidence_refs: vec!["app-game-session-evidence-ref".to_string()],
            host_capability_state: APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE.to_string(),
            host_capability_evidence_refs: vec!["adapter-capability-state-ref".to_string()],
            host_capability_probe_refs: vec!["windows-host-local-probe-ref".to_string()],
            linked_proof_artifacts: vec![
                "test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json".to_string(),
            ],
            manual_proof_requirements: Vec::new(),
            claim_boundary: "Scoped Windows owned-process app/game timer execution only."
                .to_string(),
            fallback_behavior: "Targets without process/session identity stay manual-required."
                .to_string(),
            adapter_execution_claimed: true,
            broad_installed_app_blocking_claimed: false,
            child_device_delivery_claimed: false,
            platform_enforcement_claimed: false,
            provider_delivery_claimed: false,
            private_diagnostics_claimed: false,
            last_checked_at: ocentra_parent_agent_protocol::policy_constants::TEST_EVALUATED_AT
                .to_string(),
        }],
    };

    let reparsed = serde_json::from_value::<AppGameAdapterExecutionReadinessReadModel>(
        serde_json::to_value(read_model)
            .expect_value(ocentra_parent_agent_protocol::constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect_value(ocentra_parent_agent_protocol::constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        reparsed.read_model_id,
        APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID
    );
    assert_eq!(reparsed.execution_allowed_count, 1);
    assert_eq!(reparsed.adapter_execution_claimed_count, 1);
    assert_eq!(reparsed.host_capability_available_count, 1);
    assert_eq!(reparsed.host_capability_probe_ref_count, 1);
    assert!(!reparsed.broad_installed_app_blocking_claimed);
    assert!(!reparsed.child_device_delivery_claimed);
    assert!(!reparsed.platform_enforcement_claimed);
    assert!(!reparsed.provider_delivery_claimed);
    assert!(!reparsed.private_diagnostics_claimed);
}
