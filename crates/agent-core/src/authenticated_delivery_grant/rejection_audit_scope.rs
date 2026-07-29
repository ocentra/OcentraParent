use crate::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantAuditOutcome,
};

pub(super) fn audit_scope(audit: &AuthenticatedDeliveryGrantAudit) -> &'static str {
    match audit.outcome {
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(_) => "validation-rejection",
        AuthenticatedDeliveryGrantAuditOutcome::Consumed
        | AuthenticatedDeliveryGrantAuditOutcome::ReplayRejected => "replay",
    }
}
