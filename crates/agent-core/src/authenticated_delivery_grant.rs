#![forbid(unsafe_code)]

use std::{path::Path, time::Duration};

use ed25519_dalek::{Signature, VerifyingKey};
use ocentra_parent_agent_protocol::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantCarrier,
};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const CREATE_CONSUMED_GRANTS: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_consumes (nonce TEXT PRIMARY KEY NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL)";
const SELECT_CONSUMED_GRANT: &str =
    "SELECT grant_json FROM authenticated_delivery_grant_consumes WHERE nonce = ?1";
const INSERT_CONSUMED_GRANT: &str = "INSERT INTO authenticated_delivery_grant_consumes (nonce, grant_json, audit_json) VALUES (?1, ?2, ?3)";
const CONSUME_BUSY_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedDeliveryGrantExpectation {
    pub issuer_key_id: String,
    pub household_id: String,
    pub child_profile_id: String,
    pub target_device_id: String,
    pub policy_decision_id: String,
    pub action_id: String,
    pub payload_digest: String,
    pub revocation_version: String,
    pub observed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedDeliveryGrantAudit {
    pub correlation_id: String,
    pub issuer_key_id: String,
    pub nonce_digest: String,
    pub grant_digest: String,
    pub outcome: AuthenticatedDeliveryGrantAuditOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticatedDeliveryGrantAuditOutcome {
    #[serde(rename = "consumed")]
    Consumed,
    #[serde(rename = "replay-rejected")]
    ReplayRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticatedDeliveryGrantConsumeOutcome {
    Consumed(AuthenticatedDeliveryGrantAudit),
    ReplayRejected(AuthenticatedDeliveryGrantAudit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticatedDeliveryGrantConsumeError {
    InvalidGrant,
    SignatureRejected,
    BindingRejected,
    Expired,
    DryRunRejected,
    Revoked,
    IntegrityRejected,
    StorageUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticatedDeliveryGrantVerifierError {
    InvalidIssuerKeyId,
    InvalidPublicKey,
}

/// A receiver-owned pinned issuer verifier. The signed delivery carrier never
/// carries this key, so a sender cannot choose its own trust root.
pub struct AuthenticatedDeliveryGrantVerifier {
    issuer_key_id: String,
    verifying_key: VerifyingKey,
}

impl AuthenticatedDeliveryGrantVerifier {
    pub fn from_pinned_public_key(
        issuer_key_id: impl Into<String>,
        public_key: [u8; 32],
    ) -> Result<Self, AuthenticatedDeliveryGrantVerifierError> {
        let issuer_key_id = issuer_key_id.into();
        if issuer_key_id.trim().is_empty() {
            return Err(AuthenticatedDeliveryGrantVerifierError::InvalidIssuerKeyId);
        }
        let verifying_key = VerifyingKey::from_bytes(&public_key)
            .map_err(|_error| AuthenticatedDeliveryGrantVerifierError::InvalidPublicKey)?;
        Ok(Self {
            issuer_key_id,
            verifying_key,
        })
    }

    pub fn issuer_key_id(&self) -> &str {
        &self.issuer_key_id
    }
}

pub struct AuthenticatedDeliveryGrantConsumer {
    connection: Connection,
    verifier: AuthenticatedDeliveryGrantVerifier,
    #[cfg(debug_assertions)]
    fail_next_commit: bool,
}

impl AuthenticatedDeliveryGrantConsumer {
    pub fn open(
        path: impl AsRef<Path>,
        verifier: AuthenticatedDeliveryGrantVerifier,
    ) -> Result<Self, AuthenticatedDeliveryGrantConsumeError> {
        let connection = Connection::open(path)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        connection
            .busy_timeout(CONSUME_BUSY_TIMEOUT)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        connection
            .execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        connection
            .execute(CREATE_CONSUMED_GRANTS, [])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        Ok(Self {
            connection,
            verifier,
            #[cfg(debug_assertions)]
            fail_next_commit: false,
        })
    }

    pub fn consume(
        &mut self,
        carrier: &AuthenticatedDeliveryGrantCarrier,
        expected: &AuthenticatedDeliveryGrantExpectation,
        correlation_id: impl Into<String>,
    ) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError>
    {
        let grant = carrier.grant();
        validate_grant(grant, expected, &self.verifier)?;
        let correlation_id = correlation_id.into();
        if correlation_id.trim().is_empty() {
            return Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected);
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let stored: Option<String> = transaction
            .query_row(SELECT_CONSUMED_GRANT, params![grant.nonce], |row| {
                row.get(0)
            })
            .optional()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        if let Some(stored) = stored {
            let stored: AuthenticatedDeliveryGrant = serde_json::from_str(&stored)
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
            if stored.signing_bytes() != grant.signing_bytes()
                || stored.signature != grant.signature
            {
                return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
            }
            let audit = audit(
                grant,
                correlation_id,
                AuthenticatedDeliveryGrantAuditOutcome::ReplayRejected,
            );
            transaction
                .commit()
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
            return Ok(AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(
                audit,
            ));
        }
        let audit = audit(
            grant,
            correlation_id,
            AuthenticatedDeliveryGrantAuditOutcome::Consumed,
        );
        let grant_json = serde_json::to_string(grant)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        let audit_json = serde_json::to_string(&audit)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        transaction
            .execute(
                INSERT_CONSUMED_GRANT,
                params![grant.nonce, grant_json, audit_json],
            )
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        #[cfg(debug_assertions)]
        if std::mem::take(&mut self.fail_next_commit) {
            return Err(AuthenticatedDeliveryGrantConsumeError::StorageUnavailable);
        }
        transaction
            .commit()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        Ok(AuthenticatedDeliveryGrantConsumeOutcome::Consumed(audit))
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_commit_failure_for_debug(&mut self) {
        self.fail_next_commit = true;
    }
}

fn validate_grant(
    grant: &AuthenticatedDeliveryGrant,
    expected: &AuthenticatedDeliveryGrantExpectation,
    verifier: &AuthenticatedDeliveryGrantVerifier,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    grant
        .validate_shape()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::InvalidGrant)?;
    if expected.issuer_key_id != verifier.issuer_key_id()
        || grant.issuer_key_id != verifier.issuer_key_id()
    {
        return Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected);
    }
    let signature = Signature::from_slice(&grant.signature)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::SignatureRejected)?;
    verifier
        .verifying_key
        .verify_strict(&grant.signing_bytes(), &signature)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::SignatureRejected)?;
    if grant.dry_run {
        return Err(AuthenticatedDeliveryGrantConsumeError::DryRunRejected);
    }
    if grant.expires_at < expected.observed_at {
        return Err(AuthenticatedDeliveryGrantConsumeError::Expired);
    }
    if grant.revocation_version != expected.revocation_version {
        return Err(AuthenticatedDeliveryGrantConsumeError::Revoked);
    }
    if grant.household_id != expected.household_id
        || grant.child_profile_id != expected.child_profile_id
        || grant.target_device_id != expected.target_device_id
        || grant.policy_decision_id != expected.policy_decision_id
        || grant.action_id != expected.action_id
        || grant.payload_digest != expected.payload_digest
    {
        return Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected);
    }
    Ok(())
}

fn audit(
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: String,
    outcome: AuthenticatedDeliveryGrantAuditOutcome,
) -> AuthenticatedDeliveryGrantAudit {
    AuthenticatedDeliveryGrantAudit {
        correlation_id,
        issuer_key_id: grant.issuer_key_id.clone(),
        nonce_digest: digest(&grant.nonce),
        grant_digest: digest(grant.signing_bytes()),
        outcome,
    }
}

fn digest(value: impl AsRef<[u8]>) -> String {
    let digest = Sha256::digest(value.as_ref());
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(output, "{byte:02x}");
    }
    output
}
