use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_schema::parent_ui_bridge::{
    ParentBridgeConnectionState, ParentRouteContext, ParentRouteDataSource,
    ParentRouteEventSnapshot, ParentRouteId, ParentUiAction, ParentUiActionKind,
};

use crate::agent_service_client::types::LanAgentServiceSnapshot;
use crate::agent_service_client::{
    dispatch_lan_agent_command, load_lan_status_snapshot, request_lan_browser_discovery_scan,
};

use super::LAN_DISCOVERY_REPORTED_EVENT;

pub(super) enum LanRouteQuery {
    NotRequired,
    Available(Box<LanAgentServiceSnapshot>),
    Unavailable(String),
}

pub(super) fn data_source_for_route(
    route: &ParentRouteId,
    lan_route_query: &LanRouteQuery,
) -> ParentRouteDataSource {
    if is_dev_tools_route(route) {
        return ParentRouteDataSource::DevDiagnostics;
    }
    if is_lan_surface_route(route) {
        return if matches!(lan_route_query, LanRouteQuery::Available(_)) {
            ParentRouteDataSource::RustReadModel
        } else {
            ParentRouteDataSource::Unavailable
        };
    }
    ParentRouteDataSource::HostBridge
}

pub(super) fn connection_state_for_route(
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

pub(super) fn command_enabled_for_route(
    route: &ParentRouteId,
    connection_state: &ParentBridgeConnectionState,
) -> bool {
    if is_lan_surface_route(route) {
        return matches!(connection_state, ParentBridgeConnectionState::Connected);
    }
    true
}

pub(super) fn is_dev_tools_route(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::Diagnostics
            | ParentRouteId::ProofPanels
            | ParentRouteId::Commands
            | ParentRouteId::Events
            | ParentRouteId::Logs
            | ParentRouteId::AppLayout
            | ParentRouteId::FrameTuner
    )
}

pub(super) fn is_lan_surface_route(route: &ParentRouteId) -> bool {
    matches!(
        route,
        ParentRouteId::Devices
            | ParentRouteId::LanPairing
            | ParentRouteId::CapabilityStatus
            | ParentRouteId::PlatformsInstall
            | ParentRouteId::InstallUpdates
    )
}

fn requires_lan_read_model(route: &ParentRouteId) -> bool {
    is_lan_surface_route(route) || matches!(route, ParentRouteId::PolicyNetwork)
}

pub(super) fn lan_route_query_for_load(
    route: &ParentRouteId,
    context: Option<&ParentRouteContext>,
) -> LanRouteQuery {
    if !requires_lan_read_model(route) {
        return LanRouteQuery::NotRequired;
    }
    match load_lan_status_snapshot(context) {
        Ok(snapshot) => LanRouteQuery::Available(Box::new(snapshot)),
        Err(error) => LanRouteQuery::Unavailable(error),
    }
}

pub(super) fn lan_route_query_for_action(action: &ParentUiAction) -> LanRouteQuery {
    if !requires_lan_read_model(&action.route) {
        return LanRouteQuery::NotRequired;
    }
    let context = action.context.as_ref();
    let response = match action.action {
        ParentUiActionKind::AgentCommandRequested => action
            .command
            .as_deref()
            .ok_or_else(|| {
                "parent Rust facade rejected LAN agent command request without a command name"
                    .to_string()
            })
            .and_then(|command_name| {
                dispatch_lan_agent_command(command_name, &action.payload, context)
            }),
        ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested => {
            request_lan_browser_discovery_scan(context)
        }
        _ => load_lan_status_snapshot(context),
    };
    match response {
        Ok(snapshot) => LanRouteQuery::Available(Box::new(snapshot)),
        Err(error) => LanRouteQuery::Unavailable(error),
    }
}

impl LanRouteQuery {
    pub(super) fn read_model(&self) -> Option<&LanBrowserAddDeviceReadModel> {
        match self {
            Self::Available(snapshot) => Some(&snapshot.read_model),
            Self::NotRequired | Self::Unavailable(_) => None,
        }
    }

    pub(super) fn event(&self) -> Option<&ParentRouteEventSnapshot> {
        match self {
            Self::Available(snapshot) => Some(&snapshot.event),
            Self::NotRequired | Self::Unavailable(_) => None,
        }
    }

    pub(super) fn events(&self) -> &[ParentRouteEventSnapshot] {
        match self {
            Self::Available(snapshot) => snapshot.events.as_slice(),
            Self::NotRequired | Self::Unavailable(_) => &[],
        }
    }

    pub(super) fn discovery_event(&self) -> Option<&ParentRouteEventSnapshot> {
        self.event()
            .filter(|event| event.event.as_deref() == Some(LAN_DISCOVERY_REPORTED_EVENT))
    }
}
