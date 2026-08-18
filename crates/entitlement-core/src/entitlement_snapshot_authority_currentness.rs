#![forbid(unsafe_code)]

//! Durable revocation-generation and active-time checks for snapshots.

use chrono::{DateTime, Utc};

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
    pub(crate) authority_generation: u64,
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
            authority_generation: update.authority_generation,
        });
    }
    if update.authority_generation != snapshot.authority_generation
        || update.revocation_cursor != snapshot.revocation_cursor
    {
        return Ok(SnapshotCurrentness {
            freshness: EntitlementSnapshotFreshnessState::Stale,
            authority_generation: update.authority_generation,
        });
    }
    let issued_at = parse_timestamp(&snapshot.issued_at)?;
    let expires_at = parse_timestamp(&snapshot.expires_at)?;
    let now = Utc::now();
    if now < issued_at {
        return Err(EntitlementSnapshotVerificationFailure::NotYetValid);
    }
    if now >= expires_at {
        return expired_or_grace(
            now,
            snapshot.grace_until.as_deref(),
            update.authority_generation,
        );
    }
    Ok(SnapshotCurrentness {
        freshness: EntitlementSnapshotFreshnessState::Fresh,
        authority_generation: update.authority_generation,
    })
}

fn expired_or_grace(
    now: DateTime<Utc>,
    grace_until: Option<&str>,
    authority_generation: u64,
) -> Result<SnapshotCurrentness, EntitlementSnapshotVerificationFailure> {
    if grace_until
        .map(parse_timestamp)
        .transpose()?
        .is_some_and(|grace_until| now < grace_until)
    {
        return Ok(SnapshotCurrentness {
            freshness: EntitlementSnapshotFreshnessState::Grace,
            authority_generation,
        });
    }
    Ok(SnapshotCurrentness {
        freshness: EntitlementSnapshotFreshnessState::Expired,
        authority_generation,
    })
}

fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, EntitlementSnapshotVerificationFailure> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|_error| EntitlementSnapshotVerificationFailure::InvalidSnapshotShape)
}
