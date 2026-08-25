#![forbid(unsafe_code)]

use crate::entitlement_snapshot::SignedEntitlementSnapshot;
use crate::entitlement_snapshot_values::EntitlementSnapshotFreshnessState;

use super::{
    currentness::SnapshotCurrentness, EntitlementSnapshotAuthority,
    EntitlementSnapshotVerificationFailure,
};

pub(super) fn verify_snapshot_currentness(
    authority: &EntitlementSnapshotAuthority,
    snapshot: &SignedEntitlementSnapshot,
) -> Result<SnapshotCurrentness, EntitlementSnapshotVerificationFailure> {
    let currentness = super::currentness::currentness(authority, snapshot)?;
    match currentness.freshness {
        EntitlementSnapshotFreshnessState::Fresh | EntitlementSnapshotFreshnessState::Grace => {
            Ok(currentness)
        }
        EntitlementSnapshotFreshnessState::Stale => {
            Err(EntitlementSnapshotVerificationFailure::Stale)
        }
        EntitlementSnapshotFreshnessState::Expired => {
            Err(EntitlementSnapshotVerificationFailure::Expired)
        }
        EntitlementSnapshotFreshnessState::Revoked => {
            Err(EntitlementSnapshotVerificationFailure::Revoked)
        }
    }
}
