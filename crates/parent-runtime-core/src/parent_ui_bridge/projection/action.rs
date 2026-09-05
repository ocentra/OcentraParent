use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_schema::parent_ui_bridge::{ParentUiAction, ParentUiActionKind, ParentUiActionResult};

use crate::agent_service_client::snapshots_lan::network_flow_snapshot_from_result;

use super::super::route_snapshot::build_parent_route_snapshot_from_dependencies;
use super::super::snapshot_overlay::{
    apply_snapshot_overlay_for_action, rust_owned_command_for_action,
};
use super::ParentAgentServiceProjection;

impl ParentAgentServiceProjection {
    /// Projects an already-observed typed action response and subsequent route
    /// read models. This does not dispatch the action or authorize transport;
    /// it only applies the same response overlay and route assembler used by
    /// production after an authenticated command completes.
    pub fn action_result(
        mut self,
        action: &ParentUiAction,
    ) -> Result<ParentUiActionResult, String> {
        let command = projected_action_command(action)?;
        let result = self.take(&command)?;
        let accepted = !result.is_rejected();
        let message = result
            .rejection_message()
            .map(|error| error.to_string())
            .unwrap_or_else(|| super::super::presentation::action_result_message(action));
        let mut snapshot_overlay = super::super::ParentRouteSnapshotOverlay::default();
        let (network_flow_snapshot, events) = if matches!(
            action.action,
            ParentUiActionKind::NetworkFlowReadModelRefreshRequested
        ) {
            let snapshot = network_flow_snapshot_from_result(result)?;
            let events = snapshot.events.clone();
            (Some(snapshot), events)
        } else {
            apply_snapshot_overlay_for_action(&action.action, &result, &mut snapshot_overlay)?;
            (None, result.events)
        };
        let lan_route_query = self.project_lan_query(&action.route);
        let dependencies = self.project_dependencies(&action.route, network_flow_snapshot);
        let snapshot = build_parent_route_snapshot_from_dependencies(
            action.route.clone(),
            &lan_route_query,
            None,
            Some(&snapshot_overlay),
            None,
            dependencies,
        );
        Ok(ParentUiActionResult {
            schema_version: super::super::PARENT_UI_BRIDGE_SCHEMA_VERSION,
            accepted,
            connection_state: snapshot.connection_state.clone(),
            message,
            snapshot: Some(snapshot),
            events,
        })
    }
}

fn projected_action_command(action: &ParentUiAction) -> Result<AgentCommandName, String> {
    if matches!(
        action.action,
        ParentUiActionKind::NetworkFlowReadModelRefreshRequested
    ) {
        return Ok(AgentCommandName::AgentNetworkFlowReadModelGet);
    }
    rust_owned_command_for_action(&action.action).ok_or_else(|| {
        "parent projection supports only typed Rust-owned action responses".to_string()
    })
}
