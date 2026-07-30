use ocentra_schema::authenticated_delivery_grant::AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES;
use rusqlite::{params, Connection, Transaction};

use super::{
    audit, persist_audit_transaction,
    sqlite_contention::immediate_transaction_with_contention_retry, AuthenticatedDeliveryGrant,
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantAuditOutcome,
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantValidationRejection,
};

#[path = "rejection_audit_scope.rs"]
mod rejection_audit_scope;

pub(super) fn persist(
    connection: &mut Connection,
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: &str,
    trusted_now_nanos: i64,
    error: AuthenticatedDeliveryGrantConsumeError,
) -> AuthenticatedDeliveryGrantConsumeError {
    let Some(rejection) = rejection(error) else {
        return error;
    };
    let audit = audit(
        grant,
        correlation_id.to_owned(),
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(rejection),
    );
    let result = immediate_transaction_with_contention_retry(connection)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
        .and_then(|transaction| {
            persist_audit_transaction(&transaction, grant, &audit, Some(trusted_now_nanos))?;
            trim_validation_rejection_audits(&transaction, trusted_now_nanos)?;
            transaction
                .commit()
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
        });
    result.map_or(
        AuthenticatedDeliveryGrantConsumeError::StorageUnavailable,
        |_| error,
    )
}

const ADD_AUDIT_RECORDED_AT_NANOS_COLUMN: &str =
    "ALTER TABLE authenticated_delivery_grant_audits_v2 ADD COLUMN recorded_at_nanos INTEGER";
const ADD_AUDIT_SCOPE_COLUMN: &str =
    "ALTER TABLE authenticated_delivery_grant_audits_v2 ADD COLUMN audit_scope TEXT NOT NULL DEFAULT 'replay'";
const SELECT_LEGACY_VALIDATION_REJECTION_AUDIT_SCOPE_METADATA: &str = "SELECT rowid, length(CAST(audit_json AS BLOB)) FROM authenticated_delivery_grant_audits_v2 WHERE rowid > ?1 AND audit_scope = 'replay' AND recorded_at_nanos IS NULL AND audit_json LIKE '%\"validation-rejected\"%' ORDER BY rowid LIMIT ?2";
const SELECT_LEGACY_VALIDATION_REJECTION_AUDIT_JSON: &str =
    "SELECT audit_json FROM authenticated_delivery_grant_audits_v2 WHERE rowid = ?1";
const UPDATE_LEGACY_AUDIT_SCOPE: &str = "UPDATE authenticated_delivery_grant_audits_v2 SET audit_scope = ?2, recorded_at_nanos = ?3 WHERE rowid = ?1";
const CREATE_VALIDATION_REJECTION_RETENTION_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_audits_v2_validation_rejection_retention_idx ON authenticated_delivery_grant_audits_v2 (audit_scope, recorded_at_nanos DESC)";
const DELETE_EXCESS_VALIDATION_REJECTIONS: &str = "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE rowid IN (SELECT rowid FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection' ORDER BY rowid DESC LIMIT -1 OFFSET ?1)";
const MAX_VALIDATION_REJECTION_AUDITS: i64 = 1_024;
const MAX_LEGACY_VALIDATION_REJECTION_AUDIT_ROWS_PER_BATCH: i64 = 128;
const MAX_LEGACY_VALIDATION_REJECTION_AUDIT_BYTES: i64 =
    (AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES * 8) as i64;

pub(super) fn ensure_retention_schema(
    connection: &mut Connection,
    startup_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    ensure_column(
        connection,
        "recorded_at_nanos",
        ADD_AUDIT_RECORDED_AT_NANOS_COLUMN,
    )?;
    ensure_column(connection, "audit_scope", ADD_AUDIT_SCOPE_COLUMN)?;
    backfill_legacy_validation_rejection_audit_scopes(connection, startup_now_nanos)?;
    connection
        .execute(CREATE_VALIDATION_REJECTION_RETENTION_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

fn backfill_legacy_validation_rejection_audit_scopes(
    connection: &mut Connection,
    startup_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let mut last_row_id = i64::MIN;
    loop {
        let legacy_audits = {
            let mut statement = connection
                .prepare(SELECT_LEGACY_VALIDATION_REJECTION_AUDIT_SCOPE_METADATA)
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
            let rows = statement
                .query_map(
                    params![
                        last_row_id,
                        MAX_LEGACY_VALIDATION_REJECTION_AUDIT_ROWS_PER_BATCH
                    ],
                    |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
                )
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?
        };
        let Some(batch_last_row_id) = legacy_audits.last().map(|(row_id, _)| *row_id) else {
            return Ok(());
        };
        let transaction = immediate_transaction_with_contention_retry(connection)
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        for (row_id, audit_json_bytes) in legacy_audits {
            (0..=MAX_LEGACY_VALIDATION_REJECTION_AUDIT_BYTES)
                .contains(&audit_json_bytes)
                .then_some(())
                .ok_or(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)?;
            let audit_json: String = transaction
                .query_row(
                    SELECT_LEGACY_VALIDATION_REJECTION_AUDIT_JSON,
                    [row_id],
                    |row| row.get(0),
                )
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
            if rejection_audit_scope::is_legacy_validation_rejection_audit(&audit_json) {
                transaction
                    .execute(
                        UPDATE_LEGACY_AUDIT_SCOPE,
                        params![row_id, "validation-rejection", startup_now_nanos],
                    )
                    .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
            }
        }
        transaction
            .commit()
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
        last_row_id = batch_last_row_id;
    }
}

pub(super) fn drain_expired_at_startup(
    connection: &mut Connection,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let _ = trusted_now_nanos;
    let transaction = immediate_transaction_with_contention_retry(connection)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    trim_validation_rejection_audits(&transaction, trusted_now_nanos)?;
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
}

pub(super) fn audit_scope(audit: &AuthenticatedDeliveryGrantAudit) -> &'static str {
    rejection_audit_scope::audit_scope(audit)
}

fn ensure_column(
    connection: &Connection,
    column: &str,
    add_column: &str,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let exists = connection
        .prepare("SELECT 1 FROM pragma_table_info('authenticated_delivery_grant_audits_v2') WHERE name = ?1")
        .and_then(|mut statement| statement.exists([column]))
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    (!exists)
        .then(|| connection.execute(add_column, []))
        .transpose()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

fn trim_validation_rejection_audits(
    transaction: &Transaction<'_>,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let _ = trusted_now_nanos;
    transaction
        .execute(
            DELETE_EXCESS_VALIDATION_REJECTIONS,
            [MAX_VALIDATION_REJECTION_AUDITS],
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

fn rejection(
    error: AuthenticatedDeliveryGrantConsumeError,
) -> Option<AuthenticatedDeliveryGrantValidationRejection> {
    match error {
        AuthenticatedDeliveryGrantConsumeError::InvalidGrant => {
            Some(AuthenticatedDeliveryGrantValidationRejection::InvalidGrant)
        }
        AuthenticatedDeliveryGrantConsumeError::SignatureRejected => {
            Some(AuthenticatedDeliveryGrantValidationRejection::SignatureRejected)
        }
        AuthenticatedDeliveryGrantConsumeError::BindingRejected => {
            Some(AuthenticatedDeliveryGrantValidationRejection::BindingRejected)
        }
        AuthenticatedDeliveryGrantConsumeError::Expired => {
            Some(AuthenticatedDeliveryGrantValidationRejection::Expired)
        }
        AuthenticatedDeliveryGrantConsumeError::NotYetValid => {
            Some(AuthenticatedDeliveryGrantValidationRejection::NotYetValid)
        }
        AuthenticatedDeliveryGrantConsumeError::DryRunRejected => {
            Some(AuthenticatedDeliveryGrantValidationRejection::DryRunRejected)
        }
        AuthenticatedDeliveryGrantConsumeError::Revoked => {
            Some(AuthenticatedDeliveryGrantValidationRejection::Revoked)
        }
        AuthenticatedDeliveryGrantConsumeError::IntegrityRejected
        | AuthenticatedDeliveryGrantConsumeError::StorageUnavailable => None,
    }
}
