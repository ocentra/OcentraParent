pub const READ_MODEL_ID: &str = "v0-8-enforcement-product-control-spine";
pub const COMMAND_GET: &str = "agent.enforcement.product-control-spine.get";
pub const EVENT_REPORTED: &str = "agent.enforcement.product-control-spine.reported";

pub const ENTRY_ID_OWNED_PROCESS: &str = "v0-8-product-control-owned-process-time-limit";
pub const ENTRY_ID_APP_TIME_LIMIT: &str = "v0-8-product-control-app-time-limit";
pub const ENTRY_ID_MANAGED_BROWSER_SESSION: &str = "v0-8-product-control-managed-browser-session";
pub const ENTRY_ID_UNMANAGED_BROWSER_PROCESS: &str =
    "v0-8-product-control-unmanaged-browser-process";
pub const ENTRY_ID_POLICY_DRY_RUN: &str = "v0-8-product-control-policy-dry-run-preview";
pub const ENTRY_ID_APPROVAL_OVERRIDE: &str = "v0-8-product-control-approval-override-audit";
pub const ENTRY_ID_RESTART_RECOVERY: &str = "v0-8-product-control-restart-recovery-timer";
pub const ENTRY_ID_ROLLBACK_AUDIT: &str = "v0-8-product-control-rollback-audit";
pub const ENTRY_ID_CHILD_EXPLANATION: &str = "v0-8-product-control-child-facing-explanation";
pub const ENTRY_ID_BROAD_APP: &str = "v0-8-product-control-broad-app-blocking";
pub const ENTRY_ID_NETWORK_DOMAIN: &str = "v0-8-product-control-network-domain-blocking";
pub const ENTRY_ID_MANAGED_EXACT_URL: &str = "v0-8-product-control-managed-exact-url-control";
pub const ENTRY_ID_UNMANAGED_EXACT_URL: &str =
    "v0-8-product-control-unmanaged-exact-url-not-claimed";
pub const ENTRY_ID_PERMISSION_LOSS: &str = "v0-8-product-control-permission-loss-alerts";
pub const ENTRY_ID_TAMPER_UNINSTALL: &str = "v0-8-product-control-tamper-uninstall-alerts";

pub const SURFACE_OWNED_PROCESS: &str = "windows-owned-process-time-limit";
pub const SURFACE_APP_TIME_LIMIT: &str = "windows-app-time-limit-lifecycle";
pub const SURFACE_MANAGED_BROWSER_SESSION: &str = "windows-managed-browser-session-intervention";
pub const SURFACE_UNMANAGED_BROWSER_PROCESS: &str = "windows-unmanaged-browser-process-fallback";
pub const SURFACE_POLICY_DRY_RUN: &str = "windows-policy-dry-run-preview";
pub const SURFACE_APPROVAL_OVERRIDE: &str = "windows-approval-override-audit";
pub const SURFACE_RESTART_RECOVERY: &str = "windows-restart-recovery-timer";
pub const SURFACE_ROLLBACK_AUDIT: &str = "windows-rollback-audit-boundary";
pub const SURFACE_CHILD_EXPLANATION: &str = "windows-child-facing-explanation";
pub const SURFACE_BROAD_APP: &str = "windows-broad-app-blocking";
pub const SURFACE_NETWORK_DOMAIN: &str = "windows-network-domain-blocking";
pub const SURFACE_MANAGED_EXACT_URL: &str = "windows-managed-exact-url-control";
pub const SURFACE_UNMANAGED_EXACT_URL: &str = "windows-unmanaged-exact-url-not-claimed";
pub const SURFACE_PERMISSION_LOSS: &str = "windows-permission-loss-alerts";
pub const SURFACE_TAMPER_UNINSTALL: &str = "windows-tamper-uninstall-alerts";

pub const KIND_PROCESS: &str = "process";
pub const KIND_APP_GAME: &str = "app-game";
pub const KIND_MANAGED_BROWSER: &str = "managed-browser";
pub const KIND_UNMANAGED_BROWSER: &str = "unmanaged-browser";
pub const KIND_NETWORK_DOMAIN: &str = "network-domain";
pub const KIND_POLICY: &str = "policy";
pub const KIND_RECOVERY: &str = "recovery";
pub const KIND_AUDIT: &str = "audit";
pub const KIND_CHILD_EXPLANATION: &str = "child-explanation";
pub const KIND_INTEGRITY: &str = "integrity";

pub const CAPABILITY_OWNED_PROCESS_TERMINATE: &str = "owned-process-terminate";
pub const CAPABILITY_APP_TIME_LIMIT: &str = "app-time-limit";
pub const CAPABILITY_APP_BLOCKING: &str = "app-blocking";
pub const CAPABILITY_NETWORK_DOMAIN_BLOCKING: &str = "network-domain-blocking";
pub const CAPABILITY_MANAGED_BROWSER_CONTROL: &str = "managed-browser-control";
pub const CAPABILITY_UNMANAGED_BROWSER_DETECTION: &str = "unmanaged-browser-detection";
pub const CAPABILITY_TYPED_PROTOCOL_BRIDGE: &str = "typed-protocol-bridge";
pub const CAPABILITY_NOTIFICATIONS: &str = "notifications";
pub const CAPABILITY_PACKAGE_LIFECYCLE: &str = "package-lifecycle";

pub const STATUS_IMPLEMENTED: &str = "implemented";
pub const STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const STATUS_NOT_IMPLEMENTED: &str = "not-implemented";

pub const CLAIM_IMPLEMENTED_BOUNDARY: &str = "implemented-boundary";
pub const CLAIM_DEGRADED_BOUNDARY: &str = "degraded-boundary";
pub const CLAIM_DRY_RUN_ONLY: &str = "dry-run-only";
pub const CLAIM_MANUAL_REQUIRED: &str = "manual-required";
pub const CLAIM_UNAVAILABLE: &str = "unavailable";
pub const CLAIM_NOT_CLAIMED: &str = "not-claimed";

pub const EXECUTES_REAL_SERVICE: &str = "executes-real-service";
pub const RETURNS_DRY_RUN_PREVIEW: &str = "returns-dry-run-preview";
pub const RETURNS_DEGRADED_NOOP: &str = "returns-degraded-noop";
pub const RETURNS_MANUAL_REQUIRED: &str = "returns-manual-required";
pub const RETURNS_UNAVAILABLE: &str = "returns-unavailable";
pub const NOT_INVOKED: &str = "not-invoked";

pub const DEVICE_POLICY_CONTROL_CAPABLE: &str = "control-capable";
pub const DEVICE_POLICY_PREVIEW_ONLY: &str = "preview-only";
pub const DEVICE_POLICY_REPORT_ONLY: &str = "report-only";
pub const DEVICE_POLICY_MANUAL_REQUIRED: &str = "manual-required";
pub const DEVICE_POLICY_UNAVAILABLE: &str = "unavailable";
pub const DEVICE_POLICY_NOT_CLAIMED: &str = "not-claimed";

pub const ACTION_OBSERVE: &str = "observe";
pub const ACTION_WARN: &str = "warn";
pub const ACTION_TIME_LIMIT: &str = "time-limit";
pub const ACTION_BLOCK_SCOPED_PROCESS: &str = "block-scoped-process";
pub const ACTION_ASK_PARENT: &str = "ask-parent";
pub const ACTION_DRY_RUN_PREVIEW: &str = "dry-run-preview";
pub const ACTION_REPORT_ONLY: &str = "report-only";

pub const SOURCE_CROSS_PLATFORM_CAPABILITY: &str =
    "v0-8-cross-platform-enforcement-capability-proof";
pub const SOURCE_BROWSER_DOMAIN: &str = "v0-8-browser-domain-adapter-proof";
pub const SOURCE_OS_ADAPTER_PRODUCT: &str = "v0-8-os-adapter-product-proof";
pub const SOURCE_BROWSER_POLICY_PREVIEW: &str = "browser-policy-preview";

pub const COMMAND_WINDOWS_UNMANAGED_PROOF: &str =
    "node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs";
pub const COMMAND_WINDOWS_TIMER_PROOF: &str =
    "node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs";
pub const COMMAND_MANAGED_BROWSER_PROOF: &str =
    "node scripts/test/managed-browser-intervention-proof.mjs";
pub const COMMAND_POLICY_PREVIEW: &str =
    "cargo test -p ocentra-parent-agent-service policy_preview";
pub const COMMAND_PROTOCOL_ENFORCEMENT: &str =
    "cargo test -p ocentra-parent-agent-protocol enforcement";
pub const COMMAND_BROWSER_POLICY_ROLLBACK: &str = "cargo test -p ocentra-parent-agent-service browser_policy_rollback_restores_earlier_persisted_revision";

pub const ARTIFACT_WINDOWS_UNMANAGED_PROOF: &str =
    "test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json";
pub const ARTIFACT_WINDOWS_TIMER_PROOF: &str =
    "test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json";
pub const ARTIFACT_MANAGED_BROWSER_PROOF: &str =
    "test-results/managed-browser-intervention-proof/proof.json";
pub const ARTIFACT_POLICY_PREVIEW: &str = "test-results/policy-preview-proof/proof.json";
pub const ARTIFACT_PRODUCT_PROOF: &str =
    "test-results/enforcement-lan-mobile-product-proof/proof.json";
pub const ARTIFACT_BROWSER_DOMAIN_PROOF: &str =
    "test-results/v0-8-browser-domain-adapter-proof/proof.json";

pub const REQUIREMENT_CHILD_EXPLANATION_COPY: &str = "child-facing status copy contract";
pub const REQUIREMENT_CHILD_EXPLANATION_DELIVERY: &str = "delivery surface proof";
pub const REQUIREMENT_CHILD_EXPLANATION_AUDIT: &str =
    "audit link from explanation to policy decision";
pub const REQUIREMENT_OS_APP_IDENTITY: &str = "OS-approved installed-app identity";
pub const REQUIREMENT_BLOCK_APPLY: &str = "block apply result";
pub const REQUIREMENT_ROLLBACK: &str = "rollback result";
pub const REQUIREMENT_AUDIT_CUSTODY: &str = "audit custody artifact";
pub const REQUIREMENT_NETWORK_FILTER: &str = "host DNS/VPN/filter adapter";
pub const REQUIREMENT_DOMAIN_APPLY: &str = "domain filter apply result";
pub const REQUIREMENT_ACTIVE_TAB: &str = "active-tab evidence proof";
pub const REQUIREMENT_EXACT_URL_APPLY: &str = "exact URL apply result";
pub const REQUIREMENT_UNMANAGED_INTEGRATION: &str =
    "explicit unmanaged browser integration proof before exact URL evidence";
pub const REQUIREMENT_NOTIFICATION_PROVIDER: &str = "notification delivery provider";
pub const REQUIREMENT_PERMISSION_DETECTOR: &str = "permission-loss detector";
pub const REQUIREMENT_DELIVERY_RECEIPT: &str = "parent-visible delivery receipt";
pub const REQUIREMENT_TAMPER_DESIGN: &str = "explicit tamper/uninstall product design";
pub const REQUIREMENT_REMOVAL_DETECTOR: &str = "service removal detector";
pub const REQUIREMENT_NON_STEALTH_ALERT: &str = "non-stealth parent alert proof";

pub const CLAIM_POLICY_DRY_RUN: &str = "Policy preview is dry-run-only and must not execute adapter behavior from portal-authored rules.";
pub const FALLBACK_POLICY_DRY_RUN: &str = "Return preview-only state until a child-device agent validates and executes a typed policy decision.";
pub const CLAIM_APPROVAL_OVERRIDE: &str = "Approval and override audit references are typed control state, not portal-side enforcement authority.";
pub const FALLBACK_APPROVAL_OVERRIDE: &str = "Reject stale or missing approval references and preserve audit-only state when execution is unavailable.";
pub const CLAIM_CHILD_EXPLANATION: &str = "Child-facing explanation remains manual-required until the child device can show policy reason and request flow state.";
pub const FALLBACK_CHILD_EXPLANATION: &str =
    "Report parent-visible audit state only until child delivery and acknowledgement are proved.";
pub const CLAIM_PERMISSION_LOSS: &str =
    "Permission-loss alerts remain manual-required until detector and delivery status are proved.";
pub const FALLBACK_PERMISSION_LOSS: &str =
    "Report local status only until notification delivery and acknowledgement proof exist.";
pub const CLAIM_TAMPER_UNINSTALL: &str = "Tamper/uninstall alerts remain manual-required and do not imply stealth or persistence hardening.";
pub const FALLBACK_TAMPER_UNINSTALL: &str =
    "Report manual-required until product/security design and non-stealth alert proof exist.";
