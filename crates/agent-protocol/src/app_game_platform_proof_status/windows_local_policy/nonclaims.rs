use super::{AppGameWindowsLocalPolicyEvidence, AppGameWindowsLocalPolicyEvidenceError};

pub(super) fn validate(
    evidence: &AppGameWindowsLocalPolicyEvidence,
) -> Result<(), AppGameWindowsLocalPolicyEvidenceError> {
    if !evidence.identifiers_redacted {
        return Err(AppGameWindowsLocalPolicyEvidenceError::IdentifiersNotRedacted);
    }
    let claims = [
        evidence.adapter_dispatch_claimed,
        evidence.broad_installed_app_blocking_claimed,
        evidence.platform_enforcement_claimed,
        evidence.rollback_claimed,
        evidence.audit_custody_claimed,
        evidence.provider_delivery_claimed,
        evidence.child_device_delivery_claimed,
        evidence.private_diagnostics_claimed,
    ];
    if claims.into_iter().any(|claimed| claimed) {
        return Err(AppGameWindowsLocalPolicyEvidenceError::UnsupportedClaim);
    }
    Ok(())
}
