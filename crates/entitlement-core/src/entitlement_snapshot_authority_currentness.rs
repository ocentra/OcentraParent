#![forbid(unsafe_code)]

//! Durable revocation-generation and active-time checks for snapshots.

use crate::{
    entitlement_snapshot::SignedEntitlementSnapshot,
    entitlement_snapshot_values::EntitlementSnapshotFreshnessState,
};

use super::{
    revocation::{map_cache_error, verify_revocation_update},
    EntitlementSnapshotAuthority, EntitlementSnapshotVerificationFailure,
};

pub(crate) struct SnapshotCurrentness {
    pub(crate) freshness: EntitlementSnapshotFreshnessState,
}

pub(crate) fn currentness(
    authority: &EntitlementSnapshotAuthority,
    snapshot: &SignedEntitlementSnapshot,
) -> Result<SnapshotCurrentness, EntitlementSnapshotVerificationFailure> {
    let update = authority
        .revocation_state
        .read_signed()
        .map_err(map_cache_error)?
        .ok_or(EntitlementSnapshotVerificationFailure::AuthorityUnavailable)?;
    verify_revocation_update(&update, authority.key_provider.as_ref())?;
    authority
        .currentness
        .validate_revocation_generation(update.authority_generation)?;
    if update
        .revoked_snapshot_ids
        .iter()
        .any(|snapshot_id| snapshot_id == &snapshot.snapshot_id)
    {
        return Ok(SnapshotCurrentness {
            freshness: EntitlementSnapshotFreshnessState::Revoked,
        });
    }
    if update.authority_generation != snapshot.authority_generation
        || update.revocation_cursor != snapshot.revocation_cursor
    {
        return Ok(SnapshotCurrentness {
            freshness: EntitlementSnapshotFreshnessState::Stale,
        });
    }

    let freshness = authority
        .currentness
        .evaluate_snapshot_freshness(snapshot, &update)?;
    Ok(SnapshotCurrentness { freshness })
}
