pub const COMMAND_GET: &str = "agent.enforcement.policy-dispatch.get";
pub const EVENT_REPORTED: &str = "agent.enforcement.policy-dispatch.reported";
pub const EVENT_ID_REPORTED: &str = "enforcement-policy-dispatch-reported";
pub const FIELD_READ_MODEL: &str = "enforcementPolicyDispatchReadModel";

pub const READ_MODEL_ID: &str = "v0-8-enforcement-policy-dispatch";
pub const GENERATED_AT: &str = "2026-06-02T05:45:00.000Z";
pub const LOCAL_DEV_AGENT_DEVICE_ID: &str = "local-dev-agent";
pub const LOCAL_DEV_AGENT_ROUTE_REF: &str = "route-localhost-agent-service";
pub const LOCAL_DEV_CHILD_PROFILE_ID: &str = "child-profile-v0-8-dispatch";
pub const LOCAL_DEV_CHILD_DEVICE_LABEL: &str = "Local dev child device";
pub const WINDOWS_PLATFORM: &str = "windows";
pub const PARENT_ACTOR_PRIMARY_ID: &str = "parent-actor-primary";
pub const POLICY_VERSION_V0_8_DISPATCH: &str = "policy-version-v0-8-dispatch";
pub const POLICY_VERSION_V0_8_DISPATCH_STALE: &str = "policy-version-v0-8-dispatch-stale";

pub const INTENT_OWNED_PROCESS_TIME_LIMIT: &str = "dispatch-owned-process-time-limit";
pub const INTENT_APP_GAME_SESSION_HANDOFF: &str = "dispatch-app-game-session-handoff";
pub const INTENT_ASK_PARENT_DRY_RUN: &str = "dispatch-ask-parent-dry-run";
pub const INTENT_UNMANAGED_BROWSER_REPORT_ONLY: &str = "dispatch-unmanaged-browser-report-only";
pub const INTENT_NETWORK_DOMAIN_MANUAL_REQUIRED: &str = "dispatch-network-domain-manual-required";
pub const INTENT_STALE_POLICY_VERSION_REJECTED: &str = "dispatch-stale-policy-version-rejected";
pub const INTENT_MISSING_SOURCE_REJECTED: &str = "dispatch-missing-source-rejected";
pub const INTENT_TAMPER_ALERT_SCAFFOLD: &str = "dispatch-tamper-alert-scaffold";

pub const MATRIX_OWNED_PROCESS_IMPLEMENTED: &str = "matrix-owned-process-implemented";
pub const MATRIX_APP_GAME_TIME_LIMIT_MANUAL_REQUIRED: &str =
    "matrix-app-game-time-limit-manual-required";
pub const MATRIX_ASK_PARENT_DRY_RUN: &str = "matrix-ask-parent-dry-run";
pub const MATRIX_UNMANAGED_BROWSER_REPORT_ONLY: &str = "matrix-unmanaged-browser-report-only";
pub const MATRIX_NETWORK_DOMAIN_MANUAL_REQUIRED: &str = "matrix-network-domain-manual-required";
pub const MATRIX_STALE_POLICY_VERSION_REJECTED: &str = "matrix-stale-policy-version-rejected";
pub const MATRIX_MISSING_SOURCE_REJECTED: &str = "matrix-missing-source-rejected";
pub const MATRIX_TAMPER_SCAFFOLD: &str = "matrix-tamper-scaffold";

pub const TARGET_OWNED_PROCESS_DEMO: &str = "owned-process:ocentra-child-demo.exe";
pub const TARGET_APP_GAME_LAUNCHER: &str = "app-session:game-launcher";
pub const TARGET_ASK_PARENT_REVIEW: &str = "app-session:ask-parent-review";
pub const TARGET_UNMANAGED_BROWSER_PROCESS: &str = "unmanaged-browser-process";
pub const TARGET_EXAMPLE_DOMAIN: &str = "example.invalid";
pub const TARGET_POLICY_SOURCE_MISSING: &str = "policy-source:missing";

pub const EVIDENCE_APP_SESSION_OWNED_PROCESS: &str = "evidence-app-session-owned-process";
pub const EVIDENCE_APP_GAME_SESSION_SUMMARY: &str = "evidence-app-game-session-summary";
pub const EVIDENCE_UNMANAGED_BROWSER_PROCESS: &str = "evidence-unmanaged-browser-process";
pub const EVIDENCE_NETWORK_FLOW_DOMAIN_SUMMARY: &str = "evidence-network-flow-domain-summary";
pub const EVIDENCE_POLICY_DECISION_STALE: &str = "evidence-policy-decision-stale";
pub const EVIDENCE_POLICY_SOURCE_MISSING: &str = "evidence-policy-source-missing";
pub const EVIDENCE_INTEGRITY_HEARTBEAT_GAP: &str = "evidence-integrity-heartbeat-gap";

pub const PREFIX_AUDIT: &str = "audit-";
pub const PREFIX_APPROVAL: &str = "approval-";
pub const PREFIX_DECISION: &str = "decision-";
pub const PREFIX_EVIDENCE: &str = "evidence-";
pub const PREFIX_INTENT: &str = "intent-";
pub const PREFIX_MATRIX: &str = "matrix-";
pub const PREFIX_POLICY: &str = "policy-";
pub const PREFIX_SCHEDULE: &str = "schedule-";
pub const PREFIX_TARGET: &str = "target-";
pub const PREFIX_TIMER: &str = "timer-";

pub const TEST_SUFFIX_DISPATCH_READY: &str = "dispatch-ready";
pub const TEST_SUFFIX_DRY_RUN_ONLY: &str = "dry-run-only";
pub const TEST_SUFFIX_MANUAL_REQUIRED: &str = "manual-required";
pub const TEST_SUFFIX_REPORT_ONLY: &str = "report-only";
pub const TEST_SUFFIX_STALE_POLICY_VERSION: &str = "stale-policy-version";
pub const TEST_SUFFIX_MISSING_POLICY_DECISION: &str = "missing-policy-decision";
pub const TEST_SUFFIX_MALFORMED_POLICY_DECISION: &str = "malformed-policy-decision";
pub const TEST_SUFFIX_WRONG_DEVICE: &str = "wrong-device";
pub const TEST_SUFFIX_MISSING_EVIDENCE: &str = "missing-evidence";
pub const TEST_DEVICE_OTHER_CHILD: &str = "other-child-device";
pub const TEST_MALFORMED_POLICY_DECISION_REF: &str = "malformed-dispatch-ref";

pub const SOURCE_READY: &str = "ready";
pub const SOURCE_UNAVAILABLE: &str = "unavailable";

pub const PROOF_IMPLEMENTED: &str = "implemented";
pub const PROOF_REPORT_ONLY: &str = "report-only";
pub const PROOF_MANUAL_REQUIRED: &str = "manual-required";
pub const PROOF_SCAFFOLD: &str = "scaffold";

pub const OUTCOME_DISPATCH_READY: &str = "dispatch-ready";
pub const OUTCOME_REPORT_ONLY: &str = "report-only";
pub const OUTCOME_MANUAL_REQUIRED: &str = "manual-required";
pub const OUTCOME_REJECTED: &str = "rejected";

pub const REJECTION_NONE: &str = "none";
pub const REJECTION_ADAPTER_MANUAL_REQUIRED: &str = "adapter-manual-required";
pub const REJECTION_STALE_POLICY_VERSION: &str = "stale-policy-version";
pub const REJECTION_SOURCE_NOT_READY: &str = "source-not-ready";
pub const REJECTION_BROAD_CLAIM_NOT_PROVED: &str = "broad-claim-not-proved";

pub const APPROVAL_NOT_REQUIRED: &str = "not-required";
pub const APPROVAL_PENDING: &str = "pending";
pub const APPROVAL_MANUAL_REQUIRED: &str = "manual-required";

pub const TIMER_NOT_REQUIRED: &str = "not-required";
pub const TIMER_ACTIVE: &str = "active";
pub const TIMER_RESTART_RECOVERED: &str = "restart-recovered";
pub const TIMER_RECOVERY_NEEDED: &str = "recovery-needed";

pub const CHILD_REASON_TIME_LIMIT: &str = "child-reason-time-limit-reached";
pub const CHILD_REASON_BONUS_TIME: &str = "child-reason-parent-approval-bonus-time";
pub const CHILD_REASON_ASK_PARENT_REVIEW: &str = "child-reason-ask-parent-review-required";
pub const CHILD_REASON_BROWSER_REPORT_ONLY: &str = "child-reason-browser-process-report-only";
pub const CHILD_REASON_MANUAL_REQUIRED: &str = "child-reason-adapter-manual-required";
pub const CHILD_REASON_STALE_POLICY_VERSION: &str = "child-reason-policy-version-stale";
pub const CHILD_REASON_SOURCE_NOT_READY: &str = "child-reason-source-not-ready";
pub const CHILD_REASON_INTEGRITY_PROOF: &str = "child-reason-integrity-proof-required";
