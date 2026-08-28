use ocentra_family_identity_core::account_identity_authority_repository::{
    AccountIdentityAuthorityRepositoryError, AccountIdentityAuthorityService,
};

#[test]
fn report_query_custody_does_not_bypass_an_unavailable_account_owner_mount() {
    // Positive derivation coverage remains blocked until Account supplies its
    // real opaque authority snapshot and a report-source owner adapter.
    let result = AccountIdentityAuthorityService::mount_account_owned();

    assert!(matches!(
        result,
        Err(AccountIdentityAuthorityRepositoryError::Unavailable)
    ));
}
