use rusqlite::Connection;

use super::{has_legacy_table, AccountIdentityAuthorityIssuerClientError, CANONICAL_SCHEMA_SQL};

pub(super) fn rebuild_legacy(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    preflight_empty_legacy(connection)?;
    backup_legacy_tables(connection)?;
    connection
        .execute_batch(CANONICAL_SCHEMA_SQL)
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
    drop_legacy_tables(connection)
}

fn preflight_empty_legacy(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let tables = [
        "account_identity_issuer_v2_key_registry",
        "account_identity_issuer_v2_receipt",
        "account_identity_issuer_v2_outbox",
    ];
    let mut has_data = false;
    for table in tables {
        if has_legacy_table(connection, table)? {
            let count = connection
                .query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| {
                    row.get::<_, i64>(0)
                })
                .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
            has_data |= count != 0;
        }
    }

    let has_receipts = has_legacy_table(connection, "account_identity_issuer_v2_receipt")?;
    let has_outbox = has_legacy_table(connection, "account_identity_issuer_v2_outbox")?;
    let orphan_count = if has_receipts && has_outbox {
        connection
            .query_row(
                "SELECT COUNT(*)
                   FROM account_identity_issuer_v2_outbox AS outbox
              LEFT JOIN account_identity_issuer_v2_receipt AS receipt
                     ON receipt.receipt_id = outbox.receipt_id
                  WHERE receipt.receipt_id IS NULL",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
    } else {
        0
    };
    has_data |= orphan_count != 0;

    let invalid_state_count = if has_outbox {
        connection
            .query_row(
                "SELECT COUNT(*)
                   FROM account_identity_issuer_v2_outbox
                  WHERE delivery_state NOT IN ('pending','claimed','sent','failed','acknowledged')",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?
    } else {
        0
    };
    has_data |= invalid_state_count != 0;

    if has_data {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidSchema);
    }
    Ok(())
}

fn backup_legacy_tables(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    for (table, legacy) in [
        (
            "account_identity_issuer_v2_key_registry",
            "account_identity_issuer_v2_key_registry_legacy",
        ),
        (
            "account_identity_issuer_v2_receipt",
            "account_identity_issuer_v2_receipt_legacy",
        ),
        (
            "account_identity_issuer_v2_outbox",
            "account_identity_issuer_v2_outbox_legacy",
        ),
    ] {
        if has_legacy_table(connection, table)? {
            connection
                .execute_batch(&format!(
                    "CREATE TABLE {legacy} AS SELECT * FROM {table}; DROP TABLE {table};"
                ))
                .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)?;
        }
    }
    Ok(())
}

fn drop_legacy_tables(
    connection: &Connection,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    connection
        .execute_batch(
            "DROP TABLE IF EXISTS account_identity_issuer_v2_key_registry_legacy;
             DROP TABLE IF EXISTS account_identity_issuer_v2_receipt_legacy;
             DROP TABLE IF EXISTS account_identity_issuer_v2_outbox_legacy;",
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}
