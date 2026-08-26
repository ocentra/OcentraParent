use ocentra_schema::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Operation;

use super::AccountIdentityAuthorityProducerV2Request;

impl AccountIdentityAuthorityProducerV2Request {
    pub fn signing_bytes(&self) -> &[u8] {
        self.signing_bytes.as_slice()
    }

    pub fn operation(&self) -> AccountIdentityAuthorityProducerV2Operation {
        self.operation
    }

    pub fn binding(
        &self,
    ) -> &ocentra_schema::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Binding
    {
        &self.binding
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_digest.as_str()
    }

    pub fn enrollment_generation(&self) -> u64 {
        self.binding.enrollment_generation
    }

    pub fn issued_at(&self) -> &str {
        self.issued_at.as_str()
    }

    pub fn expires_at(&self) -> &str {
        self.expires_at.as_str()
    }
}
