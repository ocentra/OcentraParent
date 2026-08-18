use ocentra_schema::account_identity_authority::AccountIdentityRole;

use crate::setup_lifecycle::{SetupInvitePurpose, SetupInviteTargetRole};

pub(crate) fn purpose_matches_target_role(
    purpose: SetupInvitePurpose,
    target_role: SetupInviteTargetRole,
) -> bool {
    matches!(
        (purpose, target_role),
        (
            SetupInvitePurpose::CoParentInvite,
            SetupInviteTargetRole::CoParentGuardian
        ) | (
            SetupInvitePurpose::ObserverInvite,
            SetupInviteTargetRole::Observer
        ) | (
            SetupInvitePurpose::ChildDevicePairing,
            SetupInviteTargetRole::ChildDeviceAgent
        ) | (
            SetupInvitePurpose::HouseholdTransfer,
            SetupInviteTargetRole::ParentOwner
        )
    )
}

pub(crate) fn inviter_can_issue(role: AccountIdentityRole, purpose: SetupInvitePurpose) -> bool {
    matches!(
        (role, purpose),
        (
            AccountIdentityRole::ParentOwner | AccountIdentityRole::CoParentGuardian,
            SetupInvitePurpose::CoParentInvite
                | SetupInvitePurpose::ObserverInvite
                | SetupInvitePurpose::ChildDevicePairing
        ) | (
            AccountIdentityRole::ParentOwner,
            SetupInvitePurpose::HouseholdTransfer
        )
    )
}
