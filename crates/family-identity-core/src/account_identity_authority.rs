use std::fmt;

use ocentra_schema::account_identity_authority::{
    AccountIdentityBindingLifecycleState, AccountIdentityBindingRevocationState,
    AccountIdentityChildDeviceId, AccountIdentityCurrentMemberDeviceAuthority,
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityHouseholdChildDeviceBinding,
    AccountIdentityInstallState, AccountIdentityMappingStatus,
    AccountIdentityMemberAuthoritySchemaVersion, AccountIdentityPairingState,
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
    MemberAuthorityInvalid,
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

    /// Resolve all current member/device context from the verified provider
    /// subject. Household, role, membership, device, and target binding state
    /// are durable records, never caller-supplied authority flags.
    fn read_current_member_device_authority(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<
        Option<(
            AccountIdentityProviderSubjectMapping,
            AccountIdentityCurrentMemberDeviceAuthority,
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

/// A redacted, encoded result that only the Account-owned producer can mint.
/// Cloudflare consumes the canonical encoded handoff; policy receives it only
/// through a future verified adapter and cannot construct it from request data.
pub struct AccountIdentityEncodedCurrentMemberAuthorityHandoff {
    encoded: String,
}

impl fmt::Debug for AccountIdentityEncodedCurrentMemberAuthorityHandoff {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccountIdentityEncodedCurrentMemberAuthorityHandoff")
            .field("redaction", &"encoded-authority-omitted")
            .finish()
    }
}

impl AccountIdentityEncodedCurrentMemberAuthorityHandoff {
    pub fn as_str(&self) -> &str {
        &self.encoded
    }
}

/// Account-owned producer for a current member/role/device authority handoff.
///
/// The only lookup input is the verified provider subject. The repository
/// supplies household, membership, role, device, and child-binding records;
/// no public constructor or caller-provided authority state exists here.
pub(crate) struct AccountIdentityCurrentMemberAuthorityProducer<R> {
    repository: R,
}

impl<R> AccountIdentityCurrentMemberAuthorityProducer<R> {
    pub(crate) fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> AccountIdentityCurrentMemberAuthorityProducer<R>
where
    R: AccountIdentityAuthorityRepository,
{
    pub(crate) fn produce(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<
        AccountIdentityEncodedCurrentMemberAuthorityHandoff,
        AccountIdentityCurrentBindingReadError<R::Error>,
    > {
        let (mapping, member, binding) = self
            .repository
            .read_current_member_device_authority(provider, provider_subject)
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
        if mapping.account_id != member.account_id || mapping.account_id != binding.account_id {
            return Err(AccountIdentityCurrentBindingReadError::MappingAccountMismatch);
        }

        let handoff = AccountIdentityCurrentMemberDeviceAuthorityHandoff {
            schema_version: AccountIdentityMemberAuthoritySchemaVersion::V0_1,
            mapping,
            member,
            binding,
        };
        handoff
            .validate_shape()
            .map_err(|_error| AccountIdentityCurrentBindingReadError::MemberAuthorityInvalid)?;

        serde_json::to_string(&handoff)
            .map(|encoded| AccountIdentityEncodedCurrentMemberAuthorityHandoff { encoded })
            .map_err(|_error| AccountIdentityCurrentBindingReadError::Serialization)
    }
}
