use ocentra_schema::account_identity_authority::AccountIdentitySupportScope;

pub(crate) fn support_scope_label(scope: AccountIdentitySupportScope) -> &'static str {
    match scope {
        AccountIdentitySupportScope::Household => "household",
        AccountIdentitySupportScope::DeviceControl => "device-control",
        AccountIdentitySupportScope::ReadOnly => "read-only",
    }
}
