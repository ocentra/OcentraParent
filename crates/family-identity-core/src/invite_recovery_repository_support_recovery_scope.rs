use ocentra_schema::account_identity_authority::AccountIdentitySupportScope;

use crate::setup_lifecycle::RecoveryKind;

pub(crate) fn support_authorization_scope_allows(
    kind: RecoveryKind,
    scope: AccountIdentitySupportScope,
) -> bool {
    match kind {
        RecoveryKind::HouseholdTransfer => scope == AccountIdentitySupportScope::Household,
        RecoveryKind::LostParentDevice
        | RecoveryKind::CompromisedAccount
        | RecoveryKind::ChildReinstall => scope == AccountIdentitySupportScope::DeviceControl,
        RecoveryKind::ForgotLogin => {
            matches!(
                scope,
                AccountIdentitySupportScope::Household | AccountIdentitySupportScope::DeviceControl
            )
        }
    }
}
