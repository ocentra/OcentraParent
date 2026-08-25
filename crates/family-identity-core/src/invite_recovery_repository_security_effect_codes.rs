use super::RecoveryOwnerEffect;

pub(crate) fn owner_effect_code(effect: RecoveryOwnerEffect) -> i64 {
    match effect {
        RecoveryOwnerEffect::ProviderCredentialSession => 1,
        RecoveryOwnerEffect::DeviceTrustRevoke => 2,
        RecoveryOwnerEffect::DeviceTrustReinstall => 3,
        RecoveryOwnerEffect::HouseholdAuthorityMutation => 4,
    }
}

pub(crate) fn owner_effect_from_code(code: i64) -> Option<RecoveryOwnerEffect> {
    match code {
        1 => Some(RecoveryOwnerEffect::ProviderCredentialSession),
        2 => Some(RecoveryOwnerEffect::DeviceTrustRevoke),
        3 => Some(RecoveryOwnerEffect::DeviceTrustReinstall),
        4 => Some(RecoveryOwnerEffect::HouseholdAuthorityMutation),
        _ => None,
    }
}
