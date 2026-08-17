use chrono::{DateTime, FixedOffset};

use super::{
    EntitlementSnapshotVerificationFailure, EntitlementSnapshotVerificationRequest,
    SignedEntitlementSnapshot, ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION,
};

pub(super) fn validate_snapshot_shape(
    snapshot: &SignedEntitlementSnapshot,
    request: &EntitlementSnapshotVerificationRequest,
) -> Result<(), EntitlementSnapshotVerificationFailure> {
    if snapshot.schema_version != ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION {
        return Err(EntitlementSnapshotVerificationFailure::InvalidSnapshotShape);
    }
    if snapshot.signature.is_empty() {
        return Err(EntitlementSnapshotVerificationFailure::MissingSignature);
    }
    if snapshot.account_ref != request.account_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongAccount);
    }
    if snapshot.household_ref != request.household_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongHousehold);
    }
    if snapshot.trusted_device_ref != request.trusted_device_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongDevice);
    }
    if snapshot.package_build_ref != request.package_build_ref {
        return Err(EntitlementSnapshotVerificationFailure::WrongPackageBuild);
    }

    let issued_at = parse_snapshot_timestamp(&snapshot.issued_at)?;
    let expires_at = parse_snapshot_timestamp(&snapshot.expires_at)?;
    let observed_at = parse_snapshot_timestamp(&request.observed_at)?;
    if issued_at > observed_at || observed_at >= expires_at {
        return Err(EntitlementSnapshotVerificationFailure::Expired);
    }
    if let Some(grace_until) = snapshot.grace_until.as_deref() {
        if parse_snapshot_timestamp(grace_until)? < expires_at {
            return Err(EntitlementSnapshotVerificationFailure::InvalidSnapshotShape);
        }
    }
    Ok(())
}

fn parse_snapshot_timestamp(
    value: &str,
) -> Result<DateTime<FixedOffset>, EntitlementSnapshotVerificationFailure> {
    DateTime::parse_from_rfc3339(value)
        .map_err(|_error| EntitlementSnapshotVerificationFailure::TimestampInvalid)
}
