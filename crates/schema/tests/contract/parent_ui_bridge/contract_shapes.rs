use super::fixtures::{
    browser_add_device_discovery_snapshot, route_live_activity_snapshot, route_snapshot,
};
use crate::support::ValueOrUnreachable as _;
use ocentra_schema::parent_ui_bridge::{
    ParentBridgeConnectionState, ParentChildDeviceId, ParentCommandResultDetailSnapshot,
    ParentCommandResultProjectionSnapshot, ParentPortalClipboardText, ParentPortalDetailValue,
    ParentRouteContext, ParentRouteEventCorrelationId, ParentRouteEventId,
    ParentRouteEventSnapshot, ParentRouteId, ParentRouteLiveActivitySnapshot, ParentRoutePeerId,
    ParentRoutePeerRole, ParentRouteSnapshot, ParentSubscriptionEvent,
    ParentTrackingStatusProofArtifact, ParentUiAction, ParentUiActionKind, ParentUiActionResult,
    PARENT_UI_BRIDGE_SCHEMA_VERSION,
};
use serde_json::json;

#[test]
fn portal_text_values_are_rust_owned_non_empty_bridge_contracts() {
    let detail = ParentPortalDetailValue::parse("not reported")
        .value_or_unreachable(crate::assert_context!("detail must be non-empty"));
    let clipboard = ParentPortalClipboardText::parse("diagnostics export")
        .value_or_unreachable(crate::assert_context!("clipboard text must be non-empty"));
    let proof_artifact = ParentTrackingStatusProofArtifact::parse("tracking-proof-artifact")
        .value_or_unreachable(crate::assert_context!(
            "tracking proof artifact must be non-empty"
        ));

    assert_eq!(detail.as_str(), "not reported");
    assert_eq!(clipboard.as_str(), "diagnostics export");
    assert_eq!(proof_artifact.as_str(), "tracking-proof-artifact");
    assert!(ParentPortalDetailValue::parse("").is_none());
    assert!(ParentPortalClipboardText::parse("   ").is_none());
    assert!(ParentTrackingStatusProofArtifact::parse("").is_none());
    assert_eq!(
        serde_json::to_value(&detail)
            .value_or_unreachable(crate::assert_context!("detail serializes")),
        json!("not reported")
    );

    let decoded: ParentPortalDetailValue = serde_json::from_value(json!("not reported"))
        .value_or_unreachable(crate::assert_context!("detail deserializes"));
    assert_eq!(decoded, detail);
}

#[test]
fn route_snapshot_preserves_rust_owned_encoded_shape() {
    let snapshot = route_snapshot(ParentRouteId::Devices);
    let encoded = serde_json::to_value(&snapshot)
        .value_or_unreachable(crate::assert_context!("snapshot must serialize"));

    assert_eq!(
        encoded["schemaVersion"],
        json!(PARENT_UI_BRIDGE_SCHEMA_VERSION)
    );
    assert_eq!(encoded["route"], json!("devices"));
    assert_eq!(encoded["connectionState"], json!("connected"));
    assert_eq!(encoded["dataSource"], json!("rust-read-model"));
    assert_eq!(encoded["diagnosticPanelsEnabled"], json!(false));
    assert_eq!(encoded["parentPortalRows"][0]["signalScore"], json!(92));
    assert_eq!(
        encoded["parentPortalShellStatus"]["parentAccessState"],
        json!("active-controller")
    );
    assert!(encoded.get("schema_version").is_none());
    assert!(encoded.get("connection_state").is_none());

    let decoded: ParentRouteSnapshot = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("encoded snapshot must round-trip"));
    assert_eq!(decoded, snapshot);
}

#[test]
fn route_live_activity_snapshot_preserves_rust_owned_app_game_panel_shapes() {
    let snapshot = route_live_activity_snapshot();
    let encoded = serde_json::to_value(&snapshot)
        .value_or_unreachable(crate::assert_context!("live activity must serialize"));

    assert_eq!(
        encoded["screenSummaryPanel"]["summaryDetails"][0]["label"],
        json!("Status")
    );
    assert_eq!(
        encoded["screenSummaryPanel"]["rows"][0]["title"],
        json!("screen-ready-row")
    );
    assert_eq!(
        encoded["appGameNotificationParentSurfacePanel"]["summary"],
        json!("1 parent-surface intent rows")
    );
    assert_eq!(
        encoded["appGamePolicyReadinessPanel"]["summaryDetails"][0]["label"],
        json!("Status")
    );
    assert_eq!(
        encoded["appGamePlatformProofStatusPanel"]["rows"][0]["title"],
        json!("Windows")
    );
    assert_eq!(
        encoded["appGameChildRuntimeTransportReceiptPanel"]["rows"][0]["details"][0]["value"],
        json!("manual-required")
    );
    assert!(encoded.get("app_game_policy_readiness_panel").is_none());

    let decoded: ParentRouteLiveActivitySnapshot = serde_json::from_value(encoded)
        .value_or_unreachable(crate::assert_context!("live activity must round-trip"));
    assert_eq!(decoded, snapshot);
}

#[test]
fn action_result_and_subscription_event_round_trip_generated_boundary_names() {
    let action = ParentUiAction {
        action: ParentUiActionKind::ScreenSettingsReplaceRequested,
        route: ParentRouteId::PolicyScreen,
        context: Some(ParentRouteContext {
            selected_child_device_id: ParentChildDeviceId::parse("child-device-1"),
        }),
        command: Some("screen-settings-replace".to_string()),
        payload: json!({
            "screenSettingsUpdateKind": "replace",
            "requestId": "screen-settings-request-1"
        }),
    };
    let action_encoded = serde_json::to_value(&action)
        .value_or_unreachable(crate::assert_context!("action must serialize"));

    assert_eq!(
        action_encoded,
        json!({
            "action": "screen-settings-replace-requested",
            "route": "policy-screen",
            "context": {
                "selectedChildDeviceId": "child-device-1"
            },
            "command": "screen-settings-replace",
            "payload": {
                "screenSettingsUpdateKind": "replace",
                "requestId": "screen-settings-request-1"
            }
        })
    );
    let decoded_action: ParentUiAction = serde_json::from_value(action_encoded)
        .value_or_unreachable(crate::assert_context!("action must deserialize"));
    assert_eq!(decoded_action, action);

    let snapshot = route_snapshot(ParentRouteId::PolicyScreen);
    let result = ParentUiActionResult {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        accepted: true,
        connection_state: ParentBridgeConnectionState::Connected,
        message: "accepted".to_string(),
        snapshot: Some(snapshot.clone()),
        events: Vec::new(),
    };
    let result_value = serde_json::to_value(&result)
        .value_or_unreachable(crate::assert_context!("result must serialize"));
    assert_eq!(
        result_value["schemaVersion"],
        json!(PARENT_UI_BRIDGE_SCHEMA_VERSION)
    );
    assert_eq!(result_value["connectionState"], json!("connected"));
    let decoded_result: ParentUiActionResult = serde_json::from_value(result_value)
        .value_or_unreachable(crate::assert_context!("result must deserialize"));
    assert_eq!(decoded_result, result);

    let route_context = ParentRouteContext {
        selected_child_device_id: ParentChildDeviceId::parse("child-device-1"),
    };
    let command_result_projection = ParentCommandResultProjectionSnapshot {
        projection_kind: "app-game-timer-parent-preference-setup".to_string(),
        details: vec![ParentCommandResultDetailSnapshot {
            label: "Status".to_string(),
            value: "Ready".to_string(),
        }],
    };
    let event = ParentRouteEventSnapshot {
        event: Some("agent.connection.ready".to_string()),
        event_id: ParentRouteEventId::parse("agent.connection.ready-1"),
        correlation_id: ParentRouteEventCorrelationId::parse("ready"),
        sent_at: Some("2026-06-23T00:00:00Z".to_string()),
        source_peer_id: ParentRoutePeerId::parse("local-dev-agent"),
        source_role: Some(ParentRoutePeerRole::AgentService),
        target_peer_id: ParentRoutePeerId::parse("portal-dev"),
        target_role: Some(ParentRoutePeerRole::Portal),
        severity: Some("info".to_string()),
        payload: Some(json!({ "route": "policy-screen" })),
        snapshot: None,
        command_result_projection: Some(command_result_projection),
    };
    let subscription = ParentSubscriptionEvent {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        route: ParentRouteId::PolicyScreen,
        snapshot,
        events: Some(vec![event]),
    };
    let subscription_value = serde_json::to_value(&subscription)
        .value_or_unreachable(crate::assert_context!("subscription must serialize"));
    assert_eq!(subscription_value["route"], json!("policy-screen"));
    assert_eq!(
        subscription_value["events"][0]["eventId"],
        json!("agent.connection.ready-1")
    );
    assert_eq!(
        subscription_value["events"][0]["commandResultProjection"]["details"][0]["value"],
        json!("Ready")
    );
    let decoded_subscription: ParentSubscriptionEvent = serde_json::from_value(subscription_value)
        .value_or_unreachable(crate::assert_context!("subscription must deserialize"));
    assert_eq!(decoded_subscription, subscription);
    assert_eq!(
        route_context
            .selected_child_device_id
            .value_or_unreachable(crate::assert_context!(
                "selected child device id must exist"
            ))
            .as_str(),
        "child-device-1"
    );
}

#[test]
fn browser_add_device_discovery_snapshot_serializes_probe_evidence_shape() {
    let snapshot = browser_add_device_discovery_snapshot();
    let encoded = serde_json::to_value(&snapshot)
        .value_or_unreachable(crate::assert_context!("snapshot must serialize"));

    assert_eq!(
        encoded["serviceIdentityProbeEvidence"][0]["evidenceKind"],
        json!("http-status")
    );
    assert_eq!(
        encoded["serviceIdentityProbeEvidence"][0]["value"],
        json!("200")
    );
}
