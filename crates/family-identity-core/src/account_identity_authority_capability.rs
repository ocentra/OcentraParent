use std::fmt;

use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityProvider,
    AccountIdentityProviderSubject,
};
use ocentra_schema::report_query_custody::{ChildProfileId, FamilyId, ParentAccountId};

#[derive(Clone)]
pub(super) struct AccountIdentityAuthorityProvenance {
    pub(super) provider: AccountIdentityProvider,
    pub(super) provider_subject: AccountIdentityProviderSubject,
    pub(super) session_id: ocentra_schema::account_identity_authority::AccountIdentitySessionId,
    pub(super) session_generation: u64,
    pub(super) authority_generation: u64,
}

pub(super) fn provenance_from_handoff(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> AccountIdentityAuthorityProvenance {
    AccountIdentityAuthorityProvenance {
        provider: handoff.mapping.provider.clone(),
        provider_subject: handoff.mapping.provider_subject.clone(),
        session_id: handoff.member.session_id.clone(),
        session_generation: handoff.member.session_generation,
        authority_generation: handoff.member.authority_generation,
    }
}

impl fmt::Debug for super::VerifiedAccountIdentityAuthority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifiedAccountIdentityAuthority")
            .field("provenance", &"verified-current-authority-omitted")
            .finish()
    }
}

impl super::VerifiedAccountIdentityAuthority {
    pub fn account_id(&self) -> &ParentAccountId {
        &self.handoff.member.account_id
    }

    pub fn household_id(&self) -> &FamilyId {
        &self.handoff.member.household_id
    }

    pub fn member_id(
        &self,
    ) -> &ocentra_schema::account_identity_authority::AccountIdentityMemberId {
        &self.handoff.member.member_id
    }

    pub fn role(&self) -> ocentra_schema::account_identity_authority::AccountIdentityRole {
        self.handoff.member.role
    }

    pub fn device_id(
        &self,
    ) -> &ocentra_schema::account_identity_authority::AccountIdentityDeviceId {
        &self.handoff.member.device_id
    }

    pub fn child_profile_id(&self) -> &ChildProfileId {
        &self.handoff.binding.child_profile_id
    }

    pub fn child_device_id(
        &self,
    ) -> &ocentra_schema::account_identity_authority::AccountIdentityChildDeviceId {
        &self.handoff.binding.child_device_id
    }

    pub fn session_id(
        &self,
    ) -> &ocentra_schema::account_identity_authority::AccountIdentitySessionId {
        &self.provenance.session_id
    }

    pub fn session_generation(&self) -> u64 {
        self.provenance.session_generation
    }

    pub fn authority_generation(&self) -> u64 {
        self.provenance.authority_generation
    }

    pub fn provider(&self) -> &AccountIdentityProvider {
        &self.provenance.provider
    }

    pub fn provider_subject(&self) -> &AccountIdentityProviderSubject {
        &self.provenance.provider_subject
    }

    pub fn support_audit_identity(
        &self,
    ) -> Option<&ocentra_schema::account_identity_authority::AccountIdentityAuditIdentity> {
        self.handoff
            .member
            .support_receipt
            .as_ref()
            .map(|receipt| &receipt.audit_identity)
    }

    pub fn support_scope(
        &self,
    ) -> Option<&ocentra_schema::account_identity_authority::AccountIdentitySupportScope> {
        self.handoff
            .member
            .support_receipt
            .as_ref()
            .map(|receipt| &receipt.scope)
    }

    pub fn support_issuer(
        &self,
    ) -> Option<&ocentra_schema::account_identity_authority::AccountIdentitySupportIssuerId> {
        self.handoff
            .member
            .support_receipt
            .as_ref()
            .map(|receipt| &receipt.issuer)
    }

    /// Export only the canonical DTO/evidence representation. Callers cannot
    /// use this value to construct or reconstitute a verified capability.
    pub fn evidence_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.handoff)
    }
}
