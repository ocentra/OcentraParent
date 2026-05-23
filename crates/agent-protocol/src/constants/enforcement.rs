pub const INTENT_SOURCE_PARENT_PORTAL: &str = "parent-portal";
pub const INTENT_SOURCE_PARENT_RULE: &str = "parent-rule";
pub const INTENT_SOURCE_LOCAL_POLICY_EVALUATOR: &str = "local-policy-evaluator";
pub const INTENT_SOURCE_SYSTEM_RECOVERY: &str = "system-recovery";

pub const ADAPTER_KIND_PROCESS_CONTROL: &str = "process-control";
pub const ADAPTER_KIND_NETWORK_CONTROL: &str = "network-control";
pub const ADAPTER_KIND_MANAGED_BROWSER_CONTROL: &str = "managed-browser-control";
pub const ADAPTER_KIND_TIMER_CONTROL: &str = "timer-control";

pub const MODE_TERMINATE_PROCESS: &str = "terminate-process";
pub const MODE_BLOCK_PROCESS: &str = "block-process";
pub const MODE_TEMPORARY_BLOCK: &str = "temporary-block";
pub const MODE_TIME_LIMIT: &str = "time-limit";
pub const MODE_ASK_PARENT: &str = "ask-parent";
pub const MODE_OBSERVE_ONLY: &str = "observe-only";

pub const CAPABILITY_SUPPORTED: &str = "supported";
pub const CAPABILITY_UNAVAILABLE: &str = "unavailable";
pub const CAPABILITY_DEGRADED: &str = "degraded";
pub const CAPABILITY_DRY_RUN: &str = "dry-run";
pub const CAPABILITY_OBSERVE_ONLY: &str = "observe-only";

pub const UNAVAILABLE_UNSUPPORTED_PLATFORM: &str = "unsupported-platform";
pub const UNAVAILABLE_UNSUPPORTED_ACTION: &str = "unsupported-action";
pub const UNAVAILABLE_MISSING_PERMISSION: &str = "missing-permission";
pub const UNAVAILABLE_MISSING_DEPENDENCY: &str = "missing-dependency";
pub const UNAVAILABLE_ADAPTER_UNAVAILABLE: &str = "adapter-unavailable";
pub const UNAVAILABLE_ADAPTER_ERROR: &str = "adapter-error";

pub const PERMISSION_ALLOWED: &str = "allowed";
pub const PERMISSION_MISSING: &str = "missing-permission";
pub const PERMISSION_NOT_REQUIRED: &str = "not-required";
pub const PERMISSION_UNKNOWN: &str = "unknown";

pub const DEPENDENCY_INSTALLED: &str = "installed";
pub const DEPENDENCY_MISSING: &str = "missing";
pub const DEPENDENCY_NOT_REQUIRED: &str = "not-required";
pub const DEPENDENCY_UNKNOWN: &str = "unknown";

pub const RESULT_WOULD_ENFORCE: &str = "would-enforce";
pub const RESULT_ACTUALLY_ENFORCED: &str = "actually-enforced";
pub const RESULT_UNAVAILABLE: &str = "unavailable";
pub const RESULT_FAILED: &str = "failed";
pub const RESULT_EXPIRED: &str = "expired";
pub const RESULT_ROLLED_BACK: &str = "rolled-back";
pub const RESULT_SUPERSEDED: &str = "superseded";
pub const RESULT_NO_OP: &str = "no-op";

pub const ROLLBACK_NOT_REQUIRED: &str = "not-required";
pub const ROLLBACK_AVAILABLE: &str = "available";
pub const ROLLBACK_REQUESTED: &str = "requested";
pub const ROLLBACK_COMPLETED: &str = "completed";
pub const ROLLBACK_UNAVAILABLE: &str = "unavailable";
pub const ROLLBACK_FAILED: &str = "failed";

pub const ADAPTER_PROCESS_TERMINATED: &str = "process-terminated";
pub const ADAPTER_PROCESS_ALREADY_EXITED: &str = "process-already-exited";
pub const ADAPTER_LEFT_RUNNING_OBSERVE_ONLY: &str = "left-running-observe-only";
pub const ADAPTER_DRY_RUN_NO_ACTION: &str = "dry-run-no-action";
pub const ADAPTER_UNSUPPORTED_PLATFORM: &str = "unsupported-platform";
pub const ADAPTER_UNAVAILABLE: &str = "adapter-unavailable";
pub const ADAPTER_FAILED: &str = "adapter-failed";
pub const ADAPTER_TIMER_EXPIRED: &str = "timer-expired";
pub const ADAPTER_ROLLBACK_COMPLETED: &str = "rollback-completed";
pub const ADAPTER_NO_OP: &str = "no-op";

pub const TIMER_CREATED: &str = "created";
pub const TIMER_EXTENDED: &str = "extended";
pub const TIMER_EXPIRED: &str = "expired";
pub const TIMER_CANCELLED: &str = "cancelled";
pub const TIMER_RESTART_RECOVERED: &str = "restart-recovered";
pub const TIMER_ROLLBACK_REQUESTED: &str = "rollback-requested";
pub const TIMER_ROLLBACK_COMPLETED: &str = "rollback-completed";
pub const TIMER_RECOVERY_NEEDED: &str = "recovery-needed";
pub const TIMER_UNAVAILABLE: &str = "unavailable";

pub const AUDIT_ATTEMPTED: &str = "attempted";
pub const AUDIT_SUCCEEDED: &str = "succeeded";
pub const AUDIT_FAILED: &str = "failed";
pub const AUDIT_ROLLBACK_REQUESTED: &str = "rollback-requested";
pub const AUDIT_ROLLBACK_COMPLETED: &str = "rollback-completed";
pub const AUDIT_EXPIRED: &str = "expired";
pub const AUDIT_UNAVAILABLE: &str = "unavailable";

pub const PLATFORM_WINDOWS: &str = "windows";
pub const PLATFORM_LINUX: &str = "linux";
pub const PLATFORM_MACOS: &str = "macos";
pub const PLATFORM_ANDROID: &str = "android";
pub const PLATFORM_IOS: &str = "ios";

pub const REJECTION_DECISION_ID_MISMATCH: &str = "policy-decision-id-mismatch";
pub const REJECTION_TARGET_MISMATCH: &str = "policy-target-mismatch";
pub const REJECTION_MISSING_EVIDENCE: &str = "missing-policy-evidence-reference";
pub const REJECTION_POLICY_ACTION_NOT_ENFORCEABLE: &str = "policy-action-not-enforceable";
pub const REJECTION_UNSUPPORTED_CAPABILITY: &str = "unsupported-enforcement-capability";
pub const REJECTION_ADAPTER_RESULT_REQUIRED: &str = "adapter-result-required";

pub const TEST_INTENT_ID: &str = "intent-1";
pub const TEST_ACTION_ID: &str = "action-1";
pub const TEST_RESULT_ID: &str = "result-1";
pub const TEST_AUDIT_EVENT_ID: &str = "audit-1";
pub const TEST_TIMER_EVENT_ID: &str = "timer-1";
pub const TEST_ROLLBACK_TOKEN: &str = "rollback-1";
pub const TEST_IDEMPOTENCY_KEY: &str = "decision-1:target-process-1";
pub const TEST_JOURNAL_SEQUENCE: &str = "journal-sequence-1";
pub const TEST_PROCESS_TARGET_ID: &str = "target-process-1";
pub const TEST_PROCESS_TARGET_VALUE: &str = "owned-child-process";
pub const TEST_CHILD_DEVICE_ID: &str = "child-device-1";
pub const TEST_CHILD_DEVICE_LABEL: &str = "Child Windows PC";
