use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventHistoryState;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::{AgentCommandName, AgentEventName};
use ocentra_parent_runtime_core::parent_service_health::{
    ParentAgentServiceHealth, ParentAgentServiceHealthReason,
};
use ocentra_parent_runtime_core::parent_ui_bridge::load_parent_route_snapshot_with_service_health;
use ocentra_parent_runtime_core::parent_ui_bridge::projection::ParentAgentServiceProjectionResponse;
use ocentra_schema::parent_ui_bridge::{
    ParentBridgeConnectionState, ParentChildDeviceId, ParentRouteContext, ParentRouteDataSource,
};
use serde_json::{json, Value};

use super::tests_support::{
    lan_event, projected_action_result, projected_route_snapshot, projected_subscription_event,
    projection_response, require_ok, sample_lan_read_model, signed_child_agent_reported_event,
    with_isolated_agent_addr,
};
use super::{
    dispatch_parent_ui_action, load_parent_route_snapshot, ParentRouteId, ParentUiAction,
    ParentUiActionKind,
};

use super::common::events::responses::*;
use super::common::events::samples::*;
use super::common::events::tracking::tracking_read_model_response_event;
use super::common::helpers::*;

#[path = "snapshot_and_dispatch_tests_activity.rs"]
mod snapshot_and_dispatch_tests_activity;
#[path = "snapshot_and_dispatch_tests_lan.rs"]
mod snapshot_and_dispatch_tests_lan;
#[path = "snapshot_and_dispatch_tests_social.rs"]
mod snapshot_and_dispatch_tests_social;
#[path = "snapshot_and_dispatch_tests_start.rs"]
mod snapshot_and_dispatch_tests_start;
#[path = "snapshot_and_dispatch_tests_support.rs"]
mod snapshot_and_dispatch_tests_support;

use snapshot_and_dispatch_tests_support::*;

#[test]
fn start_route_projects_lan_as_observation_without_minting_setup_authority() {
    let value = projected_route_snapshot_json(
        ParentRouteId::Start,
        lan_status_projection(sample_lan_read_model()),
        TestContext("start route available LAN snapshot serializes"),
    );
    let panel = &value["setupFirstRunPanel"];

    assert_eq!(value["dataSource"], "host-bridge");
    assert_eq!(value["connectionState"], "connected");
    assert_eq!(panel["summaryDetails"][1]["value"], "unavailable");
    assert_panel_detail_value(
        &panel["cards"][0]["details"],
        TestLabel("LAN source"),
        TestValue("LAN"),
    );
    assert_panel_detail_value(
        &panel["cards"][0]["details"],
        TestLabel("Selected device"),
        TestValue("not-selected"),
    );
    assert_panel_detail_value(
        &panel["cards"][0]["details"],
        TestLabel("Selected device status"),
        TestValue("observation; trust=unpaired, reachability=offline, control=false, authority=unavailable"),
    );
    assert_panel_detail_value(
        &panel["cards"][0]["details"],
        TestLabel("LAN authority"),
        TestValue("observation only; ownership and trust remain unavailable"),
    );
    assert_panel_detail_value(
        &panel["cards"][1]["details"],
        TestLabel("Setup state"),
        TestValue("manual-required"),
    );
    assert_panel_detail_value(
        &panel["cards"][1]["details"],
        TestLabel("Account identity"),
        TestValue("manual-required"),
    );
    assert_panel_detail_value(
        &panel["cards"][1]["details"],
        TestLabel("Device authority"),
        TestValue("manual-required"),
    );
    assert_panel_detail_value(
        &panel["cards"][3]["details"],
        TestLabel("Degraded/manual state"),
        TestValue("manual-required"),
    );
    assert_eq!(panel["productClaim"], "This panel reports only whether the Start route has a live Rust-owned setup-first-run snapshot. It does not claim live account readiness, signed installer readiness, pairing trust, data-custody execution, or onboarding completion.");
}

#[test]
fn start_route_unavailable_snapshot_keeps_recovery_and_authority_fail_closed() {
    let value = with_isolated_agent_addr(|| {
        route_snapshot_json(
            ParentRouteId::Start,
            None,
            TestContext("start route unavailable snapshot serializes"),
        )
    });
    let panel = &value["setupFirstRunPanel"];

    assert_eq!(value["dataSource"], "unavailable");
    assert_eq!(value["connectionState"], "error");
    assert_eq!(panel["summaryDetails"][1]["value"], "unavailable");
    assert_panel_detail_value(
        &panel["cards"][0]["details"],
        TestLabel("LAN source"),
        TestValue("unavailable"),
    );
    assert_panel_detail_value(
        &panel["cards"][0]["details"],
        TestLabel("Diagnostic detail"),
        TestValue("captured-in-rust-bridge"),
    );
    assert_panel_detail_value(
        &panel["cards"][1]["details"],
        TestLabel("Recovery"),
        TestValue("manual-required"),
    );
    assert_panel_detail_value(
        &panel["cards"][1]["details"],
        TestLabel("Device authority"),
        TestValue("manual-required"),
    );
    assert_panel_detail_value(
        &panel["cards"][3]["details"],
        TestLabel("Source and custody"),
        TestValue("Rust-owned boundary; unavailable owners stay explicit"),
    );
    assert_eq!(
        panel["cards"][3]["details"]
            .as_array()
            .and_then(|details| details
                .iter()
                .find(|detail| detail["label"] == "Action planning"))
            .and_then(|detail| detail["value"].as_str()),
        Some("not invoked")
    );
}

#[test]
fn non_start_route_does_not_attach_first_run_panel() {
    let value = projected_route_snapshot_json(
        ParentRouteId::Overview,
        Vec::new(),
        TestContext("overview route snapshot serializes without setup panel"),
    );

    assert_eq!(value["route"], "overview");
    assert!(value["setupFirstRunPanel"].is_null());
}

#[test]
fn desktop_distribution_routes_project_host_status_without_lan_commands() {
    for route in [
        ParentRouteId::PlatformsInstall,
        ParentRouteId::InstallUpdates,
    ] {
        let value = projected_route_snapshot_json(
            route,
            Vec::new(),
            TestContext("desktop distribution route snapshot serializes"),
        );
        let distribution = &value["parentDesktopDistribution"];

        assert_eq!(value["dataSource"], "host-bridge");
        assert_eq!(value["connectionState"], "connected");
        assert_eq!(value["commandEnabled"], false);
        assert!(value["parentPortalRows"].is_null());
        assert_eq!(distribution["payloadSource"], "rust-parent-runtime");
        assert_eq!(
            distribution["sourceCustodyState"],
            "source-custody-manual-required"
        );
        assert_eq!(
            distribution["productClaimState"],
            "read-only-contract-status-no-execution-owner"
        );
        assert_eq!(
            distribution["noClaim"],
            "no-installer-updater-rollback-signing-notarization-store-execution"
        );
        assert_eq!(distribution["packageFrontendState"], "built-portal-dist");
        assert_eq!(
            distribution["packageServiceManagerState"],
            "package-installs-auto-start-service"
        );
        assert_eq!(
            distribution["packageHealthProbeState"],
            "package-health-probe-required"
        );
        assert_eq!(
            distribution["packagePreviewState"],
            "unsigned-package-preview"
        );
        assert_eq!(
            distribution["updateChannelState"],
            "update-channel-scaffold"
        );
        assert_eq!(distribution["rollbackState"], "rollback-unavailable");
        assert_eq!(distribution["signingState"], "signing-manual-required");
        assert_eq!(distribution["actionsAvailable"], false);
    }
}

#[test]
fn desktop_distribution_status_survives_unavailable_agent_health() {
    let service_health = ParentAgentServiceHealth::unavailable_with_reason(
        ParentAgentServiceHealthReason::TransportUnavailable,
    );
    let value = serialize_json(
        &load_parent_route_snapshot_with_service_health(
            ParentRouteId::PlatformsInstall,
            None,
            &service_health,
        ),
        TestContext("unavailable desktop distribution snapshot serializes"),
    );

    assert_eq!(value["dataSource"], "unavailable");
    assert_eq!(value["connectionState"], "error");
    assert_eq!(value["commandEnabled"], false);
    assert_eq!(
        value["parentDesktopDistribution"]["artifactProofState"],
        "ci-package-preview-artifact-proof"
    );
    assert_eq!(
        value["parentDesktopDistribution"]["storeDistributionState"],
        "store-distribution-manual-required"
    );
    assert_eq!(
        value["parentDesktopDistribution"]["actionsAvailable"],
        false
    );
}

#[test]
fn unrelated_routes_do_not_attach_desktop_distribution_status() {
    let value = projected_route_snapshot_json(
        ParentRouteId::Overview,
        Vec::new(),
        TestContext("overview route excludes desktop distribution status"),
    );

    assert!(value["parentDesktopDistribution"].is_null());
}

#[test]
fn proof_panels_route_snapshot_excludes_browser_product_panels() {
    let value = projected_route_snapshot_json(
        ParentRouteId::ProofPanels,
        proof_panels_projection(),
        TestContext("proof-panels route snapshot serializes"),
    );

    assert_eq!(value["route"], "proof-panels");
    assert!(value["browserPanels"].is_null());
}

#[test]
fn parent_route_snapshot_keeps_overlay_diagnostics_on_dev_routes_only() {
    let value = projected_route_snapshot_json(
        ParentRouteId::Devices,
        lan_status_projection(sample_lan_read_model()),
        TestContext("parent route snapshot serializes"),
    );
    let browser_value = projected_route_snapshot_json(
        ParentRouteId::Browser,
        Vec::new(),
        TestContext("browser route snapshot serializes"),
    );
    let diagnostics_value = projected_route_snapshot_json(
        ParentRouteId::Diagnostics,
        Vec::new(),
        TestContext("diagnostics route snapshot serializes"),
    );
    let frame_tuner_value = projected_route_snapshot_json(
        ParentRouteId::FrameTuner,
        Vec::new(),
        TestContext("frame tuner route snapshot serializes"),
    );

    assert_eq!(value["dataSource"], "rust-read-model");
    assert_eq!(value["connectionState"], "connected");
    assert_eq!(value["summary"]["household"], "1 device visible");
    assert_eq!(value["summary"]["childDevice"], "1 discoverable");
    assert_eq!(value["summary"]["parentAccess"], "active-controller");
    assert_eq!(value["diagnosticPanelsEnabled"], false);
    assert!(value["parentPortalRows"].is_array());
    assert!(value["parentPortalShellStatus"].is_object());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["scanSummary"].is_object());
    assert_eq!(
        value["liveActivity"]["lanAddDeviceReadModel"]["discoverySource"],
        json!("physical-household-lan")
    );
    assert_eq!(
        value["liveActivity"]["lanAddDeviceReadModel"]["physicalHouseholdLanState"],
        json!("discovered")
    );
    assert_eq!(
        value["liveActivity"]["lanAddDeviceReadModel"]["cloudRelayState"],
        json!("unavailable")
    );
    assert_eq!(
        value["liveActivity"]["lanAddDeviceReadModel"]["discoveredDevices"][0]["childDevice"]
            ["label"],
        json!("Study Laptop")
    );
    assert_eq!(
        value["liveActivity"]["lanAddDeviceReadModel"]["canonicalHouseholdDevices"][0]
            ["displayName"],
        json!("study-laptop")
    );
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["pairingRequests"].is_array());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["trustedDeviceRegistry"].is_array());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["householdDeviceDecisions"].is_array());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["lanDiscoverySourceMatrix"].is_object());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["trustedDeviceIds"].is_array());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["revokedDeviceIds"].is_array());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["routeRequirementLabels"].is_array());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["auditCheckLabels"].is_array());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["honestNonClaims"].is_array());
    assert!(value["liveActivity"]["activityTrackingReadModel"].is_null());
    assert!(value["liveActivity"]["localAiRuntimeStatusEvent"].is_null());
    assert!(value["liveActivity"]["browserManagedStatus"].is_null());
    assert!(browser_value["browserPanels"].is_null());
    assert_eq!(diagnostics_value["diagnosticPanelsEnabled"], true);
    assert_eq!(frame_tuner_value["route"], "frame-tuner");
    assert_eq!(frame_tuner_value["dataSource"], "dev-diagnostics");
    assert_eq!(frame_tuner_value["summary"]["title"], "Frame tuner");
    assert_eq!(frame_tuner_value["diagnosticPanelsEnabled"], true);
    assert!(frame_tuner_value["liveActivity"].is_null());
}

#[test]
fn policy_network_route_load_keeps_host_bridge_surface_and_attaches_lan_read_model() {
    let value = projected_route_snapshot_json(
        ParentRouteId::PolicyNetwork,
        vec![
            projection_response(
                AgentCommandName::AgentLanPairingStatusGet,
                lan_event(
                    AgentEventName::AgentLanPairingStatusReported,
                    &sample_lan_read_model(),
                ),
            ),
            projection_response(
                AgentCommandName::AgentPolicyPreviewReadModelGet,
                policy_preview_response_event(),
            ),
        ],
        TestContext("policy-network route snapshot serializes"),
    );

    assert_eq!(value["route"], "policy-network");
    assert_eq!(value["dataSource"], "host-bridge");
    assert_eq!(value["connectionState"], "connected");
    assert_eq!(value["summary"]["household"], "1 device visible");
    assert_eq!(value["summary"]["childDevice"], "1 discoverable");
    assert_eq!(
        require_ok(
            serde_json::to_value(AgentCommandName::AgentLanPairingStatusGet),
            "LAN status command serializes",
        ),
        json!("agent.lan-pairing.status.get")
    );
    assert_eq!(
        require_ok(
            serde_json::to_value(AgentCommandName::AgentPolicyPreviewReadModelGet),
            "policy preview command serializes",
        ),
        json!("agent.policy.preview.read-model.get")
    );
    assert!(value["parentPortalRows"].is_null());
    assert!(value["liveActivity"]["lanAddDeviceReadModel"]["scanSummary"].is_object());
    assert_eq!(
        value["liveActivity"]["lanAddDeviceReadModel"]["discoveredDevices"][0]["childDevice"]
            ["deviceId"],
        json!("network-neighbor-1")
    );
    assert_eq!(
        value["liveActivity"]["lanAddDeviceReadModel"]["canonicalHouseholdDevices"][0]
            ["displayName"],
        json!("study-laptop")
    );
    assert_eq!(
        value["liveActivity"]["policyPreviewPanel"]["summaryDetails"][1]["value"],
        json!("policy-preview.network.1")
    );
    assert_eq!(
        value["liveActivity"]["policyPreviewPanel"]["cards"][2]["title"],
        json!("Approval authority")
    );
}

#[test]
fn parent_subscription_event_serializes_for_host_bridge() {
    let value = projected_subscription_event_json(
        ParentRouteId::Devices,
        lan_status_projection(sample_lan_read_model()),
        TestContext("parent subscription event serializes"),
    );

    assert_eq!(value["schemaVersion"], 1);
    assert_eq!(value["route"], "devices");
    assert_eq!(value["snapshot"]["route"], "devices");
    assert_eq!(value["snapshot"]["connectionState"], "connected");
    assert_eq!(value["snapshot"]["dataSource"], "rust-read-model");
    assert_eq!(
        value["events"].as_array().map(|events| events.len()),
        Some(3)
    );
    assert_eq!(
        value["events"][0]["eventId"],
        json!("agent.connection.ready-1")
    );
    assert_eq!(
        value["events"][1]["event"],
        json!("agent.lan-pairing.status.reported")
    );
    assert_eq!(value["events"][1]["correlationId"], json!("lan"));
    let warning = &value["events"][2];
    let warning_event_id = require_some(
        warning["eventId"].as_str(),
        TestContext("rejected replay warning has a host-owned event id"),
    );
    assert!(warning_event_id.starts_with("lan-runtime-event-chain-replay-rejected-"));
    let warning_sent_at = require_some(
        warning["sentAt"].as_str(),
        TestContext("rejected replay warning has a sent-at timestamp"),
    );
    let _parsed_warning_sent_at = require_ok(
        chrono::DateTime::parse_from_rfc3339(warning_sent_at),
        "rejected replay warning sent-at parses as RFC3339",
    );
    let mut warning_without_host_fields = warning.clone();
    warning_without_host_fields["eventId"] = json!(null);
    warning_without_host_fields["sentAt"] = json!(null);
    assert_eq!(
        warning_without_host_fields,
        json!({
            "event": "lan-runtime-event-chain-replay-rejected",
            "eventId": null,
            "correlationId": null,
            "sentAt": null,
            "sourcePeerId": constants::peer::LOCAL_DEV_AGENT,
            "sourceRole": "agent-service",
            "targetPeerId": constants::peer::PORTAL_DEV,
            "targetRole": "portal",
            "severity": "warn",
            "payload": null,
            "snapshot": null,
            "commandResultProjection": null
        })
    );
}

#[test]
fn parent_subscription_event_dedupes_duplicate_event_ids_while_preserving_latest_lan_event() {
    let mut duplicate_id_response = lan_event(
        AgentEventName::AgentLanPairingStatusReported,
        &sample_lan_read_model_with_explicit_history(),
    );
    duplicate_id_response.event_id = "agent.connection.ready-1".to_string();
    let value = projected_subscription_event_json(
        ParentRouteId::Devices,
        vec![projection_response(
            AgentCommandName::AgentLanPairingStatusGet,
            duplicate_id_response,
        )],
        TestContext("parent subscription event serializes with duplicate event ids"),
    );

    assert_eq!(
        value["events"].as_array().map(|events| events.len()),
        Some(2)
    );
    assert_eq!(
        value["events"][0]["eventId"],
        json!("agent.connection.ready-1")
    );
    assert_eq!(
        value["events"][0]["event"],
        json!("agent.lan-pairing.status.reported")
    );
    assert_eq!(value["events"][0]["correlationId"], json!("lan"));
    assert_eq!(
        value["events"][1]["event"],
        json!("lan-runtime-event-chain-replay-rejected")
    );
    let warning_event_id = require_some(
        value["events"][1]["eventId"].as_str(),
        TestContext("deduped rejected replay warning has a host-owned event id"),
    );
    assert!(warning_event_id.starts_with("lan-runtime-event-chain-replay-rejected-"));
    let warning_sent_at = require_some(
        value["events"][1]["sentAt"].as_str(),
        TestContext("deduped rejected replay warning has a sent-at timestamp"),
    );
    let _parsed_warning_sent_at = require_ok(
        chrono::DateTime::parse_from_rfc3339(warning_sent_at),
        "deduped rejected replay warning sent-at parses as RFC3339",
    );
    assert_eq!(value["events"][1]["correlationId"], json!(null));
    assert_eq!(value["events"][1]["payload"], json!(null));
    assert_eq!(
        value["snapshot"]["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]
            ["latestEventId"],
        json!("lan-history-2")
    );
}

#[test]
fn parent_subscription_event_keeps_lan_diagnostics_and_history_surface_intact() {
    let snapshot_value = projected_route_snapshot_json(
        ParentRouteId::Devices,
        lan_status_projection(sample_lan_read_model_with_explicit_history()),
        TestContext("devices route snapshot serializes with explicit history"),
    );
    let subscription_value = projected_subscription_event_json(
        ParentRouteId::Devices,
        lan_status_projection(sample_lan_read_model_with_explicit_history()),
        TestContext("devices subscription event serializes with explicit history"),
    );

    assert_eq!(subscription_value["snapshot"], snapshot_value);
    assert_eq!(snapshot_value["diagnosticPanelsEnabled"], false);
    assert_eq!(
        snapshot_value["parentPortalRows"]
            .as_array()
            .map(|rows| rows.len()),
        Some(6)
    );
    assert_eq!(
        snapshot_value["liveActivity"]["lanAddDeviceReadModel"]["discoveredDevices"]
            .as_array()
            .map(|rows| rows.len()),
        Some(1)
    );
    assert_eq!(
        snapshot_value["liveActivity"]["lanAddDeviceReadModel"]["canonicalHouseholdDevices"]
            .as_array()
            .map(|rows| rows.len()),
        Some(1)
    );
    assert_eq!(
        snapshot_value["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]["state"],
        json!("ready")
    );
    assert_eq!(
        snapshot_value["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]
            ["latestEventId"],
        json!("lan-history-2")
    );
    assert_eq!(
        snapshot_value["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]
            ["latestObservedAt"],
        json!("2026-06-23T00:00:02Z")
    );
    assert_eq!(
        snapshot_value["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]["rows"]
            .as_array()
            .map(|rows| rows.len()),
        Some(2)
    );
}

#[test]
fn parent_subscription_event_preserves_explicit_lan_history_state_labels() {
    for (state, expected) in [
        (LanDiscoveryEventHistoryState::Empty, "empty"),
        (LanDiscoveryEventHistoryState::AgentOffline, "agent-offline"),
        (
            LanDiscoveryEventHistoryState::ManualRequired,
            "manual-required",
        ),
        (LanDiscoveryEventHistoryState::Unavailable, "unavailable"),
        (LanDiscoveryEventHistoryState::Degraded, "degraded"),
    ] {
        snapshot_and_dispatch_tests_lan::assert_history_state_label(state, TestValue(expected));
    }
}

#[test]
fn parent_subscription_event_projects_stale_lan_history_from_rust_metadata() {
    let mut read_model = sample_lan_read_model_with_explicit_history();
    read_model.selected_device_readiness.reachability = LanPairingDeviceReachability::Stale;
    read_model.selected_device_readiness.stale_at = Some("2026-06-23T00:10:00Z".to_string());
    let value = projected_subscription_event_json(
        ParentRouteId::Devices,
        lan_status_projection(read_model),
        TestContext("devices subscription event serializes with stale LAN metadata"),
    );

    assert_eq!(
        value["snapshot"]["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]
            ["state"],
        json!("stale")
    );
    assert_eq!(
        value["snapshot"]["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]
            ["latestEventId"],
        json!("lan-history-2")
    );
    assert_eq!(
        value["snapshot"]["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]
            ["latestObservedAt"],
        json!("2026-06-23T00:00:02Z")
    );
}

#[test]
fn devices_route_degrades_honestly_when_agent_service_is_unavailable() {
    let value = route_snapshot_json(
        ParentRouteId::Devices,
        None,
        TestContext("parent route snapshot serializes"),
    );

    assert_eq!(value["connectionState"], "error");
    assert_eq!(value["commandEnabled"], false);
    assert_eq!(value["dataSource"], "unavailable");
    assert_eq!(value["summary"]["household"], "unavailable");
    assert!(value["liveActivity"]["lanAddDeviceReadModel"].is_null());
}

#[test]
fn devices_route_owner_gate_precedes_transport_connection() {
    let value = route_snapshot_json(
        ParentRouteId::Devices,
        None,
        TestContext("parent route snapshot serializes before transport connection"),
    );

    assert_eq!(value["connectionState"], "error");
    assert_eq!(value["commandEnabled"], false);
    assert_eq!(value["dataSource"], "unavailable");
    assert_eq!(value["summary"]["household"], "unavailable");
    assert!(value["liveActivity"]["lanAddDeviceReadModel"].is_null());
}

#[test]
fn devices_route_load_uses_passive_status_get_with_default_origin() {
    assert_eq!(
        serialize_json(
            &AgentCommandName::AgentLanPairingStatusGet,
            TestContext("LAN status command serializes"),
        ),
        json!("agent.lan-pairing.status.get")
    );
    assert_eq!(
        constants::bind::DEFAULT_ALLOWED_ORIGINS.first().copied(),
        Some(constants::bind::DEFAULT_ALLOWED_ORIGINS[0])
    );
    let snapshot = load_parent_route_snapshot(ParentRouteId::Devices, None);
    assert_eq!(
        snapshot.connection_state,
        ParentBridgeConnectionState::Error
    );
    assert_eq!(snapshot.data_source, ParentRouteDataSource::Unavailable);
}

#[test]
fn devices_route_load_ignores_selected_child_device_context_for_local_status_target() {
    let context = ParentRouteContext {
        selected_child_device_id: ParentChildDeviceId::parse("lan-physical-mac-54271e97c331"),
    };
    let context_json = serialize_json(
        &context,
        TestContext("selected child route context serializes"),
    );
    assert_eq!(
        context_json["selectedChildDeviceId"],
        json!("lan-physical-mac-54271e97c331")
    );
    let snapshot = load_parent_route_snapshot(ParentRouteId::Devices, Some(&context));
    assert_eq!(
        snapshot.connection_state,
        ParentBridgeConnectionState::Error
    );
    assert_eq!(snapshot.data_source, ParentRouteDataSource::Unavailable);
}

#[test]
fn devices_route_scan_action_uses_browser_discovery_scan_with_default_origin() {
    let action = empty_action(
        ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested,
        ParentRouteId::Devices,
    );
    let result = dispatch_parent_ui_action(&action);

    assert_eq!(
        serialize_json(
            &AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
            TestContext("LAN discovery command serializes"),
        ),
        json!("agent.lan-pairing.browser-discovery.scan")
    );
    assert_owner_unavailable_action(&result);
}

#[test]
fn parent_ui_action_serializes_and_returns_snapshot() {
    let projection_action = empty_action(
        ParentUiActionKind::NetworkFlowReadModelRefreshRequested,
        ParentRouteId::Activity,
    );
    let mut responses = vec![projection_response(
        AgentCommandName::AgentNetworkFlowReadModelGet,
        network_flow_response_event(),
    )];
    responses.extend(activity_route_projection_after_network_refresh());
    let projected = projected_action_result(&projection_action, responses);
    let action = ParentUiAction {
        action: ParentUiActionKind::AgentCommandRequested,
        route: ParentRouteId::Activity,
        context: None,
        command: Some("agent.network.flow.read-model.get".to_string()),
        payload: json!({ "source": "ui" }),
    };
    let transport_result = dispatch_parent_ui_action(&action);

    assert_eq!(
        serialize_json(&action, TestContext("parent UI action serializes"))["command"],
        json!("agent.network.flow.read-model.get")
    );
    assert_owner_unavailable_action(&transport_result);
    assert!(projected.accepted);
    assert_eq!(
        projected
            .snapshot
            .as_ref()
            .map(|snapshot| snapshot.route.clone()),
        Some(ParentRouteId::Activity)
    );
    assert_eq!(projected.events.len(), 2);
    assert_eq!(
        result_network_flow_row_event_id(&projected),
        Some(CommandText("network-ui-flow-1".to_string()))
    );
    let live_activity = live_activity_json(
        &projected,
        TestContext("network flow projection returns snapshot"),
        TestContext("network flow projection returns live activity snapshot"),
        TestContext("live activity snapshot serializes"),
    );
    assert_network_policy_bridge_snapshot(&live_activity, &json!(3));
}

#[test]
fn typed_network_projection_retains_runtime_and_policy_envelopes() {
    let value = projected_route_snapshot_json(
        ParentRouteId::ProofPanels,
        vec![
            projection_response(
                AgentCommandName::AgentNetworkFlowReadModelGet,
                network_flow_response_event(),
            ),
            projection_response(
                AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
                network_runtime_event_chain_response_event(),
            ),
            projection_response(
                AgentCommandName::AgentPolicyPreviewReadModelGet,
                policy_preview_response_event(),
            ),
            projection_response(
                AgentCommandName::AgentActivityTrackingReadModelGet,
                tracking_read_model_response_event(),
            ),
        ],
        TestContext("network projection preserves typed responses"),
    );
    assert_eq!(value["connectionState"], "connected");
}

#[test]
fn lan_agent_command_requested_for_devices_route_forwards_signed_child_observe_payload_and_replay_fields(
) {
    let action = ParentUiAction {
        action: ParentUiActionKind::AgentCommandRequested,
        route: ParentRouteId::Devices,
        context: Some(ParentRouteContext {
            selected_child_device_id: ParentChildDeviceId::parse("selected-child-android-1"),
        }),
        command: Some("agent.lan-pairing.signed-child-agent.observe".to_string()),
        payload: json!({
            "lanSignedChildAgentEnvelopeJson": "{\"schemaVersion\":1}",
        }),
    };
    let result = dispatch_parent_ui_action(&action);
    let action_json = serialize_json(&action, TestContext("signed child action serializes"));
    let response = signed_child_agent_reported_event(&sample_lan_read_model());

    assert_eq!(
        action_json["command"],
        json!("agent.lan-pairing.signed-child-agent.observe")
    );
    assert_eq!(
        action_json["payload"]["lanSignedChildAgentEnvelopeJson"],
        json!("{\"schemaVersion\":1}")
    );
    assert_eq!(
        action_json["context"]["selectedChildDeviceId"],
        json!("selected-child-android-1")
    );
    assert_eq!(
        response.event,
        AgentEventName::AgentLanPairingSignedChildAgentReported
    );
    assert_eq!(
        response
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_VERIFICATION),
        Some(&LogFieldValue::String(
            constants::value::LAN_SIGNED_CHILD_AGENT_VERIFICATION_ACCEPTED.to_string(),
        ))
    );
    assert_eq!(
        response
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_REPLAY_OBSERVED_COUNT),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_owner_unavailable_action(&result);
}

#[test]
fn lan_scan_action_returns_bounded_error_when_response_is_unavailable() {
    let result = dispatch_parent_ui_action(&empty_action(
        ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested,
        ParentRouteId::Devices,
    ));
    assert_eq!(
        serialize_json(
            &AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
            TestContext("LAN scan command serializes"),
        ),
        json!("agent.lan-pairing.browser-discovery.scan")
    );
    assert_owner_unavailable_action(&result);
}

#[test]
fn product_bridge_actions_return_route_snapshots_without_invented_overlay_data() {
    let lan_scan = dispatch_parent_ui_action(&empty_action(
        ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested,
        ParentRouteId::Devices,
    ));
    let network_refresh = dispatch_parent_ui_action(&empty_action(
        ParentUiActionKind::NetworkFlowReadModelRefreshRequested,
        ParentRouteId::Activity,
    ));
    let tracking_retention_write = dispatch_parent_ui_action(&empty_action(
        ParentUiActionKind::TrackingRetentionSettingsWriteRequested,
        ParentRouteId::Activity,
    ));
    let app_game_dispatch_execute = dispatch_parent_ui_action(&empty_action(
        ParentUiActionKind::AppGameAdapterDispatchExecuteRequested,
        ParentRouteId::AppGameSessions,
    ));
    let app_game_timer_parent_preference_setup = dispatch_parent_ui_action(&empty_action(
        ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested,
        ParentRouteId::AppGameSessions,
    ));

    for result in [
        &lan_scan,
        &network_refresh,
        &tracking_retention_write,
        &app_game_dispatch_execute,
        &app_game_timer_parent_preference_setup,
    ] {
        assert_owner_unavailable_action(result);
        assert!(result
            .snapshot
            .as_ref()
            .and_then(|snapshot| snapshot.live_activity.as_ref())
            .is_none());
    }
}
