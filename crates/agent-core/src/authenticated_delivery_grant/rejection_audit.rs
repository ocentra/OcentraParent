use rusqlite::{Connection, TransactionBehavior};

use super::{
    audit, persist_audit_transaction, AuthenticatedDeliveryGrant,
    AuthenticatedDeliveryGrantAuditOutcome, AuthenticatedDeliveryGrantConsumeError,
    AuthenticatedDeliveryGrantValidationRejection,
};

pub(super) fn persist(
    connection: &mut Connection,
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: &str,
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
            persist_audit_transaction(&transaction, grant, &audit)?;
            transaction
                .commit()
                .map_err(|_error| AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
        });
    result.map_or(
        AuthenticatedDeliveryGrantConsumeError::StorageUnavailable,
        |_| error,
    )
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
