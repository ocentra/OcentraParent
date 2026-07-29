#![forbid(unsafe_code)]

use std::{path::Path, time::Duration};

use chrono::{SecondsFormat, Utc};
use ed25519_dalek::{Signature, VerifyingKey};
use ocentra_parent_agent_protocol::authenticated_delivery_grant::{
    parse_authenticated_delivery_grant_instant, AuthenticatedDeliveryGrant,
    AuthenticatedDeliveryGrantInstant,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

mod authenticated_delivery_grant_retention;

const CREATE_CONSUMED_GRANTS: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL, PRIMARY KEY (issuer_key_id, nonce))";
const SELECT_CONSUMED_GRANT: &str =
    "SELECT grant_json FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";
const INSERT_CONSUMED_GRANT: &str = "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_json, audit_json) VALUES (?1, ?2, ?3, ?4)";
const CREATE_GRANT_AUDITS: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_audits_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, audit_json TEXT NOT NULL)";
const INSERT_GRANT_AUDIT: &str = "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json) VALUES (?1, ?2, ?3)";
const CONSUME_BUSY_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedDeliveryGrantExpectation {
    pub issuer_actor_id: String,
    pub household_id: String,
    pub parent_device_id: String,
    pub child_profile_id: String,
    pub target_device_id: String,
    pub policy_decision_id: String,
    pub policy_version: String,
    pub action_id: String,
    pub capability_id: String,
    pub evidence_digest: String,
    pub payload_digest: String,
    pub revocation_version: String,
    pub observed_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedDeliveryGrantTrustedIssuer {
    pub key_id: String,
    pub verifying_key: VerifyingKey,
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
    NotYetValid,
    DryRunRejected,
    Revoked,
    IntegrityRejected,
    StorageUnavailable,
}

pub struct AuthenticatedDeliveryGrantConsumer {
    connection: Connection,
    trusted_issuer: AuthenticatedDeliveryGrantTrustedIssuer,
    trusted_now: AuthenticatedDeliveryGrantInstant,
    #[cfg(debug_assertions)]
    fail_next_commit: bool,
}

impl AuthenticatedDeliveryGrantConsumer {
    pub fn open(
        path: impl AsRef<Path>,
        trusted_issuer: AuthenticatedDeliveryGrantTrustedIssuer,
    ) -> Result<Self, AuthenticatedDeliveryGrantConsumeError> {
        Self::open_at(
            path,
            trusted_issuer,
            Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true),
        )
    }

    fn open_at(
        path: impl AsRef<Path>,
        trusted_issuer: AuthenticatedDeliveryGrantTrustedIssuer,
        trusted_now: String,
    ) -> Result<Self, AuthenticatedDeliveryGrantConsumeError> {
        let trusted_now = parse_observed_at(&trusted_now)?;
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
        connection
            .execute(CREATE_GRANT_AUDITS, [])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        Ok(Self {
            connection,
            trusted_issuer,
            trusted_now,
            #[cfg(debug_assertions)]
            fail_next_commit: false,
        })
    }

    #[cfg(debug_assertions)]
    pub fn open_at_for_debug_test(
        path: impl AsRef<Path>,
        trusted_issuer: AuthenticatedDeliveryGrantTrustedIssuer,
        trusted_now: impl Into<String>,
    ) -> Result<Self, AuthenticatedDeliveryGrantConsumeError> {
        Self::open_at(path, trusted_issuer, trusted_now.into())
    }

    pub fn consume(
        &mut self,
        grant: &AuthenticatedDeliveryGrant,
        expected: &AuthenticatedDeliveryGrantExpectation,
        correlation_id: impl Into<String>,
    ) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError>
    {
        validate_grant(grant, expected, &self.trusted_issuer, self.trusted_now)?;
        authenticated_delivery_grant_retention::purge_expired_replay_records(
            &mut self.connection,
            self.trusted_now,
        )?;
        let correlation_id = correlation_id.into();
        if correlation_id.trim().is_empty()
            || correlation_id.len() > ocentra_parent_agent_protocol::authenticated_delivery_grant::AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
        {
            return Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected);
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let stored: Option<String> = transaction
            .query_row(
                SELECT_CONSUMED_GRANT,
                params![grant.issuer_key_id, grant.nonce],
                |row| row.get(0),
            )
            .optional()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        if let Some(stored) = stored {
            return reject_replay(transaction, grant, correlation_id, stored);
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
                params![grant.issuer_key_id, grant.nonce, grant_json, audit_json],
            )
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
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

fn reject_replay(
    transaction: Transaction<'_>,
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: String,
    stored: String,
) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError> {
    let stored: AuthenticatedDeliveryGrant = serde_json::from_str(&stored)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    if stored.signing_bytes() != grant.signing_bytes() || stored.signature != grant.signature {
        return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
    }
    let audit = audit(
        grant,
        correlation_id,
        AuthenticatedDeliveryGrantAuditOutcome::ReplayRejected,
    );
    let audit_json = serde_json::to_string(&audit)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    transaction
        .execute(
            INSERT_GRANT_AUDIT,
            params![grant.issuer_key_id, grant.nonce, audit_json],
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(
        audit,
    ))
}

fn validate_grant(
    grant: &AuthenticatedDeliveryGrant,
    expected: &AuthenticatedDeliveryGrantExpectation,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
    trusted_now: AuthenticatedDeliveryGrantInstant,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    grant
        .validate_shape()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::InvalidGrant)?;
    let signature = Signature::from_slice(&grant.signature)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::SignatureRejected)?;
    trusted_issuer
        .verifying_key
        .verify_strict(&grant.signing_bytes(), &signature)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::SignatureRejected)?;
    if grant.dry_run {
        return Err(AuthenticatedDeliveryGrantConsumeError::DryRunRejected);
    }
    let issued_at = grant
        .issued_at_instant()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::InvalidGrant)?;
    if issued_at > trusted_now {
        return Err(AuthenticatedDeliveryGrantConsumeError::NotYetValid);
    }
    let expires_at = grant
        .expires_at_instant()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::InvalidGrant)?;
    if expires_at <= trusted_now {
        return Err(AuthenticatedDeliveryGrantConsumeError::Expired);
    }
    if grant.revocation_version != expected.revocation_version {
        return Err(AuthenticatedDeliveryGrantConsumeError::Revoked);
    }
    if grant.issuer_key_id != trusted_issuer.key_id
        || grant.issuer_actor_id != expected.issuer_actor_id
        || grant.household_id != expected.household_id
        || grant.parent_device_id != expected.parent_device_id
        || grant.child_profile_id != expected.child_profile_id
        || grant.target_device_id != expected.target_device_id
        || grant.policy_decision_id != expected.policy_decision_id
        || grant.policy_version != expected.policy_version
        || grant.action_id != expected.action_id
        || grant.capability_id != expected.capability_id
        || grant.evidence_digest != expected.evidence_digest
        || grant.payload_digest != expected.payload_digest
    {
        return Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected);
    }
    Ok(())
}

fn parse_observed_at(
    observed_at: &str,
) -> Result<AuthenticatedDeliveryGrantInstant, AuthenticatedDeliveryGrantConsumeError> {
    parse_authenticated_delivery_grant_instant(observed_at)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::BindingRejected)
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
