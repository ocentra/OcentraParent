use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityRole, AccountIdentitySupportReceiptRevocationState,
    AccountIdentitySupportScope,
};

#[path = "account_identity_mutation_authority_protocol_identity.rs"]
mod identity;
#[path = "account_identity_mutation_authority_protocol_support.rs"]
mod support;

pub(crate) fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    identity::provider_label(provider)
}

pub(crate) fn role_label(role: AccountIdentityRole) -> &'static str {
    identity::role_label(role)
}

pub(crate) fn support_scope_label(scope: AccountIdentitySupportScope) -> &'static str {
    support::scope_label(scope)
}

pub(crate) fn support_revocation_label(
    state: AccountIdentitySupportReceiptRevocationState,
) -> &'static str {
    support::revocation_label(state)
}
