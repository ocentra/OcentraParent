use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPasskeyError {
    InvalidCredential,
    InvalidAssertion,
    InvalidClientData,
    InvalidAuthenticatorData,
    WrongCredential,
    WrongRelyingParty,
    WrongOrigin,
    UserVerificationRequired,
    SignCountRollback,
    SignatureRejected,
}

/// A registered platform credential. Registration is deliberately separate
/// from assertion verification: the public key must come from the durable
/// parent-identity/platform credential owner, never from an assertion.
pub struct PlatformPasskeyCredential {
    pub(crate) relying_party_id: String,
    pub(crate) origin: String,
}

impl fmt::Debug for PlatformPasskeyCredential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PlatformPasskeyCredential")
            .field("credential_id", &"[redacted]")
            .field("relying_party_id", &self.relying_party_id)
            .field("origin", &self.origin)
            .finish()
    }
}

impl PlatformPasskeyCredential {}

/// The native bridge's WebAuthn assertion. It is intentionally neither
/// serializable nor cloneable; a family-owned bridge adapter must construct it
/// from the platform response and the ceremony consumes it once.
pub struct PlatformPasskeyAssertion {}

impl fmt::Debug for PlatformPasskeyAssertion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PlatformPasskeyAssertion")
            .field("credential_id", &"[redacted]")
            .field("client_data_json", &"[redacted]")
            .field("authenticator_data", &"[redacted]")
            .field("signature", &"[redacted]")
            .finish()
    }
}

impl PlatformPasskeyAssertion {}
