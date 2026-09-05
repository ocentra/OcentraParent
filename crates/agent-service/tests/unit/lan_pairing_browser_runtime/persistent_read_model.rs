use std::fs::remove_file;

use super::*;
use crate::test_require_ok::require_ok;
use crate::test_text::TestText;

#[tokio::test]
async fn persistent_runtime_rejects_uncomposed_selection_and_rename_without_persisting() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);

    let pairing_event = handle_command_text_for_test(
        serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let route_selection_event = handle_command_text_for_test(
        serialize_command(route_select_command(intent_payload(
            constants::lan_pairing::SELECT_INTENT_ID,
            constants::lan_pairing::CHILD_DEVICE_ID,
            constants::lan_pairing::PROOF_DIGEST,
            constants::lan_pairing::EXPIRES_AT,
        ))),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let household_decision_event = handle_command_text_for_test(
        serialize_command(add_device_request_command(household_decision_payload())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejected_without_read_model(&pairing_event, constants::value::LAN_REASON_ANONYMOUS);
    assert_rejected_without_read_model(
        &route_selection_event,
        constants::value::LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE,
    );
    assert_rejected_without_read_model(
        &household_decision_event,
        constants::value::LAN_REASON_ANONYMOUS,
    );
    assert_persistent_runtime_has_no_owned_state(&runtime, &registry_path);

    let restarted_runtime = LanPairingRuntime::persistent_json(&registry_path);
    assert_persistent_runtime_has_no_owned_state(&restarted_runtime, &registry_path);
    let _ = remove_file(&registry_path);
}

#[tokio::test]
async fn persistent_runtime_scan_reports_without_persisting_to_uninitialized_registry() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    let event = handle_command_text_for_test(
        serialize_command(browser_discovery_scan_command(LogFields::new())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    assert_persistent_runtime_has_no_owned_state(&runtime, &registry_path);

    let _ = remove_file(&registry_path);
}

#[tokio::test]
async fn persistent_runtime_rejects_unowned_known_household_device_without_persisting() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    let result = {
        let mut registry = require_ok(runtime.registry.lock(), "registry lock available for test");
        runtime.merge_known_household_devices(&mut registry, vec![stored_known_router()])
    };

    assert_eq!(
        result,
        Err(ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::SignedChildAgentContextUnavailable)
    );
    assert_persistent_runtime_has_no_owned_state(&runtime, &registry_path);

    let _ = remove_file(&registry_path);
}

#[tokio::test]
async fn persistent_runtime_rejects_unowned_offline_device_without_persisting() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    let result = {
        let mut registry = require_ok(runtime.registry.lock(), "registry lock available for test");
        runtime.merge_known_household_devices(&mut registry, vec![stored_offline_known_router()])
    };

    assert_eq!(
        result,
        Err(ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::SignedChildAgentContextUnavailable)
    );
    assert_persistent_runtime_has_no_owned_state(&runtime, &registry_path);

    let _ = remove_file(&registry_path);
}

fn assert_persistent_runtime_has_no_owned_state(
    runtime: &LanPairingRuntime,
    registry_path: &std::path::Path,
) {
    assert_eq!(runtime.trusted_device_count(), 0);
    assert!(crate::lan_pairing_browser_add_device_state::registry_projection::household_device_decisions(runtime).is_empty());
    assert!(
        crate::lan_pairing_browser_add_device_state::registry_projection::known_household_devices(
            runtime
        )
        .is_empty()
    );
    assert!(!registry_path.exists());
}
