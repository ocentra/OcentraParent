use crate::agent_service_client::{
    load_activity_screen_read_model_snapshot, load_tracking_read_model_snapshot,
};
use crate::parent_ui_bridge::route_requirements::{
    route_requires_screen_summary_read_model, route_requires_tracking_read_model,
};
use crate::parent_ui_bridge::route_snapshot::dependencies::{
    ScreenReadModelAgentServiceSnapshot, TrackingReadModelAgentServiceSnapshot,
};
use crate::parent_ui_bridge::ParentRouteId;

pub(super) struct ActivityDependencies {
    pub(super) tracking_read_model_snapshot: Option<TrackingReadModelAgentServiceSnapshot>,
    pub(super) screen_read_model_snapshot: Option<ScreenReadModelAgentServiceSnapshot>,
}

pub(super) fn load(route: &ParentRouteId) -> ActivityDependencies {
    let tracking_read_model_snapshot = if route_requires_tracking_read_model(route) {
        load_tracking_read_model_snapshot(None).ok()
    } else {
        None
    };
    let screen_read_model_snapshot = if route_requires_screen_summary_read_model(route) {
        load_activity_screen_read_model_snapshot(None).ok()
    } else {
        None
    };
    ActivityDependencies {
        tracking_read_model_snapshot,
        screen_read_model_snapshot,
    }
}
