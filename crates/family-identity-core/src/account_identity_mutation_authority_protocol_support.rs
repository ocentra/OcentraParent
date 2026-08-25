use ocentra_schema::account_identity_authority::{
    AccountIdentitySupportReceiptRevocationState, AccountIdentitySupportScope,
};

pub(super) fn scope_label(scope: AccountIdentitySupportScope) -> &'static str {
    match scope {
        AccountIdentitySupportScope::ReadOnly => "read-only",
        AccountIdentitySupportScope::Household => "household",
        AccountIdentitySupportScope::DeviceControl => "device-control",
    }
}

pub(super) fn revocation_label(
    state: AccountIdentitySupportReceiptRevocationState,
) -> &'static str {
    match state {
        AccountIdentitySupportReceiptRevocationState::Active => "active",
        AccountIdentitySupportReceiptRevocationState::Revoked => "revoked",
    }
}
