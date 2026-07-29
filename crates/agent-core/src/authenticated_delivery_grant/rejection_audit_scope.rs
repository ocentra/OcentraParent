use crate::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantAuditOutcome,
};

pub(super) fn is_legacy_validation_rejection_audit(audit_json: &str) -> bool {
    serde_json::from_str::<AuthenticatedDeliveryGrantAudit>(audit_json).is_ok_and(|audit| {
        matches!(
            audit.outcome,
            AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(_)
        )
    })
}

pub(super) fn audit_scope(audit: &AuthenticatedDeliveryGrantAudit) -> &'static str {
    match audit.outcome {
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(_) => "validation-rejection",
        AuthenticatedDeliveryGrantAuditOutcome::Consumed
        | AuthenticatedDeliveryGrantAuditOutcome::ReplayRejected => "replay",
    }
}
