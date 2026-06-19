pub const READ_MODEL_ID: &str = "v0-8-enforcement-integrity-runtime-audit";

pub const SOURCE_SUPPORTED_ADAPTER_RUNTIME_PROOF: &str = "v0-8-supported-adapter-runtime-proof";
pub const SOURCE_POLICY_DISPATCH_PROOF: &str = "v0-8-enforcement-policy-dispatch-proof";
pub const SOURCE_PRODUCT_CONTROL_SPINE: &str = "v0-8-enforcement-product-control-spine";
pub const SOURCE_ENFORCEMENT_AUDIT_JOURNAL: &str = "enforcement-audit-journal";
pub const SOURCE_TIMER_RECOVERY_STATE: &str = "enforcement-timer-recovery-state";
pub const SOURCE_NOTIFICATION_PROVIDER_STATUS_BOUNDARY: &str =
    "v0-8-notification-provider-status-boundary";

pub const ENTRY_APP_TIME_LIMIT_SUCCEEDED: &str = "app-time-limit-action-succeeded";
pub const ENTRY_APP_TIME_LIMIT_EXPIRED: &str = "app-time-limit-action-expired";
pub const ENTRY_APP_TIME_LIMIT_ROLLED_BACK: &str = "app-time-limit-action-rolled-back";
pub const ENTRY_PARENT_OVERRIDE_SUPERSEDED: &str = "parent-override-superseded-action";
pub const ENTRY_DRY_RUN_NO_OP: &str = "dry-run-preview-no-op";
pub const ENTRY_STALE_POLICY_REJECTED: &str = "stale-policy-decision-rejected";
pub const ENTRY_WRONG_DEVICE_REJECTED: &str = "wrong-device-intent-rejected";
pub const ENTRY_NETWORK_OBSERVE_ONLY: &str = "network-domain-observe-only";
pub const ENTRY_HOST_NETWORK_MANUAL: &str = "host-network-domain-filter-manual-required";
pub const ENTRY_PERMISSION_LOSS: &str = "permission-loss-unavailable";
pub const ENTRY_ADAPTER_UNAVAILABLE: &str = "adapter-unavailable-recovery-needed";
pub const ENTRY_STALE_HEARTBEAT: &str = "stale-integrity-heartbeat";
pub const ENTRY_MOBILE_UNSUPPORTED: &str = "mobile-child-control-unsupported";
pub const ENTRY_TAMPER_MANUAL: &str = "tamper-uninstall-detection-manual-required";

pub const SURFACE_APP_GAME_TIME_LIMIT: &str = "app-game-time-limit";
pub const SURFACE_NETWORK_OBSERVE_ONLY: &str = "network-domain-observe-only";
pub const SURFACE_HOST_NETWORK_FILTER: &str = "host-network-domain-filter";
pub const SURFACE_INTEGRITY_HEARTBEAT: &str = "integrity-heartbeat";
pub const SURFACE_TAMPER_UNINSTALL_SIGNAL: &str = "tamper-uninstall-signal";
pub const SURFACE_MOBILE_CHILD_CONTROL: &str = "mobile-child-control";

pub const RESULT_SUCCEEDED: &str = "succeeded";
pub const RESULT_FAILED: &str = "failed";
pub const RESULT_UNAVAILABLE: &str = "unavailable";
pub const RESULT_EXPIRED: &str = "expired";
pub const RESULT_ROLLED_BACK: &str = "rolled-back";
pub const RESULT_SUPERSEDED: &str = "superseded";
pub const RESULT_NO_OP: &str = "no-op";
pub const RESULT_MANUAL_REQUIRED: &str = "manual-required";
pub const RESULT_UNSUPPORTED: &str = "unsupported";
pub const RESULT_OBSERVE_ONLY: &str = "observe-only";

pub const EXECUTION_EXECUTED_SUPPORTED_BOUNDARY: &str = "executed-supported-boundary";
pub const EXECUTION_DRY_RUN_NO_ADAPTER: &str = "dry-run-no-adapter-execution";
pub const EXECUTION_REJECTED_BEFORE_ADAPTER: &str = "rejected-before-adapter";
pub const EXECUTION_MANUAL_REQUIRED_NO_EXECUTION: &str = "manual-required-no-execution";
pub const EXECUTION_OBSERVE_ONLY_NO_EXECUTION: &str = "observe-only-no-execution";
pub const EXECUTION_UNAVAILABLE_NO_EXECUTION: &str = "unavailable-no-execution";
pub const EXECUTION_UNSUPPORTED_NO_EXECUTION: &str = "unsupported-no-execution";
pub const EXECUTION_RECOVERY_NEEDED_NO_EXECUTION: &str = "recovery-needed-no-execution";

pub const INTEGRITY_RUNNING: &str = "running";
pub const INTEGRITY_PERMISSION_MISSING: &str = "permission-missing";
pub const INTEGRITY_ADAPTER_UNAVAILABLE: &str = "adapter-unavailable";
pub const INTEGRITY_STALE_HEARTBEAT: &str = "stale-heartbeat";
pub const INTEGRITY_SERVICE_STOPPED: &str = "service-stopped";
pub const INTEGRITY_UNINSTALL_DETECTION_MANUAL_REQUIRED: &str =
    "uninstall-detection-manual-required";
pub const INTEGRITY_TAMPER_SIGNAL_MANUAL_REQUIRED: &str = "tamper-signal-manual-required";
pub const INTEGRITY_ANTI_TAMPER_NOT_CLAIMED: &str = "anti-tamper-not-claimed";
pub const INTEGRITY_NOT_APPLICABLE: &str = "not-applicable";

pub const REF_POLICY_DECISION: &str = "policy-decision-ref";
pub const REF_ENFORCEMENT_AUDIT: &str = "enforcement-audit-ref";
pub const REF_CHILD_STATUS: &str = "child-status-ref";
pub const REF_CHILD_STATUS_STALE_POLICY: &str = "child-status-stale-policy-ref";
pub const REF_CHILD_STATUS_WRONG_DEVICE: &str = "child-status-wrong-device-ref";
pub const REF_CHILD_STATUS_UNAVAILABLE: &str = "child-status-unavailable-ref";
pub const REF_INTEGRITY_HEARTBEAT: &str = "integrity-heartbeat-ref";
pub const REF_INTEGRITY_STATE: &str = "integrity-state-ref";
pub const REF_TIMER_STATE: &str = "timer-state-ref";
pub const REF_TIMER_RECOVERY_NEEDED: &str = "timer-recovery-needed-ref";
pub const REF_APP_SESSION_EVIDENCE: &str = "app-session-evidence-ref";
pub const REF_OWNED_PROCESS_IDENTITY: &str = "owned-process-identity-ref";
pub const REF_ADAPTER_OUTCOME: &str = "adapter-outcome-ref";
pub const REF_NETWORK_FLOW_SUMMARY: &str = "network-flow-summary-ref";
pub const REF_PARENT_OVERRIDE_INTENT: &str = "parent-override-intent-ref";
pub const REF_POLICY_PREVIEW: &str = "policy-preview-ref";
pub const REF_ROLLBACK_TOKEN: &str = "rollback-token-ref";

pub const REQUIREMENT_HOST_DNS_OR_FILTER_APPLY: &str = "host DNS or filter apply artifact";
pub const REQUIREMENT_HOST_FILTER_ROLLBACK: &str = "host filter rollback artifact";
pub const REQUIREMENT_PERMISSION_RESTORE: &str = "permission restoration artifact";
pub const REQUIREMENT_OPERATOR_PERMISSION_STATE: &str = "operator-visible permission state";
pub const REQUIREMENT_ADAPTER_RECOVERY: &str = "adapter recovery artifact";
pub const REQUIREMENT_SERVICE_RESTART_RECOVERY: &str = "service restart recovery proof";
pub const REQUIREMENT_FRESH_HEARTBEAT: &str = "fresh heartbeat proof";
pub const REQUIREMENT_PARENT_VISIBLE_STALE_ALERT: &str = "parent-visible stale agent alert";
pub const REQUIREMENT_IOS_FAMILY_CONTROLS: &str = "Family Controls entitlement artifact";
pub const REQUIREMENT_IOS_DEVICE_ACTIVITY: &str = "DeviceActivity proof artifact";
pub const REQUIREMENT_SERVICE_MANAGER_STOP_PROOF: &str = "service-manager stop proof";
pub const REQUIREMENT_UNINSTALL_DETECTION_ARTIFACT: &str = "uninstall detection artifact";
pub const REQUIREMENT_SECURITY_REVIEW: &str = "security review before hardening";

pub const BOUNDARY_APP_TIME_LIMIT_SUCCEEDED: &str = "Owned-process app/game time-limit actions can execute only with policy, evidence, timer, rollback, child-reason, and audit references.";
pub const BOUNDARY_APP_TIME_LIMIT_EXPIRED: &str = "Expiry is audit-backed by timer state and child-facing status refs; it does not imply broad app blocking.";
pub const BOUNDARY_APP_TIME_LIMIT_ROLLED_BACK: &str = "Rollback is a typed supported-boundary state with rollback refs and audit refs, not an implicit unblock claim for unproved adapters.";
pub const BOUNDARY_PARENT_OVERRIDE_SUPERSEDED: &str = "Parent override supersedes a validated action through auditable intent refs owned by the agent runtime.";
pub const BOUNDARY_DRY_RUN_NO_OP: &str =
    "Dry-run and observe previews are audit-visible no-ops and must not execute adapters.";
pub const BOUNDARY_STALE_POLICY_REJECTED: &str = "Stale policy decisions reject before adapter execution and keep a child-facing reason/status ref.";
pub const BOUNDARY_WRONG_DEVICE_REJECTED: &str =
    "Wrong-device intents reject before adapter execution and remain auditable.";
pub const BOUNDARY_NETWORK_OBSERVE_ONLY: &str = "Network/domain runtime state is observe-only over stored flow evidence; host DNS/filter enforcement is not executed.";
pub const BOUNDARY_HOST_NETWORK_MANUAL: &str = "Host network/domain filtering remains manual-required until apply, rollback, and audit artifacts exist.";
pub const BOUNDARY_PERMISSION_LOSS: &str = "Permission loss is explicit unavailable state and must not be reported as enforcement success.";
pub const BOUNDARY_ADAPTER_UNAVAILABLE: &str = "Adapter recovery is explicit recovery-needed/unavailable state where persisted restart recovery is not proved.";
pub const BOUNDARY_STALE_HEARTBEAT: &str =
    "Stale heartbeat is parent-visible degraded integrity state, not anti-tamper hardening.";
pub const BOUNDARY_MOBILE_UNSUPPORTED: &str =
    "Mobile child control remains unsupported without platform entitlement and device proof.";
pub const BOUNDARY_TAMPER_MANUAL: &str = "Tamper/uninstall is represented as detectable/manual-required state only; no stealth, persistence, or anti-tamper hardening is claimed.";
