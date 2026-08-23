use crate::setup_lifecycle::RecoveryKind;

use super::RecoveryOwnerEffect;

pub(crate) fn owner_effect(kind: RecoveryKind) -> RecoveryOwnerEffect {
    match kind {
        RecoveryKind::ForgotLogin => RecoveryOwnerEffect::ProviderCredentialSession,
        RecoveryKind::LostParentDevice | RecoveryKind::CompromisedAccount => {
            RecoveryOwnerEffect::DeviceTrustRevoke
        }
        RecoveryKind::ChildReinstall => RecoveryOwnerEffect::DeviceTrustReinstall,
        RecoveryKind::HouseholdTransfer => RecoveryOwnerEffect::HouseholdAuthorityMutation,
    }
}
