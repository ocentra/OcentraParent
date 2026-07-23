use crate::device_trust_registry::{
    DeviceTrustLifecycleState, DeviceTrustRegistryDecision, DeviceTrustRegistryFailure,
    DeviceTrustRegistryRecord, DeviceTrustRegistryRejection,
};

pub(super) fn record_from_row(
    device_id: &str,
    state: &str,
) -> Result<DeviceTrustRegistryRecord, DeviceTrustRegistryFailure> {
    let state = match state {
        "pending-sealing" => DeviceTrustLifecycleState::PendingSealing,
        // WP01 has no platform-sealing receipt verifier. A preexisting trusted
        // row is hostile/unverifiable rather than a trust grant.
        "trusted" => return Err(DeviceTrustRegistryFailure::StorageIntegrityRejected),
        "revoked" => DeviceTrustLifecycleState::Revoked,
        "reset-required" => DeviceTrustLifecycleState::ResetRequired,
        _ => return Err(DeviceTrustRegistryFailure::StorageIntegrityRejected),
    };
    Ok(DeviceTrustRegistryRecord {
        device_id: device_id.to_owned(),
        state,
    })
}

pub(super) fn journal_fields(
    decision: &DeviceTrustRegistryDecision,
) -> (&'static str, &'static str) {
    match decision {
        DeviceTrustRegistryDecision::PendingSealing(_) => ("accepted", "pending-sealing"),
        DeviceTrustRegistryDecision::Revoked(_) => ("accepted", "revoked"),
        DeviceTrustRegistryDecision::Rejected(
            DeviceTrustRegistryRejection::RevokedDeviceCannotRePair,
        ) => ("rejected", "revoked"),
        DeviceTrustRegistryDecision::Rejected(DeviceTrustRegistryRejection::OwnershipConflict) => {
            ("rejected", "ownership-conflict")
        }
    }
}
