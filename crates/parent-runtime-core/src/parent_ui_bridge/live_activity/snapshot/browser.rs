use super::*;

pub(super) fn apply_browser_live_activity_impl(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
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
