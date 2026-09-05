use super::super::tracking_panel::activity_tracking_proof_panel_snapshot_impl;
use super::*;

use crate::agent_service_client::types::{
    AppUseReadModelAgentServiceSnapshot, GamesReadModelAgentServiceSnapshot,
};

pub(super) fn apply_tracking_and_screen_live_activity_impl(
    tracking_read_model_snapshot: Option<&TrackingReadModelAgentServiceSnapshot>,
    screen_read_model_snapshot: Option<&ScreenReadModelAgentServiceSnapshot>,
    app_use_read_model_snapshot: Option<&AppUseReadModelAgentServiceSnapshot>,
    games_read_model_snapshot: Option<&GamesReadModelAgentServiceSnapshot>,
    route: &ParentRouteId,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if let Some(tracking_read_model_snapshot) = tracking_read_model_snapshot {
        snapshot.activity_tracking_read_model_event =
            Some(tracking_read_model_snapshot.event.clone());
        snapshot.activity_tracking_read_model =
            Some(tracking_read_model_snapshot.read_model.clone());
    }
    if snapshot.activity_tracking_read_model.is_some() || route_requires_tracking_read_model(route)
    {
        let read_model = snapshot.activity_tracking_read_model.as_ref();
        let write_result = snapshot
            .activity_tracking_retention_settings_write_result
            .as_ref();
        snapshot.activity_tracking_panel = Some(match route {
            ParentRouteId::ProofPanels => {
                activity_tracking_proof_panel_snapshot_impl(read_model, write_result)
            }
            _ => activity_tracking_panel_snapshot(read_model, write_result),
        });
    }
    snapshot.activity_screen_read_model = screen_read_model_snapshot
        .and_then(|snapshot| activity_surface_adapter_value(&snapshot.read_model));
    if screen_read_model_snapshot.is_some()
        || matches!(
            route,
            ParentRouteId::Activity
                | ParentRouteId::CapabilityStatus
                | ParentRouteId::ScreenAnalysis
        )
    {
        snapshot.screen_summary_panel = Some(screen_summary_panel_snapshot(
            screen_read_model_snapshot.map(|snapshot| &snapshot.read_model),
        ));
    }
    snapshot.activity_app_use_read_model = app_use_read_model_snapshot
        .and_then(|snapshot| activity_surface_adapter_value(&snapshot.read_model));
    snapshot.activity_games_read_model = games_read_model_snapshot
        .and_then(|snapshot| activity_surface_adapter_value(&snapshot.read_model));
}

fn activity_surface_adapter_value<T>(read_model: &T) -> Option<Value>
where
    T: serde::Serialize,
{
    let value = serde_json::to_value(read_model).ok()?;
    let state = value.get("state")?.clone();
    Some(serde_json::json!({
        "ok": true,
        "state": state,
        "value": value,
    }))
}
