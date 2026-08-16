use std::fs::{read_to_string, remove_file};
use std::string::String as TestString;

use serde_json::Value;

use super::*;
use crate::test_invariants::{require_ok, require_some};
use crate::test_text::TestText;

#[tokio::test]
async fn persistent_runtime_restores_selected_route_and_household_rename_into_read_model() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);

    pair_and_select_route(&runtime).await;
    let selected_canonical_device_id = selected_child_canonical_device_id(&runtime).await;
    let _ = handle_command_text_for_test(
        serialize_command(add_device_request_command(
            household_decision_payload_for_device(&selected_canonical_device_id),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    let event = handle_command_text_for_test(
        serialize_command(browser_discovery_scan_command(LogFields::new())),
        LanPairingRuntime::persistent_json(&registry_path),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    assert_selected_route_and_rename_restored(
        &typed_read_model_payload(&event.payload),
        &selected_canonical_device_id,
    );
    let _ = remove_file(&registry_path);
}

#[tokio::test]
async fn persistent_runtime_scan_persists_known_household_device_store() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    let event = handle_command_text_for_test(
        serialize_command(browser_discovery_scan_command(LogFields::new())),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    let registry_json = read_to_string(&registry_path).unwrap_or_default();
    let registry_value: Value = require_ok(
        serde_json::from_str(&registry_json),
        "persistent registry json parses",
    );
    assert!(
        registry_value[constants::lan_pairing::REGISTRY_KEY_KNOWN_HOUSEHOLD_DEVICES]
            .as_array()
            .map(|devices| !devices.is_empty())
            .unwrap_or(false)
    );

    let _ = remove_file(&registry_path);
}

#[tokio::test]
async fn persistent_runtime_restores_known_household_device_into_read_model_as_stale() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    {
        let mut registry = require_ok(runtime.registry.lock(), "registry lock available for test");
        let changed = registry.merge_known_household_devices(vec![stored_known_router()]);
        assert!(changed);
        assert!(runtime.persist_registry(&registry));
    }

    let event = handle_command_text_for_test(
        serialize_command(browser_discovery_scan_command(LogFields::new())),
        LanPairingRuntime::persistent_json(&registry_path),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    let read_model = typed_read_model_payload(&event.payload);
    let device = require_some(
        read_model
            .canonical_household_devices
            .iter()
            .find(|device| {
                device.canonical_device_id
                    == canonical_device_id_for_mac(constants::lan_pairing::TEST_ROUTER_MAC)
            }),
        "stored known router restored into read model",
    );

    assert_eq!(
        device.discovery_state,
        LanPairingProductionDiscoveryState::Stale
    );
    assert_eq!(
        device.network_identity.reachability,
        LanPairingDeviceReachability::Stale
    );
    assert_eq!(
        device.network_identity.stale_at.as_deref(),
        Some(read_model.generated_at.as_str())
    );

    let _ = remove_file(&registry_path);
}

#[tokio::test]
async fn persistent_runtime_restores_offline_known_household_device_as_offline() {
    let registry_path = temp_registry_path();
    let _ = remove_file(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    {
        let mut registry = require_ok(runtime.registry.lock(), "registry lock available for test");
        let changed = registry.merge_known_household_devices(vec![stored_offline_known_router()]);
        assert!(changed);
        assert!(runtime.persist_registry(&registry));
    }

    let event = handle_command_text_for_test(
        serialize_command(browser_discovery_scan_command(LogFields::new())),
        LanPairingRuntime::persistent_json(&registry_path),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );
    let read_model = typed_read_model_payload(&event.payload);
    let device = require_some(
        read_model
            .canonical_household_devices
            .iter()
            .find(|device| {
                device.canonical_device_id
                    == canonical_device_id_for_mac(constants::lan_pairing::TEST_ROUTER_MAC)
            }),
        "stored offline known router restored into read model",
    );

    assert_eq!(
        device.discovery_state,
        LanPairingProductionDiscoveryState::Offline
    );
    assert_eq!(
        device.network_identity.reachability,
        LanPairingDeviceReachability::Offline
    );
    assert_eq!(
        device.network_identity.offline_at.as_deref(),
        Some(STORED_OFFLINE_AT)
    );
    assert_eq!(device.network_identity.stale_at, None);

    let _ = remove_file(&registry_path);
}

async fn pair_and_select_route(runtime: &LanPairingRuntime) {
    let _ = handle_command_text_for_test(
        serialize_command(pairing_command(proof_payload())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let _ = handle_command_text_for_test(
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
}

async fn selected_child_canonical_device_id(runtime: &LanPairingRuntime) -> TestString {
    let initial_scan = handle_command_text_for_test(
        serialize_command(browser_discovery_scan_command(LogFields::new())),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    assert_eq!(
        initial_scan.event,
        AgentEventName::AgentLanPairingBrowserDiscoveryReported
    );

    paired_child_canonical_device_id(&typed_read_model_payload(&initial_scan.payload))
}

fn paired_child_canonical_device_id(read_model: &LanBrowserAddDeviceReadModel) -> TestString {
    require_some(
        read_model
            .canonical_household_devices
            .iter()
            .find(|device| {
                device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
                    && device.trust_state == LanPairingTrustState::Paired
                    && device.route_id.as_deref()
                        == Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK)
            })
            .map(|device| device.canonical_device_id.clone()),
        "selected paired child device present before restart",
    )
}

fn assert_selected_route_and_rename_restored(
    read_model: &LanBrowserAddDeviceReadModel,
    selected_canonical_device_id: impl Into<TestString>,
) {
    let selected_canonical_device_id = selected_canonical_device_id.into();
    assert_eq!(
        read_model
            .selected_device_readiness
            .selected_child_device_id
            .as_deref(),
        Some(constants::lan_pairing::CHILD_DEVICE_ID)
    );
    assert_eq!(
        read_model.selected_device_readiness.route_id.as_deref(),
        Some(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK)
    );
    assert!(read_model.selected_device_readiness.ready_for_control);
    assert_eq!(read_model.household_device_decisions.len(), 1);
    let device = restored_canonical_device(read_model, selected_canonical_device_id);
    assert_eq!(
        device.display_name,
        constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL
    );
    assert_eq!(device.trust_state, LanPairingTrustState::Paired);
}

fn restored_canonical_device(
    read_model: &LanBrowserAddDeviceReadModel,
    selected_canonical_device_id: impl Into<TestString>,
) -> &LanCanonicalHouseholdDevice {
    let selected_canonical_device_id = selected_canonical_device_id.into();
    require_some(
        read_model
            .canonical_household_devices
            .iter()
            .find(|device| device.canonical_device_id == selected_canonical_device_id),
        "selected child canonical device restored into read model",
    )
}
