use ocentra_schema::parent_ui_bridge::ParentRouteContext;

use super::loaders;
use super::types::{
    AgentServiceResult, LanRuntimeReplaySnapshot, NetworkRuntimeEventChainAgentServiceSnapshot,
    PolicyPreviewAgentServiceSnapshot, TrackingReadModelAgentServiceSnapshot,
};

pub(crate) fn load_lan_runtime_event_chain_replay_events(
) -> AgentServiceResult<LanRuntimeReplaySnapshot> {
    loaders::load_lan_runtime_event_chain_replay_events()
}

pub(crate) fn load_network_runtime_event_chain_stream_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<NetworkRuntimeEventChainAgentServiceSnapshot> {
    loaders::load_network_runtime_event_chain_stream_snapshot(context)
}

pub(crate) fn load_policy_preview_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<PolicyPreviewAgentServiceSnapshot> {
    loaders::load_policy_preview_read_model_snapshot(context)
}

pub(crate) fn load_tracking_read_model_snapshot(
    context: Option<&ParentRouteContext>,
) -> AgentServiceResult<TrackingReadModelAgentServiceSnapshot> {
    loaders::load_tracking_read_model_snapshot(context)
}
