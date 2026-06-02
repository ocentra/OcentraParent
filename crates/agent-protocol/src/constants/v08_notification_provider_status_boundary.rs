pub const READ_MODEL_ID: &str = "v0-8-notification-provider-status-boundary";

pub const SOURCE_REPORTS_NOTIFICATIONS_SYNC: &str = "reports-notifications-sync-provider-status";
pub const SOURCE_INTEGRITY_ALERT_STATUS_BRIDGE: &str = "v0-8-integrity-alert-status-bridge";
pub const SOURCE_DATA_CUSTODY: &str = "data-custody-provider-boundary";

pub const ENTRY_QUEUED: &str = "notification-provider-queued-contract";
pub const ENTRY_DELIVERED: &str = "notification-provider-delivered-receipt-required";
pub const ENTRY_FAILED: &str = "notification-provider-failed-contract";
pub const ENTRY_UNAVAILABLE: &str = "notification-provider-unavailable-contract";
pub const ENTRY_MANUAL_REQUIRED: &str = "notification-provider-manual-required-contract";

pub const STATUS_QUEUED: &str = "queued";
pub const STATUS_DELIVERED: &str = "delivered";
pub const STATUS_FAILED: &str = "failed";
pub const STATUS_UNAVAILABLE: &str = "unavailable";
pub const STATUS_MANUAL_REQUIRED: &str = "manual-required";

pub const PROOF_QUEUED_CONTRACT_ONLY: &str = "queued-contract-only";
pub const PROOF_DELIVERY_RECEIPT_REQUIRED: &str = "delivery-receipt-required";
pub const PROOF_FAILURE_CONTRACT_ONLY: &str = "failure-contract-only";
pub const PROOF_PROVIDER_UNAVAILABLE_CONTRACT: &str = "provider-unavailable-contract";
pub const PROOF_MANUAL_ACTION_REQUIRED: &str = "manual-action-required";

pub const QUIET_HOURS_READY: &str = "ready";
pub const QUIET_HOURS_DEFER_NONCRITICAL: &str = "defer-noncritical";
pub const QUIET_HOURS_MANUAL_REQUIRED: &str = "manual-required";
pub const QUIET_HOURS_UNAVAILABLE: &str = "unavailable";

pub const ESCALATION_READY: &str = "ready";
pub const ESCALATION_WAITING_WINDOW: &str = "waiting-window";
pub const ESCALATION_MANUAL_REQUIRED: &str = "manual-required";
pub const ESCALATION_UNAVAILABLE: &str = "unavailable";

pub const DELIVERY_CLAIM_NOT_IMPLEMENTED: &str = "not-implemented";
pub const DELIVERY_CLAIM_NOT_OBSERVED: &str = "not-observed";
pub const DELIVERY_CLAIM_RECEIPT_REQUIRED: &str = "receipt-required";

pub const REF_NOTIFICATION_INTENT: &str = "notification-intent-provider-status-boundary-ref";
pub const REF_AUDIT: &str = "notification-provider-status-audit-ref";
pub const REF_PARENT_PREFERENCES: &str = "notification-parent-preferences-ref";
pub const REF_STATUS_QUEUED: &str = "notification-status-queued-ref";
pub const REF_STATUS_DELIVERED: &str = "notification-status-delivered-contract-ref";
pub const REF_STATUS_FAILED: &str = "notification-status-failed-ref";
pub const REF_STATUS_UNAVAILABLE: &str = "notification-status-provider-unavailable-ref";
pub const REF_STATUS_MANUAL_REQUIRED: &str = "notification-status-manual-required-ref";
pub const REF_ATTEMPT_QUEUED: &str = "provider-attempt-queued-ref";
pub const REF_ATTEMPT_DELIVERED: &str = "provider-attempt-delivered-contract-ref";
pub const REF_ATTEMPT_FAILED: &str = "provider-attempt-failed-ref";
pub const REF_ATTEMPT_UNAVAILABLE: &str = "provider-attempt-unavailable-ref";
pub const REF_ATTEMPT_MANUAL_REQUIRED: &str = "provider-attempt-manual-required-ref";
pub const REF_QUIET_READY: &str = "quiet-hours-ready-ref";
pub const REF_QUIET_DEFER_NONCRITICAL: &str = "quiet-hours-defer-noncritical-ref";
pub const REF_QUIET_UNAVAILABLE: &str = "quiet-hours-unavailable-ref";
pub const REF_QUIET_MANUAL_REQUIRED: &str = "quiet-hours-manual-required-ref";
pub const REF_ESCALATION_READY: &str = "escalation-ready-ref";
pub const REF_ESCALATION_WAITING_WINDOW: &str = "escalation-waiting-window-ref";
pub const REF_ESCALATION_MANUAL_REQUIRED: &str = "escalation-manual-required-ref";
pub const REF_ESCALATION_UNAVAILABLE: &str = "escalation-unavailable-ref";
pub const REF_PROVIDER_RECEIPT_REQUIRED: &str = "provider-delivery-receipt-required-ref";

pub const REQUIREMENT_PROVIDER_RECEIPT_ARTIFACT: &str =
    "real provider receipt artifact before delivery can be claimed";
pub const REQUIREMENT_PROVIDER_ERROR_ARTIFACT: &str =
    "provider error artifact before retry behavior is claimed";
pub const REQUIREMENT_PROVIDER_CONFIGURATION: &str = "provider configuration or credential review";
pub const REQUIREMENT_PARENT_PROVIDER_SETUP: &str = "parent/provider preference setup";
pub const REQUIREMENT_PROVIDER_SECURITY_REVIEW: &str = "security review before provider enablement";

pub const BOUNDARY_QUEUED: &str = "Queued means a typed provider attempt can be represented; no provider adapter sends or delivers the alert in this proof.";
pub const BOUNDARY_DELIVERED: &str = "Delivered is a contract state for future provider receipts; this read model records no observed delivered notification.";
pub const BOUNDARY_FAILED: &str = "Failed status is visible and auditable as a contract state; retry behavior and provider error ingestion remain unimplemented.";
pub const BOUNDARY_UNAVAILABLE: &str = "Unavailable status keeps child safety local and records that no provider adapter is configured or reachable.";
pub const BOUNDARY_MANUAL_REQUIRED: &str = "Manual-required status covers provider setup, quiet-hours, and escalation readiness gaps without sending third-party payloads.";
