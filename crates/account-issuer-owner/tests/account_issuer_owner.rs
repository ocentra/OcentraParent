use ocentra_account_issuer_owner::repository::AccountIssuerRepositoryError;
use ocentra_account_issuer_owner::rpc::{AccountIssuerOwner, AccountIssuerRpcError};

#[test]
fn account_owner_mount_stays_fail_closed_until_protected_store_custody_exists() {
    assert!(matches!(
        AccountIssuerOwner::mount_account_owned(),
        Err(AccountIssuerRpcError::Repository(
            AccountIssuerRepositoryError::Unavailable
        ))
    ));
}
