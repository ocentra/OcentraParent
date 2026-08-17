use crate::household_authority::{
    authorize_household_action, HouseholdAuthorityAction, HouseholdAuthorityDecision,
    HouseholdAuthorityInput, ParentControllerLeaseState,
};
use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthority,
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityHouseholdChildDeviceBinding,
    AccountIdentityMappingStatus, AccountIdentityProvider, AccountIdentityProviderSubject,
    AccountIdentityProviderSubjectMapping,
};

#[path = "account_identity_authority_capability.rs"]
mod account_identity_authority_capability;
#[path = "account_identity_authority_validation.rs"]
mod account_identity_authority_validation;
#[path = "account_identity_authority_value_mapping.rs"]
mod account_identity_authority_value_mapping;

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

/// Opaque authority minted only by the family-owned producer after the
/// durable repository has established currentness. It intentionally does not
/// implement serde: a JSON/TS handoff is evidence, never authority.
pub struct VerifiedAccountIdentityAuthority {
    handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    provenance: account_identity_authority_capability::AccountIdentityAuthorityProvenance,
}

/// Downstream family-policy adapter. All household/member/device state is
/// derived from the opaque capability; the caller supplies only the action's
/// separate capability grant and optional controller lease.
pub fn authorize_household_action_from_verified_authority(
    authority: &VerifiedAccountIdentityAuthority,
    action: HouseholdAuthorityAction,
    capability_granted: bool,
    controller_lease_state: Option<ParentControllerLeaseState>,
) -> HouseholdAuthorityDecision {
    let member = &authority.handoff.member;
    let binding = &authority.handoff.binding;
    authorize_household_action(HouseholdAuthorityInput {
        actor_role: account_identity_authority_value_mapping::map_role(member.role),
        same_family: true,
        actor_account_state: account_identity_authority_value_mapping::map_account_state(
            member.account_state,
        ),
        membership_state: account_identity_authority_value_mapping::map_membership_state(
            member.membership_state,
        ),
        child_profile_binding_state: account_identity_authority_value_mapping::map_binding_state(
            binding,
        ),
        device_ownership_scope: account_identity_authority_value_mapping::map_device_scope(
            member.role,
        ),
        device_trust_state: account_identity_authority_value_mapping::map_device_trust(
            member.device_trust_state,
        ),
        session_freshness_state: account_identity_authority_value_mapping::map_session_freshness(
            member.session_freshness_state,
        ),
        capability_granted,
        controller_lease_state,
        action,
    })
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
            .map_err(|_| AccountIdentityCurrentBindingReadError::MemberAuthorityInvalid)?;
        account_identity_authority_validation::validate_current_session(&handoff)
            .map_err(|_| AccountIdentityCurrentBindingReadError::SessionInvalid)?;
        account_identity_authority_validation::validate_support_receipt(&handoff, provider_subject)
            .map_err(|_| AccountIdentityCurrentBindingReadError::SupportReceiptInvalid)?;

        let provenance = account_identity_authority_capability::provenance_from_handoff(&handoff);
        Ok(VerifiedAccountIdentityAuthority {
            handoff,
            provenance,
        })
    }
}
