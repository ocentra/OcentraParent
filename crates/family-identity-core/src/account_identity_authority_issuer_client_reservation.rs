/// Family-issued proof that one durable issue reservation owns the exact
/// unsigned request and protected signing attempt.  The constructor remains
/// family-private; an account caller can carry this value to the final commit
/// but cannot mint or alter its authority binding.
pub(crate) struct AccountIdentityIssuerReservation {
    reservation_id: String,
    account_id: String,
    household_id: String,
    provider: String,
    provider_subject: String,
    service_binding_id: String,
    key_id: String,
    key_generation: u64,
    enrollment_generation: u64,
    authority_generation: u64,
    session_generation: u64,
    correlation_id: String,
    idempotency_key: String,
    request_wire: Vec<u8>,
    attempt_token: String,
}

impl AccountIdentityIssuerReservation {
    pub(crate) fn reservation_id(&self) -> &str {
        self.reservation_id.as_str()
    }

    pub(crate) fn account_id(&self) -> &str {
        self.account_id.as_str()
    }

    pub(crate) fn household_id(&self) -> &str {
        self.household_id.as_str()
    }

    pub(crate) fn provider(&self) -> &str {
        self.provider.as_str()
    }

    pub(crate) fn provider_subject(&self) -> &str {
        self.provider_subject.as_str()
    }

    pub(crate) fn service_binding_id(&self) -> &str {
        self.service_binding_id.as_str()
    }

    pub(crate) fn key_id(&self) -> &str {
        self.key_id.as_str()
    }

    pub(crate) fn key_generation(&self) -> u64 {
        self.key_generation
    }

    pub(crate) fn enrollment_generation(&self) -> u64 {
        self.enrollment_generation
    }

    pub(crate) fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub(crate) fn session_generation(&self) -> u64 {
        self.session_generation
    }

    pub(crate) fn correlation_id(&self) -> &str {
        self.correlation_id.as_str()
    }

    pub(crate) fn idempotency_key(&self) -> &str {
        self.idempotency_key.as_str()
    }

    pub(crate) fn request_wire(&self) -> &[u8] {
        self.request_wire.as_slice()
    }

    pub(crate) fn attempt_token(&self) -> &str {
        self.attempt_token.as_str()
    }

    pub(crate) fn from_storage(
        reservation_id: String,
        account_id: String,
        household_id: String,
        provider: String,
        provider_subject: String,
        service_binding_id: String,
        key_id: String,
        key_generation: u64,
        enrollment_generation: u64,
        authority_generation: u64,
        session_generation: u64,
        correlation_id: String,
        idempotency_key: String,
        request_wire: Vec<u8>,
        attempt_token: String,
    ) -> Self {
        Self {
            reservation_id,
            account_id,
            household_id,
            provider,
            provider_subject,
            service_binding_id,
            key_id,
            key_generation,
            enrollment_generation,
            authority_generation,
            session_generation,
            correlation_id,
            idempotency_key,
            request_wire,
            attempt_token,
        }
    }
}
