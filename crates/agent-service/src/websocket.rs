use axum::extract::ws::WebSocket;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};
use std::{future::Future, pin::Pin, sync::Arc};

#[path = "websocket/activity_app_game_action_reports.rs"]
mod activity_app_game_action_reports;
#[path = "websocket/activity_app_game_command_reports.rs"]
mod activity_app_game_command_reports;
#[path = "websocket/activity_app_game_read_model_reports.rs"]
mod activity_app_game_read_model_reports;
#[path = "websocket/activity_command_reports.rs"]
mod activity_command_reports;
#[path = "websocket/activity_social_reports.rs"]
mod activity_social_reports;
#[path = "websocket/activity_summary_reports.rs"]
mod activity_summary_reports;
#[path = "websocket/activity_surface_command_reports.rs"]
mod activity_surface_command_reports;
#[path = "websocket/ai_command_reports.rs"]
mod ai_command_reports;
#[path = "websocket/basic_reports.rs"]
mod basic_reports;
#[path = "websocket/browser_command_reports.rs"]
mod browser_command_reports;
#[path = "websocket/browser_network_command_reports.rs"]
mod browser_network_command_reports;
#[path = "websocket/command_classifiers.rs"]
mod command_classifiers;
#[path = "websocket/command_dispatch.rs"]
mod command_dispatch;
#[path = "websocket/command_entry.rs"]
mod command_entry;
#[path = "websocket/enforcement_command_reports.rs"]
mod enforcement_command_reports;
#[path = "websocket/health_nonce.rs"]
mod health_nonce;
#[path = "websocket/lan_command_reports.rs"]
mod lan_command_reports;
#[path = "websocket/network_command_reports.rs"]
mod network_command_reports;
#[path = "websocket/parent_runtime_intent.rs"]
mod parent_runtime_intent;
#[path = "websocket/policy_request_confirm.rs"]
mod policy_request_confirm;
#[path = "websocket/policy_request_resolution.rs"]
mod policy_request_resolution;
#[path = "websocket/socket_session.rs"]
mod socket_session;
#[path = "websocket/tracking_retention_settings_write.rs"]
mod tracking_retention_settings_write;
#[path = "websocket/transport_admission.rs"]
mod transport_admission;

use self::basic_reports::{
    temp_runtime_store_path, BROWSER_POLICY_TEST_STORE_PREFIX, SCREEN_SETTINGS_TEST_STORE_PREFIX,
};
use crate::activity_api::app_game_platform_probe_cache::PlatformProbeCache;
use crate::activity_api::app_game_platform_proof_status_payload::platform_probe_dispatcher;
use crate::parent_local_bridge_admission::ParentLocalBridgeAdmission;
use crate::{
    browser_policy_runtime::BrowserPolicyRuntime, browser_runtime::BrowserManagedRuntime,
    lan_pairing::LanPairingRuntime, screen_settings_runtime::ScreenSettingsRuntime,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebsocketCommandText(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebsocketBrowserPolicyStorePath(pub std::path::PathBuf);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WebsocketCommandOrigin(pub(crate) Option<String>);

/// Server-derived peer provenance used at the WebSocket command boundary.
/// Envelope fields cannot construct or override this value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WebsocketPeerProvenance {
    Loopback,
    LocalNetwork,
    Unknown,
}

pub(crate) type WebsocketPlatformProbeDispatcher = dyn Fn(
        AgentCommandEnvelope,
        WebsocketPeerProvenance,
    ) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>>
    + Send
    + Sync;

pub(crate) fn handle_command_text_for_test(
    text: WebsocketCommandText,
    lan_pairing: LanPairingRuntime,
    origin: WebsocketCommandOrigin,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    command_entry::handle_command_text(
        text,
        lan_pairing,
        BrowserPolicyRuntime::for_store_path(
            temp_runtime_store_path(BROWSER_POLICY_TEST_STORE_PREFIX).0,
        ),
        BrowserManagedRuntime::new(),
        ScreenSettingsRuntime::for_store_path(
            temp_runtime_store_path(SCREEN_SETTINGS_TEST_STORE_PREFIX).0,
        ),
        origin,
        platform_probe_dispatcher(PlatformProbeCache::new()),
        WebsocketPeerProvenance::Loopback,
    )
}

pub(crate) fn handle_command_text_with_browser_policy_for_test(
    text: WebsocketCommandText,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    origin: WebsocketCommandOrigin,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    command_entry::handle_command_text(
        text,
        lan_pairing,
        browser_policy,
        BrowserManagedRuntime::new(),
        ScreenSettingsRuntime::for_store_path(
            temp_runtime_store_path(SCREEN_SETTINGS_TEST_STORE_PREFIX).0,
        ),
        origin,
        platform_probe_dispatcher(PlatformProbeCache::new()),
        WebsocketPeerProvenance::Loopback,
    )
}

pub(crate) fn dispatch_local_command_text(
    text: WebsocketCommandText,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    command_entry::handle_command_text(
        text,
        LanPairingRuntime::empty(),
        BrowserPolicyRuntime::for_store_path(
            temp_runtime_store_path(BROWSER_POLICY_TEST_STORE_PREFIX).0,
        ),
        BrowserManagedRuntime::new(),
        ScreenSettingsRuntime::for_store_path(
            temp_runtime_store_path(SCREEN_SETTINGS_TEST_STORE_PREFIX).0,
        ),
        WebsocketCommandOrigin(None),
        platform_probe_dispatcher(PlatformProbeCache::new()),
        WebsocketPeerProvenance::Loopback,
    )
}

pub(crate) fn dispatch_local_command_text_with_browser_policy_store(
    text: WebsocketCommandText,
    store_path: WebsocketBrowserPolicyStorePath,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    command_entry::handle_command_text(
        text,
        LanPairingRuntime::empty(),
        BrowserPolicyRuntime::for_store_path(store_path.0),
        BrowserManagedRuntime::new(),
        ScreenSettingsRuntime::for_store_path(
            temp_runtime_store_path(SCREEN_SETTINGS_TEST_STORE_PREFIX).0,
        ),
        WebsocketCommandOrigin(None),
        platform_probe_dispatcher(PlatformProbeCache::new()),
        WebsocketPeerProvenance::Loopback,
    )
}

pub(crate) fn handle_socket(
    socket: WebSocket,
    lan_pairing: LanPairingRuntime,
    browser_policy: BrowserPolicyRuntime,
    browser_runtime: BrowserManagedRuntime,
    screen_settings: ScreenSettingsRuntime,
    origin: WebsocketCommandOrigin,
    probe_dispatcher: Arc<WebsocketPlatformProbeDispatcher>,
    provenance: WebsocketPeerProvenance,
    admission: ParentLocalBridgeAdmission,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    socket_session::handle_socket(
        socket,
        lan_pairing,
        browser_policy,
        browser_runtime,
        screen_settings,
        origin,
        probe_dispatcher,
        provenance,
        admission,
    )
}
