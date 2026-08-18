//! Account-owned signed mutation-authority transport.
//!
//! The durable Account producer resolves a current opaque authority first,
//! then signs a canonical, short-lived envelope. The envelope is a transport
//! value only: it has no serde implementation or public field constructors,
//! and callers cannot mint it by supplying authority scalars or headers.

use chrono::{SecondsFormat, Utc};
use ed25519_dalek::{Signer, SigningKey};
use getrandom::fill;
use sha2::{Digest, Sha256};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::family_identity::{RecoveryId, SetupInviteId};

#[path = "account_identity_mutation_authority_envelope.rs"]
mod envelope;
#[path = "account_identity_mutation_authority_validation.rs"]
mod validation;

use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountIdentityMutationAction {
    RevokeChildDevice,
    RevokeSetupInvite,
    RevokeRecovery,
}

impl AccountIdentityMutationAction {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::RevokeChildDevice => "revoke-child-device",
            Self::RevokeSetupInvite => "revoke-setup-invite",
            Self::RevokeRecovery => "revoke-recovery",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccountIdentityMutationTarget {
    ChildDevice {
        child_profile_id: String,
        child_device_id: String,
    },
    SetupInvite(String),
    Recovery(String),
}

impl AccountIdentityMutationTarget {
    pub fn child_device(
        child_profile_id: &crate::family_identity::ChildProfileId,
        child_device_id: &ocentra_schema::account_identity_authority::AccountIdentityChildDeviceId,
    ) -> Self {
        Self::ChildDevice {
            child_profile_id: child_profile_id.as_str().to_owned(),
            child_device_id: child_device_id.as_str().to_owned(),
        }
    }

    pub fn setup_invite(invite_id: &SetupInviteId) -> Self {
        Self::SetupInvite(invite_id.as_str().to_owned())
    }

    pub fn recovery(recovery_id: &RecoveryId) -> Self {
        Self::Recovery(recovery_id.as_str().to_owned())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIdentityMutationAuthorityRequest {
    pub(super) action: AccountIdentityMutationAction,
    pub(super) target: AccountIdentityMutationTarget,
    pub(super) idempotency_key: String,
}

impl AccountIdentityMutationAuthorityRequest {
    pub fn new(
        action: AccountIdentityMutationAction,
        target: AccountIdentityMutationTarget,
        idempotency_key: impl Into<String>,
    ) -> Result<Self, AccountIdentityMutationAuthorityError> {
        let request = Self {
            action,
            target,
            idempotency_key: idempotency_key.into(),
        };
        validation::validate_request(&request)?;
        Ok(request)
    }

    pub fn action(&self) -> AccountIdentityMutationAction {
        self.action
    }

    pub fn idempotency_key(&self) -> &str {
        self.idempotency_key.as_str()
    }
}

/// Opaque signed transport value. It is deliberately not Serialize/Deserialize,
/// Clone, or Debug; only the Account-owned producer can construct it.
pub struct AccountIdentityMutationAuthority {
    payload: Vec<u8>,
    signature: [u8; 64],
}

impl AccountIdentityMutationAuthority {
    pub fn wire_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.payload.len() + self.signature.len());
        bytes.extend_from_slice(&(self.payload.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes.extend_from_slice(&self.signature);
        bytes
    }

    pub fn payload_digest(&self) -> String {
        format!("sha256:{:x}", Sha256::digest(&self.payload))
    }

    pub(super) fn signed_parts(&self) -> (&[u8], &[u8; 64]) {
        (&self.payload, &self.signature)
    }
}

pub(crate) struct AccountIdentityMutationAuthorityIssuer {
    key_id: String,
    signing_key: SigningKey,
}

impl AccountIdentityMutationAuthorityIssuer {
    pub(crate) fn generate() -> Result<Self, AccountIdentityMutationAuthorityError> {
        let mut private_key = [0_u8; 32];
        fill(&mut private_key)
            .map_err(|_| AccountIdentityMutationAuthorityError::EntropyUnavailable)?;
        let signing_key = SigningKey::from_bytes(&private_key);
        let key_id = format!(
            "sha256:{:x}",
            Sha256::digest(signing_key.verifying_key().as_bytes())
        );
        Ok(Self {
            key_id,
            signing_key,
        })
    }

    pub(crate) fn issue(
        &self,
        authority: &VerifiedAccountIdentityAuthority,
        request: &AccountIdentityMutationAuthorityRequest,
    ) -> Result<AccountIdentityMutationAuthority, AccountIdentityMutationAuthorityError> {
        validation::validate_request(request)?;
        validation::validate_against_current_authority(authority, request)?;

        let issued_at = Utc::now();
        let expires_at = issued_at + validation::MAX_AUTHORITY_LIFETIME;
        validation::validate_lifetime(issued_at, expires_at)?;
        let issued_at = issued_at.to_rfc3339_opts(SecondsFormat::Millis, true);
        let expires_at = expires_at.to_rfc3339_opts(SecondsFormat::Millis, true);
        let envelope = envelope::from_request(
            self.key_id.as_str(),
            authority,
            request,
            &issued_at,
            &expires_at,
        );
        let payload = envelope::encode(&envelope);
        let signature = self.signing_key.sign(&payload).to_bytes();
        Ok(AccountIdentityMutationAuthority { payload, signature })
    }

    pub(crate) fn verifying_key(&self) -> ed25519_dalek::VerifyingKey {
        self.signing_key.verifying_key()
    }
}

pub struct AccountIdentityMutationAuthorityVerifier {
    verifying_key: ed25519_dalek::VerifyingKey,
}

impl AccountIdentityMutationAuthorityVerifier {
    pub(crate) fn from_account_issuer(issuer: &AccountIdentityMutationAuthorityIssuer) -> Self {
        Self {
            verifying_key: issuer.verifying_key(),
        }
    }

    pub(crate) fn verify(
        &self,
        authority: &AccountIdentityMutationAuthority,
    ) -> Result<(), AccountIdentityMutationAuthorityError> {
        let signature = ed25519_dalek::Signature::from_bytes(authority.signed_parts().1);
        self.verifying_key
            .verify_strict(authority.signed_parts().0, &signature)
            .map_err(|_| AccountIdentityMutationAuthorityError::SignatureUnavailable)
    }
}

impl AccountIdentityMutationAuthorityRequest {
    pub(crate) fn target(&self) -> &AccountIdentityMutationTarget {
        &self.target
    }
}
