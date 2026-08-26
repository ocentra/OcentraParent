use ocentra_schema::account_identity_authority_producer_v2::{
    AccountIdentityAuthorityProducerV2Claims, AccountIdentityAuthorityProducerV2Operation,
    AccountIdentityAuthorityProducerV2Receipt,
};

use super::{
    AccountIdentityAuthorityProducerV2Verified, AccountIdentityAuthorityProducerV2VerifiedReceipt,
};

impl AccountIdentityAuthorityProducerV2VerifiedReceipt {
    pub fn receipt(&self) -> &AccountIdentityAuthorityProducerV2Receipt {
        &self.receipt
    }
}

impl AccountIdentityAuthorityProducerV2Verified {
    pub fn operation(&self) -> AccountIdentityAuthorityProducerV2Operation {
        self.operation
    }

    pub fn key_id(&self) -> &str {
        self.key_id.as_str()
    }

    pub fn receipt_id(&self) -> &str {
        self.receipt_id.as_str()
    }

    pub fn service_binding_id(&self) -> &str {
        self.service_binding_id.as_str()
    }

    pub fn key_generation(&self) -> u64 {
        self.key_generation
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn enrollment_generation(&self) -> u64 {
        self.enrollment_generation
    }

    pub fn session_generation(&self) -> u64 {
        self.session_generation
    }

    pub fn correlation_id(&self) -> &str {
        self.correlation_id.as_str()
    }

    pub fn idempotency_key(&self) -> &str {
        self.idempotency_key.as_str()
    }

    pub fn issued_at(&self) -> &str {
        self.issued_at.as_str()
    }

    pub fn expires_at(&self) -> &str {
        self.expires_at.as_str()
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_digest.as_str()
    }

    pub fn claims(&self) -> &AccountIdentityAuthorityProducerV2Claims {
        &self.claims
    }
}
