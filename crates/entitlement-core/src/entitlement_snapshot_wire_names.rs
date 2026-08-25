#![forbid(unsafe_code)]

use crate::entitlement_snapshot_values::{
    EntitlementSnapshotPlanTier, EntitlementSnapshotReleaseChannel,
};

pub(super) fn plan_tier_wire_name(plan_tier: EntitlementSnapshotPlanTier) -> &'static str {
    match plan_tier {
        EntitlementSnapshotPlanTier::Starter => "starter",
        EntitlementSnapshotPlanTier::Paid => "paid",
    }
}

pub(super) fn release_channel_wire_name(
    channel: EntitlementSnapshotReleaseChannel,
) -> &'static str {
    match channel {
        EntitlementSnapshotReleaseChannel::Stable => "stable",
        EntitlementSnapshotReleaseChannel::Beta => "beta",
        EntitlementSnapshotReleaseChannel::Preview => "preview",
        EntitlementSnapshotReleaseChannel::Development => "development",
    }
}
