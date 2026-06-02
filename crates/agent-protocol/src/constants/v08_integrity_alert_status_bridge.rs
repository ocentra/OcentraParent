pub const READ_MODEL_ID: &str = "v0-8-integrity-alert-status-bridge";

pub const SOURCE_ENFORCEMENT_INTEGRITY_RUNTIME_AUDIT: &str =
    "v0-8-enforcement-integrity-runtime-audit";
pub const SOURCE_SUPPORTED_ADAPTER_RUNTIME_PROOF: &str = "v0-8-supported-adapter-runtime-proof";
pub const SOURCE_REPORTS_NOTIFICATIONS_SYNC: &str = "reports-notifications-sync-intent-status";

pub const ENTRY_PERMISSION_LOSS: &str = "permission-loss-alert-status";
pub const ENTRY_STALE_HEARTBEAT: &str = "stale-heartbeat-alert-status";
pub const ENTRY_STOPPED_OR_REMOVED: &str = "stopped-or-removed-alert-status";
pub const ENTRY_TAMPER_MANUAL: &str = "tamper-manual-alert-status";

pub const STATE_PERMISSION_LOSS: &str = "permission-loss";
pub const STATE_STALE_HEARTBEAT: &str = "stale-heartbeat";
pub const STATE_STOPPED_OR_REMOVED: &str = "stopped-or-removed";
pub const STATE_TAMPER_MANUAL_REQUIRED: &str = "tamper-manual-required";

pub const STATUS_PERMISSION_ACTION_REQUIRED: &str = "permission-action-required";
pub const STATUS_AGENT_HEARTBEAT_STALE: &str = "agent-heartbeat-stale";
pub const STATUS_AGENT_STOPPED_OR_REMOVED: &str = "agent-stopped-or-removed";
pub const STATUS_TAMPER_REVIEW_REQUIRED: &str = "tamper-review-required";

pub const NOTIFICATION_INTENT_CREATED: &str = "intent-created";
pub const NOTIFICATION_MANUAL_REVIEW_REQUIRED: &str = "manual-review-required";
pub const DELIVERY_PROVIDER_NOT_CONFIGURED: &str = "not-delivered-provider-not-configured";
pub const AUDIT_REF_BACKED: &str = "audit-ref-backed";
pub const AUDIT_MANUAL_REQUIRED: &str = "manual-required";

pub const REF_REASON_PERMISSION_LOSS: &str = "reason-permission-loss-ref";
pub const REF_REASON_STALE_HEARTBEAT: &str = "reason-stale-heartbeat-ref";
pub const REF_REASON_STOPPED_OR_REMOVED: &str = "reason-agent-stopped-or-removed-ref";
pub const REF_REASON_TAMPER_MANUAL: &str = "reason-tamper-manual-required-ref";
pub const REF_STATUS_PERMISSION_ACTION_REQUIRED: &str = "status-permission-action-required-ref";
pub const REF_STATUS_AGENT_HEARTBEAT_STALE: &str = "status-agent-heartbeat-stale-ref";
pub const REF_STATUS_AGENT_STOPPED_OR_REMOVED: &str = "status-agent-stopped-or-removed-ref";
pub const REF_STATUS_TAMPER_REVIEW_REQUIRED: &str = "status-tamper-review-required-ref";
pub const REF_NOTIFICATION_STATUS_PROVIDER_NOT_CONFIGURED: &str =
    "notification-status-provider-not-configured-ref";
pub const REF_NOTIFICATION_INTENT_PERMISSION_LOSS: &str = "notification-intent-permission-loss-ref";
pub const REF_NOTIFICATION_INTENT_STALE_HEARTBEAT: &str = "notification-intent-stale-heartbeat-ref";
pub const REF_NOTIFICATION_INTENT_STOPPED_OR_REMOVED: &str =
    "notification-intent-agent-stopped-or-removed-ref";
pub const REF_NOTIFICATION_INTENT_TAMPER_MANUAL: &str = "notification-intent-tamper-manual-ref";
pub const REF_AUDIT_PERMISSION_LOSS: &str = "enforcement-audit-permission-loss-ref";
pub const REF_AUDIT_STALE_HEARTBEAT: &str = "enforcement-audit-stale-heartbeat-ref";
pub const REF_AUDIT_STOPPED_OR_REMOVED: &str = "enforcement-audit-agent-stopped-or-removed-ref";
pub const REF_AUDIT_TAMPER_MANUAL: &str = "enforcement-audit-tamper-manual-ref";
pub const REF_INTEGRITY_PERMISSION_STATE: &str = "integrity-permission-state-ref";
pub const REF_INTEGRITY_HEARTBEAT: &str = "integrity-heartbeat-ref";
pub const REF_INTEGRITY_SERVICE_STATE: &str = "integrity-service-state-ref";
pub const REF_INTEGRITY_TAMPER_SIGNAL: &str = "integrity-tamper-signal-ref";
pub const REF_DRILL_IN_PERMISSION_LOSS: &str = "drill-in-permission-loss-audit-ref";
pub const REF_DRILL_IN_STALE_HEARTBEAT: &str = "drill-in-stale-heartbeat-audit-ref";
pub const REF_DRILL_IN_STOPPED_OR_REMOVED: &str = "drill-in-agent-stopped-or-removed-audit-ref";
pub const REF_DRILL_IN_TAMPER_MANUAL: &str = "drill-in-tamper-manual-audit-ref";

pub const REQUIREMENT_PERMISSION_RESTORE: &str = "permission restoration artifact";
pub const REQUIREMENT_FRESH_HEARTBEAT: &str = "fresh heartbeat proof";
pub const REQUIREMENT_SERVICE_RESTART_RECOVERY: &str = "service restart recovery proof";
pub const REQUIREMENT_UNINSTALL_DETECTION_ARTIFACT: &str = "uninstall detection artifact";
pub const REQUIREMENT_SERVICE_MANAGER_STOP_PROOF: &str = "service-manager stop proof";
pub const REQUIREMENT_SECURITY_REVIEW: &str = "security review before hardening";

pub const BOUNDARY_PERMISSION_LOSS: &str = "Permission loss is parent-visible unavailable status with notification intent and audit refs; provider delivery remains unconfigured and unclaimed.";
pub const BOUNDARY_STALE_HEARTBEAT: &str = "Stale heartbeat is a degraded integrity status and alert intent; it is not anti-tamper hardening or provider-delivery proof.";
pub const BOUNDARY_STOPPED_OR_REMOVED: &str = "Stopped or removed service state is represented as an auditable parent status and alert intent; it does not install persistence or anti-removal behavior.";
pub const BOUNDARY_TAMPER_MANUAL: &str = "Tamper/uninstall remains manual-required evidence review only; no stealth, persistence, privilege escalation, or anti-tamper resistance is claimed.";
