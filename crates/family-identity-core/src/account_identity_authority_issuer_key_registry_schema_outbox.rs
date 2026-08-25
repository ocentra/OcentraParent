use rusqlite::Connection;

use super::super::super::outbox::OUTBOX_SCHEMA_SQL;
use super::super::AccountIdentityIssuerError;
use super::definitions::{
    validate_columns, validate_index, validate_index_sql, validate_table_sql,
};

pub(super) fn validate(connection: &Connection) -> Result<(), AccountIdentityIssuerError> {
    validate_table_sql(
        connection,
        "account_identity_issuer_transport_outbox",
        OUTBOX_SCHEMA_SQL,
    )?;
    validate_columns(
        connection,
        "account_identity_issuer_transport_outbox",
        &[
            ("receipt_id", "TEXT", 0, 1),
            ("account_id", "TEXT", 1, 0),
            ("household_id", "TEXT", 1, 0),
            ("service_binding_id", "TEXT", 1, 0),
            ("service_label", "TEXT", 1, 0),
            ("authority_generation", "INTEGER", 1, 0),
            ("key_id", "TEXT", 1, 0),
            ("key_version", "INTEGER", 1, 0),
            ("wire", "BLOB", 1, 0),
            ("created_at_millis", "INTEGER", 1, 0),
            ("delivery_state", "TEXT", 1, 0),
            ("claim_id", "TEXT", 0, 0),
            ("claim_expires_at_millis", "INTEGER", 0, 0),
            ("attempt_count", "INTEGER", 1, 0),
            ("acknowledgement_id", "TEXT", 0, 0),
            ("acknowledged_at_millis", "INTEGER", 0, 0),
            ("terminal_at_millis", "INTEGER", 0, 0),
        ],
    )?;
    validate_index_sql(
        connection,
        "account_identity_issuer_transport_outbox_delivery",
        OUTBOX_SCHEMA_SQL,
    )?;
    validate_index(
        connection,
        "account_identity_issuer_transport_outbox",
        "account_identity_issuer_transport_outbox_delivery",
        &[
            "service_label",
            "delivery_state",
            "claim_expires_at_millis",
            "created_at_millis",
        ],
        &[(
            "sqlite_autoindex_account_identity_issuer_transport_outbox_1",
            "pk",
            &["receipt_id"],
        )],
    )
}
