use ocentra_schema::account_identity_authority::AccountIdentityRole;

use crate::recovery_lifecycle::RecoveryKind;
use crate::setup_lifecycle::RecoverySupportChannel;

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
