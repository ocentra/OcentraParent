use super::*;

pub(super) fn apply_browser_live_activity_impl(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if !matches!(
        input.route,
        ParentRouteId::Activity | ParentRouteId::Browser
    ) {
        return;
    }

    if let Some(activity) = input.browser_activity_read_model_snapshot {
        snapshot.activity_browser_read_model = activity_surface_adapter_value(&activity.read_model);
    }
    if !matches!(input.route, ParentRouteId::Browser) {
        return;
    }

    if let Some(status) = input.browser_managed_status_snapshot {
        snapshot.browser_managed_event = Some(status.event.clone());
        snapshot.browser_managed_status = serde_json::to_value(&status.status).ok();
    }
    if let Some(intervention) = input.browser_intervention_read_model_snapshot {
        snapshot.browser_intervention_event = Some(intervention.event.clone());
        snapshot.browser_intervention_read_model =
            serde_json::to_value(&intervention.read_model).ok();
    }
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
