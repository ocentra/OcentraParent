use super::{
    VerifiedManagedBrowserStructuredExtractionReceipt, VerifiedStructuredExtractionOutcome,
    MANAGED_BROWSER_SENSITIVITY_PROTECTED, MANAGED_BROWSER_SENSITIVITY_STRUCTURAL_SAFE,
    MANAGED_BROWSER_SENSITIVITY_UNAVAILABLE, MANAGED_BROWSER_SENSITIVITY_UNKNOWN,
    MANAGED_BROWSER_STRUCTURED_SIGNAL_PROTECTED, MANAGED_BROWSER_STRUCTURED_SIGNAL_UNAVAILABLE,
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

pub(super) fn valid_sensitivity_digest(value: &str) -> bool {
    matches!(
        value,
        MANAGED_BROWSER_SENSITIVITY_STRUCTURAL_SAFE
            | MANAGED_BROWSER_SENSITIVITY_UNKNOWN
            | MANAGED_BROWSER_SENSITIVITY_PROTECTED
            | MANAGED_BROWSER_SENSITIVITY_UNAVAILABLE
    )
}

pub(super) fn valid_body_digest(value: &str) -> bool {
    value
        .strip_prefix("managed-browser-body-sha256-v1-")
        .is_some_and(valid_digest)
}
