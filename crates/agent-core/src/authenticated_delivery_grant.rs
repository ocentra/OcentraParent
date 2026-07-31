#![forbid(unsafe_code)]

use std::path::Path;

use ed25519_dalek::VerifyingKey;
use ocentra_schema::authenticated_delivery_grant::{
    authenticated_delivery_grant_audit_fingerprint, AuthenticatedDeliveryGrant,
    AuthenticatedDeliveryGrantInstant, AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

mod authenticated_delivery_grant_retention;
mod rejection_audit;
mod sqlite_contention;
mod storage_keys;
mod validation;

use sqlite_contention::{immediate_transaction_with_contention_retry, CONNECTION_BUSY_TIMEOUT};
use storage_keys::{audit, storage_key_digest, validate_trusted_issuer};

const CREATE_CONSUMED_GRANTS: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_fingerprint TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_nanos INTEGER NOT NULL, PRIMARY KEY (issuer_key_id, nonce))";
const SELECT_CONSUMED_GRANT: &str =
    "SELECT grant_fingerprint FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 UNION ALL SELECT ?3 WHERE EXISTS (SELECT 1 FROM authenticated_delivery_grant_replay_purge_floor_v1 WHERE singleton = 1 AND expires_at_nanos >= ?4) LIMIT 1";
const INSERT_CONSUMED_GRANT: &str = "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_fingerprint, audit_json, expires_at_nanos) VALUES (?1, ?2, ?3, ?4, ?5)";
const CREATE_GRANT_AUDITS: &str = "CREATE TABLE IF NOT EXISTS authenticated_delivery_grant_audits_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, audit_json TEXT NOT NULL, recorded_at_nanos INTEGER, audit_scope TEXT NOT NULL DEFAULT 'replay')";
const INSERT_GRANT_AUDIT: &str = "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json, recorded_at_nanos, audit_scope) VALUES (?1, ?2, ?3, ?4, ?5)";
const TRIM_GRANT_AUDITS: &str = "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 AND audit_scope = 'replay' AND rowid NOT IN (SELECT rowid FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 AND audit_scope = 'replay' ORDER BY rowid DESC LIMIT ?3)";
const TRIM_VALIDATION_REJECTION_AUDITS: &str = "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE rowid IN (SELECT rowid FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection' ORDER BY rowid DESC LIMIT -1 OFFSET ?1)";
const MAX_REPLAY_AUDIT_ROWS_PER_GRANT: i64 = 16;
const MAX_VALIDATION_REJECTION_AUDITS: i64 = 1_024;
const VALIDATION_REJECTION_AUDIT_SCOPE: &str = "validation-rejection";

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
    #[serde(rename = "integrity-rejected")]
    IntegrityRejected,
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
    debug_trusted_now_after_transaction: Option<(AuthenticatedDeliveryGrantInstant, i64)>,
    #[cfg(debug_assertions)]
    fail_next_commit: bool,
}

impl AuthenticatedDeliveryGrantConsumer {
    pub fn open(
        path: impl AsRef<Path>,
        trusted_issuer: AuthenticatedDeliveryGrantTrustedIssuer,
    ) -> Result<Self, AuthenticatedDeliveryGrantConsumeError> {
        let trusted_now = validation::trusted_now()?;
        Self::open_at(path, trusted_issuer, trusted_now.1)
    }

    fn open_at(
        path: impl AsRef<Path>,
        trusted_issuer: AuthenticatedDeliveryGrantTrustedIssuer,
        startup_now_nanos: i64,
    ) -> Result<Self, AuthenticatedDeliveryGrantConsumeError> {
        validate_trusted_issuer(&trusted_issuer)?;
        let connection = Connection::open(path)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        connection
            .busy_timeout(CONNECTION_BUSY_TIMEOUT)
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
        rejection_audit::ensure_retention_schema(&connection, startup_now_nanos)?;
        authenticated_delivery_grant_retention::migrate_legacy_replay_records(
            &connection,
            &trusted_issuer,
        )?;
        authenticated_delivery_grant_retention::ensure_retention_indexes(&connection)?;
        let replay_retention_now_nanos =
            authenticated_delivery_grant_retention::advance_replay_retention_clock(
                &connection,
                startup_now_nanos,
                None,
            )?;
        rejection_audit::drain_expired_at_startup(&connection, startup_now_nanos)?;
        authenticated_delivery_grant_retention::drain_expired_replay_records_at_startup(
            &connection,
            replay_retention_now_nanos,
        )?;
        Ok(Self {
            connection,
            trusted_issuer,
            #[cfg(debug_assertions)]
            debug_trusted_now: None,
            #[cfg(debug_assertions)]
            debug_trusted_now_after_transaction: None,
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
        let trusted_now = validation::parse_trusted_now(trusted_now.as_ref())?;
        let mut consumer = Self::open_at(path, trusted_issuer, trusted_now.1)?;
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
        #[cfg(debug_assertions)]
        let trusted_now = self
            .debug_trusted_now
            .map_or_else(validation::trusted_now, Ok)?;
        #[cfg(not(debug_assertions))]
        let trusted_now = validation::trusted_now()?;
        self.consume_at(
            grant,
            expected,
            delivered_payload.as_ref(),
            correlation_id,
            trusted_now,
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
            validation::parse_trusted_now(trusted_now.as_ref())?,
        )
    }

    fn consume_at(
        &mut self,
        grant: &AuthenticatedDeliveryGrant,
        expected: &AuthenticatedDeliveryGrantExpectation,
        payload: &[u8],
        correlation_id: impl Into<String>,
        trusted_now: (AuthenticatedDeliveryGrantInstant, i64),
    ) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError>
    {
        let correlation = correlation_id.into();
        let replay_retention_now_nanos =
            authenticated_delivery_grant_retention::advance_replay_retention_clock(
                &self.connection,
                trusted_now.1,
                None,
            )?;
        let trusted_now =
            validation::trusted_now_at_least(trusted_now, replay_retention_now_nanos)?;
        self.validate_request(grant, expected, payload, &correlation, trusted_now)?;
        let authenticated_issued_at_nanos = validation::authenticated_issued_at_nanos(grant)?;
        let outcome = self.consume_after_replay_retention_validation(
            grant,
            &correlation,
            trusted_now,
            authenticated_issued_at_nanos,
        )?;
        authenticated_delivery_grant_retention::purge_expired_replay_records(
            &self.connection,
            trusted_now.1,
        )?;
        Ok(outcome)
    }

    fn consume_after_replay_retention_validation(
        &mut self,
        grant: &AuthenticatedDeliveryGrant,
        correlation: &str,
        trusted_now: (AuthenticatedDeliveryGrantInstant, i64),
        authenticated_issued_at_nanos: i64,
    ) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError>
    {
        #[cfg(debug_assertions)]
        let debug_trusted_now_after_transaction = self.debug_trusted_now_after_transaction;
        let transaction = immediate_transaction_with_contention_retry(&self.connection)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        let post_begin_observed_now = validation::trusted_now_after_transaction(
            #[cfg(debug_assertions)]
            debug_trusted_now_after_transaction,
            trusted_now,
        )?;
        if let Err(error) =
            validation::validate_temporal_window_at(grant, post_begin_observed_now.0)
        {
            return reject_post_begin_temporal_window(
                transaction,
                grant,
                correlation,
                post_begin_observed_now.1,
                error,
            );
        }
        let post_begin_retention_now_nanos =
            authenticated_delivery_grant_retention::advance_replay_retention_clock_transaction(
                &transaction,
                post_begin_observed_now.1,
                Some(authenticated_issued_at_nanos),
            )?;
        let post_begin_now = validation::trusted_now_at_least(
            post_begin_observed_now,
            post_begin_retention_now_nanos,
        )?;
        let stored: Option<String> = transaction
            .query_row(
                SELECT_CONSUMED_GRANT,
                params![
                    storage_key_digest(&grant.issuer_key_id),
                    storage_key_digest(&grant.nonce),
                    authenticated_delivery_grant_audit_fingerprint(grant),
                    validation::instant_nanos(&grant.expires_at)?,
                ],
                |row| row.get(0),
            )
            .optional()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        if let Some(stored) = stored {
            return reject_replay(transaction, grant, correlation, &stored, post_begin_now.1);
        }
        let audit = audit(
            grant,
            correlation,
            AuthenticatedDeliveryGrantAuditOutcome::Consumed,
        );
        persist_consumed_grant(&transaction, grant, &audit)?;
        persist_audit_transaction(&transaction, grant, &audit, Some(post_begin_now.1))?;
        #[cfg(debug_assertions)]
        if std::mem::take(&mut self.fail_next_commit) {
            return Err(AuthenticatedDeliveryGrantConsumeError::StorageUnavailable);
        }
        transaction
            .commit()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        Ok(AuthenticatedDeliveryGrantConsumeOutcome::Consumed(audit))
    }

    fn validate_request(
        &self,
        grant: &AuthenticatedDeliveryGrant,
        expected: &AuthenticatedDeliveryGrantExpectation,
        delivered_payload: &[u8],
        correlation_id: &str,
        trusted_now: (AuthenticatedDeliveryGrantInstant, i64),
    ) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
        validation::validate_correlation_id(correlation_id).map_err(|error| {
            self.persist_bounded_shape_rejection(
                grant,
                &validation::bounded_correlation_id(correlation_id),
                trusted_now.1,
                error,
                AuthenticatedDeliveryGrantValidationRejection::BindingRejected,
            )
        })?;
        if grant.validate_shape().is_err() {
            return Err(self.persist_bounded_shape_rejection(
                grant,
                correlation_id,
                trusted_now.1,
                AuthenticatedDeliveryGrantConsumeError::InvalidGrant,
                AuthenticatedDeliveryGrantValidationRejection::InvalidGrant,
            ));
        }
        validation::validate_grant(grant, expected, &self.trusted_issuer, trusted_now.0).map_err(
            |error| self.persist_validation_rejection(grant, correlation_id, trusted_now.1, error),
        )?;
        validation::validate_storage_range(grant).map_err(|error| {
            self.persist_validation_rejection(grant, correlation_id, trusted_now.1, error)
        })?;
        validation::validate_delivered_payload(grant, delivered_payload).map_err(|error| {
            self.persist_validation_rejection(grant, correlation_id, trusted_now.1, error)
        })
    }

    #[cfg(debug_assertions)]
    pub fn inject_next_commit_failure_for_debug(&mut self) {
        self.fail_next_commit = true;
    }

    #[cfg(debug_assertions)]
    pub fn inject_trusted_now_after_transaction_for_debug(
        &mut self,
        trusted_now: impl AsRef<str>,
    ) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
        self.debug_trusted_now_after_transaction =
            Some(validation::parse_trusted_now(trusted_now.as_ref())?);
        Ok(())
    }

    fn persist_validation_rejection(
        &self,
        grant: &AuthenticatedDeliveryGrant,
        correlation_id: &str,
        trusted_now_nanos: i64,
        error: AuthenticatedDeliveryGrantConsumeError,
    ) -> AuthenticatedDeliveryGrantConsumeError {
        rejection_audit::persist(
            &self.connection,
            grant,
            correlation_id,
            trusted_now_nanos,
            error,
        )
    }

    fn persist_bounded_shape_rejection(
        &self,
        grant: &AuthenticatedDeliveryGrant,
        correlation_id: &str,
        trusted_now_nanos: i64,
        error: AuthenticatedDeliveryGrantConsumeError,
        rejection: AuthenticatedDeliveryGrantValidationRejection,
    ) -> AuthenticatedDeliveryGrantConsumeError {
        let audit = bounded_shape_rejection_audit(grant, correlation_id, rejection);
        let issuer_key_id = audit.issuer_key_id_digest.clone();
        let nonce = audit.nonce_digest.clone();
        let result = immediate_transaction_with_contention_retry(&self.connection)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
            .and_then(|transaction| {
                let audit_json = serde_json::to_string(&audit)
                    .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
                transaction
                    .execute(
                        INSERT_GRANT_AUDIT,
                        params![
                            issuer_key_id,
                            nonce,
                            audit_json,
                            trusted_now_nanos,
                            VALIDATION_REJECTION_AUDIT_SCOPE,
                        ],
                    )
                    .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
                transaction
                    .execute(
                        TRIM_GRANT_AUDITS,
                        params![issuer_key_id, nonce, MAX_REPLAY_AUDIT_ROWS_PER_GRANT],
                    )
                    .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
                transaction
                    .execute(
                        TRIM_VALIDATION_REJECTION_AUDITS,
                        [MAX_VALIDATION_REJECTION_AUDITS],
                    )
                    .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
                transaction
                    .commit()
                    .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
            });
        result.map_or(
            AuthenticatedDeliveryGrantConsumeError::StorageUnavailable,
            |_| error,
        )
    }
}

fn reject_post_begin_temporal_window(
    transaction: Transaction<'_>,
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: &str,
    trusted_now_nanos: i64,
    error: AuthenticatedDeliveryGrantConsumeError,
) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError> {
    let rejection = match error {
        AuthenticatedDeliveryGrantConsumeError::Expired => {
            AuthenticatedDeliveryGrantValidationRejection::Expired
        }
        AuthenticatedDeliveryGrantConsumeError::NotYetValid => {
            AuthenticatedDeliveryGrantValidationRejection::NotYetValid
        }
        _ => return Err(error),
    };
    let audit = audit(
        grant,
        correlation_id,
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(rejection),
    );
    persist_audit_transaction(&transaction, grant, &audit, Some(trusted_now_nanos))?;
    transaction
        .execute(
            TRIM_VALIDATION_REJECTION_AUDITS,
            [MAX_VALIDATION_REJECTION_AUDITS],
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Err(error)
}

fn reject_replay(
    transaction: Transaction<'_>,
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: &str,
    stored_fingerprint: &str,
    recorded_at_nanos: i64,
) -> Result<AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumeError> {
    if stored_fingerprint != authenticated_delivery_grant_audit_fingerprint(grant) {
        let audit = audit(
            grant,
            correlation_id,
            AuthenticatedDeliveryGrantAuditOutcome::IntegrityRejected,
        );
        persist_audit_transaction(&transaction, grant, &audit, Some(recorded_at_nanos))?;
        transaction
            .commit()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        return Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected);
    }
    let audit = audit(
        grant,
        correlation_id,
        AuthenticatedDeliveryGrantAuditOutcome::ReplayRejected,
    );
    persist_audit_transaction(&transaction, grant, &audit, Some(recorded_at_nanos))?;
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
                storage_key_digest(&grant.issuer_key_id),
                storage_key_digest(&grant.nonce),
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
                storage_key_digest(&grant.issuer_key_id),
                storage_key_digest(&grant.nonce),
                MAX_REPLAY_AUDIT_ROWS_PER_GRANT
            ],
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

fn persist_consumed_grant(
    transaction: &Transaction<'_>,
    grant: &AuthenticatedDeliveryGrant,
    audit: &AuthenticatedDeliveryGrantAudit,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let audit_json = serde_json::to_string(audit)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
    transaction
        .execute(
            INSERT_CONSUMED_GRANT,
            params![
                storage_key_digest(&grant.issuer_key_id),
                storage_key_digest(&grant.nonce),
                authenticated_delivery_grant_audit_fingerprint(grant),
                audit_json,
                validation::instant_nanos(&grant.expires_at)?
            ],
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

fn bounded_shape_rejection_audit(
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: &str,
    rejection: AuthenticatedDeliveryGrantValidationRejection,
) -> AuthenticatedDeliveryGrantAudit {
    let update_bounded_digest = |hasher: &mut Sha256, value: &[u8]| {
        let bounded_len = value
            .len()
            .min(AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES);
        hasher.update((value.len() as u64).to_be_bytes());
        hasher.update((bounded_len as u64).to_be_bytes());
        hasher.update(&value[..bounded_len]);
        // The prefix remains useful for compact, bounded diagnostic structure,
        // but the digest of the full value makes every omitted tail
        // collision-resistant without persisting that sensitive tail.
        hasher.update(Sha256::digest(value));
    };
    let bounded_digest = |value: &[u8]| {
        let mut hasher = Sha256::new();
        update_bounded_digest(&mut hasher, value);
        digest(hasher.finalize())
    };
    let mut grant_hasher = Sha256::new();
    grant_hasher.update(grant.schema_version.to_be_bytes());
    grant_hasher.update((grant.payload_length as u64).to_be_bytes());
    grant_hasher.update([u8::from(grant.dry_run)]);
    for value in [
        grant.issuer_key_id.as_str(),
        grant.issuer_actor_id.as_str(),
        grant.household_id.as_str(),
        grant.parent_device_id.as_str(),
        grant.child_profile_id.as_str(),
        grant.target_device_id.as_str(),
        grant.policy_decision_id.as_str(),
        grant.policy_version.as_str(),
        grant.action_id.as_str(),
        grant.capability_id.as_str(),
        grant.evidence_digest.as_str(),
        grant.payload_digest.as_str(),
        grant.nonce.as_str(),
        grant.issued_at.as_str(),
        grant.expires_at.as_str(),
        grant.revocation_version.as_str(),
    ] {
        update_bounded_digest(&mut grant_hasher, value.as_bytes());
    }
    update_bounded_digest(&mut grant_hasher, &grant.signature);
    AuthenticatedDeliveryGrantAudit {
        // This path runs before the normal request validator has established
        // that caller supplied data is safe to persist.  Keep the audit
        // correlatable without copying an untrusted caller identifier.
        correlation_id: bounded_digest(correlation_id.as_bytes()),
        issuer_key_id_digest: bounded_digest(grant.issuer_key_id.as_bytes()),
        nonce_digest: bounded_digest(grant.nonce.as_bytes()),
        grant_digest: digest(grant_hasher.finalize()),
        outcome: AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(rejection),
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
