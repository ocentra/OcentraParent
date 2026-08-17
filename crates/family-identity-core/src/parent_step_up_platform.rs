use std::fmt;

use ed25519_dalek::VerifyingKey;

const MAX_PLATFORM_FIELD_BYTES: usize = 512;
const AUTHENTICATOR_DATA_MIN_BYTES: usize = 37;

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
    pub(crate) credential_id: String,
    pub(crate) relying_party_id: String,
    pub(crate) origin: String,
    pub(crate) verifying_key: VerifyingKey,
    pub(crate) stored_sign_count: u32,
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

impl PlatformPasskeyCredential {
    /// Construct only inside the family-identity owner after loading the
    /// credential from its trusted platform credential store.  There is no
    /// public constructor: a runtime caller cannot self-authorize by
    /// supplying a public key alongside an assertion.
    pub(crate) fn from_registered(
        credential_id: String,
        relying_party_id: String,
        origin: String,
        public_key: [u8; 32],
        stored_sign_count: u32,
    ) -> Result<Self, PlatformPasskeyError> {
        if credential_id.trim().is_empty()
            || relying_party_id.trim().is_empty()
            || origin.trim().is_empty()
            || credential_id.len() > MAX_PLATFORM_FIELD_BYTES
            || relying_party_id.len() > MAX_PLATFORM_FIELD_BYTES
            || origin.len() > MAX_PLATFORM_FIELD_BYTES
        {
            return Err(PlatformPasskeyError::InvalidCredential);
        }
        let verifying_key = VerifyingKey::from_bytes(&public_key)
            .map_err(|_error| PlatformPasskeyError::InvalidCredential)?;
        if verifying_key.is_weak() {
            return Err(PlatformPasskeyError::InvalidCredential);
        }
        Ok(Self {
            credential_id,
            relying_party_id,
            origin,
            verifying_key,
            stored_sign_count,
        })
    }
}

/// The native bridge's WebAuthn assertion. It is intentionally neither
/// serializable nor cloneable; a family-owned bridge adapter must construct it
/// from the platform response and the ceremony consumes it once.
pub struct PlatformPasskeyAssertion {
    pub(crate) credential_id: String,
    pub(crate) client_data_json: Vec<u8>,
    pub(crate) authenticator_data: Vec<u8>,
    pub(crate) signature: Vec<u8>,
}

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

impl PlatformPasskeyAssertion {
    pub(crate) fn from_web_platform(
        credential_id: String,
        client_data_json: Vec<u8>,
        authenticator_data: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<Self, PlatformPasskeyError> {
        if credential_id.trim().is_empty()
            || credential_id.len() > MAX_PLATFORM_FIELD_BYTES
            || client_data_json.is_empty()
            || client_data_json.len() > MAX_PLATFORM_FIELD_BYTES
            || authenticator_data.len() < AUTHENTICATOR_DATA_MIN_BYTES
            || signature.len() != 64
        {
            return Err(PlatformPasskeyError::InvalidAssertion);
        }
        Ok(Self {
            credential_id,
            client_data_json,
            authenticator_data,
            signature,
        })
    }

    pub(crate) fn verify_for_challenge(
        self,
        challenge_ref: &str,
        credential: &PlatformPasskeyCredential,
    ) -> Result<
        super::parent_step_up_platform_verification::VerifiedPlatformAssertion,
        PlatformPasskeyError,
    > {
        super::parent_step_up_platform_verification::verify(self, challenge_ref, credential)
    }
}
