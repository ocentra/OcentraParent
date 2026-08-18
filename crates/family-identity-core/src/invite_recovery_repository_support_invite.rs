use super::*;

pub(crate) fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}

pub(crate) fn role_label(role: AccountIdentityRole) -> &'static str {
    match role {
        AccountIdentityRole::ParentOwner => "parent-owner",
        AccountIdentityRole::CoParentGuardian => "co-parent-guardian",
        AccountIdentityRole::Observer => "observer",
        AccountIdentityRole::ChildProfile => "child-profile",
        AccountIdentityRole::ChildDeviceAgent => "child-device-agent",
        AccountIdentityRole::SupportAdmin => "support-admin",
    }
}

pub(crate) fn purpose_label(purpose: SetupInvitePurpose) -> &'static str {
    match purpose {
        SetupInvitePurpose::CoParentInvite => "co-parent-invite",
        SetupInvitePurpose::ObserverInvite => "observer-invite",
        SetupInvitePurpose::ChildDevicePairing => "child-device-pairing",
        SetupInvitePurpose::HouseholdTransfer => "household-transfer",
    }
}

pub(crate) fn target_role_label(role: SetupInviteTargetRole) -> &'static str {
    match role {
        SetupInviteTargetRole::CoParentGuardian => "co-parent-guardian",
        SetupInviteTargetRole::Observer => "observer",
        SetupInviteTargetRole::ChildDeviceAgent => "child-device-agent",
        SetupInviteTargetRole::ParentOwner => "parent-owner",
    }
}

pub(crate) fn target_role_from_label(value: &str) -> Option<SetupInviteTargetRole> {
    Some(match value {
        "co-parent-guardian" => SetupInviteTargetRole::CoParentGuardian,
        "observer" => SetupInviteTargetRole::Observer,
        "child-device-agent" => SetupInviteTargetRole::ChildDeviceAgent,
        "parent-owner" => SetupInviteTargetRole::ParentOwner,
        _ => return None,
    })
}
