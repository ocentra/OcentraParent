use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthority,
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityHouseholdChildDeviceBinding,
    AccountIdentityMappingStatus, AccountIdentityProvider, AccountIdentityProviderSubject,
    AccountIdentityProviderSubjectMapping,
};

#[path = "account_identity_authority_capability.rs"]
mod account_identity_authority_capability;
#[path = "account_identity_authority_query_custody.rs"]
mod account_identity_authority_query_custody;
#[path = "account_identity_authority_validation.rs"]
mod account_identity_authority_validation;
#[path = "account_identity_authority_value_mapping.rs"]
pub(crate) mod account_identity_authority_value_mapping;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AccountIdentityCurrentBindingReadError<E> {
    Repository(E),
    Missing,
    ProviderMismatch,
    ProviderSubjectMismatch,
    InactiveProviderMapping,
    MappingAccountMismatch,
    MemberAuthorityInvalid,
    SessionInvalid,
    SupportReceiptInvalid,
}

/// Account-owned repository boundary. The only lookup key is the provider
/// subject already verified by the external provider adapter. Household,
/// member, role, device, target, session, and receipt state come from durable
/// account records and cannot be supplied by the request caller.
pub(crate) trait AccountIdentityAuthorityRepository {
    type Error;

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

/// Opaque authority snapshot minted only by the family-owned producer after a
/// durable repository read established currentness at issuance. It does not
/// claim race-safe currentness after that read; a runtime that requires
/// revocation-linearized authority must re-read in its owning transaction. It
/// intentionally does not implement serde: a JSON/TS handoff is evidence,
/// never authority.
pub struct VerifiedAccountIdentityAuthority {
    handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    provenance: account_identity_authority_capability::AccountIdentityAuthorityProvenance,
}

impl VerifiedAccountIdentityAuthority {
    pub(crate) fn handoff(&self) -> &AccountIdentityCurrentMemberDeviceAuthorityHandoff {
        &self.handoff
    }

    pub(crate) fn current_binding(&self) -> &AccountIdentityHouseholdChildDeviceBinding {
        &self.handoff.binding
    }

    pub(crate) fn support_receipt(
        &self,
    ) -> Option<&ocentra_schema::account_identity_authority::AccountIdentitySupportAuthorityReceipt>
    {
        self.handoff.member.support_receipt.as_ref()
    }
}

pub(crate) struct AccountIdentityCurrentMemberAuthorityProducer<'a, R> {
    repository: &'a R,
}

impl<'a, R> AccountIdentityCurrentMemberAuthorityProducer<'a, R> {
    pub(crate) fn new(repository: &'a R) -> Self {
        Self { repository }
    }
}

impl<'a, R> AccountIdentityCurrentMemberAuthorityProducer<'a, R>
where
    R: AccountIdentityAuthorityRepository,
{
    pub(crate) fn produce(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<VerifiedAccountIdentityAuthority, AccountIdentityCurrentBindingReadError<R::Error>>
    {
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
            schema_version:
                ocentra_schema::account_identity_authority::AccountIdentityMemberAuthoritySchemaVersion::V0_1,
            mapping,
            member,
            binding,
        };
        handoff
            .validate_shape()
            .map_err(|_error| AccountIdentityCurrentBindingReadError::MemberAuthorityInvalid)?;
        account_identity_authority_validation::validate_current_session(&handoff)
            .map_err(|_error| AccountIdentityCurrentBindingReadError::SessionInvalid)?;
        account_identity_authority_validation::validate_support_receipt(&handoff, provider_subject)
            .map_err(|_error| AccountIdentityCurrentBindingReadError::SupportReceiptInvalid)?;

        let provenance = account_identity_authority_capability::provenance_from_handoff(&handoff);
        Ok(VerifiedAccountIdentityAuthority {
            handoff,
            provenance,
        })
    }
}
