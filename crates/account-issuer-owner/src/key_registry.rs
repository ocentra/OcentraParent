//! Owner view of the family-owned v2 public-key registry.

use ocentra_family_identity_core::account_identity_authority_issuer_client::
    AccountIdentityIssuerV2KeyRecord;
use ocentra_family_identity_core::account_identity_authority_issuer_client::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerV2KeyId, AccountIdentityIssuerV2ServiceBindingId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyRecord {
    pub(crate) inner: AccountIdentityIssuerV2KeyRecord,
}

impl KeyRecord {
    pub fn key_id(&self) -> &AccountIdentityIssuerV2KeyId {
        self.inner.key_id()
    }

    pub fn key_generation(&self) -> u64 {
        self.inner.key_generation()
    }

    pub fn public_key(&self) -> &[u8; 65] {
        self.inner.public_key()
    }

    pub fn service_binding_id(&self) -> &AccountIdentityIssuerV2ServiceBindingId {
        self.inner.service_binding_id()
    }

    pub(crate) fn from(inner: AccountIdentityIssuerV2KeyRecord) -> Self {
        Self { inner }
    }
}
