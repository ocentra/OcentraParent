use ocentra_schema::account_identity_authority::{
    AccountIdentityBindingLifecycleState, AccountIdentityBindingRevocationState,
    AccountIdentityChildDeviceId, AccountIdentityHouseholdChildDeviceBinding,
    AccountIdentityInstallState, AccountIdentityPairingState,
};
use ocentra_schema::report_query_custody::{ChildProfileId, FamilyId, ParentAccountId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AccountIdentityBindingSelector {
    pub account_id: ParentAccountId,
    pub household_id: FamilyId,
    pub child_profile_id: ChildProfileId,
    pub child_device_id: AccountIdentityChildDeviceId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AccountIdentityCurrentBindingReadError<E> {
    Repository(E),
    Missing,
    InvalidGeneration,
    SelectorMismatch,
    PairingNotComplete,
    InstallNotComplete,
    LifecycleNotActive,
    Revoked,
}

/// Authority-owned repository boundary. This remains crate-private until the
/// real authority adapter exists; a selector is a lookup only and never
/// supplies authority state.
pub(crate) trait AccountIdentityAuthorityRepository {
    type Error;

    fn read_current_binding(
        &self,
        selector: &AccountIdentityBindingSelector,
    ) -> Result<Option<AccountIdentityHouseholdChildDeviceBinding>, Self::Error>;
}

pub(crate) struct AccountIdentityCurrentBindingReadPort<R> {
    repository: R,
}

impl<R> AccountIdentityCurrentBindingReadPort<R> {
    pub(crate) fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> AccountIdentityCurrentBindingReadPort<R>
where
    R: AccountIdentityAuthorityRepository,
{
    pub(crate) fn read_current_binding(
        &self,
        selector: &AccountIdentityBindingSelector,
    ) -> Result<
        TrustedAccountIdentityCurrentBinding,
        AccountIdentityCurrentBindingReadError<R::Error>,
    > {
        let binding = self
            .repository
            .read_current_binding(selector)
            .map_err(AccountIdentityCurrentBindingReadError::Repository)?
            .ok_or(AccountIdentityCurrentBindingReadError::Missing)?;

        binding
            .validate_shape()
            .map_err(|_error| AccountIdentityCurrentBindingReadError::InvalidGeneration)?;

        if binding.account_id != selector.account_id
            || binding.household_id != selector.household_id
            || binding.child_profile_id != selector.child_profile_id
            || binding.child_device_id != selector.child_device_id
        {
            return Err(AccountIdentityCurrentBindingReadError::SelectorMismatch);
        }
        if binding.pairing_state != AccountIdentityPairingState::Paired {
            return Err(AccountIdentityCurrentBindingReadError::PairingNotComplete);
        }
        if binding.install_state != AccountIdentityInstallState::Installed {
            return Err(AccountIdentityCurrentBindingReadError::InstallNotComplete);
        }
        if binding.lifecycle_state != AccountIdentityBindingLifecycleState::Active {
            return Err(AccountIdentityCurrentBindingReadError::LifecycleNotActive);
        }
        if binding.revocation_state == AccountIdentityBindingRevocationState::Revoked {
            return Err(AccountIdentityCurrentBindingReadError::Revoked);
        }

        Ok(TrustedAccountIdentityCurrentBinding { binding })
    }
}

pub(crate) struct TrustedAccountIdentityCurrentBinding {
    binding: AccountIdentityHouseholdChildDeviceBinding,
}

impl TrustedAccountIdentityCurrentBinding {
    pub(crate) fn binding(&self) -> &AccountIdentityHouseholdChildDeviceBinding {
        &self.binding
    }
}
