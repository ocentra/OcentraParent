use crate::device_trust_registry::{DeviceTrustRegistryFailure, DeviceTrustRegistryRejection};

pub(super) enum MutationPlan {
    PairPendingSealing,
    Revoke,
    Rejected(DeviceTrustRegistryRejection),
}

pub(super) fn mutation_plan(
    existing: Option<(&str, &str, &str)>,
    family_id: &str,
    parent_account_id: &str,
    action: &str,
) -> Result<MutationPlan, DeviceTrustRegistryFailure> {
    if action != "pair-child-device" && action != "revoke-child-device" {
        return Err(DeviceTrustRegistryFailure::StorageIntegrityRejected);
    }
    if let Some((_existing_family, _existing_parent, "trusted")) = existing {
        return Err(DeviceTrustRegistryFailure::StorageIntegrityRejected);
    }
    if let Some((existing_family, existing_parent, _state)) = existing {
        if existing_family != family_id || existing_parent != parent_account_id {
            return Ok(MutationPlan::Rejected(
                DeviceTrustRegistryRejection::OwnershipConflict,
            ));
        }
    }
    if matches!(existing, Some((_family, _parent, "revoked"))) && action == "pair-child-device" {
        return Ok(MutationPlan::Rejected(
            DeviceTrustRegistryRejection::RevokedDeviceCannotRePair,
        ));
    }
    Ok(if action == "pair-child-device" {
        MutationPlan::PairPendingSealing
    } else {
        MutationPlan::Revoke
    })
}
