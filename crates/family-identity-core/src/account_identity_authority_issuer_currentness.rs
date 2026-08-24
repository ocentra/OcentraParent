use std::path::Path;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_issuer::AccountIdentityIssuerError;

use super::service_binding::{
    AccountIdentityIssuerAuthenticatedBinding, AccountIdentityIssuerServiceBinding,
    AccountIdentityIssuerServiceBindingAuthenticator,
};

/// Account-owned currentness boundary. Implementations must acquire a durable
/// lease from the Account repository; a caller-held snapshot is not sufficient
/// to authorize issuer mutation or signing.
pub(crate) trait AccountIdentityIssuerAuthorityResolver: Send + Sync {
    fn acquire_current(
        &self,
        observed: &VerifiedAccountIdentityAuthority,
    ) -> Result<Box<dyn AccountIdentityIssuerAuthorityLease>, AccountIdentityIssuerError>;
}

/// A non-mintable Account currentness lease held across the issuer operation.
/// The owner must keep revocation/rotation from committing while the lease is
/// live and must fail `assert_current` if the observed authority is stale.
pub(crate) trait AccountIdentityIssuerAuthorityLease: Send {
    fn authority(&self) -> &VerifiedAccountIdentityAuthority;
    fn assert_current(&self) -> Result<(), AccountIdentityIssuerError>;
}

pub(crate) struct CurrentIssuerContext {
    lease: Box<dyn AccountIdentityIssuerAuthorityLease>,
    binding: AccountIdentityIssuerServiceBinding,
}

impl CurrentIssuerContext {
    pub(crate) fn authority(&self) -> &VerifiedAccountIdentityAuthority {
        self.lease.authority()
    }

    pub(crate) fn binding(&self) -> &AccountIdentityIssuerServiceBinding {
        &self.binding
    }

    pub(crate) fn assert_current(&self) -> Result<(), AccountIdentityIssuerError> {
        self.lease.assert_current()
    }
}

pub(crate) fn acquire_current_context(
    resolver: Option<&dyn AccountIdentityIssuerAuthorityResolver>,
    observed: &VerifiedAccountIdentityAuthority,
    requested_binding: &AccountIdentityIssuerServiceBinding,
) -> Result<CurrentIssuerContext, AccountIdentityIssuerError> {
    let resolver = resolver.ok_or(AccountIdentityIssuerError::CurrentAuthorityUnavailable)?;
    let lease = resolver.acquire_current(observed)?;
    let current = lease.authority();
    if current.account_id() != observed.account_id()
        || current.household_id() != observed.household_id()
    {
        return Err(AccountIdentityIssuerError::CurrentAuthorityRejected);
    }
    let binding =
        AccountIdentityIssuerServiceBinding::from_authority(current, requested_binding.service())?;
    lease.assert_current()?;
    Ok(CurrentIssuerContext { lease, binding })
}

pub(crate) fn authenticate_binding(
    authenticator: Option<&dyn AccountIdentityIssuerServiceBindingAuthenticator>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
) -> Result<AccountIdentityIssuerAuthenticatedBinding, AccountIdentityIssuerError> {
    if !binding.matches_authority(authority) {
        return Err(AccountIdentityIssuerError::BindingMismatch);
    }
    let authenticator =
        authenticator.ok_or(AccountIdentityIssuerError::ServiceBindingUnavailable)?;
    let authenticated = authenticator.authenticate(binding)?;
    (authenticated.binding_id() == binding.binding_id())
        .then_some(authenticated)
        .ok_or(AccountIdentityIssuerError::ServiceBindingRejected)
}

pub(crate) fn validate_durable_path(path: &Path) -> Result<(), AccountIdentityIssuerError> {
    if !path.is_absolute() || path.as_os_str().is_empty() {
        return Err(AccountIdentityIssuerError::NonDurableStorage);
    }
    let path_text = path.to_string_lossy().to_ascii_lowercase();
    if path_text == ":memory:"
        || path_text.starts_with("file:")
        || path_text.contains("mode=memory")
        || path_text.contains("cache=shared")
    {
        return Err(AccountIdentityIssuerError::NonDurableStorage);
    }
    if path.exists() && path.is_dir() {
        return Err(AccountIdentityIssuerError::NonDurableStorage);
    }
    Ok(())
}
