use crate::setup_lifecycle::SetupInviteTargetRole;

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
