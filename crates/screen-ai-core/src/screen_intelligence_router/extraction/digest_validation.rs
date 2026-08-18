use super::{
    VerifiedManagedBrowserStructuredExtractionReceipt, VerifiedStructuredExtractionOutcome,
    MANAGED_BROWSER_SENSITIVITY_PROTECTED, MANAGED_BROWSER_SENSITIVITY_STRUCTURAL_SAFE,
    MANAGED_BROWSER_SENSITIVITY_UNAVAILABLE, MANAGED_BROWSER_SENSITIVITY_UNKNOWN,
    MANAGED_BROWSER_STRUCTURED_AUTHORITY_DIGEST_UNAVAILABLE,
    MANAGED_BROWSER_STRUCTURED_EVIDENCE_DIGEST_UNAVAILABLE,
    MANAGED_BROWSER_STRUCTURED_EVIDENCE_KIND_UNAVAILABLE,
    MANAGED_BROWSER_STRUCTURED_SIGNAL_PROTECTED, MANAGED_BROWSER_STRUCTURED_SIGNAL_UNAVAILABLE,
    MANAGED_BROWSER_TARGET_REF_PREFIX, MANAGED_BROWSER_TARGET_REF_UNAVAILABLE,
    MANAGED_BROWSER_TITLE_REF_PREFIX, MANAGED_BROWSER_TITLE_REF_UNAVAILABLE,
    MANAGED_BROWSER_URL_REF_PREFIX, MANAGED_BROWSER_URL_REF_UNAVAILABLE,
};

pub(super) fn valid_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub(super) fn valid_signal_digest(
    receipt: &VerifiedManagedBrowserStructuredExtractionReceipt,
) -> bool {
    match &receipt.outcome {
        VerifiedStructuredExtractionOutcome::ProtectedContentSkipped => {
            receipt.structured_signal_digest == MANAGED_BROWSER_STRUCTURED_SIGNAL_PROTECTED
        }
        VerifiedStructuredExtractionOutcome::Unavailable => {
            receipt.structured_signal_digest == MANAGED_BROWSER_STRUCTURED_SIGNAL_UNAVAILABLE
        }
        VerifiedStructuredExtractionOutcome::ReviewRequired => {
            valid_digest(&receipt.structured_signal_digest)
        }
    }
}

pub(super) fn valid_sensitivity_digest(
    receipt: &VerifiedManagedBrowserStructuredExtractionReceipt,
) -> bool {
    match &receipt.outcome {
        VerifiedStructuredExtractionOutcome::ProtectedContentSkipped => {
            receipt.structured_sensitivity_digest == MANAGED_BROWSER_SENSITIVITY_PROTECTED
        }
        VerifiedStructuredExtractionOutcome::Unavailable => {
            receipt.structured_sensitivity_digest == MANAGED_BROWSER_SENSITIVITY_UNAVAILABLE
        }
        VerifiedStructuredExtractionOutcome::ReviewRequired => matches!(
            receipt.structured_sensitivity_digest.as_str(),
            MANAGED_BROWSER_SENSITIVITY_STRUCTURAL_SAFE | MANAGED_BROWSER_SENSITIVITY_UNKNOWN
        ),
    }
}

/// The producer digest covers launch-private inputs that this neutral handoff
/// intentionally does not expose. Real outcomes therefore receive shape-only
/// validation here; cryptographic recomputation remains an upstream boundary.
pub(super) fn valid_authority_digest(
    receipt: &VerifiedManagedBrowserStructuredExtractionReceipt,
) -> bool {
    let unavailable = matches!(
        &receipt.outcome,
        VerifiedStructuredExtractionOutcome::Unavailable
    );
    (unavailable
        && receipt.authority_digest == MANAGED_BROWSER_STRUCTURED_AUTHORITY_DIGEST_UNAVAILABLE)
        || (!unavailable && valid_digest(&receipt.authority_digest))
}

pub(super) fn valid_evidence_refs(
    receipt: &VerifiedManagedBrowserStructuredExtractionReceipt,
) -> bool {
    if receipt.evidence_refs.len() != 3 {
        return false;
    }

    match &receipt.outcome {
        VerifiedStructuredExtractionOutcome::Unavailable => {
            receipt.evidence_refs.iter().all(|reference| {
                reference.kind == MANAGED_BROWSER_STRUCTURED_EVIDENCE_KIND_UNAVAILABLE
                    && reference.uri.is_none()
                    && reference.digest == MANAGED_BROWSER_STRUCTURED_EVIDENCE_DIGEST_UNAVAILABLE
            }) && receipt
                .evidence_refs
                .iter()
                .any(|reference| reference.evidence_id == MANAGED_BROWSER_TARGET_REF_UNAVAILABLE)
                && receipt
                    .evidence_refs
                    .iter()
                    .any(|reference| reference.evidence_id == MANAGED_BROWSER_URL_REF_UNAVAILABLE)
                && receipt
                    .evidence_refs
                    .iter()
                    .any(|reference| reference.evidence_id == MANAGED_BROWSER_TITLE_REF_UNAVAILABLE)
        }
        VerifiedStructuredExtractionOutcome::ProtectedContentSkipped
        | VerifiedStructuredExtractionOutcome::ReviewRequired => {
            receipt.evidence_refs.iter().all(|reference| {
                !reference.evidence_id.trim().is_empty()
                    && reference.digest == receipt.structured_evidence_digest
                    && reference.uri.is_none()
            }) && receipt.evidence_refs.iter().any(|reference| {
                reference.evidence_id == receipt.authority.target_ref
                    && reference
                        .evidence_id
                        .starts_with(MANAGED_BROWSER_TARGET_REF_PREFIX)
            }) && receipt.evidence_refs.iter().any(|reference| {
                reference
                    .evidence_id
                    .starts_with(MANAGED_BROWSER_URL_REF_PREFIX)
            }) && receipt.evidence_refs.iter().any(|reference| {
                reference
                    .evidence_id
                    .starts_with(MANAGED_BROWSER_TITLE_REF_PREFIX)
            })
        }
    }
}

pub(super) fn valid_body_digest(value: &str) -> bool {
    value
        .strip_prefix("managed-browser-body-sha256-v1-")
        .is_some_and(valid_digest)
}
