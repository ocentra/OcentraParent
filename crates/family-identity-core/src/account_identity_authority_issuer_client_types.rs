#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountIdentityIssuerAccountId(String);

impl AccountIdentityIssuerAccountId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn from_value(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountIdentityIssuerHouseholdId(String);

impl AccountIdentityIssuerHouseholdId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn from_value(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountIdentityIssuerV2KeyId(String);

impl AccountIdentityIssuerV2KeyId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn from_value(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountIdentityIssuerV2ServiceBindingId(String);

impl AccountIdentityIssuerV2ServiceBindingId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn from_value(value: String) -> Self {
        Self(value)
    }
}

/// Protected-adapter key enrollment capability.  No public constructor
/// accepts raw key material; a future broker/Windows adapter must mint this
/// value inside the protected custody boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtectedAccountIssuerKeyRegistration {
    public_key: [u8; 65],
    enrollment_generation: u64,
}

impl ProtectedAccountIssuerKeyRegistration {
    pub fn public_key(&self) -> &[u8; 65] {
        &self.public_key
    }

    pub(crate) fn from_protected_adapter(
        public_key: [u8; 65],
        enrollment_generation: u64,
    ) -> Option<Self> {
        (enrollment_generation > 0
            && enrollment_generation
                <= ocentra_schema::account_identity_authority_producer_v2::
                    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_ENROLLMENT_GENERATION)
        .then_some(Self {
            public_key,
            enrollment_generation,
        })
    }

    pub(crate) fn enrollment_generation(&self) -> u64 {
        self.enrollment_generation
    }
}

/// The durable winner of an issue insert.  On an idempotency collision this
/// always contains the already-stored verified transport, never the losing
/// newly-signed candidate.
pub struct AccountIdentityIssuerRecordedTransport {
    pub(crate) transport: AccountIdentityAuthorityProducerV2Transport,
    pub(crate) replayed: bool,
}

impl AccountIdentityIssuerRecordedTransport {
    pub fn transport(&self) -> &AccountIdentityAuthorityProducerV2Transport {
        &self.transport
    }

    pub fn replayed(&self) -> bool {
        self.replayed
    }
}

/// Durable outbox lease returned by a short `BEGIN IMMEDIATE` claim.
pub struct AccountIdentityIssuerOutboxClaim {
    pub(crate) receipt_id: String,
    pub(crate) claim_id: String,
    pub(crate) claim_expires_at: String,
    pub(crate) wire: Vec<u8>,
}

impl AccountIdentityIssuerOutboxClaim {
    pub fn receipt_id(&self) -> &str {
        self.receipt_id.as_str()
    }

    pub fn wire(&self) -> &[u8] {
        self.wire.as_slice()
    }

    pub(crate) fn claim_id(&self) -> &str {
        self.claim_id.as_str()
    }

    pub(crate) fn claim_expires_at(&self) -> &str {
        self.claim_expires_at.as_str()
    }
}
use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Transport;
