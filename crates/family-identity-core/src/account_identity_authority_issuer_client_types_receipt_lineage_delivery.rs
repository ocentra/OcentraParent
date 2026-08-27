use super::account_identity_authority_issuer_client_types::AccountIdentityIssuerReceiptLineage;

impl AccountIdentityIssuerReceiptLineage {
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

    pub fn correlation_id(&self) -> &str {
        self.correlation_id.as_str()
    }

    pub fn idempotency_key(&self) -> &str {
        self.idempotency_key.as_str()
    }

    pub fn receipt_id(&self) -> &str {
        self.receipt_id.as_str()
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_digest.as_str()
    }

    pub fn signed_transport_digest(&self) -> &str {
        self.signed_transport_digest.as_str()
    }

    pub fn issued_at(&self) -> &str {
        self.issued_at.as_str()
    }

    pub fn expires_at(&self) -> &str {
        self.expires_at.as_str()
    }
}
