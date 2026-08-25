#![forbid(unsafe_code)]

use crate::entitlement_snapshot::{
    EntitlementSnapshotShapeError, SignedEntitlementSnapshot, ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION,
};

use super::{
    verifier_request::EntitlementSnapshotVerificationRequest,
    EntitlementSnapshotVerificationFailure,
};

pub(super) fn validate_snapshot_binding(
    snapshot: &SignedEntitlementSnapshot,
    request: &EntitlementSnapshotVerificationRequest,
) -> Result<(), EntitlementSnapshotVerificationFailure> {
    snapshot.validate_shape().map_err(map_shape_error)?;
    if snapshot.schema_version != ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION {
        return Err(EntitlementSnapshotVerificationFailure::InvalidSnapshotShape);
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
    if snapshot.package_build_ref != request.package_build_ref
        || snapshot.release_channel != request.release_channel
    {
        return Err(EntitlementSnapshotVerificationFailure::WrongPackageBuild);
    }
    Ok(())
}

fn map_shape_error(error: EntitlementSnapshotShapeError) -> EntitlementSnapshotVerificationFailure {
    match error {
        EntitlementSnapshotShapeError::InvalidSignatureLength => {
            EntitlementSnapshotVerificationFailure::MissingSignature
        }
        _ => EntitlementSnapshotVerificationFailure::InvalidSnapshotShape,
    }
}
