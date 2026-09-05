use rusqlite::Connection;

use super::AccountIdentityIssuerError;

#[path = "account_identity_authority_issuer_outbox_validation.rs"]
mod validation;

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

pub(crate) fn validate(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    validation::validate(connection)
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
