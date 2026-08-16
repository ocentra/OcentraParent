use super::*;
use ocentra_schema::parent_ui_bridge::ParentRouteEventSnapshot;

pub(super) struct ActionDispatchState {
    pub(super) accepted: bool,
    pub(super) message: String,
    pub(super) events: Vec<ParentRouteEventSnapshot>,
    pub(super) network_flow_snapshot: Option<NetworkFlowAgentServiceSnapshot>,
    pub(super) snapshot_overlay: ParentRouteSnapshotOverlay,
}

impl ActionDispatchState {
    pub(super) fn new(
        accepted: bool,
        message: String,
        events: Vec<ParentRouteEventSnapshot>,
    ) -> Self {
        Self {
            accepted,
            message,
            events,
            network_flow_snapshot: None,
            snapshot_overlay: ParentRouteSnapshotOverlay::default(),
        }
    }

    pub(super) fn reject(&mut self, message: impl Into<String>) {
        self.accepted = false;
        self.message = message.into();
        self.events.clear();
    }
}
