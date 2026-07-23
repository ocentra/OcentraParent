use crate::device_trust_registry::{DeviceTrustRegistryFailure, DeviceTrustRegistryRejection};

pub(super) enum MutationPlan {
    PairPendingSealing,
    Revoke,
    Rejected(DeviceTrustRegistryRejection),
}

pub(super) fn mutation_plan(
    existing: Option<(&str, &str)>,
    family_id: &str,
    action: &str,
    recovery_repair_authorized: bool,
) -> Result<MutationPlan, DeviceTrustRegistryFailure> {
    if action != "pair-child-device" && action != "revoke-child-device" {
        return Err(DeviceTrustRegistryFailure::StorageIntegrityRejected);
    }
    if let Some((_existing_family, "trusted")) = existing {
        return Err(DeviceTrustRegistryFailure::StorageIntegrityRejected);
    }
    if let Some((existing_family, _state)) = existing {
        if existing_family != family_id {
            return Ok(MutationPlan::Rejected(
                DeviceTrustRegistryRejection::OwnershipConflict,
            ));
        }
    }
    if matches!(existing, Some((_family, "revoked")))
        && action == "pair-child-device"
        && !recovery_repair_authorized
    {
        return Ok(MutationPlan::Rejected(
            DeviceTrustRegistryRejection::RevokedDeviceCannotRePair,
        ));
    }
    if existing.is_none() && action == "revoke-child-device" {
        return Ok(MutationPlan::Rejected(
            DeviceTrustRegistryRejection::UnknownDevice,
        ));
    }
    Ok(if action == "pair-child-device" {
        MutationPlan::PairPendingSealing
    } else {
        MutationPlan::Revoke
    })
}
