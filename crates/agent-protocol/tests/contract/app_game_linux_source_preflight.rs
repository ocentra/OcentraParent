use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED, APP_GAME_ADAPTER_PRODUCT_NATIVE_APP,
    APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME,
};
use ocentra_parent_agent_protocol::app_game_authority_classifier::APP_GAME_PARENT_PLATFORM_LINUX;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{
    APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY, APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE,
    APP_GAME_PLATFORM_PROOF_LINUX_HOST_NOT_DETECTED,
};
use ocentra_parent_agent_protocol::AppGamePlatformProofStatusRow;

fn linux_unavailable_row() -> AppGamePlatformProofStatusRow {
    AppGamePlatformProofStatusRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: "app-game-platform-proof-status:linux".to_string(),
        platform: APP_GAME_PARENT_PLATFORM_LINUX.to_string(),
        proof_state: APP_GAME_PLATFORM_PROOF_LINUX_HOST_NOT_DETECTED.to_string(),
        authority_state: APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY.to_string(),
        host_capability_state: APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED.to_string(),
        host_capability_evidence_refs: Vec::new(),
        host_capability_probe_refs: Vec::new(),
        linux_docker_host_preflight: None,
        product_meanings: vec![
            APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
            APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
        ],
        proof_refs: Vec::new(),
        open_gaps: vec![APP_GAME_PLATFORM_GAP_LINUX_FOREGROUND_CAPTURE.to_string()],
        adapter_dispatch_claimed: false,
        broad_installed_app_blocking_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        child_device_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: "2026-08-28T00:00:00Z".to_string(),
    }
}

#[test]
fn linux_unavailable_row_round_trips_without_authority_claims() {
    let row = linux_unavailable_row();
    let encoded = serde_json::to_value(&row).expect("Linux status row must serialize");
    let object = encoded
        .as_object()
        .expect("Linux status row must be an object");

    assert_eq!(
        object.get("platform"),
        Some(&serde_json::Value::String(
            APP_GAME_PARENT_PLATFORM_LINUX.to_string()
        ))
    );
    assert_eq!(
        object.get("hostCapabilityState"),
        Some(&serde_json::Value::String(
            APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED.to_string(),
        ))
    );
    assert_eq!(
        object.get("authorityState"),
        Some(&serde_json::Value::String(
            APP_GAME_PLATFORM_AUTHORITY_VISIBILITY_ONLY.to_string(),
        ))
    );
    assert_eq!(
        object.get("adapterDispatchClaimed"),
        Some(&serde_json::Value::Bool(false))
    );
    assert_eq!(object.get("linuxDockerHostPreflight"), None);

    let decoded: AppGamePlatformProofStatusRow =
        serde_json::from_value(encoded).expect("Linux status row must deserialize");
    assert_eq!(decoded, row);
}
