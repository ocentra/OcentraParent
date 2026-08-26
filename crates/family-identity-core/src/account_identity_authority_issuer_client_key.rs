use super::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerV2KeyId, AccountIdentityIssuerV2ServiceBindingId,
};
use super::AccountIdentityIssuerV2KeyRecord;

impl AccountIdentityIssuerV2KeyRecord {
    pub fn key_id(&self) -> &AccountIdentityIssuerV2KeyId {
        &self.key_id
    }

    pub fn key_generation(&self) -> u64 {
        self.key_generation
    }

    pub fn public_key(&self) -> &[u8; 65] {
        &self.public_key
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn service_binding_id(&self) -> &AccountIdentityIssuerV2ServiceBindingId {
        &self.service_binding_id
    }
}
