use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventHistoryState;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_schema::parent_ui_bridge::{
    ParentBridgeConnectionState, ParentChildDeviceId, ParentRouteContext, ParentRouteDataSource,
};
use serde_json::{json, Value};

use super::tests_support::{
    lan_event, require_ok, sample_lan_read_model, signed_child_agent_reported_event,
    start_lan_local_server, start_lan_local_server_with_capture,
    start_local_server_with_capture_responses, start_local_server_with_ready_only, with_agent_addr,
    with_isolated_agent_addr,
};
use super::{
    dispatch_parent_ui_action, load_parent_route_snapshot, ParentRouteId, ParentUiAction,
    ParentUiActionKind,
};

use super::common::events::responses::*;
use super::common::events::samples::*;
use super::common::helpers::*;

#[test]
fn parent_route_snapshot_serializes_with_host_bridge_snapshot_fields() {
    let value = route_snapshot_json(
        ParentRouteId::Activity,
        None,
        TestContext("parent route snapshot serializes"),
    );

    assert_eq!(value["schemaVersion"], 1);
    assert_eq!(value["route"], "activity");
    assert_eq!(value["agentEndpoint"], "host-bridge://tauri-parent");
    assert_eq!(value["connectionState"], "connected");
    assert_eq!(value["seasonLabel"], "live");
    assert_eq!(value["dataSource"], "host-bridge");
    assert_eq!(value["summary"]["title"], "Activity");
    assert_eq!(value["summary"]["routeCapability"], "available");
    assert_eq!(value["summary"]["parentAccess"], "proof-missing");
    assert!(value["parentPortalShellStatus"].is_object());
    assert!(value["liveActivity"].is_object());
}

#[test]
fn start_route_snapshot_attaches_setup_first_run_panel() {
    let value = route_snapshot_json(
        ParentRouteId::Start,
        None,
        TestContext("start route snapshot serializes"),
    );

    assert_eq!(value["route"], "start");
    assert_eq!(
        value["setupFirstRunPanel"]["title"],
        "Setup-first-run boundary status"
    );
    assert_eq!(
        value["setupFirstRunPanel"]["summaryCardTitle"],
        "Current boundary status"
    );
    assert_eq!(
        value["setupFirstRunPanel"]["summaryDetails"][1]["value"],
        "unavailable"
    );
    assert_eq!(
        value["setupFirstRunPanel"]["cards"][1]["details"][0]["value"],
        "not wired"
    );
    assert_eq!(
        value["setupFirstRunPanel"]["cards"][3]["details"][1]["value"],
        "presentation only"
    );
}

#[test]
fn start_route_keeps_account_and_session_states_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        state_details,
        TestLabel("No account / session"),
        TestValue("unavailable — Account/session owner must provide current state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — no account / session"),
        TestValue("manual-required — request an owner-backed current session"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Account exists / no household"),
        TestValue("unavailable — Account authority must provide household membership state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — account exists / no household"),
        TestValue("manual-required — request owner-backed household membership"),
    );
}

#[test]
fn start_route_keeps_household_and_child_profile_states_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let missing_details = first_run_missing_details(&value);
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        missing_details,
        TestLabel("Child profile state"),
        TestValue("manual-required"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Household exists / no child profile"),
        TestValue("unavailable — family authority must provide child-profile state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — household exists / no child profile"),
        TestValue("manual-required — request an owner-backed child profile"),
    );
}

#[test]
fn start_route_keeps_device_and_pairing_states_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        state_details,
        TestLabel("Child profile exists / no device"),
        TestValue("unavailable — setup/device-trust owner must provide device state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — child profile exists / no device"),
        TestValue("manual-required — request owner-backed device registration"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Discovered unpaired device"),
        TestValue("unavailable — LAN may only observe discovery; pairing and ownership are not bound"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — discovered unpaired device"),
        TestValue("manual-required — use the trusted pairing owner flow"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Paired device / service unavailable"),
        TestValue("unavailable — child-service owner must provide current availability"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — paired device / service unavailable"),
        TestValue("manual-required — wait for an owner-backed service receipt"),
    );
}

#[test]
fn start_route_keeps_membership_roles_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let missing_details = first_run_missing_details(&value);
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        missing_details,
        TestLabel("Parent controller role"),
        TestValue("manual-required — membership and controller lease owner not bound"),
    );
    assert_panel_detail_value(
        missing_details,
        TestLabel("Co-parent role"),
        TestValue("manual-required — membership role owner not bound"),
    );
    assert_panel_detail_value(
        missing_details,
        TestLabel("Observer role"),
        TestValue("manual-required — membership role owner not bound; observer remains read-only"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Parent controller / co-parent / observer"),
        TestValue("unavailable — Account authority must provide the membership role and controller lease"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — parent role"),
        TestValue("manual-required — request the owner-backed role and lease state"),
    );
}

#[test]
fn start_route_keeps_expiry_states_owner_gated() {
    let value = owner_gated_start_route_snapshot();
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        state_details,
        TestLabel("Invite expiry"),
        TestValue("unavailable — invite owner must report active, expired, or consumed state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — invite expiry"),
        TestValue("manual-required — request a current invite receipt before retrying"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Pairing expiry"),
        TestValue("unavailable — trusted pairing owner must report active, expired, or revoked state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — pairing expiry"),
        TestValue("manual-required — request a current pairing receipt before retrying"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Session expiry"),
        TestValue("unavailable — Account/session owner must report fresh, stale, or expired state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — session expiry"),
        TestValue("manual-required — request a current session receipt before retrying"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Recovery expiry"),
        TestValue("unavailable — recovery owner must report current or expired state"),
    );
    assert_panel_detail_value(
        state_details,
        TestLabel("Next action — recovery expiry"),
        TestValue("manual-required — request a current recovery receipt before retrying"),
    );
}

#[test]
fn start_route_keeps_child_activity_private_on_first_run() {
    let value = owner_gated_start_route_snapshot();
    let state_details = first_run_state_details(&value);

    assert_panel_detail_value(
        state_details,
        TestLabel("Child safety"),
        TestValue(
            "Private child activity is not shown on setup; only authority and readiness boundaries are projected",
        ),
    );
}

fn owner_gated_start_route_snapshot() -> Value {
    with_isolated_agent_addr(|| {
        route_snapshot_json(
            ParentRouteId::Start,
            None,
            TestContext("start route first-run states serialize with owner boundaries"),
        )
    })
}

fn first_run_missing_details(snapshot: &Value) -> &Value {
    &snapshot["setupFirstRunPanel"]["cards"][1]["details"]
}

fn first_run_state_details(snapshot: &Value) -> &Value {
    &snapshot["setupFirstRunPanel"]["cards"][2]["details"]
}

#[test]
fn start_route_projects_lan_as_observation_without_minting_setup_authority() {
    let address = start_lan_local_server(
        AgentEventName::AgentLanPairingStatusReported,
        sample_lan_read_model(),
    );
    let value = with_agent_addr(&address, || {
        route_snapshot_json(
            ParentRouteId::Start,
            None,
            TestContext("start route available LAN snapshot serializes"),
        )
    });
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
    let value = with_isolated_agent_addr(|| {
        route_snapshot_json(
            ParentRouteId::Overview,
            None,
            TestContext("overview route snapshot serializes without setup panel"),
        )
    });

    assert_eq!(value["route"], "overview");
    assert!(value["setupFirstRunPanel"].is_null());
}

#[test]
fn proof_panels_route_snapshot_attaches_browser_route_panels() {
    let value = with_isolated_agent_addr(|| {
        route_snapshot_json(
            ParentRouteId::ProofPanels,
            None,
            TestContext("proof-panels route snapshot serializes"),
        )
    });

    assert_eq!(value["route"], "proof-panels");
    assert_eq!(
        value["browserPanels"]["browserParentExplanation"]["title"],
        "Browser parent explanations"
    );
    assert_eq!(
        value["browserPanels"]["socialAuditExplanation"]["title"],
        "Social explanations"
    );
    assert_eq!(
        value["browserPanels"]["socialDashboard"]["title"],
        "Social dashboard"
    );
    assert_eq!(
        value["browserPanels"]["socialAlertReport"]["title"],
        "Social alerts and reports"
    );
    assert_eq!(
        value["browserPanels"]["socialAlertReportParentSurface"]["title"],
        "Social parent surface status"
    );
    assert_eq!(
        value["browserPanels"]["socialParentNotificationDelivery"]["title"],
        "Social parent notification delivery readiness"
    );
    assert_eq!(
        value["browserPanels"]["browserActionIntentStreamStatus"]["title"],
        "Browser action-intent stream status"
    );
    assert_eq!(
        value["browserPanels"]["browserSocialProviderReceiptStreamStatus"]["title"],
        "Social provider receipt stream status"
    );
    assert_eq!(
        value["browserPanels"]["browserSocialProviderReceiptIngestionReadinessStatus"]["title"],
        "Social provider receipt ingestion readiness"
    );
}

#[test]
fn parent_route_snapshot_keeps_overlay_diagnostics_on_dev_routes_only() {
    let address = start_lan_local_server(
        AgentEventName::AgentLanPairingStatusReported,
        sample_lan_read_model(),
    );
    let value = with_agent_addr(&address, || {
        route_snapshot_json(
            ParentRouteId::Devices,
            None,
            TestContext("parent route snapshot serializes"),
        )
    });
    let browser_value = with_isolated_agent_addr(|| {
        route_snapshot_json(
            ParentRouteId::Browser,
            None,
            TestContext("browser route snapshot serializes"),
        )
    });
    let diagnostics_value = with_isolated_agent_addr(|| {
        route_snapshot_json(
            ParentRouteId::Diagnostics,
            None,
            TestContext("diagnostics route snapshot serializes"),
        )
    });
    let frame_tuner_value = with_isolated_agent_addr(|| {
        route_snapshot_json(
            ParentRouteId::FrameTuner,
            None,
            TestContext("frame tuner route snapshot serializes"),
        )
    });

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
    let (address, capture) = start_local_server_with_capture_responses(vec![
        lan_event(
            AgentEventName::AgentLanPairingStatusReported,
            &sample_lan_read_model(),
        ),
        policy_preview_response_event(),
    ]);
    let value = with_agent_addr(&address, || {
        route_snapshot_json(
            ParentRouteId::PolicyNetwork,
            None,
            TestContext("policy-network route snapshot serializes"),
        )
    });
    let status_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured policy-network LAN status load arrives",
    );
    let policy_preview_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured policy-network policy preview load arrives",
    );

    assert_eq!(value["route"], "policy-network");
    assert_eq!(value["dataSource"], "host-bridge");
    assert_eq!(value["connectionState"], "connected");
    assert_eq!(value["summary"]["household"], "1 device visible");
    assert_eq!(value["summary"]["childDevice"], "1 discoverable");
    assert_eq!(
        status_request.command["command"],
        json!("agent.lan-pairing.status.get")
    );
    assert_eq!(
        policy_preview_request.command["command"],
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
    let address = start_lan_local_server(
        AgentEventName::AgentLanPairingStatusReported,
        sample_lan_read_model(),
    );
    let value = with_agent_addr(&address, || {
        subscription_event_json(
            ParentRouteId::Devices,
            None,
            TestContext("parent subscription event serializes"),
        )
    });

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
    let (address, _capture) =
        start_local_server_with_capture_responses(vec![duplicate_id_response]);
    let value = with_agent_addr(&address, || {
        subscription_event_json(
            ParentRouteId::Devices,
            None,
            TestContext("parent subscription event serializes with duplicate event ids"),
        )
    });

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
    let snapshot_address = start_lan_local_server(
        AgentEventName::AgentLanPairingStatusReported,
        sample_lan_read_model_with_explicit_history(),
    );
    let subscription_address = start_lan_local_server(
        AgentEventName::AgentLanPairingStatusReported,
        sample_lan_read_model_with_explicit_history(),
    );
    let snapshot_value = with_agent_addr(&snapshot_address, || {
        route_snapshot_json(
            ParentRouteId::Devices,
            None,
            TestContext("devices route snapshot serializes with explicit history"),
        )
    });
    let subscription_value = with_agent_addr(&subscription_address, || {
        subscription_event_json(
            ParentRouteId::Devices,
            None,
            TestContext("devices subscription event serializes with explicit history"),
        )
    });

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
        assert_history_state_label(state, TestValue(expected));
    }
}

#[test]
fn parent_subscription_event_projects_stale_lan_history_from_rust_metadata() {
    let mut read_model = sample_lan_read_model_with_explicit_history();
    read_model.selected_device_readiness.reachability = LanPairingDeviceReachability::Stale;
    read_model.selected_device_readiness.stale_at = Some("2026-06-23T00:10:00Z".to_string());
    let address = start_lan_local_server(AgentEventName::AgentLanPairingStatusReported, read_model);
    let value = with_agent_addr(&address, || {
        subscription_event_json(
            ParentRouteId::Devices,
            None,
            TestContext("devices subscription event serializes with stale LAN metadata"),
        )
    });

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
    let value = with_agent_addr("127.0.0.1:9", || {
        route_snapshot_json(
            ParentRouteId::Devices,
            None,
            TestContext("parent route snapshot serializes"),
        )
    });

    assert_eq!(value["connectionState"], "error");
    assert_eq!(value["commandEnabled"], false);
    assert_eq!(value["dataSource"], "unavailable");
    assert_eq!(value["summary"]["household"], "unavailable");
    assert!(value["liveActivity"]["lanAddDeviceReadModel"].is_null());
}

#[test]
fn devices_route_degrades_honestly_when_agent_service_response_times_out() {
    let (address, capture) = start_local_server_with_ready_only();
    let value = with_agent_addr(&address, || {
        route_snapshot_json(
            ParentRouteId::Devices,
            None,
            TestContext("parent route snapshot serializes after agent timeout"),
        )
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured LAN load command arrives before response timeout",
    );

    assert_eq!(
        request.command["command"],
        json!("agent.lan-pairing.status.get")
    );
    assert_eq!(value["connectionState"], "error");
    assert_eq!(value["commandEnabled"], false);
    assert_eq!(value["dataSource"], "unavailable");
    assert_eq!(value["summary"]["household"], "unavailable");
    assert!(value["liveActivity"]["lanAddDeviceReadModel"].is_null());
}

#[test]
fn devices_route_load_uses_passive_status_get_with_default_origin() {
    let (address, capture) = start_lan_local_server_with_capture(
        AgentEventName::AgentLanPairingStatusReported,
        sample_lan_read_model(),
    );
    with_agent_addr(&address, || {
        let _ = load_parent_route_snapshot(ParentRouteId::Devices, None);
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured LAN load command arrives",
    );

    assert_eq!(
        request.command["command"],
        json!("agent.lan-pairing.status.get")
    );
    assert_eq!(request.command["target"]["route"], json!("localhost"));
    assert_eq!(
        request.origin.as_deref(),
        Some(constants::bind::DEFAULT_ALLOWED_ORIGINS[0])
    );
}

#[test]
fn devices_route_load_ignores_selected_child_device_context_for_local_status_target() {
    let (address, capture) = start_lan_local_server_with_capture(
        AgentEventName::AgentLanPairingStatusReported,
        sample_lan_read_model(),
    );
    let context = ParentRouteContext {
        selected_child_device_id: ParentChildDeviceId::parse("lan-physical-mac-54271e97c331"),
    };
    with_agent_addr(&address, || {
        let _ = load_parent_route_snapshot(ParentRouteId::Devices, Some(&context));
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured LAN load command arrives when UI context carries a passive device id",
    );

    assert_eq!(
        request.command["command"],
        json!("agent.lan-pairing.status.get")
    );
    assert_eq!(request.command["target"]["route"], json!("localhost"));
    assert_eq!(
        request.command["target"]["deviceId"],
        json!(constants::lan_pairing::CHILD_DEVICE_ID)
    );
}

#[test]
fn devices_route_scan_action_uses_browser_discovery_scan_with_default_origin() {
    let (address, capture) = start_lan_local_server_with_capture(
        AgentEventName::AgentLanPairingBrowserDiscoveryReported,
        sample_lan_read_model(),
    );
    let result = with_agent_addr(&address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested,
            ParentRouteId::Devices,
        ))
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured LAN scan command arrives",
    );

    assert!(result.accepted);
    assert_eq!(
        request.command["command"],
        json!("agent.lan-pairing.browser-discovery.scan")
    );
    assert_eq!(request.command["target"]["route"], json!("local-network"));
    assert_eq!(
        request.origin.as_deref(),
        Some(constants::bind::DEFAULT_ALLOWED_ORIGINS[0])
    );
}

#[test]
fn parent_ui_action_serializes_and_returns_snapshot() {
    let (address, capture) = start_local_server_with_capture_responses(vec![
        network_flow_response_event(),
        network_runtime_event_chain_response_event(),
        policy_preview_response_event(),
    ]);
    let action = ParentUiAction {
        action: ParentUiActionKind::AgentCommandRequested,
        route: ParentRouteId::Activity,
        context: None,
        command: Some("agent.network.flow.read-model.get".to_string()),
        payload: json!({ "source": "ui" }),
    };

    let result = with_agent_addr(&address, || dispatch_parent_ui_action(&action));
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured host-bridge activity command arrives",
    );
    let runtime_stream_request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured host-bridge runtime chain command arrives",
    );
    assert!(result.accepted);
    assert_eq!(
        result
            .snapshot
            .as_ref()
            .map(|snapshot| snapshot.route.clone()),
        Some(ParentRouteId::Activity)
    );
    assert_eq!(result.events.len(), 2);
    assert_eq!(
        result
            .events
            .first()
            .and_then(|value| value.event.as_deref()),
        Some("agent.connection.ready")
    );
    assert_eq!(
        result
            .events
            .last()
            .and_then(|value| value.event.as_deref()),
        Some("agent.network.flow.read-model.reported")
    );
    assert_eq!(
        request.command["command"],
        json!("agent.network.flow.read-model.get")
    );
    assert_eq!(
        runtime_stream_request.command["command"],
        json!("agent.network.runtime.event-chain.stream.get")
    );
    assert_eq!(request.command["target"]["route"], json!("localhost"));
    assert_eq!(
        result_network_flow_row_event_id(&result),
        Some(CommandText("network-ui-flow-1".to_string()))
    );
    let live_activity = live_activity_json(
        &result,
        TestContext("network flow command returns snapshot"),
        TestContext("network flow command returns live activity snapshot"),
        TestContext("live activity snapshot serializes"),
    );
    assert_network_policy_bridge_snapshot(&live_activity, &json!(3));
}

#[test]
fn lan_agent_command_requested_for_devices_route_forwards_signed_child_observe_payload_and_replay_fields(
) {
    let (address, capture) =
        start_local_server_with_capture_responses(vec![signed_child_agent_reported_event(
            &sample_lan_read_model(),
        )]);
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
    let result = with_agent_addr(&address, || dispatch_parent_ui_action(&action));
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured LAN action command arrives",
    );

    assert!(result.accepted);
    assert_eq!(
        result.message,
        "parent Rust facade forwarded LAN agent command request"
    );
    assert_eq!(
        request.command["command"],
        json!("agent.lan-pairing.signed-child-agent.observe")
    );
    assert_eq!(
        request.command["payload"]["lanSignedChildAgentEnvelopeJson"],
        json!("{\"schemaVersion\":1}")
    );
    assert_eq!(
        request.command["target"]["deviceId"],
        json!("selected-child-android-1")
    );
    assert_eq!(
        request.origin.as_deref(),
        Some(constants::bind::DEFAULT_ALLOWED_ORIGINS[0])
    );
    assert_eq!(
        lan_add_device_discovery_source(&result),
        Some(CommandText("physical-household-lan".to_string()))
    );
    assert_eq!(result.events.len(), 2);
    assert_eq!(
        last_event_name(&result),
        Some(CommandText(
            "agent.lan-pairing.signed-child-agent.reported".to_string(),
        ))
    );
    assert_eq!(
        last_event_payload_field(
            &result,
            TestLabel(constants::field::LAN_SIGNED_CHILD_AGENT_VERIFICATION),
        ),
        Some(&json!(
            constants::value::LAN_SIGNED_CHILD_AGENT_VERIFICATION_ACCEPTED
        ))
    );
    assert_eq!(
        last_event_payload_field(
            &result,
            TestLabel(constants::field::LAN_SIGNED_CHILD_AGENT_REPLAY_OBSERVED_COUNT),
        ),
        Some(&json!(1.0))
    );
}

#[test]
fn lan_scan_action_returns_bounded_error_when_response_is_unavailable() {
    let (address, capture) = start_local_server_with_ready_only();
    let result = with_agent_addr(&address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested,
            ParentRouteId::Devices,
        ))
    });
    let request = require_ok(
        capture.recv_timeout(Duration::from_secs(1)),
        "captured LAN action command arrives before response timeout",
    );

    assert!(!result.accepted);
    assert_eq!(result.connection_state, ParentBridgeConnectionState::Error);
    assert!(result
        .message
        .contains("agent-service WebSocket command-response"));
    assert!(result.message.contains("timed out") || result.message.contains("failed"));
    assert_eq!(result.events.len(), 0);
    assert_eq!(
        request.command["command"],
        json!("agent.lan-pairing.browser-discovery.scan")
    );
    assert_eq!(
        result
            .snapshot
            .as_ref()
            .map(|snapshot| snapshot.data_source.clone()),
        Some(ParentRouteDataSource::Unavailable)
    );
    assert_snapshot_field_is_none(&result.snapshot, |live_activity| {
        live_activity.lan_add_device_read_model.is_some()
    });
}

#[test]
fn product_bridge_actions_return_route_snapshots_without_invented_overlay_data() {
    let address = start_lan_local_server(
        AgentEventName::AgentLanPairingBrowserDiscoveryReported,
        sample_lan_read_model(),
    );
    let (ready_only_address, _ready_only_capture) = start_local_server_with_ready_only();
    let lan_scan = with_agent_addr(&address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested,
            ParentRouteId::Devices,
        ))
    });
    let network_refresh = with_agent_addr(&ready_only_address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::NetworkFlowReadModelRefreshRequested,
            ParentRouteId::Activity,
        ))
    });
    let tracking_retention_write = with_agent_addr(&ready_only_address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::TrackingRetentionSettingsWriteRequested,
            ParentRouteId::Activity,
        ))
    });
    let app_game_dispatch_execute = with_agent_addr(&ready_only_address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::AppGameAdapterDispatchExecuteRequested,
            ParentRouteId::AppGameSessions,
        ))
    });
    let app_game_timer_parent_preference_setup = with_agent_addr(&ready_only_address, || {
        dispatch_parent_ui_action(&empty_action(
            ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested,
            ParentRouteId::AppGameSessions,
        ))
    });

    assert_lan_scan_snapshot(&lan_scan);
    assert_snapshot_field_is_none(&network_refresh.snapshot, |live_activity| {
        live_activity.network_flow_read_model.is_some()
    });
    assert_snapshot_field_is_none(&tracking_retention_write.snapshot, |live_activity| {
        live_activity
            .activity_tracking_retention_settings_write_result
            .is_some()
    });
    assert_app_game_snapshots_are_empty(
        &app_game_dispatch_execute,
        &app_game_timer_parent_preference_setup,
    );
}

fn result_network_flow_row_event_id(result: &super::ParentUiActionResult) -> Option<CommandText> {
    result
        .snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.live_activity.as_ref())
        .and_then(|live_activity| live_activity.network_flow_read_model.as_ref())
        .and_then(|read_model| read_model.rows.first())
        .map(|row| CommandText(row.event_id.to_string()))
}

fn lan_add_device_discovery_source(result: &super::ParentUiActionResult) -> Option<CommandText> {
    result
        .snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.live_activity.as_ref())
        .and_then(|live_activity| live_activity.lan_add_device_read_model.as_ref())
        .map(|read_model| CommandText(read_model.discovery_source.clone()))
}

fn last_event_name(result: &super::ParentUiActionResult) -> Option<CommandText> {
    result
        .events
        .last()
        .and_then(|value| value.event.clone())
        .map(CommandText)
}

fn empty_action(action: ParentUiActionKind, route: ParentRouteId) -> ParentUiAction {
    ParentUiAction {
        action,
        route,
        context: None,
        command: None,
        payload: json!({}),
    }
}

fn assert_history_state_label(state: LanDiscoveryEventHistoryState, expected: TestValue) {
    let address = start_lan_local_server(
        AgentEventName::AgentLanPairingStatusReported,
        sample_lan_read_model_with_history_state(state),
    );
    let value = with_agent_addr(&address, || {
        subscription_event_json(
            ParentRouteId::Devices,
            None,
            TestContext("devices subscription event serializes with explicit history state"),
        )
    });

    assert_eq!(
        value["snapshot"]["liveActivity"]["lanAddDeviceReadModel"]["discoveryEventHistory"]
            ["state"],
        json!(expected.0)
    );
}
