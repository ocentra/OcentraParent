use crate::support::ValueOrUnreachable as _;
use ocentra_schema::parent_ui_bridge::{
    ParentAppGameNotificationParentSurfacePanelRowSnapshot,
    ParentAppGameNotificationParentSurfacePanelSnapshot, ParentAppGamePanelDetailSnapshot,
    ParentAppGamePanelRowSnapshot, ParentAppGamePanelSnapshot, ParentBridgeConnectionState,
    ParentChildDeviceId, ParentLanAddressRef, ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot,
    ParentLanDeviceId, ParentLanPairingDeviceRefSnapshot, ParentLanRouteId,
    ParentLanServiceIdentityProbeEvidenceSnapshot, ParentPortalClipboardText,
    ParentPortalDetailValue, ParentPortalParentAccessState, ParentPortalRowSnapshot,
    ParentPortalShellStatusCardId, ParentPortalShellStatusCardSnapshot,
    ParentPortalShellStatusSnapshot, ParentPortalTone, ParentRouteContext, ParentRouteDataSource,
    ParentRouteEventCorrelationId, ParentRouteEventId, ParentRouteEventSnapshot, ParentRouteId,
    ParentRouteLiveActivitySnapshot, ParentRoutePeerId, ParentRoutePeerRole, ParentRouteSnapshot,
    ParentRouteSummary, ParentScreenSummaryPanelDetailSnapshot,
    ParentScreenSummaryPanelRowSnapshot, ParentScreenSummaryPanelSnapshot, ParentSubscriptionEvent,
    ParentTrackingStatusProofArtifact, ParentUiAction, ParentUiActionKind, ParentUiActionResult,
    PARENT_BRIDGE_COMMAND_DISPATCH, PARENT_BRIDGE_COMMAND_LOAD_ROUTE,
    PARENT_BRIDGE_COMMAND_SUBSCRIBE, PARENT_BRIDGE_COMMAND_UNSUBSCRIBE,
    PARENT_DEV_BRIDGE_ROUTE_DISPATCH, PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE, PARENT_ROUTE_HASH_PREFIX,
    PARENT_ROUTE_HASH_QUERY_SEPARATOR, PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX,
    PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS, PARENT_UI_BRIDGE_SCHEMA_VERSION,
};
use ocentra_schema::parent_ui_bridge_ts::{
    parent_ui_bridge_typescript, portal_contracts_typescript,
};
use serde_json::json;

const EXPECTED_PARENT_DEV_BRIDGE_URL_ENV_KEY: &str = "VITE_PARENT_DEV_BRIDGE_URL";
const EXPECTED_PARENT_TAURI_INTERNAL_WINDOW_KEY: &str = "__TAURI_INTERNALS__";
const EXPECTED_PARENT_TYPEOF_UNDEFINED: &str = "undefined";

fn shell_card_id(value: &str) -> ParentPortalShellStatusCardId {
    ParentPortalShellStatusCardId::parse(value).value_or_unreachable("card id must be non-empty")
}

fn portal_detail_value(value: &str) -> ParentPortalDetailValue {
    ParentPortalDetailValue::parse(value).value_or_unreachable("detail value must be non-empty")
}

fn generated_line<'a>(generated: &'a str, line_start: &str) -> &'a str {
    generated
        .lines()
        .find(|line| line.trim_start().starts_with(line_start))
        .value_or_unreachable("expected generated line to exist")
}

fn generated_section<'a>(generated: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = generated
        .find(start)
        .value_or_unreachable("expected generated section start to exist");
    let section_tail = &generated[start_index..];
    let end_index = section_tail
        .find(end)
        .value_or_unreachable("expected generated section end to exist")
        + end.len();
    &section_tail[..end_index]
}

fn route_snapshot(route: ParentRouteId) -> ParentRouteSnapshot {
    ParentRouteSnapshot {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        route,
        generated_at: "2026-06-26T07:20:00Z".to_string(),
        season_label: "LOCAL".to_string(),
        last_updated: "2026-06-26T07:20:01Z".to_string(),
        connection_state: ParentBridgeConnectionState::Connected,
        command_enabled: true,
        agent_endpoint: "host-bridge://parent-runtime".to_string(),
        data_source: ParentRouteDataSource::RustReadModel,
        summary: ParentRouteSummary {
            title: "Devices".to_string(),
            route_capability: "available".to_string(),
            parent_access: "active-controller".to_string(),
            household: "household-alpha".to_string(),
            child_device: "device-alpha".to_string(),
        },
        diagnostic_panels_enabled: false,
        parent_portal_rows: Some(vec![ParentPortalRowSnapshot {
            label: "Device trust".to_string(),
            order: 1,
            signal_score: 92,
            ready_count: 3,
            gap_count: 0,
            primary_area: "Devices".to_string(),
            trend: "stable".to_string(),
            tone: ParentPortalTone::Cyan,
        }]),
        parent_portal_shell_status: Some(ParentPortalShellStatusSnapshot {
            route_label: "Devices".to_string(),
            parent_access_state: ParentPortalParentAccessState::ActiveController,
            global_connection_state: "connected".to_string(),
            route_capability_state: "available".to_string(),
            data_source_label: "Rust read model".to_string(),
            cards: vec![ParentPortalShellStatusCardSnapshot {
                id: shell_card_id("runtime"),
                label: "Runtime".to_string(),
                value: "connected".to_string(),
                detail: "Rust parent runtime snapshot".to_string(),
                tone: ParentPortalTone::Gold,
            }],
        }),
        live_activity: None,
        browser_panels: None,
        setup_first_run_panel: None,
        screen_settings_service_response: None,
    }
}

fn sample_app_game_panel_detail(label: &str, value: &str) -> ParentAppGamePanelDetailSnapshot {
    ParentAppGamePanelDetailSnapshot {
        label: label.to_string(),
        value: value.to_string(),
    }
}

fn route_live_activity_snapshot() -> ParentRouteLiveActivitySnapshot {
    ParentRouteLiveActivitySnapshot {
        recent_summary: None,
        ingest_status: None,
        activity_screen_read_model: None,
        activity_tracking_panel: None,
        screen_summary_panel: Some(ParentScreenSummaryPanelSnapshot {
            eyebrow: "Activity kind".to_string(),
            title: "Screen analysis".to_string(),
            body: "Stored activity".to_string(),
            load_state: "Ready".to_string(),
            summary_details: vec![
                ParentScreenSummaryPanelDetailSnapshot {
                    label: "Status".to_string(),
                    value: "Ready".to_string(),
                },
                ParentScreenSummaryPanelDetailSnapshot {
                    label: "Rows returned".to_string(),
                    value: "1".to_string(),
                },
            ],
            rows: vec![ParentScreenSummaryPanelRowSnapshot {
                title: "screen-ready-row".to_string(),
                details: vec![ParentScreenSummaryPanelDetailSnapshot {
                    label: "Reason".to_string(),
                    value: "Ready".to_string(),
                }],
            }],
            empty_message: "No recent activity is available yet.".to_string(),
            product_claim: "No family setting is configured for this area yet.".to_string(),
        }),
        browser_managed_event: None,
        browser_managed_status: None,
        local_ai_runtime_status_event: None,
        lan_ai_job_event: None,
        parent_assistant_boundary_event: None,
        activity_memory_graph_read_model: None,
        network_flow_event: None,
        network_flow_read_model: None,
        network_evidence_summary: None,
        network_runtime_event_chain_stream: None,
        lan_pairing_browser_discovery_event: None,
        lan_add_device_read_model: None,
        policy_preview_panel: None,
        app_game_notification_parent_surface_panel: Some(
            ParentAppGameNotificationParentSurfacePanelSnapshot {
                eyebrow: "Runtime reference".to_string(),
                title: "App/game notification parent surface".to_string(),
                body: "Parent-safe app/game notification parent surface summary.".to_string(),
                state: "ready".to_string(),
                summary: "1 parent-surface intent rows".to_string(),
                product_claim: "Parent-visible evidence only.".to_string(),
                metrics: vec![sample_app_game_panel_detail("Rows returned", "1")],
                rows: vec![ParentAppGameNotificationParentSurfacePanelRowSnapshot {
                    key: "surface-row-1".to_string(),
                    title: "surface-row-1".to_string(),
                    details: vec![sample_app_game_panel_detail("Status", "ready")],
                }],
                empty_message: "No data".to_string(),
            },
        ),
        app_game_policy_readiness_panel: Some(ParentAppGamePanelSnapshot {
            eyebrow: "Policy readiness".to_string(),
            title: "App/game policy readiness".to_string(),
            body: "Policy readiness summary.".to_string(),
            load_state: "ready".to_string(),
            summary_details: vec![sample_app_game_panel_detail("Status", "ready")],
            rows: vec![ParentAppGamePanelRowSnapshot {
                title: "Policy evidence".to_string(),
                details: vec![sample_app_game_panel_detail("Status", "ready")],
            }],
            empty_message: "No data".to_string(),
            product_claim: "Adapter dispatch unclaimed.".to_string(),
        }),
        app_game_platform_proof_status_panel: Some(ParentAppGamePanelSnapshot {
            eyebrow: "Runtime reference".to_string(),
            title: "App/game platform proof status".to_string(),
            body: "Platform proof summary.".to_string(),
            load_state: "warn".to_string(),
            summary_details: vec![sample_app_game_panel_detail("Platform proofs", "2")],
            rows: vec![ParentAppGamePanelRowSnapshot {
                title: "Windows".to_string(),
                details: vec![sample_app_game_panel_detail("Status", "ready")],
            }],
            empty_message: "No data".to_string(),
            product_claim: "Enforcement remains unclaimed.".to_string(),
        }),
        app_game_child_runtime_transport_receipt_panel: Some(ParentAppGamePanelSnapshot {
            eyebrow: "Runtime reference".to_string(),
            title: "App/game child runtime transport receipts".to_string(),
            body: "Transport receipt summary.".to_string(),
            load_state: "warn".to_string(),
            summary_details: vec![sample_app_game_panel_detail("Transport rows", "1")],
            rows: vec![ParentAppGamePanelRowSnapshot {
                title: "transport-row-1".to_string(),
                details: vec![sample_app_game_panel_detail("Status", "manual-required")],
            }],
            empty_message: "No data".to_string(),
            product_claim: "Transport execution remains unclaimed.".to_string(),
        }),
        app_game_adapter_dispatch_panel: None,
        app_game_timer_parent_surface_panel: None,
        browser_intervention_event: None,
        browser_intervention_read_model: None,
        activity_tracking_read_model_event: None,
        activity_tracking_read_model: None,
        activity_tracking_retention_settings_write_result: None,
    }
}

#[test]
fn portal_text_values_are_rust_owned_non_empty_bridge_contracts() {
    let detail = portal_detail_value("not reported");
    let clipboard = ParentPortalClipboardText::parse("diagnostics export")
        .value_or_unreachable("clipboard text must be non-empty");
    let proof_artifact = ParentTrackingStatusProofArtifact::parse("tracking-proof-artifact")
        .value_or_unreachable("tracking proof artifact must be non-empty");

    assert_eq!(detail.as_str(), "not reported");
    assert_eq!(clipboard.as_str(), "diagnostics export");
    assert_eq!(proof_artifact.as_str(), "tracking-proof-artifact");
    assert!(ParentPortalDetailValue::parse("").is_none());
    assert!(ParentPortalClipboardText::parse("   ").is_none());
    assert!(ParentTrackingStatusProofArtifact::parse("").is_none());
    assert_eq!(
        serde_json::to_value(&detail).value_or_unreachable("detail serializes"),
        json!("not reported")
    );

    let decoded: ParentPortalDetailValue =
        serde_json::from_value(json!("not reported")).value_or_unreachable("detail deserializes");
    assert_eq!(decoded, detail);
}

#[test]
fn route_snapshot_preserves_rust_owned_encoded_shape() {
    let snapshot = route_snapshot(ParentRouteId::Devices);

    let encoded = serde_json::to_value(&snapshot).value_or_unreachable("snapshot must serialize");

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

    let decoded: ParentRouteSnapshot =
        serde_json::from_value(encoded).value_or_unreachable("encoded snapshot must round-trip");
    assert_eq!(decoded, snapshot);
}

#[test]
fn route_live_activity_snapshot_preserves_rust_owned_app_game_panel_shapes() {
    let snapshot = route_live_activity_snapshot();

    let encoded =
        serde_json::to_value(&snapshot).value_or_unreachable("live activity must serialize");

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

    let decoded: ParentRouteLiveActivitySnapshot =
        serde_json::from_value(encoded).value_or_unreachable("live activity must round-trip");
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

    let action_encoded =
        serde_json::to_value(&action).value_or_unreachable("action must serialize");
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
    let decoded_action: ParentUiAction =
        serde_json::from_value(action_encoded).value_or_unreachable("action must deserialize");
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
    let result_value = serde_json::to_value(&result).value_or_unreachable("result must serialize");
    assert_eq!(
        result_value["schemaVersion"],
        json!(PARENT_UI_BRIDGE_SCHEMA_VERSION)
    );
    assert_eq!(result_value["connectionState"], json!("connected"));
    let decoded_result: ParentUiActionResult =
        serde_json::from_value(result_value).value_or_unreachable("result must deserialize");
    assert_eq!(decoded_result, result);

    let subscription = ParentSubscriptionEvent {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        route: ParentRouteId::PolicyScreen,
        snapshot,
        events: Some(vec![ParentRouteEventSnapshot {
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
        }]),
    };
    let subscription_value =
        serde_json::to_value(&subscription).value_or_unreachable("subscription must serialize");
    assert_eq!(subscription_value["route"], json!("policy-screen"));
    assert_eq!(
        subscription_value["events"][0]["eventId"],
        json!("agent.connection.ready-1")
    );
    let decoded_subscription: ParentSubscriptionEvent = serde_json::from_value(subscription_value)
        .value_or_unreachable("subscription must deserialize");
    assert_eq!(decoded_subscription, subscription);
}

#[test]
fn browser_add_device_discovery_snapshot_serializes_probe_evidence_shape() {
    let snapshot = ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        discovered_at: "2026-06-26T07:20:00Z".to_string(),
        child_device: ParentLanPairingDeviceRefSnapshot {
            device_id: ParentLanDeviceId::parse("lan-device-1")
                .value_or_unreachable("device id must be non-empty"),
            child_profile_id: None,
            label: "GAMEDEV".to_string(),
            platform: "windows".to_string(),
            ip_address: Some("192.168.2.42".to_string()),
            mac_address: Some("54-27-1e-97-c3-31".to_string()),
            hostname: Some("GAMEDEV".to_string()),
            network_interface: Some("Ethernet 2".to_string()),
            agent_status: Some("ocentra-service-identity-probe".to_string()),
        },
        agent_peer_id: ParentRoutePeerId::parse("portal")
            .value_or_unreachable("peer id must be non-empty"),
        route_id: ParentLanRouteId::parse("lan-route-local-network")
            .value_or_unreachable("route id must be non-empty"),
        network_mode: "local-network".to_string(),
        reachability: "online".to_string(),
        address_ref: ParentLanAddressRef::parse("lan-address-ref-direct-websocket")
            .value_or_unreachable("address ref must be non-empty"),
        discovery_status: "websocket-direct".to_string(),
        discovery_state: "discovered".to_string(),
        evidence_sources: vec!["local-service".to_string()],
        service_identity_probe_evidence: vec![ParentLanServiceIdentityProbeEvidenceSnapshot {
            evidence_kind: "http-status".to_string(),
            value: "200".to_string(),
        }],
        hint_sources: vec!["service-identity-probe".to_string()],
    };

    let encoded = serde_json::to_value(&snapshot).value_or_unreachable("snapshot must serialize");

    assert_eq!(
        encoded["serviceIdentityProbeEvidence"][0]["evidenceKind"],
        json!("http-status")
    );
    assert_eq!(
        encoded["serviceIdentityProbeEvidence"][0]["value"],
        json!("200")
    );
}

#[test]
fn generated_typescript_bridge_uses_rust_constants() {
    let generated = parent_ui_bridge_typescript();

    assert_eq!(
        generated_line(&generated, "readonly networkEvidenceGrade?:"),
        "  readonly networkEvidenceGrade?: string | null;"
    );

    for stale_token in [
        "__PARENT_BRIDGE_COMMAND_LOAD_ROUTE__",
        "__PARENT_BRIDGE_COMMAND_DISPATCH__",
        "__PARENT_BRIDGE_COMMAND_SUBSCRIBE__",
        "__PARENT_BRIDGE_COMMAND_UNSUBSCRIBE__",
        "__PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE__",
        "__PARENT_DEV_BRIDGE_ROUTE_DISPATCH__",
        "__PARENT_DEV_BRIDGE_URL_ENV_KEY__",
        "__PARENT_ROUTE_HASH_PREFIX__",
        "__PARENT_ROUTE_HASH_QUERY_SEPARATOR__",
        "__PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX__",
        "__PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS__",
        "__PARENT_TAURI_INTERNAL_WINDOW_KEY__",
        "__PARENT_TYPEOF_UNDEFINED__",
        "__PARENT_UI_BRIDGE_SCHEMA_VERSION__",
    ] {
        assert_eq!(
            generated.find(stale_token),
            None,
            "generated bridge must replace template token {stale_token}"
        );
    }

    assert_eq!(
        generated_section(
            &generated,
            "export const ParentBridgeCommand = {",
            "} as const;"
        ),
        format!(
            "export const ParentBridgeCommand = {{\n  LoadRoute: '{PARENT_BRIDGE_COMMAND_LOAD_ROUTE}',\n  Dispatch: '{PARENT_BRIDGE_COMMAND_DISPATCH}',\n  Subscribe: '{PARENT_BRIDGE_COMMAND_SUBSCRIBE}',\n  Unsubscribe: '{PARENT_BRIDGE_COMMAND_UNSUBSCRIBE}',\n}} as const;"
        )
    );
    assert_eq!(
        generated_section(
            &generated,
            "export const ParentDevBridgeRoute = {",
            "} as const;"
        ),
        format!(
            "export const ParentDevBridgeRoute = {{\n  LoadRoute: '{PARENT_DEV_BRIDGE_ROUTE_LOAD_ROUTE}',\n  Dispatch: '{PARENT_DEV_BRIDGE_ROUTE_DISPATCH}',\n}} as const;"
        )
    );
    assert_eq!(
        generated_line(&generated, "SchemaVersion:"),
        format!("  SchemaVersion: {PARENT_UI_BRIDGE_SCHEMA_VERSION},")
    );
    assert_eq!(
        generated_line(&generated, "DevRouteSubscriptionPollMs:"),
        format!("  DevRouteSubscriptionPollMs: {PARENT_ROUTE_SUBSCRIPTION_POLL_INTERVAL_MS},")
    );
    assert_eq!(
        generated_line(&generated, "RouteSubscriptionEventPrefix:"),
        format!("  RouteSubscriptionEventPrefix: '{PARENT_ROUTE_SUBSCRIPTION_EVENT_PREFIX}',")
    );
    assert_eq!(
        generated_line(&generated, "RouteHashPrefix:"),
        format!("  RouteHashPrefix: '{PARENT_ROUTE_HASH_PREFIX}',")
    );
    assert_eq!(
        generated_line(&generated, "RouteHashQuerySeparator:"),
        format!("  RouteHashQuerySeparator: '{PARENT_ROUTE_HASH_QUERY_SEPARATOR}',")
    );
    assert_eq!(
        generated_line(&generated, "TypeofUndefined:"),
        format!("  TypeofUndefined: '{EXPECTED_PARENT_TYPEOF_UNDEFINED}',")
    );
    assert_eq!(
        generated_line(&generated, "DevBridgeUrlEnvKey:"),
        format!("  DevBridgeUrlEnvKey: '{EXPECTED_PARENT_DEV_BRIDGE_URL_ENV_KEY}',")
    );
    assert_eq!(
        generated_line(&generated, "TauriInternalWindowKey:"),
        format!("  TauriInternalWindowKey: '{EXPECTED_PARENT_TAURI_INTERNAL_WINDOW_KEY}',")
    );
}

#[test]
fn route_titles_and_dev_diagnostics_are_rust_owned_bridge_metadata() {
    let generated = parent_ui_bridge_typescript();
    assert_eq!(
        generated_section(
            &generated,
            "export const ParentRoutes: readonly ParentRouteId[] = [",
            "] as const;"
        ),
        "export const ParentRoutes: readonly ParentRouteId[] = [\n  ParentRoute.Overview,\n  ParentRoute.Assistant,\n  ParentRoute.Start,\n  ParentRoute.Activity,\n  ParentRoute.Browser,\n  ParentRoute.BrowserSettings,\n  ParentRoute.Policy,\n  ParentRoute.PolicyApps,\n  ParentRoute.PolicyGames,\n  ParentRoute.PolicyScreen,\n  ParentRoute.PolicyNetwork,\n  ParentRoute.PolicyTracking,\n  ParentRoute.PolicyRemoteScreen,\n  ParentRoute.RuleManagement,\n  ParentRoute.Schedules,\n  ParentRoute.Approvals,\n  ParentRoute.Enforcement,\n  ParentRoute.PrivacyDesign,\n  ParentRoute.Memory,\n  ParentRoute.MemorySettings,\n  ParentRoute.AiGuide,\n  ParentRoute.AiRuntime,\n  ParentRoute.ApiProviders,\n  ParentRoute.ReportsGuide,\n  ParentRoute.ScreenAnalysis,\n  ParentRoute.AppGameSessions,\n  ParentRoute.NetworkActivity,\n  ParentRoute.Devices,\n  ParentRoute.LanPairing,\n  ParentRoute.CapabilityStatus,\n  ParentRoute.Notifications,\n  ParentRoute.NotificationChannels,\n  ParentRoute.DriveConnections,\n  ParentRoute.ExportRetention,\n  ParentRoute.RemoteAccess,\n  ParentRoute.ReportCompiler,\n  ParentRoute.AuditHistory,\n  ParentRoute.Subscription,\n  ParentRoute.Entitlements,\n  ParentRoute.PlatformsInstall,\n  ParentRoute.InstallUpdates,\n  ParentRoute.Diagnostics,\n  ParentRoute.ProofPanels,\n  ParentRoute.SettingsRules,\n  ParentRoute.AppLayout,\n  ParentRoute.FrameTuner,\n  ParentRoute.Commands,\n  ParentRoute.Events,\n  ParentRoute.Logs,\n] as const;"
    );
    assert_eq!(
        generated_line(&generated, "[ParentRoute.Devices]:"),
        "  [ParentRoute.Devices]: 'Devices',"
    );
    assert_eq!(
        generated_line(&generated, "[ParentRoute.AppLayout]:"),
        "  [ParentRoute.AppLayout]: 'App layout',"
    );
    assert_eq!(
        generated_line(&generated, "[ParentRoute.FrameTuner]:"),
        "  [ParentRoute.FrameTuner]: 'Frame tuner',"
    );
    assert_eq!(
        generated_section(
            &generated,
            "export const ParentDevDiagnosticRoutes: readonly ParentRouteId[] = [",
            "] as const;"
        ),
        "export const ParentDevDiagnosticRoutes: readonly ParentRouteId[] = [\n  ParentRoute.Diagnostics,\n  ParentRoute.ProofPanels,\n  ParentRoute.AppLayout,\n  ParentRoute.FrameTuner,\n  ParentRoute.Commands,\n  ParentRoute.Events,\n  ParentRoute.Logs,\n] as const;"
    );
    assert_eq!(
        generated_section(
            &generated,
            "export const ParentAppGameParentSurfaceRoutes: readonly ParentRouteId[] = [",
            "] as const;"
        ),
        "export const ParentAppGameParentSurfaceRoutes: readonly ParentRouteId[] = [\n  ParentRoute.AppGameSessions,\n] as const;"
    );
    assert_eq!(
        generated_section(
            &generated,
            "export const ParentBrowserParentSurfaceRoutes: readonly ParentRouteId[] = [",
            "] as const;"
        ),
        "export const ParentBrowserParentSurfaceRoutes: readonly ParentRouteId[] = [\n  ParentRoute.ProofPanels,\n] as const;"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export function isParentAppGameParentSurfaceRoute(route: ParentRouteId): boolean {"
        ),
        "export function isParentAppGameParentSurfaceRoute(route: ParentRouteId): boolean {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "function parentRouteMatches(route: ParentRouteId, routes: readonly ParentRouteId[]): boolean {"
        ),
        "function parentRouteMatches(route: ParentRouteId, routes: readonly ParentRouteId[]): boolean {"
    );
    assert_eq!(
        generated_section(&generated, "export const ParentRouteGroup = {", "} as const;"),
        "export const ParentRouteGroup = {\n  Monitor: 'monitor',\n  Guide: 'guide',\n  Operate: 'operate',\n  DevTools: 'dev-tools',\n} as const;"
    );
    assert_eq!(
        generated_section(
            &generated,
            "export const ParentSidebarRouteGroups: readonly ParentRouteGroupId[] = [",
            "] as const;"
        ),
        "export const ParentSidebarRouteGroups: readonly ParentRouteGroupId[] = [\n  ParentRouteGroup.Monitor,\n  ParentRouteGroup.Guide,\n  ParentRouteGroup.Operate,\n] as const;"
    );
    assert_eq!(
        generated_line(
            &generated,
            "[ParentRoute.Overview]: { route: ParentRoute.Overview, group: ParentRouteGroup.Monitor, sidebar: true },"
        ),
        "  [ParentRoute.Overview]: { route: ParentRoute.Overview, group: ParentRouteGroup.Monitor, sidebar: true },"
    );
    assert_eq!(
        generated_line(
            &generated,
            "[ParentRoute.AppLayout]: { route: ParentRoute.AppLayout, group: ParentRouteGroup.DevTools, sidebar: false },"
        ),
        "  [ParentRoute.AppLayout]: { route: ParentRoute.AppLayout, group: ParentRouteGroup.DevTools, sidebar: false },"
    );
    assert_eq!(
        generated_line(
            &generated,
            "[ParentRoute.FrameTuner]: { route: ParentRoute.FrameTuner, group: ParentRouteGroup.DevTools, sidebar: false },"
        ),
        "  [ParentRoute.FrameTuner]: { route: ParentRoute.FrameTuner, group: ParentRouteGroup.DevTools, sidebar: false },"
    );
    assert_eq!(
        generated_section(
            &generated,
            "export const ParentSidebarRoutes: readonly ParentRouteId[] = ParentRoutes.filter(",
            ");"
        ),
        "export const ParentSidebarRoutes: readonly ParentRouteId[] = ParentRoutes.filter(\n  (route) => ParentRouteMetadata[route].sidebar\n);"
    );
}

#[test]
fn generated_typescript_carries_rust_owned_portal_text_value_edges() {
    let generated = parent_ui_bridge_typescript();

    assert_eq!(
        generated_line(&generated, "export type ParentPortalDetailValue = string;"),
        "export type ParentPortalDetailValue = string;"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export type ParentPortalClipboardText = string;"
        ),
        "export type ParentPortalClipboardText = string;"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export type ParentTrackingStatusProofArtifact = string;"
        ),
        "export type ParentTrackingStatusProofArtifact = string;"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export function decodeParentPortalDetailValue(value: string): ParentPortalDetailValue {"
        ),
        "export function decodeParentPortalDetailValue(value: string): ParentPortalDetailValue {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export function decodeParentPortalClipboardText(value: string): ParentPortalClipboardText {"
        ),
        "export function decodeParentPortalClipboardText(value: string): ParentPortalClipboardText {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export function decodeParentTrackingStatusProofArtifact("
        ),
        "export function decodeParentTrackingStatusProofArtifact("
    );
    assert_eq!(
        generated_line(
            &generated,
            "throw new TypeError(`${field} must be non-empty`);"
        ),
        "    throw new TypeError(`${field} must be non-empty`);"
    );
}

#[test]
fn generated_typescript_artifact_stays_checked_in() {
    let checked_in = include_str!("../../../../apps/portal/generated/parent-ui-bridge.ts");
    let generated = parent_ui_bridge_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(&generated, "export interface ParentRouteSnapshot"),
        "export interface ParentRouteSnapshot {"
    );
    assert_eq!(
        generated_line(&generated, "export interface ParentUiAction"),
        "export interface ParentUiAction {"
    );
    assert_eq!(
        generated_line(&generated, "export interface ParentUiActionResult"),
        "export interface ParentUiActionResult {"
    );
    assert_eq!(
        generated_line(&generated, "export interface ParentSubscriptionEvent"),
        "export interface ParentSubscriptionEvent {"
    );
}

#[test]
fn generated_schema_domain_portal_contracts_artifact_stays_checked_in() {
    let checked_in =
        include_str!("../../../../packages/schema-domain/src/generated/portal-contracts.ts");
    let generated = portal_contracts_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated_line(&generated, "export const GeneratedPortalRouteLiteral = {"),
        "export const GeneratedPortalRouteLiteral = {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export const GeneratedPortalConnectionState = {"
        ),
        "export const GeneratedPortalConnectionState = {"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export type GeneratedPortalDetailValue = string;"
        ),
        "export type GeneratedPortalDetailValue = string;"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export type GeneratedPortalClipboardText = string;"
        ),
        "export type GeneratedPortalClipboardText = string;"
    );
    assert_eq!(
        generated_line(
            &generated,
            "export type GeneratedTrackingStatusProofArtifact = string;"
        ),
        "export type GeneratedTrackingStatusProofArtifact = string;"
    );
}

#[test]
fn schema_domain_portal_contracts_adapter_stays_generated_backed() {
    let adapter = include_str!("../../../../packages/schema-domain/src/portal-contracts.ts");

    assert_eq!(
        adapter
            .lines()
            .find(|line| line.contains("from './generated/portal-contracts'"))
            .value_or_unreachable("expected generated line to exist"),
        "} from './generated/portal-contracts';"
    );
    assert_eq!(
        generated_line(
            adapter,
            "export const PortalRouteLiteral = GeneratedPortalRouteLiteral;"
        ),
        "export const PortalRouteLiteral = GeneratedPortalRouteLiteral;"
    );
    assert_eq!(
        generated_line(
            adapter,
            "export const PortalRouteHashPrefix = GeneratedPortalRouteHashPrefix;"
        ),
        "export const PortalRouteHashPrefix = GeneratedPortalRouteHashPrefix;"
    );
    assert_eq!(
        generated_line(
            adapter,
            "export const PortalConnectionStateSchema = withParser("
        ),
        "export const PortalConnectionStateSchema = withParser("
    );
    assert_eq!(
        generated_line(
            adapter,
            "export type PortalDetailValue = Infer<typeof PortalDetailValueSchema> & GeneratedPortalDetailValue;"
        ),
        "export type PortalDetailValue = Infer<typeof PortalDetailValueSchema> & GeneratedPortalDetailValue;"
    );
}
