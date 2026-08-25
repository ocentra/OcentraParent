use std::collections::BTreeMap;

use ed25519_dalek::Signature;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::parent_step_up_challenge_codec::is_canonical_base64url;
use super::parent_step_up_platform::{
    PlatformPasskeyAssertion, PlatformPasskeyCredential, PlatformPasskeyError,
};

const AUTHENTICATOR_DATA_MIN_BYTES: usize = 37;
const USER_PRESENT: u8 = 0x01;
const USER_VERIFIED: u8 = 0x04;
const RESERVED_FLAGS: u8 = 0x22;
const BACKUP_FLAGS: u8 = 0x18;
const ATTESTED_CREDENTIAL_DATA: u8 = 0x40;
const EXTENSION_DATA: u8 = 0x80;
const COSE_EDDSA: i32 = -8;

/// Opaque result of cryptographically verifying one platform assertion.
/// Callers cannot construct or clone this value and therefore cannot inject
/// credential identity, algorithm, or sign-count into signer registration.
pub(crate) struct VerifiedPlatformAssertion {
    credential_id: String,
    algorithm: i32,
    sign_count: u32,
}

impl VerifiedPlatformAssertion {
    pub(crate) fn credential_id(&self) -> &str {
        &self.credential_id
    }

    pub(crate) fn algorithm(&self) -> i32 {
        self.algorithm
    }

    pub(crate) fn sign_count(&self) -> u32 {
        self.sign_count
    }
}

pub(crate) fn verify(
    assertion: PlatformPasskeyAssertion,
    challenge_ref: &str,
    credential: &PlatformPasskeyCredential,
) -> Result<VerifiedPlatformAssertion, PlatformPasskeyError> {
    if assertion.credential_id != credential.credential_id {
        return Err(PlatformPasskeyError::WrongCredential);
    }
    let client_data: ClientData = serde_json::from_slice(&assertion.client_data_json)
        .map_err(|_error| PlatformPasskeyError::InvalidClientData)?;
    if client_data.typ != "webauthn.get" {
        return Err(PlatformPasskeyError::InvalidClientData);
    }
    if client_data.challenge != challenge_ref || !is_canonical_base64url(challenge_ref) {
        return Err(PlatformPasskeyError::InvalidClientData);
    }
    if client_data.origin != credential.origin
        || client_data.cross_origin == Some(true)
        || client_data.extra.contains_key("topOrigin")
    {
        return Err(PlatformPasskeyError::WrongOrigin);
    }
    if assertion.authenticator_data.len() < AUTHENTICATOR_DATA_MIN_BYTES {
        return Err(PlatformPasskeyError::InvalidAuthenticatorData);
    }
    let relying_party_hash = Sha256::digest(credential.relying_party_id.as_bytes());
    if assertion.authenticator_data[..32] != relying_party_hash[..] {
        return Err(PlatformPasskeyError::WrongRelyingParty);
    }
    let flags = assertion.authenticator_data[32];
    if flags & RESERVED_FLAGS != 0
        || flags & BACKUP_FLAGS != 0
        || flags & ATTESTED_CREDENTIAL_DATA != 0
        || flags & EXTENSION_DATA != 0
        || assertion.authenticator_data.len() != AUTHENTICATOR_DATA_MIN_BYTES
    {
        return Err(PlatformPasskeyError::InvalidAuthenticatorData);
    }
    if flags & USER_PRESENT == 0 || flags & USER_VERIFIED == 0 {
        return Err(PlatformPasskeyError::UserVerificationRequired);
    }
    let sign_count = u32::from_be_bytes([
        assertion.authenticator_data[33],
        assertion.authenticator_data[34],
        assertion.authenticator_data[35],
        assertion.authenticator_data[36],
    ]);
    if (credential.stored_sign_count != 0 && sign_count == 0)
        || (sign_count != 0 && sign_count <= credential.stored_sign_count)
    {
        return Err(PlatformPasskeyError::SignCountRollback);
    }
    let client_data_hash = Sha256::digest(&assertion.client_data_json);
    let mut signed_bytes = Vec::with_capacity(assertion.authenticator_data.len() + 32);
    signed_bytes.extend_from_slice(&assertion.authenticator_data);
    signed_bytes.extend_from_slice(&client_data_hash);
    let signature = Signature::from_slice(&assertion.signature)
        .map_err(|_error| PlatformPasskeyError::InvalidAssertion)?;
    credential
        .verifying_key
        .verify_strict(&signed_bytes, &signature)
        .map_err(|_error| PlatformPasskeyError::SignatureRejected)?;
    Ok(VerifiedPlatformAssertion {
        credential_id: assertion.credential_id,
        algorithm: COSE_EDDSA,
        sign_count,
    })
}

#[derive(Deserialize)]
struct ClientData {
    #[serde(rename = "type")]
    typ: String,
    challenge: String,
    origin: String,
    #[serde(rename = "crossOrigin")]
    cross_origin: Option<bool>,
    #[serde(flatten)]
    extra: BTreeMap<String, serde_json::Value>,
}
