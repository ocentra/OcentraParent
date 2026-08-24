use std::fmt;

use chrono::{DateTime, Utc};
use getrandom::fill;
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use sha2::{Digest, Sha256};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::current_key_record::AccountIdentityIssuerCurrentPublicKeyRecord;
use super::service_binding::{AccountIdentityIssuerService, AccountIdentityIssuerServiceBinding};
use super::transport::AccountIdentityIssuerTransport;
use super::AccountIdentityIssuerError;

#[path = "account_identity_authority_issuer_outbox_claim.rs"]
pub(crate) mod claim;
#[path = "account_identity_authority_issuer_outbox_reconcile.rs"]
pub(crate) mod reconcile;
#[path = "account_identity_authority_issuer_outbox_validation.rs"]
mod validation;

const CLAIM_LEASE_SECONDS: i64 = 5 * 60;

pub(crate) const OUTBOX_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_issuer_transport_outbox (
        receipt_id TEXT PRIMARY KEY CHECK (length(receipt_id) > 0),
        account_id TEXT NOT NULL CHECK (length(account_id) > 0),
        household_id TEXT NOT NULL CHECK (length(household_id) > 0),
        service_binding_id TEXT NOT NULL CHECK (length(service_binding_id) > 0),
        service_label TEXT NOT NULL CHECK (length(service_label) > 0),
        authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
        key_id TEXT NOT NULL CHECK (length(key_id) > 0),
        key_version INTEGER NOT NULL CHECK (key_version > 0),
        wire BLOB NOT NULL CHECK (length(wire) > 0),
        created_at_millis INTEGER NOT NULL CHECK (created_at_millis >= 0),
        delivery_state TEXT NOT NULL CHECK (
            delivery_state IN ('pending','claimed','acknowledged','expired','superseded')
        ),
        claim_id TEXT,
        claim_expires_at_millis INTEGER,
        attempt_count INTEGER NOT NULL CHECK (attempt_count >= 0),
        acknowledgement_id TEXT,
        acknowledged_at_millis INTEGER,
        terminal_at_millis INTEGER,
        CHECK (
            (delivery_state = 'pending' AND claim_id IS NULL
                AND claim_expires_at_millis IS NULL AND acknowledgement_id IS NULL
                AND acknowledged_at_millis IS NULL AND terminal_at_millis IS NULL)
            OR (delivery_state = 'claimed' AND claim_id IS NOT NULL
                AND claim_expires_at_millis > created_at_millis
                AND acknowledgement_id IS NULL AND acknowledged_at_millis IS NULL
                AND terminal_at_millis IS NULL)
            OR (delivery_state = 'acknowledged' AND claim_id IS NULL
                AND claim_expires_at_millis IS NULL AND acknowledgement_id IS NOT NULL
                AND acknowledged_at_millis >= created_at_millis
                AND terminal_at_millis IS NULL)
            OR (delivery_state IN ('expired','superseded') AND claim_id IS NULL
                AND claim_expires_at_millis IS NULL AND acknowledgement_id IS NULL
                AND acknowledged_at_millis IS NULL
                AND terminal_at_millis >= created_at_millis)
        ),
        FOREIGN KEY (receipt_id) REFERENCES account_identity_issuer_transport_receipt(receipt_id)
            ON DELETE RESTRICT
    ) STRICT;
    CREATE INDEX IF NOT EXISTS account_identity_issuer_transport_outbox_delivery
        ON account_identity_issuer_transport_outbox (
            service_label, delivery_state, claim_expires_at_millis, created_at_millis
        );";

pub(crate) struct AccountIdentityIssuerDeliveryAttempt {
    receipt_id: String,
    claim_id: String,
    service: AccountIdentityIssuerService,
    binding_id: String,
    account_id: String,
    household_id: String,
    authority_generation: u64,
    wire: Vec<u8>,
    current_key_record: AccountIdentityIssuerCurrentPublicKeyRecord,
}

impl AccountIdentityIssuerDeliveryAttempt {
    pub(crate) fn receipt_id(&self) -> &str {
        &self.receipt_id
    }

    pub(crate) fn wire_bytes(&self) -> &[u8] {
        &self.wire
    }

    pub(crate) fn service(&self) -> AccountIdentityIssuerService {
        self.service
    }

    pub(crate) fn cloudflare_delivery_parts(
        &self,
    ) -> (
        &str,
        &str,
        AccountIdentityIssuerService,
        &str,
        &str,
        &str,
        u64,
        &[u8],
        &AccountIdentityIssuerCurrentPublicKeyRecord,
    ) {
        (
            &self.receipt_id,
            &self.claim_id,
            self.service,
            &self.binding_id,
            &self.account_id,
            &self.household_id,
            self.authority_generation,
            &self.wire,
            &self.current_key_record,
        )
    }
}

impl fmt::Debug for AccountIdentityIssuerDeliveryAttempt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccountIdentityIssuerDeliveryAttempt")
            .field("receipt_id", &"redacted")
            .field("claim_id", &"redacted")
            .field("service", &self.service)
            .field("wire", &"redacted")
            .finish()
    }
}

pub(crate) struct AccountIdentityIssuerDeliveryAcknowledgement {
    receipt_id: String,
    claim_id: String,
    acknowledgement_id: String,
}

impl AccountIdentityIssuerDeliveryAcknowledgement {
    /// Only a sealed Account-owned delivery adapter can mint success from a
    /// verified Cloudflare acknowledgement.
    pub(super) fn new(
        attempt: &AccountIdentityIssuerDeliveryAttempt,
        acknowledgement_id: String,
    ) -> Result<Self, AccountIdentityIssuerError> {
        if !is_sha256_digest(&acknowledgement_id) {
            return Err(AccountIdentityIssuerError::DeliveryAcknowledgementRejected);
        }
        Ok(Self {
            receipt_id: attempt.receipt_id.clone(),
            claim_id: attempt.claim_id.clone(),
            acknowledgement_id,
        })
    }
}

/// Future Account-owned adapter seam for an idempotent Cloudflare handoff.
/// The acknowledgement constructor is visible only to this issuer boundary
/// and its sealed owner adapters, so unrelated crate callers cannot mint
/// successful delivery evidence.
pub(crate) trait AccountIdentityIssuerDeliveryOwnerAdapter: Send + Sync {
    fn deliver(
        &self,
        attempt: &AccountIdentityIssuerDeliveryAttempt,
    ) -> Result<AccountIdentityIssuerDeliveryAcknowledgement, AccountIdentityIssuerError>;
}

pub(crate) fn enqueue(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    transport: &AccountIdentityIssuerTransport,
) -> Result<(), AccountIdentityIssuerError> {
    transaction
        .execute(
            "INSERT INTO account_identity_issuer_transport_outbox (
                receipt_id, account_id, household_id, service_binding_id, service_label,
                authority_generation, key_id, key_version, wire, created_at_millis,
                delivery_state, claim_id, claim_expires_at_millis, attempt_count,
                acknowledgement_id, acknowledged_at_millis, terminal_at_millis
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                       'pending', NULL, NULL, 0, NULL, NULL, NULL)",
            params![
                transport.receipt_id(),
                authority.account_id().to_string(),
                authority.household_id().to_string(),
                binding.binding_id(),
                binding.service().label(),
                to_sql_generation(authority.authority_generation())?,
                transport.key_id(),
                to_sql_generation(transport.key_version())?,
                transport.wire_bytes(),
                transport.issued_at().timestamp_millis(),
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::ReplayDetected)?;
    Ok(())
}

pub(crate) fn release(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    attempt: &AccountIdentityIssuerDeliveryAttempt,
) -> Result<(), AccountIdentityIssuerError> {
    ensure_attempt_binding(authority, binding, attempt)?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_transport_outbox
                SET delivery_state = 'pending', claim_id = NULL,
                    claim_expires_at_millis = NULL
              WHERE receipt_id = ?1 AND claim_id = ?2 AND delivery_state = 'claimed'",
            params![attempt.receipt_id, attempt.claim_id],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(AccountIdentityIssuerError::DeliveryUnavailable)
}

pub(crate) fn acknowledge_claim(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    attempt: &AccountIdentityIssuerDeliveryAttempt,
    acknowledgement: &AccountIdentityIssuerDeliveryAcknowledgement,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityIssuerError> {
    ensure_attempt_binding(authority, binding, attempt)?;
    if acknowledgement.receipt_id != attempt.receipt_id
        || acknowledgement.claim_id != attempt.claim_id
    {
        return Err(AccountIdentityIssuerError::DeliveryAcknowledgementRejected);
    }
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_transport_outbox
                SET delivery_state = 'acknowledged', claim_id = NULL,
                    claim_expires_at_millis = NULL, acknowledgement_id = ?1,
                    acknowledged_at_millis = ?2
              WHERE receipt_id = ?3 AND claim_id = ?4 AND delivery_state = 'claimed'",
            params![
                acknowledgement.acknowledgement_id,
                now.timestamp_millis(),
                attempt.receipt_id,
                attempt.claim_id,
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    (changed == 1).then_some(()).map_or_else(
        || validate_terminal_or_idempotent_ack(transaction, attempt, acknowledgement),
        Ok,
    )
}

pub(crate) fn validate(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    validation::validate(connection)
}

fn validate_terminal_or_idempotent_ack(
    transaction: &Transaction<'_>,
    attempt: &AccountIdentityIssuerDeliveryAttempt,
    acknowledgement: &AccountIdentityIssuerDeliveryAcknowledgement,
) -> Result<(), AccountIdentityIssuerError> {
    let state = transaction
        .query_row(
            "SELECT delivery_state, acknowledgement_id
             FROM account_identity_issuer_transport_outbox WHERE receipt_id = ?1",
            [attempt.receipt_id.as_str()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
        )
        .optional()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?
        .ok_or(AccountIdentityIssuerError::DeliveryUnavailable)?;
    match (state.0.as_str(), state.1.as_deref()) {
        ("expired" | "superseded", None) => Ok(()),
        ("acknowledged", Some(id)) if id == acknowledgement.acknowledgement_id => Ok(()),
        _ => Err(AccountIdentityIssuerError::DeliveryUnavailable),
    }
}

fn ensure_attempt_binding(
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    attempt: &AccountIdentityIssuerDeliveryAttempt,
) -> Result<(), AccountIdentityIssuerError> {
    (attempt.service == binding.service()
        && attempt.binding_id == binding.binding_id()
        && attempt.account_id == authority.account_id().to_string()
        && attempt.household_id == authority.household_id().to_string()
        && attempt.authority_generation == authority.authority_generation())
    .then_some(())
    .ok_or(AccountIdentityIssuerError::BindingMismatch)
}

fn opaque_digest(domain: &str) -> Result<String, AccountIdentityIssuerError> {
    let mut entropy = [0_u8; 32];
    fill(&mut entropy).map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    let mut digest = Sha256::new();
    digest.update(b"ocentra.account-issuer.outbox.v1\0");
    digest.update((domain.len() as u64).to_be_bytes());
    digest.update(domain.as_bytes());
    digest.update((entropy.len() as u64).to_be_bytes());
    digest.update(entropy);
    Ok(format!("sha256:{:x}", digest.finalize()))
}

fn is_sha256_digest(value: &str) -> bool {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return false;
    };
    hex.len() == 64
        && hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn to_sql_generation(value: u64) -> Result<i64, AccountIdentityIssuerError> {
    i64::try_from(value).map_err(|_| AccountIdentityIssuerError::InvalidKeyVersion)
}
