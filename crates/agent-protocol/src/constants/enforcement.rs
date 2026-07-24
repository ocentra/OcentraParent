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
pub const CAPABILITY_MANUAL_REQUIRED: &str = "manual-required";

pub const UNAVAILABLE_UNSUPPORTED_PLATFORM: &str = "unsupported-platform";
pub const UNAVAILABLE_UNSUPPORTED_ACTION: &str = "unsupported-action";
pub const UNAVAILABLE_MISSING_PERMISSION: &str = "missing-permission";
pub const UNAVAILABLE_MISSING_DEPENDENCY: &str = "missing-dependency";
pub const UNAVAILABLE_ADAPTER_UNAVAILABLE: &str = "adapter-unavailable";
pub const UNAVAILABLE_ADAPTER_ERROR: &str = "adapter-error";
pub const UNAVAILABLE_MANUAL_REQUIRED: &str = "manual-required";

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
pub const AUDIT_CANCELLED: &str = "cancelled";

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
pub const REJECTION_COMMAND_PAYLOAD_INVALID: &str = "enforcement-command-payload-invalid";
pub const REJECTION_PROCESS_ID_REQUIRED: &str = "enforcement-process-id-required";
pub const REJECTION_POLICY_VERSION_REQUIRED: &str = "enforcement-policy-version-required";
pub const REJECTION_RULE_ID_REQUIRED: &str = "enforcement-rule-id-required";
pub const REJECTION_REASON_CODE_REQUIRED: &str = "enforcement-reason-code-required";
pub const REJECTION_ACTIVE_TIMER_STATE_REQUIRED: &str = "enforcement-active-timer-state-required";
pub const REJECTION_ACTIVE_TIMER_STATE_MISMATCH: &str = "enforcement-active-timer-state-mismatch";
pub const REJECTION_PARENT_ACTION_REQUIRED: &str = "enforcement-parent-action-required";
pub const REJECTION_APP_GAME_SESSION_EVIDENCE_REQUIRED: &str =
    "enforcement-app-game-session-evidence-required";
pub const REJECTION_APP_GAME_RUNTIME_EVIDENCE_MISMATCH: &str =
    "enforcement-app-game-runtime-evidence-mismatch";
pub const APP_GAME_RUNTIME_EVIDENCE_GENERATED_AT: &str = "";

pub const SOURCE_ID_AGENT_SERVICE: &str = "enforcement-agent-service";
pub const JOURNAL_BEFORE_ACTION_ID_PREFIX: &str = "0-before-action-";
pub const INTENT_ID_PREFIX: &str = "enforcement-intent-";
pub const ACTION_ID_PREFIX: &str = "enforcement-action-";
pub const RESULT_ID_PREFIX: &str = "enforcement-result-";
pub const AUDIT_EVENT_ID_PREFIX: &str = "enforcement-audit-";
pub const TIMER_EVENT_ID_PREFIX: &str = "enforcement-timer-";
pub const ROLLBACK_TOKEN_PREFIX: &str = "enforcement-rollback-";
pub const TIMER_STATE_ID_PREFIX: &str = "enforcement-timer-state-";
pub const TIMER_STATE_FILE_NAME: &str = "ocentra-parent-enforcement-timers.json";

pub const TEST_INTENT_ID: &str = "intent-1";
pub const TEST_ACTION_ID: &str = "action-1";
pub const TEST_RESULT_ID: &str = "result-1";
pub const TEST_AUDIT_EVENT_ID: &str = "audit-1";
pub const TEST_TIMER_EVENT_ID: &str = "timer-1";
pub const TEST_ROLLBACK_TOKEN: &str = "rollback-1";
pub const TEST_PARENT_ACTION_REFERENCE_ID: &str = "parent-action-reference-1";
pub const TEST_TIMER_STATE_ID: &str = "timer-state-1";
pub const TEST_IDEMPOTENCY_KEY: &str = "decision-1:target-process-1";
pub const TEST_JOURNAL_SEQUENCE: &str = "journal-sequence-1";
pub const TEST_PROCESS_TARGET_ID: &str = "target-process-1";
pub const TEST_PROCESS_TARGET_VALUE: &str = "owned-child-process";
pub const TEST_CHILD_DEVICE_ID: &str = "child-device-1";
pub const TEST_CHILD_DEVICE_LABEL: &str = "Child Windows PC";

pub const READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER: &str = "v0-8-broad-os-adapter-readiness";
pub const READINESS_ID_OWNED_PROCESS_TERMINATE: &str = "readiness-owned-process-terminate";
pub const READINESS_ID_APP_TIME_LIMIT: &str = "readiness-app-time-limit";
pub const READINESS_ID_BROAD_APP_BLOCKING: &str = "readiness-broad-app-blocking";
pub const READINESS_ID_NETWORK_DOMAIN_BLOCKING: &str = "readiness-network-domain-blocking";
pub const READINESS_ID_MANAGED_BROWSER_SERVICE_COMMAND: &str =
    "readiness-managed-browser-service-command";
pub const READINESS_ID_MANAGED_BROWSER_EXACT_URL: &str =
    "readiness-managed-browser-exact-url-control";
pub const READINESS_ID_UNMANAGED_BROWSER_PROCESS_ONLY: &str =
    "readiness-unmanaged-browser-process-only";
pub const READINESS_ID_UNMANAGED_BROWSER_EXACT_EVIDENCE: &str =
    "readiness-unmanaged-browser-exact-evidence";
pub const READINESS_ID_ADMIN_ANTI_TAMPER_ROLLBACK: &str = "readiness-admin-anti-tamper-rollback";

pub const BROAD_CAPABILITY_OWNED_PROCESS_TERMINATE: &str = "owned-process-terminate";
pub const BROAD_CAPABILITY_APP_TIME_LIMIT: &str = "app-time-limit";
pub const BROAD_CAPABILITY_BROAD_APP_BLOCKING: &str = "broad-app-blocking";
pub const BROAD_CAPABILITY_NETWORK_DOMAIN_BLOCKING: &str = "network-domain-blocking";
pub const BROAD_CAPABILITY_MANAGED_BROWSER_SERVICE_COMMAND: &str =
    "managed-browser-service-command";
pub const BROAD_CAPABILITY_MANAGED_BROWSER_EXACT_URL_CONTROL: &str =
    "managed-browser-exact-url-control";
pub const BROAD_CAPABILITY_UNMANAGED_BROWSER_PROCESS_ONLY: &str = "unmanaged-browser-process-only";
pub const BROAD_CAPABILITY_UNMANAGED_BROWSER_EXACT_EVIDENCE: &str =
    "unmanaged-browser-exact-evidence";
pub const BROAD_CAPABILITY_ADMIN_ANTI_TAMPER_ROLLBACK: &str = "admin-anti-tamper-rollback";

pub const READINESS_IMPLEMENTED: &str = "implemented";
pub const READINESS_MANUAL_REQUIRED: &str = "manual-required";
pub const READINESS_UNAVAILABLE: &str = "unavailable";
pub const READINESS_NOT_CLAIMED: &str = "not-claimed";

pub const PROOF_REAL_SERVICE: &str = "real-service-proof";
pub const PROOF_CI_MECHANICAL: &str = "ci-mechanical-proof";
pub const PROOF_MANUAL_REQUIRED: &str = "manual-proof-required";
pub const PROOF_NOT_PROVED: &str = "not-proved";

pub const RUNTIME_OWNER_RUST_SERVICE: &str = "rust-service";
pub const RUNTIME_OWNER_OS_ADAPTER: &str = "os-adapter";
pub const RUNTIME_OWNER_MANAGED_BROWSER_BOUNDARY: &str = "managed-browser-boundary";
pub const RUNTIME_OWNER_MANUAL_PROOF: &str = "manual-proof";
pub const RUNTIME_OWNER_NOT_IMPLEMENTED: &str = "not-implemented";

pub const CLAIM_BOUNDARY_OWNED_PROCESS_TERMINATE: &str = "Only owned-process pid plus expected-process-name termination is proved; this is not global app blocking.";
pub const CLAIM_BOUNDARY_APP_TIME_LIMIT: &str = "App time-limit proof is tied to owned-process expiration, restart recovery, cancel, expiry, audit, and storage.";
pub const CLAIM_BOUNDARY_BROAD_APP_BLOCKING: &str = "Broad installed-app blocking is not proved by owned-process termination or app time-limit behavior.";
pub const CLAIM_BOUNDARY_NETWORK_DOMAIN_BLOCKING: &str = "Network flow metadata is not decrypted content and does not prove domain blocking enforcement.";
pub const CLAIM_BOUNDARY_MANAGED_BROWSER_SERVICE_COMMAND: &str =
    "A managed-browser service-command target string is not exact URL enforcement proof.";
pub const CLAIM_BOUNDARY_MANAGED_BROWSER_EXACT_URL: &str =
    "Exact URL, active tab, and page-title control require the managed browser boundary.";
pub const CLAIM_BOUNDARY_UNMANAGED_BROWSER_PROCESS_ONLY: &str = "Unmanaged browser proof is process-only and cannot become URL, tab, title, download, page, or intent evidence.";
pub const CLAIM_BOUNDARY_UNMANAGED_BROWSER_EXACT_EVIDENCE: &str = "Unmanaged browser process/window/network evidence does not prove exact URL, active tab, title, download source, page text, HTTPS content, or intent.";
pub const CLAIM_BOUNDARY_ADMIN_ANTI_TAMPER_ROLLBACK: &str = "Admin hardening, anti-tamper, bypass resistance, and broad rollback are not proved by V0.8 adapter tests.";

pub const FALLBACK_OWNED_PROCESS_TERMINATE: &str =
    "Reject missing pid/name mismatch and return unavailable on unsupported hosts.";
pub const FALLBACK_APP_TIME_LIMIT: &str = "Return unavailable when the active timer state or platform adapter cannot support the request.";
pub const FALLBACK_BROAD_APP_BLOCKING: &str = "Return manual-required or unavailable and avoid an adapter request until OS-approved proof exists.";
pub const FALLBACK_NETWORK_DOMAIN_BLOCKING: &str =
    "Return manual-required or unavailable until a host network control adapter has proof.";
pub const FALLBACK_MANAGED_BROWSER_SERVICE_COMMAND: &str =
    "Return manual-required or unavailable until managed browser command enforcement proof exists.";
pub const FALLBACK_MANAGED_BROWSER_EXACT_URL: &str = "Keep exact URL control manual-required unless managed browser evidence and enforcement proof are present.";
pub const FALLBACK_UNMANAGED_BROWSER_PROCESS_ONLY: &str =
    "Restrict control to pid/name guardrails and preserve exact browser evidence as not-claimed.";
pub const FALLBACK_UNMANAGED_BROWSER_EXACT_EVIDENCE: &str = "Use managed browser or another explicit browser integration before representing exact evidence.";
pub const FALLBACK_ADMIN_ANTI_TAMPER_ROLLBACK: &str =
    "Keep product claims manual-required until real host hardening and rollback evidence exists.";

pub const ARTIFACT_OS_APP_IDENTITY: &str = "OS-approved app/package identity proof";
pub const ARTIFACT_APP_BLOCK_ROLLBACK: &str = "installed-app block and rollback proof";
pub const ARTIFACT_NETWORK_FILTER: &str = "OS network filter adapter proof";
pub const ARTIFACT_DOMAIN_BLOCK_ROLLBACK: &str = "domain block apply and rollback proof";
pub const ARTIFACT_MANAGED_BROWSER_COMMAND: &str = "managed-browser command enforcement proof";
pub const ARTIFACT_EXACT_URL_APPLY_AUDIT: &str = "exact URL apply and audit proof";
pub const ARTIFACT_MANAGED_BROWSER_ACTIVE_TAB: &str = "managed browser active tab proof";
pub const ARTIFACT_MANAGED_EXACT_URL: &str = "managed exact URL enforcement artifact";
pub const ARTIFACT_BROWSER_INTEGRATION: &str =
    "managed browser or explicit browser integration proof";
pub const ARTIFACT_ADMIN_HARDENING: &str = "admin hardening proof";
pub const ARTIFACT_ANTI_TAMPER: &str = "anti-tamper proof";
pub const ARTIFACT_ROLLBACK_BYPASS: &str = "rollback and bypass-resistance proof";
