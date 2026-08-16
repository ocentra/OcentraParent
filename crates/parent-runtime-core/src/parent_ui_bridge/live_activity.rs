#[path = "live_activity/snapshot.rs"]
mod snapshot;
#[path = "live_activity/tracking_panel.rs"]
mod tracking_panel;

use self::snapshot::live_activity_snapshot_impl;
use self::tracking_panel::activity_tracking_panel_snapshot_impl;
use super::*;
use crate::parent_ui_bridge::ParentRouteLiveActivitySnapshotInput;

pub(super) fn live_activity_snapshot(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
) -> Option<ParentRouteLiveActivitySnapshot> {
    live_activity_snapshot_impl(input)
}

pub(super) const SCREEN_SUMMARY_DETAIL_SEPARATOR: &str = " | ";
pub(super) const SCREEN_SUMMARY_NOT_REPORTED: &str = "Not reported";
pub(super) const SCREEN_SUMMARY_UNAVAILABLE: &str = "Unavailable";
