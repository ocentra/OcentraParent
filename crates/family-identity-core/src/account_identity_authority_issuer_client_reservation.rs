/// Family-issued proof that one durable issue reservation owns the exact
/// unsigned request and protected signing attempt.  The constructor remains
/// family-private; an account caller can carry this value to the final commit
/// but cannot mint or alter its authority binding.
pub struct AccountIdentityIssuerReservation {
    reservation_id: String,
    account_id: String,
    household_id: String,
    provider: String,
    provider_subject: String,
    service: String,
    service_binding_id: String,
    key_id: String,
    key_generation: u64,
    enrollment_generation: u64,
    authority_generation: u64,
    session_generation: u64,
    correlation_id: String,
    idempotency_key: String,
    request_digest: String,
    request_wire: Vec<u8>,
    reservation_state: String,
    signer_status: String,
    attempt_token: String,
    lease_expires_at: String,
}

impl AccountIdentityIssuerReservation {
    pub fn reservation_id(&self) -> &str {
        self.reservation_id.as_str()
    }

    pub fn account_id(&self) -> &str {
        self.account_id.as_str()
    }

    pub fn household_id(&self) -> &str {
        self.household_id.as_str()
    }

    pub fn provider(&self) -> &str {
        self.provider.as_str()
    }

    pub fn provider_subject(&self) -> &str {
        self.provider_subject.as_str()
    }

    pub fn service_binding_id(&self) -> &str {
        self.service_binding_id.as_str()
    }

    pub fn key_id(&self) -> &str {
        self.key_id.as_str()
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

    pub fn correlation_id(&self) -> &str {
        self.correlation_id.as_str()
    }

    pub fn idempotency_key(&self) -> &str {
        self.idempotency_key.as_str()
    }

    pub fn request_wire(&self) -> &[u8] {
        self.request_wire.as_slice()
    }

    pub fn attempt_token(&self) -> &str {
        self.attempt_token.as_str()
    }

    pub(crate) fn from_storage(
        reservation_id: String,
        account_id: String,
        household_id: String,
        provider: String,
        provider_subject: String,
        service: String,
        service_binding_id: String,
        key_id: String,
        key_generation: u64,
        enrollment_generation: u64,
        authority_generation: u64,
        session_generation: u64,
        correlation_id: String,
        idempotency_key: String,
        request_digest: String,
        request_wire: Vec<u8>,
        reservation_state: String,
        signer_status: String,
        attempt_token: String,
        lease_expires_at: String,
    ) -> Self {
        Self {
            reservation_id,
            account_id,
            household_id,
            provider,
            provider_subject,
            service,
            service_binding_id,
            key_id,
            key_generation,
            enrollment_generation,
            authority_generation,
            session_generation,
            correlation_id,
            idempotency_key,
            request_digest,
            request_wire,
            reservation_state,
            signer_status,
            attempt_token,
            lease_expires_at,
        }
    }
}
