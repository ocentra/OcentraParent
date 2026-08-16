use super::*;

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
        snapshot.activity_tracking_panel = Some(activity_tracking_panel_snapshot(
            snapshot.activity_tracking_read_model.as_ref(),
            snapshot
                .activity_tracking_retention_settings_write_result
                .as_ref(),
        ));
    }
    if screen_read_model_snapshot.is_some() || matches!(route, ParentRouteId::ScreenAnalysis) {
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
