use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION;

use crate::account_issuer_contract::AccountIssuerField;
use crate::types::ProtocolError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIssuerReceiptLineage {
    provider: AccountIdentityProvider,
    provider_subject: AccountIdentityProviderSubject,
    account_id: AccountIssuerField,
    household_id: AccountIssuerField,
    member_id: AccountIssuerField,
    device_id: AccountIssuerField,
    session_id: AccountIssuerField,
    service_binding_id: AccountIssuerField,
    key_generation: u64,
    enrollment_generation: u64,
    authority_generation: u64,
    session_generation: u64,
    issued_at: AccountIssuerField,
    expires_at: AccountIssuerField,
}

impl AccountIssuerReceiptLineage {
    pub fn new(
        provider: AccountIdentityProvider,
        provider_subject: AccountIdentityProviderSubject,
        account_id: AccountIssuerField,
        household_id: AccountIssuerField,
        member_id: AccountIssuerField,
        device_id: AccountIssuerField,
        session_id: AccountIssuerField,
        service_binding_id: AccountIssuerField,
        key_generation: u64,
        enrollment_generation: u64,
        authority_generation: u64,
        session_generation: u64,
        issued_at: AccountIssuerField,
        expires_at: AccountIssuerField,
    ) -> Result<Self, ProtocolError> {
        for generation in [
            key_generation,
            enrollment_generation,
            authority_generation,
            session_generation,
        ] {
            if generation == 0 || generation > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_GENERATION
            {
                return Err(ProtocolError::InvalidDiscriminant(0));
            }
        }
        Ok(Self {
            provider,
            provider_subject,
            account_id,
            household_id,
            member_id,
            device_id,
            session_id,
            service_binding_id,
            key_generation,
            enrollment_generation,
            authority_generation,
            session_generation,
            issued_at,
            expires_at,
        })
    }

    pub fn provider(&self) -> &AccountIdentityProvider {
        &self.provider
    }

    pub fn provider_subject(&self) -> &AccountIdentityProviderSubject {
        &self.provider_subject
    }

    pub fn account_id(&self) -> &AccountIssuerField {
        &self.account_id
    }

    pub fn household_id(&self) -> &AccountIssuerField {
        &self.household_id
    }

    pub fn member_id(&self) -> &AccountIssuerField {
        &self.member_id
    }

    pub fn device_id(&self) -> &AccountIssuerField {
        &self.device_id
    }

    pub fn session_id(&self) -> &AccountIssuerField {
        &self.session_id
    }

    pub fn service_binding_id(&self) -> &AccountIssuerField {
        &self.service_binding_id
    }

    pub fn key_generation(&self) -> u64 {
        self.key_generation
    }

    pub fn enrollment_generation(&self) -> u64 {
        self.enrollment_generation
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn session_generation(&self) -> u64 {
        self.session_generation
    }

    pub fn issued_at(&self) -> &AccountIssuerField {
        &self.issued_at
    }

    pub fn expires_at(&self) -> &AccountIssuerField {
        &self.expires_at
    }
}
