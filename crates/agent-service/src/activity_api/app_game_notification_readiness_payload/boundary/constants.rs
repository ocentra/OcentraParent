pub const FAMILY_ID: &str = "app-game";
pub const PROVIDER_STATUS_QUEUED: &str = "queued";
pub const PROVIDER_STATUS_DELIVERED: &str = "delivered";
pub const PROVIDER_STATUS_FAILED: &str = "failed";
pub const PROVIDER_STATUS_UNAVAILABLE: &str = "unavailable";
pub const PROVIDER_STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const DELIVERY_RESULT_NOT_SENT: &str = "not-sent";
pub const PREFERENCE_STATE_CHANNEL_DISABLED: &str = "channel-disabled";
pub const PREFERENCE_STATE_MANUAL_SETUP_REQUIRED: &str = "manual-setup-required";
pub const QUIET_HOURS_ALLOW: &str = "allow";
pub const PREVIEW_PREFIX: &str = "Verified WP59 scheduler and WP61 preflight refs only;";
pub const PREVIEW_SUFFIX: &str = " provider delivery is not implemented.";
pub const DELIVERY_RESULT_PREFIX: &str = "delivery-result-not-observed:";
pub const DELIVERY_RESULT_SUFFIX: &str = "scheduler-preflight-unavailable";
pub const SCHEDULER_INVALID_UNAVAILABLE: &str = "unavailable:invalid-scheduler-evidence";
pub const SCHEDULER_INVALID_MANUAL: &str = "manual-required:invalid-scheduler-evidence";
pub const SCHEDULER_MISSING_UNAVAILABLE: &str = "unavailable:scheduler-evidence-unavailable";
pub const SCHEDULER_MISSING_MANUAL: &str = "manual-required:scheduler-evidence-unavailable";
pub const MANUAL_PROVIDER_AVAILABILITY: &str = "manual-proof:provider-availability";
pub const MANUAL_PROVIDER_CREDENTIALS: &str = "manual-proof:provider-credentials";
pub const MANUAL_PROVIDER_DELIVERY_PREFIX: &str = "manual-proof:provider-delivery-";
pub const RECEIPT_SUFFIX: &str = "receipt";
pub const FALLBACK_PREFIX: &str = "No verified WP59 scheduler evidence;";
pub const FALLBACK_SUFFIX: &str = " status is manual-required or unavailable only.";
pub const MANUAL_PARENT_PREFERENCE: &str = "manual-proof:parent-preference";
pub const MANUAL_NOTIFICATION_CHANNEL: &str = "manual-proof:notification-channel";

pub(super) fn provider_preview_boundary() -> String {
    format!("{PREVIEW_PREFIX}{PREVIEW_SUFFIX}")
}

pub(super) fn delivery_result_unobserved() -> String {
    format!("{DELIVERY_RESULT_PREFIX}{DELIVERY_RESULT_SUFFIX}")
}

pub(super) fn provider_delivery_receipt() -> String {
    format!("{MANUAL_PROVIDER_DELIVERY_PREFIX}{RECEIPT_SUFFIX}")
}

pub(super) fn fallback_payload_boundary() -> String {
    format!("{FALLBACK_PREFIX}{FALLBACK_SUFFIX}")
}
