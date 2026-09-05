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
#[path = "websocket/socket_handshake.rs"]
mod socket_handshake;
#[path = "websocket/socket_session.rs"]
mod socket_session;
#[path = "websocket/tracking_retention_settings_write.rs"]
mod tracking_retention_settings_write;
#[path = "websocket/transport_admission.rs"]
mod transport_admission;

use crate::parent_local_bridge_admission::ParentLocalBridgeAdmission;
use crate::{
    browser_policy_runtime::BrowserPolicyRuntime, browser_runtime::BrowserManagedRuntime,
    lan_pairing::LanPairingRuntime, screen_settings_runtime::ScreenSettingsRuntime,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebsocketCommandText(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WebsocketCommandOrigin(pub(crate) Option<String>);

/// Server-derived peer provenance used at the WebSocket command boundary.
/// Envelope fields cannot construct or override this value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WebsocketPeerProvenance {
    Loopback,
    LocalNetwork,
}

pub(crate) type WebsocketPlatformProbeDispatcher = dyn Fn(
        AgentCommandEnvelope,
        WebsocketPeerProvenance,
    ) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>>
    + Send
    + Sync;

#[derive(Clone)]
pub(crate) struct WebsocketCommandRuntime {
    pub(crate) lan_pairing: LanPairingRuntime,
    pub(crate) browser_policy: BrowserPolicyRuntime,
    pub(crate) browser_runtime: BrowserManagedRuntime,
    pub(crate) screen_settings: ScreenSettingsRuntime,
    pub(crate) origin: WebsocketCommandOrigin,
    pub(crate) probe_dispatcher: Arc<WebsocketPlatformProbeDispatcher>,
    pub(crate) provenance: WebsocketPeerProvenance,
}

pub(crate) struct WebsocketSocketRuntime {
    pub(crate) command: WebsocketCommandRuntime,
    pub(crate) admission: ParentLocalBridgeAdmission,
}

fn handle_command_text(
    text: WebsocketCommandText,
    runtime: WebsocketCommandRuntime,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    command_entry::handle_command_text(text, runtime)
}

pub(crate) fn handle_socket(
    socket: WebSocket,
    runtime: WebsocketSocketRuntime,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    socket_session::handle_socket(socket, runtime)
}
