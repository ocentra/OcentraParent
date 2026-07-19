use std::{net::TcpListener, sync::atomic::Ordering};

use ocentra_parent_agent_protocol::{
    constants, DeviceRuntimeAiProviderState, DeviceRuntimeLocalAiClaim, DeviceRuntimeRouteState,
};
use ocentra_parent_desktop::parent_route_subscription_delivery::{
    deliver_parent_route_subscription_event, ParentRouteSubscriptionDelivery,
    ParentRouteSubscriptionDeliveryState,
};
use ocentra_parent_desktop::{
    agent_service_connects, parent_platform_proof_state_for_address,
    parent_route_subscription_event_name, ParentDesktopAgentAddress, ParentRouteSubscriptionId,
    ParentRouteSubscriptionRegistry,
};
use ocentra_parent_runtime_core::parent_ui_bridge::load_parent_route_snapshot;
use ocentra_schema::parent_ui_bridge::{
    ParentRouteEventId, ParentRouteEventSnapshot, ParentRouteId, ParentRouteSnapshot,
    ParentSubscriptionEvent,
};
use serde_json::Value;

fn proof_state_json(agent_address: ParentDesktopAgentAddress) -> Value {
    serde_json::to_value(parent_platform_proof_state_for_address(agent_address))
        .expect("parent desktop proof state serializes")
}

fn subscription_event_with_ids(
    snapshot: &ParentRouteSnapshot,
    event_ids: &[&str],
) -> ParentSubscriptionEvent {
    ParentSubscriptionEvent {
        schema_version: 1,
        route: snapshot.route.clone(),
        snapshot: snapshot.clone(),
        events: Some(
            event_ids
                .iter()
                .map(|event_id| ParentRouteEventSnapshot {
                    event: Some("lan-replay-row".to_string()),
                    event_id: ParentRouteEventId::parse((*event_id).to_string()),
                    correlation_id: None,
                    sent_at: None,
                    source_peer_id: None,
                    source_role: None,
                    target_peer_id: None,
                    target_role: None,
                    severity: Some("info".to_string()),
                    payload: None,
                    snapshot: None,
                    command_result_projection: None,
                })
                .collect(),
        ),
    }
}

#[test]
fn parent_platform_proof_state_uses_rust_service_connection_for_package_runtime() {
    let state = proof_state_json(ParentDesktopAgentAddress(
        constants::test_network::LOOPBACK_ANY_PORT.to_string(),
    ));

    assert_parent_platform_proof_state_shell_identity(&state);
    assert_parent_platform_proof_state_runtime_truth(&state);
    assert_parent_platform_proof_state_package_operations_truth(&state);
    assert_parent_platform_proof_state_package_distribution_truth(&state);
}

#[test]
fn parent_platform_proof_state_reports_ready_when_rust_service_socket_accepts() {
    let listener =
        TcpListener::bind((constants::test_network::LOOPBACK_IP, 0)).expect("bind listener");
    let address =
        ParentDesktopAgentAddress(listener.local_addr().expect("listener address").to_string());

    assert!(agent_service_connects(&address));

    let state = proof_state_json(address);
    assert_eq!(
        state["serviceState"],
        constants::value::PARENT_DESKTOP_SERVICE_CONNECTED
    );
    assert_eq!(
        state["runtimeReadinessState"],
        constants::value::PARENT_DESKTOP_RUNTIME_READY
    );
    assert_eq!(
        state["activityAdapterState"],
        constants::value::PARENT_DESKTOP_SERVICE_CONNECTED
    );
}

#[test]
fn parent_route_subscription_registry_unregisters_active_subscriptions() {
    let registry = ParentRouteSubscriptionRegistry::default();
    let (subscription_id, active) = registry.register();

    assert!(active.load(Ordering::SeqCst));
    assert!(registry.unregister(&subscription_id));
    assert!(!active.load(Ordering::SeqCst));
    assert!(!registry.unregister(&subscription_id));
}

#[test]
fn parent_route_subscription_event_name_uses_stable_prefix() {
    assert_eq!(
        parent_route_subscription_event_name(&ParentRouteSubscriptionId("42".to_string())).0,
        "parent-route-subscription-42"
    );
}

#[test]
fn parent_route_subscription_delivery_emits_new_event_ids_with_stable_snapshot_once() {
    let snapshot = load_parent_route_snapshot(ParentRouteId::Devices, None);
    let mut state = ParentRouteSubscriptionDeliveryState::new(snapshot.clone());
    let first_event = subscription_event_with_ids(&snapshot, &["lan-replay-1"]);
    let mut emitted_batches = Vec::new();

    let first_delivery =
        deliver_parent_route_subscription_event(&mut state, &first_event, |event| {
            emitted_batches.push(event.clone());
            Ok::<(), ()>(())
        });
    assert_eq!(first_delivery, Ok(ParentRouteSubscriptionDelivery::Emitted));

    let repeated_delivery =
        deliver_parent_route_subscription_event(&mut state, &first_event, |event| {
            emitted_batches.push(event.clone());
            Ok::<(), ()>(())
        });
    assert_eq!(
        repeated_delivery,
        Ok(ParentRouteSubscriptionDelivery::Suppressed)
    );

    let next_event = subscription_event_with_ids(&snapshot, &["lan-replay-1", "lan-replay-2"]);
    let next_delivery = deliver_parent_route_subscription_event(&mut state, &next_event, |event| {
        emitted_batches.push(event.clone());
        Ok::<(), ()>(())
    });
    assert_eq!(next_delivery, Ok(ParentRouteSubscriptionDelivery::Emitted));
    assert_eq!(emitted_batches.len(), 2);
    assert_eq!(
        emitted_batches[1]
            .events
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(|event| event.event_id.as_ref().map(|event_id| event_id.as_str()))
            .collect::<Vec<_>>(),
        vec!["lan-replay-1", "lan-replay-2"]
    );
}

fn assert_parent_platform_proof_state_shell_identity(state: &Value) {
    assert_eq!(
        state["serviceState"],
        constants::value::PARENT_DESKTOP_SERVICE_UNAVAILABLE
    );
    assert_eq!(state["serviceHealthEndpoint"], constants::endpoint::HEALTH);
    assert_eq!(
        state["runtimeReadinessState"],
        constants::value::PARENT_DESKTOP_RUNTIME_DEGRADED
    );
    assert_eq!(
        state["backendKind"],
        constants::value::PARENT_DESKTOP_BACKEND_RUST_SERVICE
    );
    assert_eq!(
        state["activityAdapterState"],
        constants::value::PARENT_DESKTOP_SERVICE_UNAVAILABLE
    );
    assert_eq!(
        state["packageFrontendState"],
        constants::value::PARENT_DESKTOP_FRONTEND_BUILT_PORTAL_DIST
    );
    assert_eq!(
        state["hmrBackendState"],
        constants::value::PARENT_DESKTOP_HMR_BACKEND_NOT_USED
    );
    assert_eq!(
        state["processOwnershipState"],
        constants::value::PARENT_DESKTOP_PROCESS_OWNER_SHELL_ONLY
    );
    assert_eq!(
        state["controllerRouteState"],
        constants::value::PARENT_DESKTOP_CONTROLLER_ROUTE_ACTIVE_CONTROLLER
    );
    assert_eq!(
        state["observerReadOnlyState"],
        constants::value::PARENT_DESKTOP_OBSERVER_READ_ONLY
    );
}

fn assert_parent_platform_proof_state_runtime_truth(state: &Value) {
    assert_eq!(
        state["routeState"],
        serde_json::json!(DeviceRuntimeRouteState::LocalNetwork)
    );
    assert_eq!(
        state["routeSourceState"],
        serde_json::json!(DeviceRuntimeRouteState::LocalNetwork)
    );
    assert_eq!(
        state["lanAiProviderState"],
        serde_json::json!(DeviceRuntimeAiProviderState::Degraded)
    );
    assert_eq!(
        state["degradedSourceState"],
        serde_json::json!(DeviceRuntimeAiProviderState::Degraded)
    );
    assert_eq!(
        state["parentAssistantProviderState"],
        serde_json::json!(DeviceRuntimeAiProviderState::Degraded)
    );
    assert_eq!(
        state["deviceRoleState"]["localAiRuntimeClaim"],
        serde_json::json!(DeviceRuntimeLocalAiClaim::SharedPhysicalDeviceSingleton)
    );
    assert_eq!(state["serviceConnectTimeoutMs"], serde_json::json!(250));
}

fn assert_parent_platform_proof_state_package_operations_truth(state: &Value) {
    assert_eq!(
        state["sourceCustodyState"],
        constants::value::PARENT_DESKTOP_SOURCE_CUSTODY_LIVE_LOCAL_NETWORK
    );
    assert_eq!(
        state["relayRouteState"],
        constants::value::PARENT_DESKTOP_RELAY_ROUTE_UNAVAILABLE
    );
    assert_eq!(
        state["parentCacheState"],
        constants::value::PARENT_DESKTOP_PARENT_CACHE_UNAVAILABLE
    );
    assert_eq!(
        state["parentStorageState"],
        constants::value::PARENT_DESKTOP_PARENT_STORAGE_UNAVAILABLE
    );
    assert_eq!(
        state["serviceLaunchOwnerState"],
        constants::value::PARENT_DESKTOP_SERVICE_LAUNCH_OWNER_PACKAGE_SERVICE
    );
    assert_eq!(
        state["serviceLaunchStrategyState"],
        constants::value::PARENT_DESKTOP_SERVICE_LAUNCH_STRATEGY_CONNECT_OR_DEGRADE
    );
}

fn assert_parent_platform_proof_state_package_distribution_truth(state: &Value) {
    assert_eq!(
        state["packageServiceManagerState"],
        constants::value::PARENT_DESKTOP_PACKAGE_SERVICE_AUTO_START
    );
    assert_eq!(
        state["packageHealthProbeState"],
        constants::value::PARENT_DESKTOP_PACKAGE_HEALTH_PROBE_REQUIRED
    );
    assert_eq!(
        state["portOwnershipState"],
        constants::value::PARENT_DESKTOP_PORT_OWNERSHIP_FIXED_LOOPBACK
    );
    assert_eq!(
        state["portConflictPolicyState"],
        constants::value::PARENT_DESKTOP_PORT_CONFLICT_POLICY_NO_FOREIGN_RECLAIM
    );
    assert_eq!(
        state["blankWindowRegressionState"],
        constants::value::PARENT_DESKTOP_BLANK_WINDOW_GUARD_FRONTEND_DIST
    );
    assert_eq!(
        state["packagePreviewState"],
        constants::value::PARENT_DESKTOP_PACKAGE_PREVIEW_UNSIGNED
    );
    assert_eq!(
        state["updateChannelState"],
        constants::value::PARENT_DESKTOP_UPDATE_CHANNEL_SCAFFOLD
    );
    assert_eq!(
        state["rollbackState"],
        constants::value::PARENT_DESKTOP_ROLLBACK_UNAVAILABLE
    );
    assert_eq!(
        state["signingState"],
        constants::value::PARENT_DESKTOP_SIGNING_MANUAL_REQUIRED
    );
    assert_eq!(
        state["notarizationState"],
        constants::value::PARENT_DESKTOP_NOTARIZATION_MANUAL_REQUIRED
    );
    assert_eq!(
        state["storeDistributionState"],
        constants::value::PARENT_DESKTOP_STORE_DISTRIBUTION_MANUAL_REQUIRED
    );
    assert_eq!(
        state["supportDiagnosticsState"],
        constants::value::PARENT_DESKTOP_SUPPORT_DIAGNOSTICS_REDACTED
    );
    assert_eq!(
        state["supportRedactionState"],
        constants::value::PARENT_DESKTOP_SUPPORT_OUTPUT_ALLOWED_FIELDS
    );
    assert_eq!(
        state["platformMatrixState"],
        constants::value::PARENT_DESKTOP_PLATFORM_MATRIX_SPLIT_PROOF_ROWS
    );
    assert_eq!(
        state["releaseBranchState"],
        constants::value::PARENT_DESKTOP_RELEASE_BRANCH_PRODUCTION_PROMOTION_REQUIRED
    );
    assert_eq!(
        state["artifactProofState"],
        constants::value::PARENT_DESKTOP_ARTIFACT_PROOF_CI_PREVIEW
    );
}
