use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentRouteId {
    Overview,
    Assistant,
    Start,
    Activity,
    Browser,
    BrowserSettings,
    Policy,
    PolicyApps,
    PolicyGames,
    PolicyScreen,
    PolicyNetwork,
    PolicyTracking,
    PolicyRemoteScreen,
    RuleManagement,
    Schedules,
    Approvals,
    Enforcement,
    PrivacyDesign,
    Memory,
    MemorySettings,
    AiGuide,
    AiRuntime,
    ApiProviders,
    ReportsGuide,
    ScreenAnalysis,
    AppGameSessions,
    NetworkActivity,
    Devices,
    LanPairing,
    CapabilityStatus,
    Notifications,
    NotificationChannels,
    DriveConnections,
    ExportRetention,
    RemoteAccess,
    ReportCompiler,
    AuditHistory,
    Subscription,
    Entitlements,
    PlatformsInstall,
    InstallUpdates,
    Diagnostics,
    ProofPanels,
    SettingsRules,
    AppLayout,
    Commands,
    Events,
    Logs,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentBridgeConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentRouteDataSource {
    HostBridge,
    RustReadModel,
    DevDiagnostics,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentPortalTone {
    Cyan,
    Gold,
    Purple,
    Red,
    Muted,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentPortalParentAccessState {
    ActiveController,
    ObserverOnly,
    Unauthenticated,
    ProofMissing,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteContext {
    pub selected_child_device_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPortalRowSnapshot {
    pub label: String,
    pub order: u16,
    pub signal_score: u16,
    pub ready_count: u16,
    pub gap_count: u16,
    pub primary_area: String,
    pub trend: String,
    pub tone: ParentPortalTone,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPortalShellStatusCardSnapshot {
    pub id: String,
    pub label: String,
    pub value: String,
    pub detail: String,
    pub tone: ParentPortalTone,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPortalShellStatusSnapshot {
    pub route_label: String,
    pub parent_access_state: ParentPortalParentAccessState,
    pub global_connection_state: String,
    pub route_capability_state: String,
    pub data_source_label: String,
    pub cards: Vec<ParentPortalShellStatusCardSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteEventSnapshot {
    pub event: Option<String>,
    pub event_id: Option<String>,
    pub sent_at: Option<String>,
    pub severity: Option<String>,
    pub payload: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteLiveActivitySnapshot {
    pub recent_summary: Option<Value>,
    pub ingest_status: Option<Value>,
    pub activity_screen_read_model: Option<Value>,
    pub browser_managed_event: Option<ParentRouteEventSnapshot>,
    pub browser_managed_status: Option<Value>,
    pub browser_runtime_event_chain_stream: Option<Value>,
    pub browser_social_provider_receipt_stream_status_intent: Option<Value>,
    pub browser_social_provider_receipt_ingestion_readiness_status_intent: Option<Value>,
    pub local_ai_runtime_status_event: Option<ParentRouteEventSnapshot>,
    pub lan_ai_job_event: Option<ParentRouteEventSnapshot>,
    pub parent_assistant_boundary_event: Option<ParentRouteEventSnapshot>,
    pub activity_memory_graph_read_model: Option<Value>,
    pub network_flow_event: Option<ParentRouteEventSnapshot>,
    pub network_flow_read_model: Option<Value>,
    pub network_runtime_event_chain_stream: Option<Value>,
    pub lan_pairing_browser_discovery_event: Option<ParentRouteEventSnapshot>,
    pub lan_add_device_read_model: Option<Value>,
    pub policy_preview_event: Option<ParentRouteEventSnapshot>,
    pub policy_preview_read_model: Option<Value>,
    pub app_game_notification_parent_surface_intent_read_model: Option<Value>,
    pub app_game_policy_readiness_read_model: Option<Value>,
    pub app_game_platform_proof_status_read_model: Option<Value>,
    pub app_game_child_runtime_transport_receipt_read_model: Option<Value>,
    pub app_game_adapter_dispatch_preflight_read_model: Option<Value>,
    pub app_game_adapter_dispatch_result_read_model: Option<Value>,
    pub app_game_adapter_dispatch_executed_result: Option<Value>,
    pub app_game_timer_parent_surface_read_model: Option<Value>,
    pub app_game_timer_parent_preference_setup_requested_result: Option<Value>,
    pub browser_intervention_event: Option<ParentRouteEventSnapshot>,
    pub browser_intervention_read_model: Option<Value>,
    pub activity_tracking_read_model_event: Option<ParentRouteEventSnapshot>,
    pub activity_tracking_read_model: Option<Value>,
    pub activity_tracking_retention_settings_write_result: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteBrowserPanelsSnapshot {
    pub social_audit_explanation: Option<Value>,
    pub social_alert_report: Option<Value>,
    pub social_alert_report_parent_surface: Option<Value>,
    pub social_parent_notification_delivery: Option<Value>,
    pub social_dashboard: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteSnapshot {
    pub schema_version: u16,
    pub route: ParentRouteId,
    pub generated_at: String,
    pub season_label: String,
    pub last_updated: String,
    pub connection_state: ParentBridgeConnectionState,
    pub command_enabled: bool,
    pub agent_endpoint: String,
    pub data_source: ParentRouteDataSource,
    pub summary: ParentRouteSummary,
    pub diagnostic_panels_enabled: bool,
    pub parent_portal_rows: Option<Vec<ParentPortalRowSnapshot>>,
    pub parent_portal_shell_status: Option<ParentPortalShellStatusSnapshot>,
    pub live_activity: Option<ParentRouteLiveActivitySnapshot>,
    pub browser_panels: Option<ParentRouteBrowserPanelsSnapshot>,
    pub screen_settings_service_response: Option<Value>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentRouteSummary {
    pub title: String,
    pub route_capability: String,
    pub parent_access: String,
    pub household: String,
    pub child_device: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentUiActionKind {
    RefreshRoute,
    Reconnect,
    AgentCommandRequested,
    LanPairingBrowserDiscoveryScanRequested,
    NetworkFlowReadModelRefreshRequested,
    TrackingRetentionSettingsWriteRequested,
    ScreenSettingsGetRequested,
    ScreenSettingsReplaceRequested,
    AppGameAdapterDispatchExecuteRequested,
    AppGameTimerParentPreferenceSetupRequested,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentUiAction {
    pub action: ParentUiActionKind,
    pub route: ParentRouteId,
    pub command: Option<String>,
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentUiActionResult {
    pub schema_version: u16,
    pub accepted: bool,
    pub connection_state: ParentBridgeConnectionState,
    pub message: String,
    pub snapshot: Option<ParentRouteSnapshot>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentSubscriptionEvent {
    pub schema_version: u16,
    pub route: ParentRouteId,
    pub snapshot: ParentRouteSnapshot,
}
