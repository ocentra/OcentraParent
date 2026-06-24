use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_schema::parent_ui_bridge::{
    ParentBridgeConnectionState, ParentPortalParentAccessState, ParentPortalRowSnapshot,
    ParentPortalShellStatusCardSnapshot, ParentPortalShellStatusSnapshot, ParentPortalTone,
    ParentRouteContext, ParentRouteDataSource, ParentRouteId, ParentRouteLiveActivitySnapshot,
    ParentRouteSnapshot, ParentRouteSummary, ParentSubscriptionEvent, ParentUiAction,
    ParentUiActionKind, ParentUiActionResult,
};
use serde::Serialize;
use serde_json::Value;

use crate::agent_service_client::{
    dispatch_lan_agent_command, load_lan_route_snapshot, request_lan_browser_discovery_scan,
    LanAgentServiceSnapshot,
};

const PARENT_UI_BRIDGE_SCHEMA_VERSION: u16 = 1;
const EMPTY_TIMESTAMP: &str = "";
const HOST_BRIDGE_URL: &str = "host-bridge://tauri-parent";
const STATUS_LOCAL: &str = "LOCAL";
const STATUS_CONNECTING: &str = "CONNECTING";
const STATUS_CHECK_SERVICE: &str = "CHECK SERVICE";
const STATUS_OFFLINE: &str = "OFFLINE";
const LAN_DISCOVERY_REPORTED_EVENT: &str = "agent.lan-pairing.browser-discovery.reported";

enum LanRouteQuery {
    NotRequired,
    Available(LanAgentServiceSnapshot),
    Unavailable(String),
}

pub fn load_parent_route_snapshot(
    route: ParentRouteId,
    context: Option<ParentRouteContext>,
) -> ParentRouteSnapshot {
    let lan_route_query = lan_route_query_for_load(&route, context.as_ref());
    build_parent_route_snapshot(route, lan_route_query)
}

pub fn load_parent_subscription_event(
    route: ParentRouteId,
    context: Option<ParentRouteContext>,
) -> ParentSubscriptionEvent {
    let snapshot = load_parent_route_snapshot(route, context);
    ParentSubscriptionEvent {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        route: snapshot.route.clone(),
        snapshot,
    }
}

pub fn dispatch_parent_ui_action(action: ParentUiAction) -> ParentUiActionResult {
    let action_owned = matches!(
        action.action,
        ParentUiActionKind::RefreshRoute
            | ParentUiActionKind::Reconnect
            | ParentUiActionKind::AgentCommandRequested
            | ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested
            | ParentUiActionKind::NetworkFlowReadModelRefreshRequested
            | ParentUiActionKind::TrackingRetentionSettingsWriteRequested
            | ParentUiActionKind::ScreenSettingsGetRequested
            | ParentUiActionKind::ScreenSettingsReplaceRequested
            | ParentUiActionKind::AppGameAdapterDispatchExecuteRequested
            | ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested
    );
    let lan_route_query = lan_route_query_for_action(&action);
    let connection_state = connection_state_for_route(&action.route, &lan_route_query);
    let accepted = action_owned && !matches!(lan_route_query, LanRouteQuery::Unavailable(_));
    let message = match &lan_route_query {
        LanRouteQuery::Unavailable(error) if is_lan_surface_route(&action.route) => error.clone(),
        _ => action_result_message(&action),
    };
    let snapshot = build_parent_route_snapshot(action.route.clone(), lan_route_query);

    ParentUiActionResult {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        accepted,
        connection_state,
        message,
        snapshot: Some(snapshot),
    }
}

fn build_parent_route_snapshot(
    route: ParentRouteId,
    lan_route_query: LanRouteQuery,
) -> ParentRouteSnapshot {
    let diagnostic_panels_enabled = is_dev_tools_route(&route);
    let data_source = data_source_for_route(&route, &lan_route_query);
    let connection_state = connection_state_for_route(&route, &lan_route_query);
    let command_enabled = command_enabled_for_route(&route, &connection_state);
    let lan_add_device_read_model = lan_route_query.read_model();
    let summary = summary_for_route(&route, &data_source, lan_add_device_read_model);
    let parent_portal_rows =
        parent_portal_rows_for_route(&route, &summary, &data_source, lan_add_device_read_model);
    let generated_at = lan_add_device_read_model
        .as_ref()
        .map(|read_model| read_model.generated_at.clone())
        .unwrap_or_else(|| EMPTY_TIMESTAMP.to_string());
    let last_updated = lan_route_query
        .event()
        .and_then(|event| event.sent_at.clone())
        .unwrap_or_else(|| generated_at.clone());

    ParentRouteSnapshot {
        schema_version: PARENT_UI_BRIDGE_SCHEMA_VERSION,
        route: route.clone(),
        generated_at,
        season_label: season_label_for_connection(&connection_state).to_string(),
        last_updated,
        connection_state: connection_state.clone(),
        command_enabled,
        agent_endpoint: HOST_BRIDGE_URL.to_string(),
        data_source: data_source.clone(),
        summary: summary.clone(),
        diagnostic_panels_enabled,
        parent_portal_rows,
        parent_portal_shell_status: Some(parent_portal_shell_status(
            &route,
            &summary,
            &data_source,
            &connection_state,
            lan_add_device_read_model,
        )),
        live_activity: live_activity_snapshot(&route, &lan_route_query),
        browser_panels: None,
        screen_settings_service_response: None,
    }
}

fn data_source_for_route(
    route: &ParentRouteId,
    lan_route_query: &LanRouteQuery,
) -> ParentRouteDataSource {
    if is_dev_tools_route(route) {
        return ParentRouteDataSource::DevDiagnostics;
    }
    match route {
        ParentRouteId::Commands | ParentRouteId::Events | ParentRouteId::Logs => {
            ParentRouteDataSource::DevDiagnostics
        }
        ParentRouteId::Devices
        | ParentRouteId::LanPairing
        | ParentRouteId::CapabilityStatus
        | ParentRouteId::PlatformsInstall
        | ParentRouteId::InstallUpdates => {
            if matches!(lan_route_query, LanRouteQuery::Available(_)) {
                ParentRouteDataSource::RustReadModel
            } else {
                ParentRouteDataSource::Unavailable
            }
        }
        _ => ParentRouteDataSource::HostBridge,
    }
}

fn connection_state_for_route(
    route: &ParentRouteId,
    lan_route_query: &LanRouteQuery,
) -> ParentBridgeConnectionState {
    if is_lan_surface_route(route) {
        return match lan_route_query {
            LanRouteQuery::Available(_) => ParentBridgeConnectionState::Connected,
            LanRouteQuery::NotRequired | LanRouteQuery::Unavailable(_) => {
                ParentBridgeConnectionState::Error
            }
        };
    }
    ParentBridgeConnectionState::Connected
}

fn command_enabled_for_route(
    route: &ParentRouteId,
    connection_state: &ParentBridgeConnectionState,
) -> bool {
    if is_lan_surface_route(route) {
        return matches!(connection_state, ParentBridgeConnectionState::Connected);
    }
    true
}

fn is_dev_tools_route(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::Diagnostics
            | ParentRouteId::ProofPanels
            | ParentRouteId::Commands
            | ParentRouteId::Events
            | ParentRouteId::Logs
            | ParentRouteId::AppLayout
    )
}

fn summary_for_route(
    route: &ParentRouteId,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentRouteSummary {
    if let Some(read_model) = lan_add_device_read_model {
        return ParentRouteSummary {
            title: route_title(route).to_string(),
            route_capability: route_capability_state_for_data_source(data_source).to_string(),
            parent_access: parent_access_summary(read_model),
            household: lan_household_summary(read_model),
            child_device: lan_child_device_summary(read_model),
        };
    }

    ParentRouteSummary {
        title: route_title(route).to_string(),
        route_capability: route_capability_state_for_data_source(data_source).to_string(),
        parent_access: if *data_source == ParentRouteDataSource::DevDiagnostics {
            "unavailable".to_string()
        } else {
            "proof-missing".to_string()
        },
        household: "unavailable".to_string(),
        child_device: "unavailable".to_string(),
    }
}

fn parent_portal_rows(
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Vec<ParentPortalRowSnapshot> {
    let local_agent_trend = lan_add_device_read_model
        .map(|read_model| serialized_enum_label(&read_model.local_service_discovery_state))
        .unwrap_or_else(|| data_source_label(data_source).to_string());
    let household_trend = summary.household.clone();
    let child_device_trend = summary.child_device.clone();

    vec![
        portal_row_snapshot(
            "Local agent",
            1,
            "Runtime",
            local_agent_trend,
            ParentPortalTone::Cyan,
        ),
        portal_row_snapshot(
            "Route capability",
            2,
            "Route",
            summary.route_capability.clone(),
            ParentPortalTone::Gold,
        ),
        portal_row_snapshot(
            "Parent access",
            3,
            "Authority",
            summary.parent_access.clone(),
            ParentPortalTone::Purple,
        ),
        portal_row_snapshot(
            "Household",
            4,
            "Custody",
            household_trend,
            ParentPortalTone::Red,
        ),
        portal_row_snapshot(
            "Child device",
            5,
            "Transport",
            child_device_trend,
            ParentPortalTone::Cyan,
        ),
        portal_row_snapshot(
            "Data source",
            6,
            "Bridge",
            data_source_label(data_source).to_string(),
            ParentPortalTone::Muted,
        ),
    ]
}

fn parent_portal_rows_for_route(
    route: &ParentRouteId,
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Option<Vec<ParentPortalRowSnapshot>> {
    if is_lan_surface_route(route) {
        return Some(parent_portal_rows(
            summary,
            data_source,
            lan_add_device_read_model,
        ));
    }
    None
}

fn parent_portal_shell_status(
    route: &ParentRouteId,
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    connection_state: &ParentBridgeConnectionState,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentPortalShellStatusSnapshot {
    let route_label = route_title(route).to_string();
    let global_connection_state =
        global_connection_state_for_connection(connection_state).to_string();
    let data_source_label = data_source_label(data_source).to_string();
    let parent_access_state = parent_access_state_for_read_model(lan_add_device_read_model);
    let household_detail = if let Some(read_model) = lan_add_device_read_model {
        format!(
            "{} canonical / {} discovered / {} trusted",
            read_model.canonical_household_devices.len(),
            read_model.discovered_devices.len(),
            read_model.trusted_device_registry.len()
        )
    } else {
        "No runtime-backed household summary is attached to this route snapshot.".to_string()
    };
    let child_device_detail = if let Some(read_model) = lan_add_device_read_model {
        if let Some(selected_child_device_id) = read_model
            .selected_device_readiness
            .selected_child_device_id
            .as_deref()
        {
            format!("selected child device: {selected_child_device_id}")
        } else {
            "No child device is selected for control.".to_string()
        }
    } else {
        format!("data source: {data_source_label}")
    };

    ParentPortalShellStatusSnapshot {
        route_label: route_label.clone(),
        parent_access_state,
        global_connection_state: global_connection_state.clone(),
        route_capability_state: summary.route_capability.clone(),
        data_source_label: data_source_label.clone(),
        cards: vec![
            ParentPortalShellStatusCardSnapshot {
                id: "parent-access".to_string(),
                label: "Parent access".to_string(),
                value: summary.parent_access.clone(),
                detail: parent_access_detail(lan_add_device_read_model),
                tone: ParentPortalTone::Muted,
            },
            ParentPortalShellStatusCardSnapshot {
                id: "connection".to_string(),
                label: "Connection".to_string(),
                value: global_connection_state,
                detail: format!("route: {route_label}"),
                tone: connection_tone(connection_state),
            },
            ParentPortalShellStatusCardSnapshot {
                id: "household".to_string(),
                label: "Household".to_string(),
                value: summary.household.clone(),
                detail: household_detail,
                tone: ParentPortalTone::Muted,
            },
            ParentPortalShellStatusCardSnapshot {
                id: "child-device".to_string(),
                label: "Child device".to_string(),
                value: summary.child_device.clone(),
                detail: child_device_detail,
                tone: ParentPortalTone::Muted,
            },
            ParentPortalShellStatusCardSnapshot {
                id: "route-capability".to_string(),
                label: "Route capability".to_string(),
                value: summary.route_capability.clone(),
                detail: format!("bridge: {data_source_label}"),
                tone: route_capability_tone(data_source),
            },
            ParentPortalShellStatusCardSnapshot {
                id: "data-source".to_string(),
                label: "Data source".to_string(),
                value: data_source_label.clone(),
                detail: "product UI now reads this route through the host bridge facade"
                    .to_string(),
                tone: data_source_tone(data_source),
            },
        ],
    }
}

fn live_activity_snapshot(
    route: &ParentRouteId,
    lan_route_query: &LanRouteQuery,
) -> Option<ParentRouteLiveActivitySnapshot> {
    if matches!(
        route,
        ParentRouteId::Commands
            | ParentRouteId::Events
            | ParentRouteId::Logs
            | ParentRouteId::AppLayout
    ) {
        return None;
    }
    let mut snapshot = empty_live_activity_snapshot();
    if let Some(read_model) = lan_route_query.read_model() {
        snapshot.lan_add_device_read_model =
            Some(current_lan_add_device_read_model_value(read_model));
    }
    if let Some(event) = lan_route_query.discovery_event() {
        snapshot.lan_pairing_browser_discovery_event = Some(event.clone());
    }
    Some(snapshot)
}

fn empty_live_activity_snapshot() -> ParentRouteLiveActivitySnapshot {
    ParentRouteLiveActivitySnapshot {
        recent_summary: None,
        ingest_status: None,
        activity_screen_read_model: None,
        browser_managed_event: None,
        browser_managed_status: None,
        browser_runtime_event_chain_stream: None,
        browser_social_provider_receipt_stream_status_intent: None,
        browser_social_provider_receipt_ingestion_readiness_status_intent: None,
        local_ai_runtime_status_event: None,
        lan_ai_job_event: None,
        parent_assistant_boundary_event: None,
        activity_memory_graph_read_model: None,
        network_flow_event: None,
        network_flow_read_model: None,
        network_runtime_event_chain_stream: None,
        lan_pairing_browser_discovery_event: None,
        lan_add_device_read_model: None,
        policy_preview_event: None,
        policy_preview_read_model: None,
        app_game_notification_parent_surface_intent_read_model: None,
        app_game_policy_readiness_read_model: None,
        app_game_platform_proof_status_read_model: None,
        app_game_child_runtime_transport_receipt_read_model: None,
        app_game_adapter_dispatch_preflight_read_model: None,
        app_game_adapter_dispatch_result_read_model: None,
        app_game_adapter_dispatch_executed_result: None,
        app_game_timer_parent_surface_read_model: None,
        app_game_timer_parent_preference_setup_requested_result: None,
        browser_intervention_event: None,
        browser_intervention_read_model: None,
        activity_tracking_read_model_event: None,
        activity_tracking_read_model: None,
        activity_tracking_retention_settings_write_result: None,
    }
}

fn is_lan_surface_route(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::Devices
            | ParentRouteId::LanPairing
            | ParentRouteId::CapabilityStatus
            | ParentRouteId::PlatformsInstall
            | ParentRouteId::InstallUpdates
    )
}

fn lan_household_summary(read_model: &LanBrowserAddDeviceReadModel) -> String {
    let device_count = lan_visible_device_count(read_model);
    if device_count == 0 {
        serialized_enum_label(&read_model.physical_household_lan_state)
    } else if device_count == 1 {
        "1 device visible".to_string()
    } else {
        format!("{device_count} devices visible")
    }
}

fn lan_child_device_summary(read_model: &LanBrowserAddDeviceReadModel) -> String {
    if read_model.selected_device_readiness.ready_for_control {
        return "ready-for-control".to_string();
    }
    if read_model
        .selected_device_readiness
        .selected_child_device_id
        .is_some()
    {
        return "selected".to_string();
    }
    let device_count = lan_visible_device_count(read_model);
    if device_count == 0 {
        serialized_enum_label(&read_model.add_device_state)
    } else if device_count == 1 {
        "1 discoverable".to_string()
    } else {
        format!("{device_count} discoverable")
    }
}

fn lan_visible_device_count(read_model: &LanBrowserAddDeviceReadModel) -> usize {
    if !read_model.canonical_household_devices.is_empty() {
        read_model.canonical_household_devices.len()
    } else if !read_model.discovered_devices.is_empty() {
        read_model.discovered_devices.len()
    } else if !read_model.trusted_device_registry.is_empty() {
        read_model.trusted_device_registry.len()
    } else {
        read_model.pairing_requests.len()
    }
}

fn parent_access_summary(read_model: &LanBrowserAddDeviceReadModel) -> String {
    serialized_enum_label(&read_model.controller_authority)
}

fn parent_access_state_for_read_model(
    read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentPortalParentAccessState {
    match read_model.map(parent_access_summary).as_deref() {
        Some("active-controller") => ParentPortalParentAccessState::ActiveController,
        Some("observer") => ParentPortalParentAccessState::ObserverOnly,
        Some("unauthenticated") => ParentPortalParentAccessState::Unauthenticated,
        _ => ParentPortalParentAccessState::ProofMissing,
    }
}

fn parent_access_detail(read_model: Option<&LanBrowserAddDeviceReadModel>) -> String {
    if let Some(read_model) = read_model {
        return format!(
            "controller authority: {} / observer authority: {}",
            serialized_enum_label(&read_model.controller_authority),
            serialized_enum_label(&read_model.observer_authority)
        );
    }
    "No LAN authority proof is attached because the local agent-service route is unavailable."
        .to_string()
}

fn lan_route_query_for_load(
    route: &ParentRouteId,
    context: Option<&ParentRouteContext>,
) -> LanRouteQuery {
    if !is_lan_surface_route(route) {
        return LanRouteQuery::NotRequired;
    }
    match load_lan_route_snapshot(context) {
        Ok(snapshot) => LanRouteQuery::Available(snapshot),
        Err(error) => LanRouteQuery::Unavailable(error),
    }
}

fn lan_route_query_for_action(action: &ParentUiAction) -> LanRouteQuery {
    if !is_lan_surface_route(&action.route) {
        return LanRouteQuery::NotRequired;
    }
    let response = match action.action {
        ParentUiActionKind::AgentCommandRequested => action
            .command
            .as_deref()
            .ok_or_else(|| {
                "parent Rust facade rejected LAN agent command request without a command name"
                    .to_string()
            })
            .and_then(|command_name| {
                dispatch_lan_agent_command(command_name, &action.payload, None)
            }),
        ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested => {
            request_lan_browser_discovery_scan(None)
        }
        _ => load_lan_route_snapshot(None),
    };
    match response {
        Ok(snapshot) => LanRouteQuery::Available(snapshot),
        Err(error) => LanRouteQuery::Unavailable(error),
    }
}

impl LanRouteQuery {
    fn read_model(&self) -> Option<&LanBrowserAddDeviceReadModel> {
        match self {
            Self::Available(snapshot) => Some(&snapshot.read_model),
            Self::NotRequired | Self::Unavailable(_) => None,
        }
    }

    fn event(&self) -> Option<&ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot> {
        match self {
            Self::Available(snapshot) => Some(&snapshot.event),
            Self::NotRequired | Self::Unavailable(_) => None,
        }
    }

    fn discovery_event(
        &self,
    ) -> Option<&ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot> {
        self.event()
            .filter(|event| event.event.as_deref() == Some(LAN_DISCOVERY_REPORTED_EVENT))
    }
}

fn portal_row_snapshot(
    label: &str,
    order: u16,
    primary_area: &str,
    trend: String,
    tone: ParentPortalTone,
) -> ParentPortalRowSnapshot {
    let available = !matches!(
        trend.as_str(),
        "manual-required" | "unavailable" | "offline" | "proof-missing" | "unauthenticated"
    );
    ParentPortalRowSnapshot {
        label: label.to_string(),
        order,
        signal_score: if available { 100 } else { 0 },
        ready_count: if available { 1 } else { 0 },
        gap_count: if available { 0 } else { 1 },
        primary_area: primary_area.to_string(),
        trend,
        tone,
    }
}

fn serialized_enum_label<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| json.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

fn data_source_label(data_source: &ParentRouteDataSource) -> &'static str {
    match data_source {
        ParentRouteDataSource::HostBridge => "host-bridge",
        ParentRouteDataSource::RustReadModel => "rust-read-model",
        ParentRouteDataSource::DevDiagnostics => "dev-diagnostics",
        ParentRouteDataSource::Unavailable => "unavailable",
    }
}

fn route_capability_state_for_data_source(data_source: &ParentRouteDataSource) -> &'static str {
    match data_source {
        ParentRouteDataSource::Unavailable => "unavailable",
        _ => "available",
    }
}

fn season_label_for_connection(connection_state: &ParentBridgeConnectionState) -> &'static str {
    match connection_state {
        ParentBridgeConnectionState::Connected => STATUS_LOCAL,
        ParentBridgeConnectionState::Connecting => STATUS_CONNECTING,
        ParentBridgeConnectionState::Error => STATUS_CHECK_SERVICE,
        ParentBridgeConnectionState::Disconnected => STATUS_OFFLINE,
    }
}

fn global_connection_state_for_connection(
    connection_state: &ParentBridgeConnectionState,
) -> &'static str {
    match connection_state {
        ParentBridgeConnectionState::Connected => "manual-required",
        ParentBridgeConnectionState::Connecting => "offline",
        ParentBridgeConnectionState::Error => "degraded",
        ParentBridgeConnectionState::Disconnected => "offline",
    }
}

fn connection_tone(connection_state: &ParentBridgeConnectionState) -> ParentPortalTone {
    match connection_state {
        ParentBridgeConnectionState::Connected => ParentPortalTone::Gold,
        ParentBridgeConnectionState::Connecting => ParentPortalTone::Muted,
        ParentBridgeConnectionState::Error => ParentPortalTone::Red,
        ParentBridgeConnectionState::Disconnected => ParentPortalTone::Muted,
    }
}

fn route_capability_tone(data_source: &ParentRouteDataSource) -> ParentPortalTone {
    match data_source {
        ParentRouteDataSource::Unavailable => ParentPortalTone::Red,
        ParentRouteDataSource::DevDiagnostics => ParentPortalTone::Gold,
        ParentRouteDataSource::HostBridge | ParentRouteDataSource::RustReadModel => {
            ParentPortalTone::Cyan
        }
    }
}

fn data_source_tone(data_source: &ParentRouteDataSource) -> ParentPortalTone {
    match data_source {
        ParentRouteDataSource::HostBridge => ParentPortalTone::Gold,
        ParentRouteDataSource::RustReadModel => ParentPortalTone::Purple,
        ParentRouteDataSource::DevDiagnostics => ParentPortalTone::Muted,
        ParentRouteDataSource::Unavailable => ParentPortalTone::Muted,
    }
}

fn current_lan_add_device_read_model_value(read_model: &LanBrowserAddDeviceReadModel) -> Value {
    serde_json::to_value(read_model)
        .unwrap_or_else(|error| unreachable!("LAN add-device read model serializes: {error}"))
}

fn action_result_message(action: &ParentUiAction) -> String {
    match action.action {
        ParentUiActionKind::RefreshRoute => "route snapshot refreshed by parent Rust facade",
        ParentUiActionKind::Reconnect => "parent Rust facade reloaded route state",
        ParentUiActionKind::AgentCommandRequested => {
            "parent Rust facade forwarded LAN agent command request"
        }
        ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested => {
            "parent Rust facade requested LAN pairing browser discovery scan"
        }
        ParentUiActionKind::NetworkFlowReadModelRefreshRequested => {
            "parent Rust facade requested network flow read model refresh"
        }
        ParentUiActionKind::TrackingRetentionSettingsWriteRequested => {
            "parent Rust facade requested tracking retention settings write"
        }
        ParentUiActionKind::ScreenSettingsGetRequested => {
            "parent Rust facade requested screen settings readback"
        }
        ParentUiActionKind::ScreenSettingsReplaceRequested => {
            "parent Rust facade requested screen settings replace"
        }
        ParentUiActionKind::AppGameAdapterDispatchExecuteRequested => {
            "parent Rust facade requested app/game adapter dispatch execution"
        }
        ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested => {
            "parent Rust facade requested app/game timer parent preference setup"
        }
    }
    .to_string()
}

fn route_title(route: &ParentRouteId) -> &'static str {
    match route {
        ParentRouteId::Overview => "Overview",
        ParentRouteId::Assistant => "Assistant",
        ParentRouteId::Start => "Start",
        ParentRouteId::Activity => "Activity",
        ParentRouteId::Browser => "Browser",
        ParentRouteId::BrowserSettings => "Browser settings",
        ParentRouteId::Policy => "Policy",
        ParentRouteId::PolicyApps => "Policy apps",
        ParentRouteId::PolicyGames => "Policy games",
        ParentRouteId::PolicyScreen => "Policy screen",
        ParentRouteId::PolicyNetwork => "Policy network",
        ParentRouteId::PolicyTracking => "Policy tracking",
        ParentRouteId::PolicyRemoteScreen => "Policy remote screen",
        ParentRouteId::RuleManagement => "Rule management",
        ParentRouteId::Schedules => "Schedules",
        ParentRouteId::Approvals => "Approvals",
        ParentRouteId::Enforcement => "Enforcement",
        ParentRouteId::PrivacyDesign => "Privacy design",
        ParentRouteId::Memory => "Memory",
        ParentRouteId::MemorySettings => "Memory settings",
        ParentRouteId::AiGuide => "AI guide",
        ParentRouteId::AiRuntime => "AI runtime",
        ParentRouteId::ApiProviders => "API providers",
        ParentRouteId::ReportsGuide => "Reports guide",
        ParentRouteId::ScreenAnalysis => "Screen analysis",
        ParentRouteId::AppGameSessions => "App game sessions",
        ParentRouteId::NetworkActivity => "Network activity",
        ParentRouteId::Devices => "Devices",
        ParentRouteId::LanPairing => "LAN pairing",
        ParentRouteId::CapabilityStatus => "Capability status",
        ParentRouteId::Notifications => "Notifications",
        ParentRouteId::NotificationChannels => "Notification channels",
        ParentRouteId::DriveConnections => "Drive connections",
        ParentRouteId::ExportRetention => "Export retention",
        ParentRouteId::RemoteAccess => "Remote access",
        ParentRouteId::ReportCompiler => "Report compiler",
        ParentRouteId::AuditHistory => "Audit history",
        ParentRouteId::Subscription => "Subscription",
        ParentRouteId::Entitlements => "Entitlements",
        ParentRouteId::PlatformsInstall => "Platforms install",
        ParentRouteId::InstallUpdates => "Install updates",
        ParentRouteId::Diagnostics => "Diagnostics",
        ParentRouteId::ProofPanels => "Proof panels",
        ParentRouteId::SettingsRules => "Settings rules",
        ParentRouteId::AppLayout => "App layout",
        ParentRouteId::Commands => "Commands",
        ParentRouteId::Events => "Events",
        ParentRouteId::Logs => "Logs",
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::net::TcpListener;
    use std::sync::{mpsc, Arc, Mutex, OnceLock};
    use std::thread;
    use std::time::Duration;

    use ocentra_lan_core::read_model_builder::{
        build_lan_add_device_read_model, LanAddDeviceReadModelInput,
    };
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::lan_pairing::{
        LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
        LanPairingNetworkMode, LanPairingProductionDiscoveryState, LanPairingTrustState,
    };
    use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
    use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
        LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDeviceReadModel,
        LanDiscoveryEvidenceSource, LanPairingDiscoverySource, LanSelectedDeviceReadiness,
    };
    use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
    use ocentra_parent_agent_protocol::transport::{
        AgentEventEnvelope, AgentEventName, AgentPeer, AgentPeerRole,
    };
    use serde_json::{json, Value};
    use tungstenite::{
        accept_hdr,
        handshake::server::{Request, Response},
        Message,
    };

    use super::{
        dispatch_parent_ui_action, load_parent_route_snapshot, load_parent_subscription_event,
        ParentRouteId, ParentUiAction, ParentUiActionKind, LAN_DISCOVERY_REPORTED_EVENT,
    };

    #[test]
    fn parent_route_snapshot_serializes_for_host_bridge() {
        let address = start_lan_stub_server(
            AgentEventName::AgentLanPairingStatusReported,
            sample_lan_read_model(),
        );
        let value = with_agent_addr(&address, || {
            serde_json::to_value(load_parent_route_snapshot(ParentRouteId::Devices, None))
                .unwrap_or_else(|error| unreachable!("parent route snapshot serializes: {error}"))
        });
        let browser_value =
            serde_json::to_value(load_parent_route_snapshot(ParentRouteId::Browser, None))
                .unwrap_or_else(|error| unreachable!("browser route snapshot serializes: {error}"));

        assert_eq!(value["schemaVersion"], 1);
        assert_eq!(value["route"], "devices");
        assert_eq!(value["seasonLabel"], "LOCAL");
        assert_eq!(value["connectionState"], "connected");
        assert_eq!(value["commandEnabled"], true);
        assert_eq!(value["summary"]["childDevice"], "1 discoverable");
        assert_eq!(value["parentPortalRows"][0]["label"], "Local agent");
        assert_eq!(value["parentPortalRows"][5]["trend"], "rust-read-model");
        assert_eq!(
            value["parentPortalShellStatus"]["parentAccessState"],
            "active-controller"
        );
        assert_eq!(
            value["liveActivity"]["lanAddDeviceReadModel"]["schemaVersion"],
            1
        );
        assert!(value["liveActivity"]["lanAddDeviceReadModel"]["scanSummary"].is_object());
        assert!(
            value["liveActivity"]["lanAddDeviceReadModel"]["canonicalHouseholdDevices"].is_array()
        );
        assert!(
            value["liveActivity"]["lanAddDeviceReadModel"]["lanDiscoverySourceMatrix"].is_object()
        );
        assert!(value["liveActivity"]["activityTrackingReadModel"].is_null());
        assert!(value["liveActivity"]["localAiRuntimeStatusEvent"].is_null());
        assert!(value["liveActivity"]["browserManagedStatus"].is_null());
        assert!(browser_value["browserPanels"].is_null());
    }

    #[test]
    fn parent_subscription_event_serializes_for_host_bridge() {
        let address = start_lan_stub_server(
            AgentEventName::AgentLanPairingBrowserDiscoveryReported,
            sample_lan_read_model(),
        );
        let value = with_agent_addr(&address, || {
            serde_json::to_value(load_parent_subscription_event(ParentRouteId::Devices, None))
                .unwrap_or_else(|error| {
                    unreachable!("parent subscription event serializes: {error}")
                })
        });

        assert_eq!(value["schemaVersion"], 1);
        assert_eq!(value["route"], "devices");
        assert_eq!(value["snapshot"]["route"], "devices");
        assert_eq!(value["snapshot"]["connectionState"], "connected");
        assert_eq!(value["snapshot"]["dataSource"], "rust-read-model");
    }

    #[test]
    fn devices_route_degrades_honestly_when_agent_service_is_unavailable() {
        let value = with_agent_addr("127.0.0.1:9", || {
            serde_json::to_value(load_parent_route_snapshot(ParentRouteId::Devices, None))
                .unwrap_or_else(|error| unreachable!("parent route snapshot serializes: {error}"))
        });

        assert_eq!(value["connectionState"], "error");
        assert_eq!(value["commandEnabled"], false);
        assert_eq!(value["dataSource"], "unavailable");
        assert_eq!(value["summary"]["household"], "unavailable");
        assert!(value["liveActivity"]["lanAddDeviceReadModel"].is_null());
    }

    #[test]
    fn devices_route_load_uses_browser_discovery_scan_with_default_origin() {
        let (address, capture) = start_lan_stub_server_with_capture(
            AgentEventName::AgentLanPairingBrowserDiscoveryReported,
            sample_lan_read_model(),
        );
        with_agent_addr(&address, || {
            let _ = load_parent_route_snapshot(ParentRouteId::Devices, None);
        });
        let request = capture
            .recv_timeout(Duration::from_secs(1))
            .unwrap_or_else(|error| unreachable!("captured LAN load command arrives: {error}"));

        assert_eq!(
            request.command["command"],
            json!("agent.lan-pairing.browser-discovery.scan")
        );
        assert_eq!(
            request.origin.as_deref(),
            Some(constants::bind::DEFAULT_ALLOWED_ORIGINS[0])
        );
    }

    #[test]
    fn parent_ui_action_serializes_and_returns_snapshot() {
        let action = ParentUiAction {
            action: ParentUiActionKind::AgentCommandRequested,
            route: ParentRouteId::Activity,
            command: Some("agent.network.flow.read-model.get".to_string()),
            payload: json!({ "source": "ui" }),
        };

        let result = dispatch_parent_ui_action(action);

        assert!(result.accepted);
        assert_eq!(
            result.snapshot.map(|snapshot| snapshot.route),
            Some(ParentRouteId::Activity)
        );
    }

    #[test]
    fn lan_agent_command_requested_for_devices_route_forwards_command_payload_and_origin() {
        let (address, capture) = start_lan_stub_server_with_capture(
            AgentEventName::AgentLanPairingBrowserDiscoveryReported,
            sample_lan_read_model(),
        );
        let result = with_agent_addr(&address, || {
            dispatch_parent_ui_action(ParentUiAction {
                action: ParentUiActionKind::AgentCommandRequested,
                route: ParentRouteId::Devices,
                command: Some("agent.lan-pairing.browser-discovery.scan".to_string()),
                payload: json!({
                    "origin": "http://127.0.0.1:4578",
                    "routeId": "lan-route-local-network",
                }),
            })
        });
        let request = capture
            .recv_timeout(Duration::from_secs(1))
            .unwrap_or_else(|error| unreachable!("captured LAN action command arrives: {error}"));

        assert!(result.accepted);
        assert_eq!(
            result.message,
            "parent Rust facade forwarded LAN agent command request"
        );
        assert_eq!(
            request.command["command"],
            json!("agent.lan-pairing.browser-discovery.scan")
        );
        assert_eq!(
            request.command["payload"]["origin"],
            json!("http://127.0.0.1:4578")
        );
        assert_eq!(
            request.command["payload"]["routeId"],
            json!("lan-route-local-network")
        );
        assert_eq!(request.origin.as_deref(), Some("http://127.0.0.1:4578"));
    }

    #[test]
    fn product_bridge_actions_return_route_snapshots_without_invented_overlay_data() {
        let address = start_lan_stub_server(
            AgentEventName::AgentLanPairingBrowserDiscoveryReported,
            sample_lan_read_model(),
        );
        let lan_scan = with_agent_addr(&address, || {
            dispatch_parent_ui_action(ParentUiAction {
                action: ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested,
                route: ParentRouteId::Devices,
                command: None,
                payload: json!({}),
            })
        });
        let network_refresh = dispatch_parent_ui_action(ParentUiAction {
            action: ParentUiActionKind::NetworkFlowReadModelRefreshRequested,
            route: ParentRouteId::Activity,
            command: None,
            payload: json!({}),
        });
        let tracking_retention_write = dispatch_parent_ui_action(ParentUiAction {
            action: ParentUiActionKind::TrackingRetentionSettingsWriteRequested,
            route: ParentRouteId::Activity,
            command: None,
            payload: json!({}),
        });
        let app_game_dispatch_execute = dispatch_parent_ui_action(ParentUiAction {
            action: ParentUiActionKind::AppGameAdapterDispatchExecuteRequested,
            route: ParentRouteId::AppGameSessions,
            command: None,
            payload: json!({}),
        });
        let app_game_timer_parent_preference_setup = dispatch_parent_ui_action(ParentUiAction {
            action: ParentUiActionKind::AppGameTimerParentPreferenceSetupRequested,
            route: ParentRouteId::AppGameSessions,
            command: None,
            payload: json!({}),
        });

        assert_eq!(
            lan_scan.message,
            "parent Rust facade requested LAN pairing browser discovery scan"
        );
        assert_eq!(
            lan_scan
                .snapshot
                .as_ref()
                .and_then(|snapshot| snapshot.live_activity.as_ref())
                .and_then(|live_activity| live_activity.lan_add_device_read_model.as_ref())
                .map(|_| ()),
            Some(())
        );
        assert_eq!(
            lan_scan
                .snapshot
                .as_ref()
                .and_then(|snapshot| snapshot.live_activity.as_ref())
                .and_then(|live_activity| live_activity
                    .lan_pairing_browser_discovery_event
                    .as_ref())
                .and_then(|event| event.event.as_deref()),
            Some(LAN_DISCOVERY_REPORTED_EVENT)
        );
        assert_eq!(
            network_refresh
                .snapshot
                .as_ref()
                .and_then(|snapshot| snapshot.live_activity.as_ref())
                .and_then(|live_activity| live_activity.network_flow_read_model.as_ref())
                .map(|_| ()),
            None
        );
        assert_eq!(
            tracking_retention_write
                .snapshot
                .as_ref()
                .and_then(|snapshot| snapshot.live_activity.as_ref())
                .and_then(|live_activity| live_activity
                    .activity_tracking_retention_settings_write_result
                    .as_ref())
                .map(|_| ()),
            None
        );
        assert_eq!(
            app_game_dispatch_execute
                .snapshot
                .as_ref()
                .and_then(|snapshot| snapshot.live_activity.as_ref())
                .and_then(|live_activity| live_activity
                    .app_game_adapter_dispatch_executed_result
                    .as_ref())
                .map(|_| ()),
            None
        );
        assert_eq!(
            app_game_timer_parent_preference_setup
                .snapshot
                .as_ref()
                .and_then(|snapshot| snapshot.live_activity.as_ref())
                .and_then(|live_activity| {
                    live_activity
                        .app_game_timer_parent_preference_setup_requested_result
                        .as_ref()
                })
                .map(|_| ()),
            None
        );
    }

    #[test]
    fn screen_settings_action_keeps_service_response_empty_until_runtime_wires_it() {
        let action = ParentUiAction {
            action: ParentUiActionKind::ScreenSettingsGetRequested,
            route: ParentRouteId::SettingsRules,
            command: None,
            payload: json!({
                "screenSettingsRequest": "{\"schemaVersion\":1,\"requestId\":\"screen-settings-request-9\",\"kind\":\"get\"}",
                "screenSettingsUpdateKind": "get"
            }),
        };

        let result = dispatch_parent_ui_action(action);
        let snapshot = result
            .snapshot
            .unwrap_or_else(|| unreachable!("screen settings action returns snapshot"));

        assert!(snapshot.screen_settings_service_response.is_none());
    }

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn with_agent_addr<T>(address: &str, action: impl FnOnce() -> T) -> T {
        let _guard = env_lock()
            .lock()
            .unwrap_or_else(|_| unreachable!("agent env lock remains available"));
        let previous = std::env::var(constants::env_var::AGENT_ADDR).ok();
        std::env::set_var(constants::env_var::AGENT_ADDR, address);
        let result = action();
        if let Some(value) = previous {
            std::env::set_var(constants::env_var::AGENT_ADDR, value);
        } else {
            std::env::remove_var(constants::env_var::AGENT_ADDR);
        }
        result
    }

    fn start_lan_stub_server(
        event_name: AgentEventName,
        read_model: LanBrowserAddDeviceReadModel,
    ) -> String {
        let (address, _capture) = start_lan_stub_server_with_capture(event_name, read_model);
        address
    }

    fn start_lan_stub_server_with_capture(
        event_name: AgentEventName,
        read_model: LanBrowserAddDeviceReadModel,
    ) -> (String, mpsc::Receiver<CapturedLanRequest>) {
        let listener = TcpListener::bind("127.0.0.1:0")
            .unwrap_or_else(|error| unreachable!("stub listener binds: {error}"));
        let address = listener
            .local_addr()
            .unwrap_or_else(|error| unreachable!("stub listener exposes address: {error}"));
        let (tx, rx) = mpsc::channel();
        let observed_origin = Arc::new(Mutex::new(None::<String>));
        let header_origin = Arc::clone(&observed_origin);
        thread::spawn(move || {
            let (stream, _) = listener
                .accept()
                .unwrap_or_else(|error| unreachable!("stub listener accepts: {error}"));
            let mut socket = accept_hdr(stream, move |request: &Request, response: Response| {
                let origin = request
                    .headers()
                    .get("origin")
                    .and_then(|value| value.to_str().ok())
                    .map(ToOwned::to_owned);
                *header_origin.lock().unwrap_or_else(|_| {
                    unreachable!("captured header origin lock remains available")
                }) = origin;
                Ok(response)
            })
            .unwrap_or_else(|error| unreachable!("stub websocket handshake succeeds: {error}"));
            socket
                .send(Message::Text(
                    serde_json::to_string(&ready_event())
                        .unwrap_or_else(|error| unreachable!("ready event serializes: {error}")),
                ))
                .unwrap_or_else(|error| unreachable!("ready event sends: {error}"));
            let command_text = match socket
                .read()
                .unwrap_or_else(|error| unreachable!("command reads: {error}"))
            {
                Message::Text(text) => text,
                _ => unreachable!("stub agent receives one text command"),
            };
            let command: Value = serde_json::from_str(&command_text)
                .unwrap_or_else(|error| unreachable!("command parses: {error}"));
            let _ = tx.send(CapturedLanRequest {
                origin: observed_origin
                    .lock()
                    .unwrap_or_else(|_| unreachable!("captured origin lock remains available"))
                    .clone(),
                command,
            });
            socket
                .send(Message::Text(
                    serde_json::to_string(&lan_event(event_name, read_model))
                        .unwrap_or_else(|error| unreachable!("lan event serializes: {error}")),
                ))
                .unwrap_or_else(|error| unreachable!("lan event sends: {error}"));
        });
        (address.to_string(), rx)
    }

    #[derive(Debug)]
    struct CapturedLanRequest {
        origin: Option<String>,
        command: Value,
    }

    fn ready_event() -> AgentEventEnvelope {
        AgentEventEnvelope {
            schema_version: 1,
            event_id: "agent.connection.ready-1".to_string(),
            correlation_id: "ready".to_string(),
            sent_at: "2026-06-23T00:00:00Z".to_string(),
            source: AgentPeer {
                peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                role: AgentPeerRole::AgentService,
            },
            target: AgentPeer {
                peer_id: constants::peer::PORTAL_DEV.to_string(),
                role: AgentPeerRole::Portal,
            },
            event: AgentEventName::AgentConnectionReady,
            severity: LogLevel::Info,
            payload: BTreeMap::new(),
            snapshot: None,
        }
    }

    fn lan_event(
        event_name: AgentEventName,
        read_model: LanBrowserAddDeviceReadModel,
    ) -> AgentEventEnvelope {
        let mut payload = BTreeMap::new();
        payload.insert(
            constants::field::LAN_ADD_DEVICE_READ_MODEL.to_string(),
            LogFieldValue::String(
                serde_json::to_string(&read_model)
                    .unwrap_or_else(|error| unreachable!("LAN read model serializes: {error}")),
            ),
        );
        AgentEventEnvelope {
            schema_version: 1,
            event_id: "agent.lan-pairing.event-1".to_string(),
            correlation_id: "lan".to_string(),
            sent_at: "2026-06-23T00:00:00Z".to_string(),
            source: AgentPeer {
                peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                role: AgentPeerRole::AgentService,
            },
            target: AgentPeer {
                peer_id: constants::peer::PORTAL_DEV.to_string(),
                role: AgentPeerRole::Portal,
            },
            event: event_name,
            severity: LogLevel::Info,
            payload,
            snapshot: None,
        }
    }

    fn sample_lan_read_model() -> LanBrowserAddDeviceReadModel {
        build_lan_add_device_read_model(LanAddDeviceReadModelInput {
            generated_at: "2026-06-23T00:00:00Z".to_string(),
            discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
            add_device_state: LanPairingProductionDiscoveryState::Discovered,
            local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
            physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
            cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
            discovered_devices: vec![LanBrowserAddDeviceDiscoveryDevice {
                schema_version: 1,
                discovered_at: "2026-06-23T00:00:00Z".to_string(),
                child_device: sample_child_device(),
                agent_peer_id: "local-dev-agent".to_string(),
                route_id: "route-local-network".to_string(),
                network_mode: LanPairingNetworkMode::LocalNetwork,
                reachability: LanPairingDeviceReachability::Online,
                address_ref: "network-neighbor".to_string(),
                discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
                discovery_state: LanPairingProductionDiscoveryState::Discovered,
                evidence_sources: vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
                hint_sources: Vec::new(),
            }],
            pairing_requests: Vec::new(),
            trusted_device_registry: Vec::new(),
            household_device_decisions: Vec::new(),
            trusted_device_ids: Vec::new(),
            revoked_device_ids: Vec::new(),
            selected_device_readiness: LanSelectedDeviceReadiness {
                schema_version: 1,
                selected_child_device_id: None,
                route_id: None,
                pairing_id: None,
                trust_state: LanPairingTrustState::Unpaired,
                reachability: LanPairingDeviceReachability::Offline,
                ready_for_control: false,
                stale_at: None,
                offline_at: None,
            },
            controller_authority: LanPairingParentAuthority::ActiveController,
            observer_authority: LanPairingParentAuthority::Observer,
        })
    }

    fn sample_child_device() -> LanPairingDeviceRef {
        let mut device = LanPairingDeviceRef::new(
            "network-neighbor-1".to_string(),
            None,
            "Study Laptop".to_string(),
            "windows".to_string(),
        );
        device.ip_address = Some("192.168.1.24".to_string());
        device.mac_address = Some("aa-bb-cc-dd-ee-ff".to_string());
        device.hostname = Some("study-laptop".to_string());
        device.network_interface = Some("Ethernet".to_string());
        device
    }
}
