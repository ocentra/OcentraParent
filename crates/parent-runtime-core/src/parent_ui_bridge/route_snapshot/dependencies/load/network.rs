use crate::agent_service_client::load_network_flow_read_model_snapshot;
use crate::agent_service_client::read_model_loaders::{
    load_network_runtime_event_chain_stream_snapshot, load_policy_preview_read_model_snapshot,
};
use crate::parent_ui_bridge::route_requirements::{
    route_requires_network_flow_read_model, route_requires_network_runtime_event_chain_stream,
    route_requires_policy_preview_read_model,
};
use crate::parent_ui_bridge::route_snapshot::dependencies::{
    DependencyFailures, NetworkFlowAgentServiceSnapshot,
    NetworkRuntimeEventChainAgentServiceSnapshot, PolicyPreviewAgentServiceSnapshot,
};
use crate::parent_ui_bridge::ParentRouteId;

pub(super) struct NetworkDependencies {
    pub(super) network_flow_snapshot: Option<NetworkFlowAgentServiceSnapshot>,
    pub(super) network_runtime_event_chain_snapshot:
        Option<NetworkRuntimeEventChainAgentServiceSnapshot>,
    pub(super) policy_preview_snapshot: Option<PolicyPreviewAgentServiceSnapshot>,
}

pub(super) fn load(
    route: &ParentRouteId,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
    failures: &mut DependencyFailures,
) -> NetworkDependencies {
    let loaded_network_flow_snapshot =
        if network_flow_snapshot.is_none() && route_requires_network_flow_read_model(route) {
            failures.capture(
                "network-flow-read-model",
                load_network_flow_read_model_snapshot(None),
            )
        } else {
            None
        };
    let effective_network_flow_snapshot =
        network_flow_snapshot.or(loaded_network_flow_snapshot.as_ref());
    let network_runtime_event_chain_snapshot = if effective_network_flow_snapshot.is_some()
        || route_requires_network_runtime_event_chain_stream(route)
    {
        failures.capture(
            "network-runtime-event-chain",
            load_network_runtime_event_chain_stream_snapshot(None),
        )
    } else {
        None
    };
    let policy_preview_snapshot = if effective_network_flow_snapshot.is_some()
        || route_requires_policy_preview_read_model(route)
    {
        failures.capture(
            "policy-preview-read-model",
            load_policy_preview_read_model_snapshot(None),
        )
    } else {
        None
    };
    NetworkDependencies {
        network_flow_snapshot: loaded_network_flow_snapshot,
        network_runtime_event_chain_snapshot,
        policy_preview_snapshot,
    }
}
