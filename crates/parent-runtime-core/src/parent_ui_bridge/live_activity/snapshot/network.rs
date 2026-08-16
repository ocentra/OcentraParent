use super::*;

pub(super) fn apply_network_live_activity_impl(
    policy_preview_snapshot: Option<&PolicyPreviewAgentServiceSnapshot>,
    parent_access_state: &ParentPortalParentAccessState,
    network_flow_snapshot: Option<&NetworkFlowAgentServiceSnapshot>,
    network_runtime_event_chain_snapshot: Option<&NetworkRuntimeEventChainAgentServiceSnapshot>,
    route: &ParentRouteId,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if let Some(network_flow_snapshot) = network_flow_snapshot {
        snapshot.network_flow_event = Some(network_flow_snapshot.event.clone());
        snapshot.network_flow_read_model = Some(network_flow_read_model_snapshot(
            &network_flow_snapshot.read_model,
        ));
    }
    if let Some(network_runtime_event_chain_snapshot) = network_runtime_event_chain_snapshot {
        snapshot.network_runtime_event_chain_stream =
            Some(network_runtime_event_chain_snapshot.stream.clone());
    }
    if policy_preview_snapshot.is_some() || route_requires_policy_preview_read_model(route) {
        snapshot.policy_preview_panel = Some(policy_preview_panel_snapshot(
            policy_preview_snapshot
                .as_ref()
                .map(|snapshot| &snapshot.event),
            policy_preview_snapshot
                .as_ref()
                .map(|snapshot| &snapshot.read_model),
            parent_access_state,
        ));
    }
    snapshot.network_evidence_summary = network_evidence_summary_snapshot(
        snapshot.network_runtime_event_chain_stream.as_ref(),
        policy_preview_snapshot
            .as_ref()
            .map(|snapshot| &snapshot.read_model),
    );
}
