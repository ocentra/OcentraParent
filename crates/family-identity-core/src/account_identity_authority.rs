use std::fmt;

use ocentra_schema::account_identity_authority::{
    AccountIdentityAuthorityHandoff, AccountIdentityAuthoritySchemaVersion,
    AccountIdentityBindingLifecycleState, AccountIdentityBindingRevocationState,
    AccountIdentityChildDeviceId, AccountIdentityHouseholdChildDeviceBinding,
    AccountIdentityInstallState, AccountIdentityMappingStatus, AccountIdentityPairingState,
    AccountIdentityProvider, AccountIdentityProviderSubject, AccountIdentityProviderSubjectMapping,
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
    ProviderMismatch,
    ProviderSubjectMismatch,
    InactiveProviderMapping,
    MappingAccountMismatch,
    Serialization,
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

    /// Read the active provider mapping and its current binding as one
    /// authority-owned lookup. The provider subject is the only caller input;
    /// household, child, and device scope must come from the durable records.
    fn read_current_authority_records(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<
        Option<(
            AccountIdentityProviderSubjectMapping,
            AccountIdentityHouseholdChildDeviceBinding,
        )>,
        Self::Error,
    >;
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

        validate_current_binding(selector, binding)
    }
}

/// A cross-boundary handoff that contains only the encoded Account-owned
/// result. Its constructor and payload remain private so a caller cannot mint
/// current authority by constructing a DTO or supplying household/device
/// headers.
pub struct AccountIdentityEncodedAuthorityHandoff {
    encoded: String,
}

impl fmt::Debug for AccountIdentityEncodedAuthorityHandoff {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccountIdentityEncodedAuthorityHandoff")
            .field("redaction", &"encoded-authority-omitted")
            .finish()
    }
}

impl AccountIdentityEncodedAuthorityHandoff {
    pub fn as_str(&self) -> &str {
        &self.encoded
    }
}

/// Account-owned producer for a server-derived current binding handoff.
///
/// A future durable adapter implements the repository boundary. The producer
/// itself remains the only code that can turn that lookup into an encoded
/// handoff; support/admin authority is intentionally not accepted here and
/// must use a separately audited Account path. No public authority DTO or
/// test-only constructor is introduced.
pub(crate) struct AccountIdentityCurrentAuthorityProducer<R> {
    repository: R,
}

impl<R> AccountIdentityCurrentAuthorityProducer<R> {
    pub(crate) fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> AccountIdentityCurrentAuthorityProducer<R>
where
    R: AccountIdentityAuthorityRepository,
{
    pub(crate) fn produce(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<
        AccountIdentityEncodedAuthorityHandoff,
        AccountIdentityCurrentBindingReadError<R::Error>,
    > {
        let (mapping, binding) = self
            .repository
            .read_current_authority_records(provider, provider_subject)
            .map_err(AccountIdentityCurrentBindingReadError::Repository)?
            .ok_or(AccountIdentityCurrentBindingReadError::Missing)?;

        if &mapping.provider != provider {
            return Err(AccountIdentityCurrentBindingReadError::ProviderMismatch);
        }
        if &mapping.provider_subject != provider_subject {
            return Err(AccountIdentityCurrentBindingReadError::ProviderSubjectMismatch);
        }
        if mapping.status != AccountIdentityMappingStatus::Active {
            return Err(AccountIdentityCurrentBindingReadError::InactiveProviderMapping);
        }
        if mapping.account_id != binding.account_id {
            return Err(AccountIdentityCurrentBindingReadError::MappingAccountMismatch);
        }

        let selector = AccountIdentityBindingSelector {
            account_id: mapping.account_id.clone(),
            household_id: binding.household_id.clone(),
            child_profile_id: binding.child_profile_id.clone(),
            child_device_id: binding.child_device_id.clone(),
        };
        let trusted_binding = validate_current_binding(&selector, binding)?;
        let handoff = AccountIdentityAuthorityHandoff {
            schema_version: AccountIdentityAuthoritySchemaVersion::V0_7,
            mapping,
            binding: trusted_binding.binding,
        };

        serde_json::to_string(&handoff)
            .map(|encoded| AccountIdentityEncodedAuthorityHandoff { encoded })
            .map_err(|_error| AccountIdentityCurrentBindingReadError::Serialization)
    }
}

fn validate_current_binding<E>(
    selector: &AccountIdentityBindingSelector,
    binding: AccountIdentityHouseholdChildDeviceBinding,
) -> Result<TrustedAccountIdentityCurrentBinding, AccountIdentityCurrentBindingReadError<E>> {
    binding
        .validate()
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

pub(crate) struct TrustedAccountIdentityCurrentBinding {
    binding: AccountIdentityHouseholdChildDeviceBinding,
}

impl TrustedAccountIdentityCurrentBinding {
    pub(crate) fn binding(&self) -> &AccountIdentityHouseholdChildDeviceBinding {
        &self.binding
    }
}
