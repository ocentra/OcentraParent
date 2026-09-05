use crate::support::ValueOrUnreachable as _;
use ocentra_schema::parent_ui_bridge::{
    ParentAppGameNotificationParentSurfacePanelRowSnapshot,
    ParentAppGameNotificationParentSurfacePanelSnapshot, ParentAppGamePanelDetailSnapshot,
    ParentAppGamePanelRowSnapshot, ParentAppGamePanelSnapshot, ParentBridgeConnectionState,
    ParentDesktopDistributionSnapshot, ParentLanAddressRef,
    ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot, ParentLanDeviceId,
    ParentLanPairingDeviceRefSnapshot, ParentLanRouteId,
    ParentLanServiceIdentityProbeEvidenceSnapshot, ParentPortalParentAccessState,
    ParentPortalRowSnapshot, ParentPortalShellStatusCardId, ParentPortalShellStatusCardSnapshot,
    ParentPortalShellStatusSnapshot, ParentPortalTone, ParentRouteDataSource, ParentRouteId,
    ParentRouteLiveActivitySnapshot, ParentRoutePeerId, ParentRouteSnapshot, ParentRouteSummary,
    ParentScreenSummaryPanelDetailSnapshot, ParentScreenSummaryPanelRowSnapshot,
    ParentScreenSummaryPanelSnapshot, PARENT_UI_BRIDGE_SCHEMA_VERSION,
};

pub(super) fn route_snapshot(route: ParentRouteId) -> ParentRouteSnapshot {
    let portal_row = ParentPortalRowSnapshot {
        label: "Device trust".to_string(),
        order: 1,
        signal_score: 92,
        ready_count: 3,
        gap_count: 0,
        primary_area: "Devices".to_string(),
        trend: "stable".to_string(),
        tone: ParentPortalTone::Cyan,
    };
    let shell_card = ParentPortalShellStatusCardSnapshot {
        id: ParentPortalShellStatusCardId::parse("runtime")
            .value_or_unreachable(crate::assert_context!("card id must be non-empty")),
        label: "Runtime".to_string(),
        value: "connected".to_string(),
        detail: "Rust parent runtime snapshot".to_string(),
        tone: ParentPortalTone::Gold,
    };
    let shell_status = ParentPortalShellStatusSnapshot {
        route_label: "Devices".to_string(),
        parent_access_state: ParentPortalParentAccessState::ActiveController,
        global_connection_state: "connected".to_string(),
        route_capability_state: "available".to_string(),
        data_source_label: "Rust read model".to_string(),
        cards: vec![shell_card],
    };

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
        service_health: None,
        parent_desktop_distribution: None,
        diagnostic_panels_enabled: false,
        parent_portal_rows: Some(vec![portal_row]),
        parent_portal_shell_status: Some(shell_status),
        live_activity: None,
        browser_panels: None,
        setup_first_run_panel: None,
        screen_settings_service_response: None,
    }
}

pub(super) fn parent_desktop_distribution_snapshot() -> ParentDesktopDistributionSnapshot {
    ParentDesktopDistributionSnapshot {
        payload_source: "rust-parent-runtime".to_string(),
        source_custody_state: "source-custody-manual-required".to_string(),
        product_claim_state: "read-only-contract-status-no-execution-owner".to_string(),
        no_claim: "no-installer-updater-rollback-signing-notarization-store-execution".to_string(),
        package_frontend_state: "built-portal-dist".to_string(),
        package_service_manager_state: "package-installs-auto-start-service".to_string(),
        package_health_probe_state: "package-health-probe-required".to_string(),
        package_preview_state: "unsigned-package-preview".to_string(),
        update_channel_state: "update-channel-scaffold".to_string(),
        rollback_state: "rollback-unavailable".to_string(),
        signing_state: "signing-manual-required".to_string(),
        notarization_state: "notarization-manual-required".to_string(),
        store_distribution_state: "store-distribution-manual-required".to_string(),
        platform_matrix_state: "platform-matrix-split-proof-rows".to_string(),
        release_branch_state: "production-promotion-required".to_string(),
        artifact_proof_state: "ci-package-preview-artifact-proof".to_string(),
        actions_available: false,
    }
}

pub(super) fn route_live_activity_snapshot() -> ParentRouteLiveActivitySnapshot {
    ParentRouteLiveActivitySnapshot {
        recent_summary: None,
        ingest_status: None,
        activity_screen_read_model: None,
        activity_app_use_read_model: None,
        activity_app_game_platform_extension_read_model: Some(platform_extension_read_model()),
        activity_browser_read_model: None,
        activity_games_read_model: None,
        activity_tracking_panel: None,
        screen_summary_panel: Some(screen_summary_panel()),
        browser_inventory_event: None,
        browser_inventory_read_model: None,
        browser_evidence_event: None,
        browser_evidence_read_model: None,
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
            app_game_notification_parent_surface_panel(),
        ),
        app_game_policy_readiness_panel: Some(app_game_policy_readiness_panel()),
        app_game_platform_proof_status_panel: Some(app_game_platform_proof_status_panel()),
        app_game_child_runtime_transport_receipt_panel: Some(
            app_game_child_runtime_transport_receipt_panel(),
        ),
        app_game_adapter_dispatch_panel: None,
        app_game_timer_parent_surface_panel: None,
        browser_intervention_event: None,
        browser_intervention_read_model: None,
        activity_tracking_read_model_event: None,
        activity_tracking_read_model: None,
        activity_tracking_retention_settings_write_result: None,
    }
}

fn platform_extension_read_model() -> serde_json::Value {
    serde_json::json!({
        "ok": true,
        "value": {
            "rows": [{"platform": "macos"}]
        }
    })
}

fn screen_summary_panel() -> ParentScreenSummaryPanelSnapshot {
    ParentScreenSummaryPanelSnapshot {
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
    }
}

fn app_game_notification_parent_surface_panel(
) -> ParentAppGameNotificationParentSurfacePanelSnapshot {
    ParentAppGameNotificationParentSurfacePanelSnapshot {
        eyebrow: "Runtime reference".to_string(),
        title: "App/game notification parent surface".to_string(),
        body: "Parent-safe app/game notification parent surface summary.".to_string(),
        state: "ready".to_string(),
        summary: "1 parent-surface intent rows".to_string(),
        product_claim: "Parent-visible evidence only.".to_string(),
        metrics: vec![ParentAppGamePanelDetailSnapshot {
            label: "Rows returned".to_string(),
            value: "1".to_string(),
        }],
        rows: vec![ParentAppGameNotificationParentSurfacePanelRowSnapshot {
            key: "surface-row-1".to_string(),
            title: "surface-row-1".to_string(),
            details: vec![ParentAppGamePanelDetailSnapshot {
                label: "Status".to_string(),
                value: "ready".to_string(),
            }],
        }],
        empty_message: "No data".to_string(),
    }
}

fn app_game_policy_readiness_panel() -> ParentAppGamePanelSnapshot {
    ParentAppGamePanelSnapshot {
        eyebrow: "Policy readiness".to_string(),
        title: "App/game policy readiness".to_string(),
        body: "Policy readiness summary.".to_string(),
        load_state: "ready".to_string(),
        summary_details: vec![ParentAppGamePanelDetailSnapshot {
            label: "Status".to_string(),
            value: "ready".to_string(),
        }],
        rows: vec![ParentAppGamePanelRowSnapshot {
            title: "Policy evidence".to_string(),
            details: vec![ParentAppGamePanelDetailSnapshot {
                label: "Status".to_string(),
                value: "ready".to_string(),
            }],
        }],
        empty_message: "No data".to_string(),
        product_claim: "Adapter dispatch unclaimed.".to_string(),
    }
}

fn app_game_platform_proof_status_panel() -> ParentAppGamePanelSnapshot {
    ParentAppGamePanelSnapshot {
        eyebrow: "Runtime reference".to_string(),
        title: "App/game platform proof status".to_string(),
        body: "Platform proof summary.".to_string(),
        load_state: "warn".to_string(),
        summary_details: vec![ParentAppGamePanelDetailSnapshot {
            label: "Platform proofs".to_string(),
            value: "2".to_string(),
        }],
        rows: vec![ParentAppGamePanelRowSnapshot {
            title: "Windows".to_string(),
            details: vec![ParentAppGamePanelDetailSnapshot {
                label: "Status".to_string(),
                value: "ready".to_string(),
            }],
        }],
        empty_message: "No data".to_string(),
        product_claim: "Enforcement remains unclaimed.".to_string(),
    }
}

fn app_game_child_runtime_transport_receipt_panel() -> ParentAppGamePanelSnapshot {
    ParentAppGamePanelSnapshot {
        eyebrow: "Runtime reference".to_string(),
        title: "App/game child runtime transport receipts".to_string(),
        body: "Transport receipt summary.".to_string(),
        load_state: "warn".to_string(),
        summary_details: vec![ParentAppGamePanelDetailSnapshot {
            label: "Transport rows".to_string(),
            value: "1".to_string(),
        }],
        rows: vec![ParentAppGamePanelRowSnapshot {
            title: "transport-row-1".to_string(),
            details: vec![ParentAppGamePanelDetailSnapshot {
                label: "Status".to_string(),
                value: "manual-required".to_string(),
            }],
        }],
        empty_message: "No data".to_string(),
        product_claim: "Transport execution remains unclaimed.".to_string(),
    }
}

pub(super) fn browser_add_device_discovery_snapshot(
) -> ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot {
    let child_device = ParentLanPairingDeviceRefSnapshot {
        device_id: ParentLanDeviceId::parse("lan-device-1")
            .value_or_unreachable(crate::assert_context!("device id must be non-empty")),
        child_profile_id: None,
        label: "GAMEDEV".to_string(),
        platform: "windows".to_string(),
        ip_address: Some("192.168.2.42".to_string()),
        mac_address: Some("54-27-1e-97-c3-31".to_string()),
        hostname: Some("GAMEDEV".to_string()),
        network_interface: Some("Ethernet 2".to_string()),
        agent_status: Some("ocentra-service-identity-probe".to_string()),
    };
    let service_identity_probe_evidence = vec![ParentLanServiceIdentityProbeEvidenceSnapshot {
        evidence_kind: "http-status".to_string(),
        value: "200".to_string(),
    }];

    ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        discovered_at: "2026-06-26T07:20:00Z".to_string(),
        child_device,
        agent_peer_id: ParentRoutePeerId::parse("portal")
            .value_or_unreachable(crate::assert_context!("peer id must be non-empty")),
        route_id: ParentLanRouteId::parse("lan-route-local-network")
            .value_or_unreachable(crate::assert_context!("route id must be non-empty")),
        network_mode: "local-network".to_string(),
        reachability: "online".to_string(),
        address_ref: ParentLanAddressRef::parse("lan-address-ref-direct-websocket")
            .value_or_unreachable(crate::assert_context!("address ref must be non-empty")),
        discovery_status: "websocket-direct".to_string(),
        discovery_state: "discovered".to_string(),
        evidence_sources: vec!["local-service".to_string()],
        service_identity_probe_evidence,
        hint_sources: vec!["service-identity-probe".to_string()],
    }
}
