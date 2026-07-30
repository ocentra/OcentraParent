use sha2::{Digest, Sha256};

use super::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantAuditOutcome,
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantTrustedIssuer,
};
use ocentra_schema::authenticated_delivery_grant::{
    authenticated_delivery_grant_audit_fingerprint, AuthenticatedDeliveryGrant,
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
};

pub(super) fn validate_trusted_issuer(
    trusted_issuer: &AuthenticatedDeliveryGrantTrustedIssuer,
) -> Result<(), AuthenticatedDeliveryGrantConsumeError> {
    (!trusted_issuer.key_id.trim().is_empty()
        && trusted_issuer.key_id.len() <= AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES)
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantConsumeError::InvalidGrant)
}

pub(super) fn storage_key_digest(value: &str) -> String {
    let first = Sha256::digest(value.as_bytes());
    format!("{:x}", Sha256::digest(first))
}

pub(super) fn audit(
    grant: &AuthenticatedDeliveryGrant,
    correlation_id: String,
    outcome: AuthenticatedDeliveryGrantAuditOutcome,
) -> AuthenticatedDeliveryGrantAudit {
    AuthenticatedDeliveryGrantAudit {
        correlation_id,
        issuer_key_id_digest: format!("{:x}", Sha256::digest(grant.issuer_key_id.as_bytes())),
        nonce_digest: format!("{:x}", Sha256::digest(grant.nonce.as_bytes())),
        grant_digest: authenticated_delivery_grant_audit_fingerprint(grant),
        outcome,
    }
}
