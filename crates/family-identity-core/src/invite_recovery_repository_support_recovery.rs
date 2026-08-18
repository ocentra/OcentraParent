use super::*;

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

pub(crate) fn support_channel_label(channel: RecoverySupportChannel) -> &'static str {
    match channel {
        RecoverySupportChannel::SelfServe => "self-serve",
        RecoverySupportChannel::HouseholdOwnerAssisted => "household-owner-assisted",
        RecoverySupportChannel::SupportAssisted => "support-assisted",
    }
}

pub(crate) fn recovery_kind_label(kind: RecoveryKind) -> &'static str {
    match kind {
        RecoveryKind::ForgotLogin => "forgot-login",
        RecoveryKind::LostParentDevice => "lost-parent-device",
        RecoveryKind::CompromisedAccount => "compromised-account",
        RecoveryKind::ChildReinstall => "child-reinstall",
        RecoveryKind::HouseholdTransfer => "household-transfer",
    }
}

pub(crate) fn recovery_kind_from_label(value: &str) -> Option<RecoveryKind> {
    Some(match value {
        "forgot-login" => RecoveryKind::ForgotLogin,
        "lost-parent-device" => RecoveryKind::LostParentDevice,
        "compromised-account" => RecoveryKind::CompromisedAccount,
        "child-reinstall" => RecoveryKind::ChildReinstall,
        "household-transfer" => RecoveryKind::HouseholdTransfer,
        _ => return None,
    })
}

pub(crate) fn recovery_request_is_allowed(
    role: AccountIdentityRole,
    kind: RecoveryKind,
    channel: RecoverySupportChannel,
) -> bool {
    match role {
        AccountIdentityRole::SupportAdmin => channel == RecoverySupportChannel::SupportAssisted,
        AccountIdentityRole::ParentOwner | AccountIdentityRole::CoParentGuardian => matches!(
            kind,
            RecoveryKind::ForgotLogin
                | RecoveryKind::LostParentDevice
                | RecoveryKind::CompromisedAccount
                | RecoveryKind::ChildReinstall
                | RecoveryKind::HouseholdTransfer
        ),
        _ => false,
    }
}

pub(crate) fn owner_approval_required(kind: RecoveryKind, channel: RecoverySupportChannel) -> bool {
    channel == RecoverySupportChannel::SupportAssisted
        || matches!(
            kind,
            RecoveryKind::LostParentDevice
                | RecoveryKind::CompromisedAccount
                | RecoveryKind::HouseholdTransfer
        )
}

pub(crate) fn durable_handoff(
    handoff_id: String,
    correlation_id: String,
    recovery_id: String,
    household_id: String,
    account_id: String,
    member_id: String,
    device_id: String,
    kind: String,
    requested_at_epoch_millis: i64,
) -> Result<RecoveryCustodyHandoff, InviteRecoveryRepositoryError> {
    Ok(RecoveryCustodyHandoff::from_durable(
        handoff_id,
        correlation_id,
        RecoveryId::parse(recovery_id).map_err(InviteRecoveryRepositoryError::InvalidValue)?,
        FamilyId::parse(household_id).ok_or(InviteRecoveryRepositoryError::InvalidInvite)?,
        ParentAccountId::parse(account_id).ok_or(InviteRecoveryRepositoryError::InvalidInvite)?,
        ocentra_schema::account_identity_authority::AccountIdentityMemberId::parse(member_id)
            .map_err(InviteRecoveryRepositoryError::InvalidValue)?,
        ocentra_schema::account_identity_authority::AccountIdentityDeviceId::parse(device_id)
            .map_err(InviteRecoveryRepositoryError::InvalidValue)?,
        recovery_kind_from_label(&kind).ok_or(InviteRecoveryRepositoryError::HandoffConflict)?,
        timestamp(requested_at_epoch_millis)?,
    ))
}
