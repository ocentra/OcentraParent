#![forbid(unsafe_code)]

use std::{path::Path, time::Duration};

use chrono::{DateTime, SecondsFormat, Utc};
use ed25519_dalek::{Signature, VerifyingKey};
use ocentra_schema::authenticated_delivery_grant::{
    parse_authenticated_delivery_grant_instant, AuthenticatedDeliveryGrant,
    AuthenticatedDeliveryGrantInstant,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

mod authenticated_delivery_grant_retention;
mod rejection_audit;

const CREATE_CONSUMED_GRANTS: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_nanos INTEGER, PRIMARY KEY (issuer_key_id, nonce))";
const SELECT_CONSUMED_GRANT: &str =
    "SELECT grant_json FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2";
const INSERT_CONSUMED_GRANT: &str = "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_json, audit_json, expires_at_nanos) VALUES (?1, ?2, ?3, ?4, ?5)";
const CREATE_GRANT_AUDITS: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_audits_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, audit_json TEXT NOT NULL, recorded_at_nanos INTEGER, audit_scope TEXT NOT NULL DEFAULT 'replay')";
const INSERT_GRANT_AUDIT: &str = "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json, recorded_at_nanos, audit_scope) VALUES (?1, ?2, ?3, ?4, ?5)";
const TRIM_GRANT_AUDITS: &str = "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 AND rowid NOT IN (SELECT rowid FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 ORDER BY rowid DESC LIMIT ?3)";
const CONSUME_BUSY_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_REPLAY_AUDIT_ROWS_PER_GRANT: i64 = 16;

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
    pub issuer_key_id_digest: String,
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
    #[serde(rename = "validation-rejected")]
    ValidationRejected(AuthenticatedDeliveryGrantValidationRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticatedDeliveryGrantValidationRejection {
    #[serde(rename = "invalid-grant")]
    InvalidGrant,
    #[serde(rename = "signature-rejected")]
    SignatureRejected,
    #[serde(rename = "binding-rejected")]
    BindingRejected,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "not-yet-valid")]
    NotYetValid,
    #[serde(rename = "dry-run-rejected")]
    DryRunRejected,
    #[serde(rename = "revoked")]
    Revoked,
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
    #[cfg(debug_assertions)]
    debug_trusted_now: Option<(AuthenticatedDeliveryGrantInstant, i64)>,
    #[cfg(debug_assertions)]
    fail_next_commit: bool,
}

impl AuthenticatedDeliveryGrantConsumer {
    pub fn open(
        path: impl AsRef<Path>,
        trusted_issuer: AuthenticatedDeliveryGrantTrustedIssuer,
    ) -> Result<Self, AuthenticatedDeliveryGrantConsumeError> {
        let trusted_now = trusted_now()?;
        Self::open_at(path, trusted_issuer, Some(trusted_now.1))
    }

    fn open_at(
        path: impl AsRef<Path>,
        trusted_issuer: AuthenticatedDeliveryGrantTrustedIssuer,
        startup_now_nanos: Option<i64>,
    ) -> Result<Self, AuthenticatedDeliveryGrantConsumeError> {
        let mut connection = Connection::open(path)
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
        rejection_audit::ensure_retention_schema(&connection)?;
        authenticated_delivery_grant_retention::ensure_retention_indexes(&connection)?;
        if let Some(startup_now_nanos) = startup_now_nanos {
            rejection_audit::drain_expired_at_startup(&mut connection, startup_now_nanos)?;
            authenticated_delivery_grant_retention::drain_expired_replay_records_at_startup(
                &mut connection,
                startup_now_nanos,
            )?;
        }
        Ok(Self {
            connection,
            trusted_issuer,
            #[cfg(debug_assertions)]
            debug_trusted_now: None,
            #[cfg(debug_assertions)]
            fail_next_commit: false,
        })
    }

    #[cfg(debug_assertions)]
    pub fn open_at_for_debug_test(
        path: impl AsRef<Path>,
        trusted_issuer: AuthenticatedDeliveryGrantTrustedIssuer,
        trusted_now: impl AsRef<str>,
    ) -> Result<Self, AuthenticatedDeliveryGrantConsumeError> {
        let trusted_now = parse_trusted_now(trusted_now.as_ref())?;
        let mut consumer = Self::open_at(path, trusted_issuer, Some(trusted_now.1))?;
        consumer.debug_trusted_now = Some(trusted_now);
        Ok(consumer)
    }

    pub fn consume(
        &mut self,
        grant: &AuthenticatedDeliveryGrant,
        expected: &AuthenticatedDeliveryGrantExpectation,
        delivered_payload: impl AsRef<[u8]>,
        correlation_id: impl Into<String>,
    ) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError>
    {
        self.consume_at(
            grant,
            expected,
            delivered_payload.as_ref(),
            correlation_id,
            self.current_trusted_now()?,
        )
    }

    #[cfg(debug_assertions)]
    pub fn consume_at_for_debug_test(
        &mut self,
        grant: &AuthenticatedDeliveryGrant,
        expected: &AuthenticatedDeliveryGrantExpectation,
        delivered_payload: impl AsRef<[u8]>,
        correlation_id: impl Into<String>,
        trusted_now: impl AsRef<str>,
    ) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError>
    {
        self.consume_at(
            grant,
            expected,
            delivered_payload.as_ref(),
            correlation_id,
            parse_trusted_now(trusted_now.as_ref())?,
        )
    }

    fn consume_at(
        &mut self,
        grant: &AuthenticatedDeliveryGrant,
        expected: &AuthenticatedDeliveryGrantExpectation,
        delivered_payload: &[u8],
        correlation_id: impl Into<String>,
        trusted_now: (AuthenticatedDeliveryGrantInstant, i64),
    ) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError>
    {
        let correlation_id = correlation_id.into();
        validate_grant(
            grant,
            expected,
            &self.trusted_issuer,
            &digest(delivered_payload),
            trusted_now.0,
        )
        .map_err(|error| {
            self.persist_validation_rejection(grant, &correlation_id, trusted_now.1, error)
        })?;
        authenticated_delivery_grant_retention::purge_expired_replay_records(
            &mut self.connection,
            trusted_now.1,
        )?;
        if correlation_id.trim().is_empty()
            || correlation_id.len() > ocentra_schema::authenticated_delivery_grant::AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
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
            return reject_replay(transaction, grant, correlation_id, &stored);
        }
        let audit = audit(
            grant,
            correlation_id,
            AuthenticatedDeliveryGrantAuditOutcome::Consumed,
        );
        let grant_json = serde_json::to_string(grant)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        let expires_at_nanos = instant_nanos(&grant.expires_at)?;
        let audit_json = serde_json::to_string(&audit)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
        transaction
            .execute(
                INSERT_CONSUMED_GRANT,
                params![
                    grant.issuer_key_id,
                    grant.nonce,
                    grant_json,
                    audit_json,
                    expires_at_nanos
                ],
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

    fn current_trusted_now(
        &self,
    ) -> Result<(AuthenticatedDeliveryGrantInstant, i64), AuthenticatedDeliveryGrantConsumeError>
    {
        #[cfg(debug_assertions)]
        {
            self.debug_trusted_now.map_or_else(trusted_now, Ok)
        }
        #[cfg(not(debug_assertions))]
        trusted_now()
    }

    fn persist_validation_rejection(
        &mut self,
        grant: &AuthenticatedDeliveryGrant,
        correlation_id: &str,
        trusted_now_nanos: i64,
        error: AuthenticatedDeliveryGrantConsumeError,
    ) -> AuthenticatedDeliveryGrantConsumeError {
        rejection_audit::persist(
            &mut self.connection,
            grant,
            correlation_id,
            trusted_now_nanos,
            error,
        )
    }
}

fn reject_replay(
    transaction: Transaction<'_>,
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: String,
    stored: &str,
) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError> {
    let stored: AuthenticatedDeliveryGrant = serde_json::from_str(stored)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    if stored.signing_bytes() != grant.signing_bytes() || stored.signature != grant.signature {
        return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
    }
    let audit = audit(
        grant,
        correlation_id,
        AuthenticatedDeliveryGrantAuditOutcome::ReplayRejected,
    );
    persist_audit_transaction(&transaction, grant, &audit, None)?;
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(
        audit,
    ))
}

fn persist_audit_transaction(
    transaction: &Transaction<'_>,
    grant: &AuthenticatedDeliveryGrant,
    audit: &AuthenticatedDeliveryGrantAudit,
    recorded_at_nanos: Option<i64>,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let audit_json = serde_json::to_string(audit)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    transaction
        .execute(
            INSERT_GRANT_AUDIT,
            params![
                grant.issuer_key_id,
                grant.nonce,
                audit_json,
                recorded_at_nanos,
                rejection_audit::audit_scope(audit),
            ],
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    transaction
        .execute(
            TRIM_GRANT_AUDITS,
            params![
                grant.issuer_key_id,
                grant.nonce,
                MAX_REPLAY_AUDIT_ROWS_PER_GRANT
            ],
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

fn validate_grant(
    grant: &AuthenticatedDeliveryGrant,
    expected: &AuthenticatedDeliveryGrantExpectation,
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
    delivered_payload_digest: &str,
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
        || grant.payload_digest != delivered_payload_digest
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

fn trusted_now(
) -> Result<(AuthenticatedDeliveryGrantInstant, i64), AuthenticatedDeliveryGrantConsumeError> {
    parse_trusted_now(&Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true))
}

fn parse_trusted_now(
    trusted_now: &str,
) -> Result<(AuthenticatedDeliveryGrantInstant, i64), AuthenticatedDeliveryGrantConsumeError> {
    Ok((parse_observed_at(trusted_now)?, instant_nanos(trusted_now)?))
}

fn instant_nanos(value: &str) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .and_then(|instant| instant.timestamp_nanos_opt())
        .ok_or(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
}

fn audit(
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: String,
    outcome: AuthenticatedDeliveryGrantAuditOutcome,
) -> AuthenticatedDeliveryGrantAudit {
    AuthenticatedDeliveryGrantAudit {
        correlation_id,
        issuer_key_id_digest: digest(&grant.issuer_key_id),
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
