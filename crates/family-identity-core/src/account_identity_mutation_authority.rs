//! Account-owned signed mutation-authority transport.
//!
//! Callers can request a typed mutation, but only the durable Account
//! repository can resolve its target and construct the signed envelope. The
//! authority value is opaque and deliberately has no serde implementation.
//! Consumption commits the mutation and returns only its durable outcome.

use sha2::{Digest, Sha256};

use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

#[path = "account_identity_mutation_action.rs"]
mod action;
#[path = "account_identity_mutation_authority_envelope.rs"]
pub(crate) mod envelope;
#[path = "account_identity_mutation_authority_parse.rs"]
pub(crate) mod parse;
#[path = "account_identity_mutation_authority_protocol.rs"]
pub(crate) mod protocol;
#[path = "account_identity_mutation_authority_request.rs"]
mod request;
#[path = "account_identity_mutation_result.rs"]
mod result;
#[path = "account_identity_mutation_authority_validation.rs"]
pub(crate) mod validation;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AccountIdentityMutationAction {
    RevokeChildDevice,
    RevokeSetupInvite,
    RevokeRecovery,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) enum AccountIdentityMutationTarget {
    ChildDevice {
        child_profile_id: String,
        child_device_id: String,
    },
    SetupInvite(String),
    Recovery(String),
}

#[derive(Clone, Eq, PartialEq)]
pub struct AccountIdentityMutationAuthorityRequest {
    action: AccountIdentityMutationAction,
    target: AccountIdentityMutationTarget,
    idempotency_key: String,
}

/// Opaque signed bytes. No public constructor or deserializer exists.
pub struct AccountIdentityMutationAuthority {
    wire: Vec<u8>,
    payload_digest: String,
}

impl AccountIdentityMutationAuthority {
    pub fn wire_bytes(&self) -> &[u8] {
        self.wire.as_slice()
    }

    pub fn payload_digest(&self) -> String {
        self.payload_digest.clone()
    }

    pub(crate) fn from_signed_parts(
        payload: Vec<u8>,
        signature: [u8; 64],
    ) -> Result<Self, AccountIdentityMutationAuthorityError> {
        let payload_length = u32::try_from(payload.len())
            .map_err(|_| AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
        let wire_capacity = 4_usize
            .checked_add(payload.len())
            .and_then(|value| value.checked_add(signature.len()))
            .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
        let payload_digest = payload_digest(&payload);
        let mut wire = Vec::with_capacity(wire_capacity);
        wire.extend_from_slice(&payload_length.to_be_bytes());
        wire.extend_from_slice(&payload);
        wire.extend_from_slice(&signature);
        Ok(Self {
            wire,
            payload_digest,
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AccountIdentityMutationResult {
    SetupInviteRevoked,
    RecoveryRevoked,
}

/// Outcome committed by the Account repository in the same transaction as
/// verification and mutation. A retry of the identical signed request returns
/// the recorded result with `repeated` set instead of applying twice.
pub struct AccountIdentityMutationOutcome {
    result: AccountIdentityMutationResult,
    repeated: bool,
}

impl AccountIdentityMutationOutcome {
    pub fn result(&self) -> AccountIdentityMutationResult {
        self.result
    }

    pub fn repeated(&self) -> bool {
        self.repeated
    }

    pub(crate) fn committed(result: AccountIdentityMutationResult) -> Self {
        Self {
            result,
            repeated: false,
        }
    }

    pub(crate) fn recorded(result: AccountIdentityMutationResult) -> Self {
        Self {
            result,
            repeated: true,
        }
    }
}

/// Internal seam for a future durable platform signer and public-key registry.
/// No implementation is supplied by this packet, so production issuance and
/// consumption remain typed unavailable instead of generating a process key.
pub(crate) trait AccountIdentityMutationAuthorityCustody: Send + Sync {
    fn signing_key_id(&self) -> &str;
    fn verification_key(
        &self,
        key_id: &str,
    ) -> Result<ed25519_dalek::VerifyingKey, AccountIdentityMutationAuthorityError>;
    fn sign(&self, payload: &[u8]) -> Result<[u8; 64], AccountIdentityMutationAuthorityError>;
}

pub(crate) struct ResolvedAccountIdentityMutationTarget {
    pub(crate) kind: String,
    pub(crate) target_id: String,
    pub(crate) child_profile_id: String,
    pub(crate) child_device_id: String,
    pub(crate) household_id: String,
    pub(crate) owner_member_id: String,
    pub(crate) state: String,
    pub(crate) expires_at_epoch_millis: i64,
    pub(crate) support_channel: String,
    pub(crate) support_authorization_id: String,
    pub(crate) support_authorization_issuer: String,
    pub(crate) support_authorization_scope: String,
    pub(crate) support_authorization_expires_at_epoch_millis: i64,
}

pub(crate) fn payload_digest(payload: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(payload))
}

pub(crate) fn expected_key_id(verifying_key: &ed25519_dalek::VerifyingKey) -> String {
    format!("sha256:{:x}", Sha256::digest(verifying_key.as_bytes()))
}
