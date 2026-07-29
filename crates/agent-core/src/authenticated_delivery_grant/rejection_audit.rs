use rusqlite::{params, Connection, Transaction, TransactionBehavior};

use super::{
    audit, persist_audit_transaction, AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantAudit,
    AuthenticatedDeliveryGrantAuditOutcome, AuthenticatedDeliveryGrantConsumeError,
    AuthenticatedDeliveryGrantValidationRejection,
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
    let result = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
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
const BACKFILL_LEGACY_VALIDATION_REJECTIONS: &str = "UPDATE authenticated_delivery_grant_audits_v2 SET audit_scope = 'validation-rejection', recorded_at_nanos = 0 WHERE audit_scope = 'replay' AND instr(audit_json, '\"validation-rejected\"') > 0";
const CREATE_VALIDATION_REJECTION_RETENTION_INDEX: &str = "CREATE INDEX IF NOT EXISTS authenticated_delivery_grant_audits_v2_validation_rejection_retention_idx ON authenticated_delivery_grant_audits_v2 (audit_scope, recorded_at_nanos DESC)";
const DELETE_EXPIRED_VALIDATION_REJECTIONS: &str = "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE rowid IN (SELECT rowid FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection' AND recorded_at_nanos <= ?1 ORDER BY recorded_at_nanos LIMIT ?2)";
const DELETE_EXCESS_VALIDATION_REJECTIONS: &str = "DELETE FROM authenticated_delivery_grant_audits_v2 WHERE rowid IN (SELECT rowid FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection' ORDER BY recorded_at_nanos DESC, rowid DESC LIMIT -1 OFFSET ?1)";
const MAX_VALIDATION_REJECTION_AUDITS: i64 = 1_024;
const MAX_VALIDATION_REJECTION_AUDITS_PER_PURGE: i64 = 128;
const VALIDATION_REJECTION_AUDIT_RETENTION_NANOS: i64 = 86_400_000_000_000;

pub(super) fn ensure_retention_schema(
    connection: &Connection,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    ensure_column(
        connection,
        "recorded_at_nanos",
        ADD_AUDIT_RECORDED_AT_NANOS_COLUMN,
    )?;
    ensure_column(connection, "audit_scope", ADD_AUDIT_SCOPE_COLUMN)?;
    connection
        .execute(BACKFILL_LEGACY_VALIDATION_REJECTIONS, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    connection
        .execute(CREATE_VALIDATION_REJECTION_RETENTION_INDEX, [])
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(())
}

pub(super) fn drain_expired_at_startup(
    connection: &mut Connection,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    while purge_expired_validation_rejections(connection, trusted_now_nanos)?
        == MAX_VALIDATION_REJECTION_AUDITS_PER_PURGE as usize
    {}
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
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
    if !exists {
        connection
            .execute(add_column, [])
            .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    }
    Ok(())
}

fn purge_expired_validation_rejections(
    connection: &mut Connection,
    trusted_now_nanos: i64,
) -> Result<usize, AuthenticatedDeliveryGrantConsumeError> {
    let cutoff_nanos = trusted_now_nanos.saturating_sub(VALIDATION_REJECTION_AUDIT_RETENTION_NANOS);
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    let count = transaction
        .execute(
            DELETE_EXPIRED_VALIDATION_REJECTIONS,
            params![cutoff_nanos, MAX_VALIDATION_REJECTION_AUDITS_PER_PURGE],
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    transaction
        .commit()
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
    Ok(count)
}

fn trim_validation_rejection_audits(
    transaction: &Transaction<'_>,
    trusted_now_nanos: i64,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    let cutoff_nanos = trusted_now_nanos.saturating_sub(VALIDATION_REJECTION_AUDIT_RETENTION_NANOS);
    transaction
        .execute(
            DELETE_EXPIRED_VALIDATION_REJECTIONS,
            params![cutoff_nanos, MAX_VALIDATION_REJECTION_AUDITS_PER_PURGE],
        )
        .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)?;
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
