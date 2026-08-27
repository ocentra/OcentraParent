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
    pub(super) lineage: AccountIdentityIssuerReceiptLineage,
    pub(crate) replayed: bool,
}

/// Exact owner-facing lineage recovered from a verified signed transport and
/// its committed Account receipt. It is evidence, never issuance authority.
pub struct AccountIdentityIssuerReceiptLineage {
    pub(super) account_id: String,
    pub(super) household_id: String,
    pub(super) provider: ocentra_schema::account_identity_authority::AccountIdentityProvider,
    pub(super) provider_subject:
        ocentra_schema::account_identity_authority::AccountIdentityProviderSubject,
    pub(super) member_id: String,
    pub(super) device_id: String,
    pub(super) session_id: String,
    pub(super) service: String,
    pub(super) service_binding_id: String,
    pub(super) key_id: String,
    pub(super) key_generation: u64,
    pub(super) enrollment_generation: u64,
    pub(super) authority_generation: u64,
    pub(super) session_generation: u64,
    pub(super) correlation_id: String,
    pub(super) idempotency_key: String,
    pub(super) receipt_id: String,
    pub(super) payload_digest: String,
    pub(super) signed_transport_digest: String,
    pub(super) issued_at: String,
    pub(super) expires_at: String,
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
