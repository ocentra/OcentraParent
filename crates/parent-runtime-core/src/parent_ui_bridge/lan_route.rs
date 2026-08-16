mod query_build;
mod query_view;
mod route_flags;
mod route_state;

use ocentra_parent_agent_protocol::{
    constants::lan_pairing::EVENT_BROWSER_DISCOVERY_REPORTED as LAN_DISCOVERY_REPORTED_EVENT,
    lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
};
use ocentra_schema::parent_ui_bridge::{
    ParentBridgeConnectionState, ParentRouteContext, ParentRouteDataSource,
    ParentRouteEventSnapshot, ParentRouteId, ParentUiAction, ParentUiActionKind,
};

use crate::agent_service_client::types::LanAgentServiceSnapshot;
use crate::agent_service_client::types::{AgentCommandText, AgentServiceError};
use crate::agent_service_client::{
    dispatch_lan_agent_command, load_lan_status_snapshot, request_lan_browser_discovery_scan,
};

use self::query_build::{lan_route_query_for_action_impl, lan_route_query_for_load_impl};
use self::query_view::{
    command_enabled_for_route_impl, connection_state_for_route_impl, data_source_for_route_impl,
};
use self::route_flags::{
    is_dev_tools_route_impl, is_lan_surface_route_impl, requires_lan_read_model,
};
pub(super) type LanRouteQuery = route_state::LanRouteQuery;

pub(super) fn data_source_for_route(
    route: &ParentRouteId,
    lan_route_query: &LanRouteQuery,
) -> ParentRouteDataSource {
    data_source_for_route_impl(route, lan_route_query)
}

pub(super) fn connection_state_for_route(
    route: &ParentRouteId,
    lan_route_query: &LanRouteQuery,
) -> ParentBridgeConnectionState {
    connection_state_for_route_impl(route, lan_route_query)
}

pub(super) fn command_enabled_for_route(
    route: &ParentRouteId,
    connection_state: &ParentBridgeConnectionState,
) -> bool {
    command_enabled_for_route_impl(route, connection_state)
}

pub(super) fn is_dev_tools_route(route: &ParentRouteId) -> bool {
    is_dev_tools_route_impl(route)
}

pub(super) fn is_lan_surface_route(route: &ParentRouteId) -> bool {
    is_lan_surface_route_impl(route)
}

pub(super) fn lan_route_query_for_load(
    route: &ParentRouteId,
    context: Option<&ParentRouteContext>,
) -> LanRouteQuery {
    lan_route_query_for_load_impl(route, context)
}

pub(super) fn lan_route_query_for_action(action: &ParentUiAction) -> LanRouteQuery {
    lan_route_query_for_action_impl(action)
}
