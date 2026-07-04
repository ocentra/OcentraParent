use super::*;

pub(super) fn apply_lan_live_activity_impl(
    lan_route_query: &LanRouteQuery,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if let Some(read_model) = lan_route_query.read_model() {
        snapshot.lan_add_device_read_model =
            current_lan_add_device_read_model_value(Some(read_model));
    }
    if let Some(event) = lan_route_query.discovery_event() {
        snapshot.lan_pairing_browser_discovery_event = Some(event.clone());
    }
}
