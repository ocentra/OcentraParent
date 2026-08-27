use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};

use super::{
    AccountIdentityAuthorityIssuerClientError, AccountIdentityIssuerCurrentness,
    AccountIdentityIssuerV2KeyRecord,
};

/// Move-only Account-owner admission for one authenticated issuer request.
///
/// There is deliberately no public constructor, decoder, default, clone, or
/// serialization path. A future family-owned adapter may mint this value only
/// after Account has authenticated the protected peer and observed the full
/// current authority/key lineage. Until that adapter exists, the public issue
/// operation is unreachable rather than accepting caller-assembled identity.
pub struct AccountIdentityIssuerOwnerAdmission {
    provider: AccountIdentityProvider,
    provider_subject: AccountIdentityProviderSubject,
    account_id: String,
    household_id: String,
    member_id: String,
    device_id: String,
    session_id: String,
    service: String,
    service_binding_id: String,
    key_id: String,
    key_generation: u64,
    enrollment_generation: u64,
    authority_generation: u64,
    session_generation: u64,
    correlation_id: String,
    idempotency_key: String,
}

impl AccountIdentityIssuerOwnerAdmission {
    pub(super) fn provider(&self) -> &AccountIdentityProvider {
        &self.provider
    }

    pub(super) fn provider_subject(&self) -> &AccountIdentityProviderSubject {
        &self.provider_subject
    }

    pub(super) fn validate_currentness(
        &self,
        currentness: &AccountIdentityIssuerCurrentness,
        correlation_id: &str,
        idempotency_key: &str,
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        let authority = currentness.authority();
        if &self.provider != authority.provider()
            || &self.provider_subject != authority.provider_subject()
            || self.account_id != currentness.account_id().as_str()
            || self.household_id != currentness.household_id().as_str()
            || self.member_id != authority.member_id().as_str()
            || self.device_id != authority.device_id().as_str()
            || self.session_id != authority.session_id().as_str()
            || self.authority_generation != authority.authority_generation()
            || self.session_generation != authority.session_generation()
            || self.correlation_id != correlation_id
            || self.idempotency_key != idempotency_key
            || self.service
                != ocentra_schema::account_identity_authority_producer_v2::
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
        {
            return Err(AccountIdentityAuthorityIssuerClientError::CurrentnessRejected);
        }
        Ok(())
    }

    pub(super) fn validate_key(
        &self,
        key: &AccountIdentityIssuerV2KeyRecord,
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        if self.service_binding_id != key.service_binding_id().as_str()
            || self.key_id != key.key_id().as_str()
            || self.key_generation != key.key_generation()
            || self.enrollment_generation != key.enrollment_generation()
            || self.authority_generation != key.authority_generation()
        {
            return Err(AccountIdentityAuthorityIssuerClientError::KeyUnavailable);
        }
        Ok(())
    }
}
