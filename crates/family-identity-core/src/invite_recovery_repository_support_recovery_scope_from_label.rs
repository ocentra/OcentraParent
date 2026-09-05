use ocentra_schema::account_identity_authority::AccountIdentitySupportScope;

pub(crate) fn support_scope_from_label(value: &str) -> Option<AccountIdentitySupportScope> {
    match value {
        "household" => Some(AccountIdentitySupportScope::Household),
        "device-control" => Some(AccountIdentitySupportScope::DeviceControl),
        "read-only" => Some(AccountIdentitySupportScope::ReadOnly),
        _ => None,
    }
}
