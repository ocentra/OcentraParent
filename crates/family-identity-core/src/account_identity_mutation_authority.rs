//! Account-owned signed mutation-authority transport.
//!
//! Callers can request a typed mutation, but only the durable Account
//! repository can resolve its target and construct the signed envelope. The
//! authority value and its verified form are opaque and deliberately have no
//! serde implementations.

use sha2::{Digest, Sha256};

use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

#[path = "account_identity_mutation_authority_envelope.rs"]
pub(crate) mod envelope;
#[path = "account_identity_mutation_authority_parse.rs"]
pub(crate) mod parse;
#[path = "account_identity_mutation_authority_request.rs"]
mod request;
#[path = "account_identity_mutation_authority_validation.rs"]
pub(crate) mod validation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountIdentityMutationAction {
    RevokeChildDevice,
    RevokeSetupInvite,
    RevokeRecovery,
}

impl AccountIdentityMutationAction {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::RevokeChildDevice => "revoke-child-device",
            Self::RevokeSetupInvite => "revoke-setup-invite",
            Self::RevokeRecovery => "revoke-recovery",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "revoke-child-device" => Some(Self::RevokeChildDevice),
            "revoke-setup-invite" => Some(Self::RevokeSetupInvite),
            "revoke-recovery" => Some(Self::RevokeRecovery),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AccountIdentityMutationTarget {
    ChildDevice {
        child_profile_id: String,
        child_device_id: String,
    },
    SetupInvite(String),
    Recovery(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIdentityMutationAuthorityRequest {
    action: AccountIdentityMutationAction,
    target: AccountIdentityMutationTarget,
    idempotency_key: String,
}

/// Opaque signed bytes. No public constructor or deserializer exists.
pub struct AccountIdentityMutationAuthority {
    payload: Vec<u8>,
    signature: [u8; 64],
}

impl AccountIdentityMutationAuthority {
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + self.payload.len() + self.signature.len());
        bytes.extend_from_slice(&(self.payload.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes.extend_from_slice(&self.signature);
        bytes
    }

    pub fn payload_digest(&self) -> String {
        payload_digest(&self.payload)
    }

    pub(crate) fn from_signed_parts(payload: Vec<u8>, signature: [u8; 64]) -> Self {
        Self { payload, signature }
    }
}

/// Opaque result returned only after signature, currentness, target, clock,
/// replay, and idempotency checks commit in the Account repository.
pub struct VerifiedAccountIdentityMutationAuthority {
    action: AccountIdentityMutationAction,
    target_id: String,
    idempotency_key: String,
    payload_digest: String,
}

impl VerifiedAccountIdentityMutationAuthority {
    pub fn action(&self) -> AccountIdentityMutationAction {
        self.action
    }

    pub fn target_id(&self) -> &str {
        self.target_id.as_str()
    }

    pub fn idempotency_key(&self) -> &str {
        self.idempotency_key.as_str()
    }

    pub fn payload_digest(&self) -> &str {
        self.payload_digest.as_str()
    }

    pub(crate) fn new(
        action: AccountIdentityMutationAction,
        target_id: String,
        idempotency_key: String,
        payload_digest: String,
    ) -> Self {
        Self {
            action,
            target_id,
            idempotency_key,
            payload_digest,
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

#[derive(Clone, Debug, Eq, PartialEq)]
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
