use serde_json::{json, Value};

pub(super) fn runtime_provider_unavailable_failure() -> Value {
    runtime_failure_state(
        "provider-unavailable",
        "unavailable",
        "local-only",
        "wait-for-provider",
        true,
        Some(super::RETRY_TIMESTAMP),
    )
}

pub(super) fn runtime_stale_snapshot_failure() -> Value {
    runtime_failure_state(
        "stale-snapshot",
        "stale",
        "grace-with-local-safety",
        "wait-for-provider",
        true,
        Some(super::RETRY_TIMESTAMP),
    )
}

pub(super) fn runtime_payment_required_failure() -> Value {
    runtime_failure_state(
        "payment-required",
        "past-due",
        "grace-with-local-safety",
        "payment-update",
        true,
        None,
    )
}

pub(super) fn runtime_validation_failure() -> Value {
    runtime_failure_state(
        "validation-failed",
        "manual-review",
        "manual-review-with-local-safety",
        "manual-support-review",
        false,
        None,
    )
}

fn runtime_failure_state(
    failure_kind: &str,
    parent_visible_state: &str,
    local_safety_behavior: &str,
    parent_resolution: &str,
    retry_allowed: bool,
    retry_after: Option<&str>,
) -> Value {
    json!({
        "failureKind": failure_kind,
        "parentVisibleState": parent_visible_state,
        "localSafetyBehavior": local_safety_behavior,
        "retainEvidenceExportAccess": true,
        "existingLocalSafetyContinues": true,
        "parentResolution": parent_resolution,
        "retryAllowed": retry_allowed,
        "retryAfter": retry_after,
    })
}
