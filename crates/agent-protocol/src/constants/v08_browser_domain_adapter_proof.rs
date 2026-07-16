pub const READ_MODEL_ID: &str = "v0-8-browser-domain-adapter-proof";

pub const ENTRY_ID_MANAGED_INTERVENTION: &str = "v0-8-browser-domain-managed-intervention-state";
pub const ENTRY_ID_MANAGED_EXACT_URL: &str = "v0-8-browser-domain-managed-exact-url-manual";
pub const ENTRY_ID_UNMANAGED_TERMINATE: &str = "v0-8-browser-domain-unmanaged-terminate-boundary";
pub const ENTRY_ID_UNMANAGED_WARN: &str = "v0-8-browser-domain-unmanaged-warn-noop";
pub const ENTRY_ID_UNMANAGED_EXACT_EVIDENCE: &str =
    "v0-8-browser-domain-unmanaged-exact-evidence-not-claimed";
pub const ENTRY_ID_NETWORK_FILTER_MANUAL: &str = "v0-8-browser-domain-network-filter-manual";
pub const ENTRY_ID_NETWORK_ADAPTER_UNAVAILABLE: &str =
    "v0-8-browser-domain-network-adapter-unavailable";
pub const ENTRY_ID_AUDIT_VISIBILITY: &str = "v0-8-browser-domain-audit-visibility";
pub const ENTRY_ID_RESTART_RECOVERY: &str = "v0-8-browser-domain-restart-recovery-visibility";
pub const ENTRY_ID_BROWSER_POLICY_ROLLBACK: &str =
    "v0-8-browser-domain-browser-policy-rollback-visibility";
pub const ENTRY_ID_LINUX_ADAPTER: &str = "v0-8-browser-domain-linux-adapter-unavailable";
pub const ENTRY_ID_MACOS_ADAPTER: &str = "v0-8-browser-domain-macos-adapter-unavailable";
pub const ENTRY_ID_ANDROID_ADAPTER: &str = "v0-8-browser-domain-android-adapter-manual";
pub const ENTRY_ID_IOS_ADAPTER: &str = "v0-8-browser-domain-ios-adapter-manual";

pub const STATE_ID_APP_CONTROL_READINESS: &str = "v0-8-windows-app-control-readiness-detect-only";
pub const STATE_ID_APP_CONTROL_AUDIT_ONLY: &str = "v0-8-windows-app-control-audit-only-visible";
pub const STATE_ID_APP_CONTROL_ENFORCED: &str = "v0-8-windows-app-control-enforced-manual-required";
pub const STATE_ID_APP_CONTROL_MANUAL_REQUIRED: &str = "v0-8-windows-app-control-manual-required";
pub const STATE_ID_APP_CONTROL_UNAVAILABLE: &str = "v0-8-windows-app-control-unavailable";
pub const STATE_ID_APP_CONTROL_FAILED: &str = "v0-8-windows-app-control-policy-failed";

pub const SURFACE_MANAGED_INTERVENTION: &str = "windows-managed-browser-intervention-state";
pub const SURFACE_MANAGED_EXACT_URL: &str = "windows-managed-browser-exact-url-manual";
pub const SURFACE_UNMANAGED_TERMINATE: &str = "windows-unmanaged-browser-terminate-boundary";
pub const SURFACE_UNMANAGED_WARN: &str = "windows-unmanaged-browser-warn-noop";
pub const SURFACE_UNMANAGED_EXACT_EVIDENCE: &str =
    "windows-unmanaged-browser-exact-evidence-not-claimed";
pub const SURFACE_NETWORK_FILTER_MANUAL: &str = "windows-network-domain-filter-manual";
pub const SURFACE_NETWORK_ADAPTER_UNAVAILABLE: &str = "windows-network-domain-adapter-unavailable";
pub const SURFACE_AUDIT_VISIBILITY: &str = "windows-audit-visibility-boundary";
pub const SURFACE_RESTART_RECOVERY: &str = "windows-restart-recovery-visibility-boundary";
pub const SURFACE_BROWSER_POLICY_ROLLBACK: &str = "windows-browser-policy-rollback-visibility";
pub const SURFACE_LINUX_ADAPTER: &str = "linux-browser-domain-adapter-unavailable";
pub const SURFACE_MACOS_ADAPTER: &str = "macos-browser-domain-adapter-unavailable";
pub const SURFACE_ANDROID_ADAPTER: &str = "android-browser-domain-adapter-manual";
pub const SURFACE_IOS_ADAPTER: &str = "ios-browser-domain-adapter-manual";

pub const CAPABILITY_APP_TIME_LIMIT: &str = "app-time-limit";
pub const CAPABILITY_LOCAL_STORAGE: &str = "local-storage";
pub const CAPABILITY_MANAGED_BROWSER_CONTROL: &str = "managed-browser-control";
pub const CAPABILITY_NETWORK_DOMAIN_BLOCKING: &str = "network-domain-blocking";
pub const CAPABILITY_NETWORK_EXTENSION: &str = "network-extension";
pub const CAPABILITY_UNMANAGED_BROWSER_DETECTION: &str = "unmanaged-browser-detection";
pub const CAPABILITY_VPN_DNS_FILTERING: &str = "vpn-dns-filtering";

pub const STATUS_IMPLEMENTED: &str = "implemented";
pub const STATUS_SUPPORTED: &str = "supported";
pub const STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const STATUS_UNAVAILABLE: &str = "unavailable";
pub const STATUS_NOT_IMPLEMENTED: &str = "not-implemented";

pub const EVIDENCE_MANAGED_BROWSER: &str = "managed-browser";
pub const EVIDENCE_UNMANAGED_BROWSER: &str = "unmanaged-browser";
pub const EVIDENCE_NETWORK_DOMAIN: &str = "network-domain";
pub const EVIDENCE_AUDIT: &str = "audit";
pub const EVIDENCE_RESTART_RECOVERY: &str = "restart-recovery";
pub const EVIDENCE_ROLLBACK: &str = "rollback";
pub const EVIDENCE_UNSUPPORTED_TARGET: &str = "unsupported-target";

pub const CLAIM_IMPLEMENTED_BOUNDARY: &str = "implemented-boundary";
pub const CLAIM_DEGRADED_BOUNDARY: &str = "degraded-boundary";
pub const CLAIM_MANUAL_REQUIRED: &str = "manual-required";
pub const CLAIM_UNAVAILABLE: &str = "unavailable";
pub const CLAIM_NOT_CLAIMED: &str = "not-claimed";

pub const EXECUTES_REAL_SERVICE: &str = "executes-real-service";
pub const RETURNS_DEGRADED_NOOP: &str = "returns-degraded-noop";
pub const RETURNS_MANUAL_REQUIRED: &str = "returns-manual-required";
pub const RETURNS_UNAVAILABLE: &str = "returns-unavailable";
pub const NOT_INVOKED: &str = "not-invoked";

pub const APP_CONTROL_READINESS_CHECK: &str = "readiness-check";
pub const APP_CONTROL_AUDIT_ONLY: &str = "audit-only";
pub const APP_CONTROL_ENFORCED: &str = "enforced";
pub const APP_CONTROL_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_CONTROL_UNAVAILABLE: &str = "unavailable";
pub const APP_CONTROL_FAILED: &str = "failed";

pub const APP_CONTROL_POLICY_DETECT_ONLY: &str = "detect-only";
pub const APP_CONTROL_POLICY_AUDIT_ONLY_VISIBLE: &str = "audit-only-visible";
pub const APP_CONTROL_POLICY_CREATE_UPDATE_MANUAL_REQUIRED: &str = "create-update-manual-required";
pub const APP_CONTROL_POLICY_MANUAL_SETUP_REQUIRED: &str = "manual-setup-required";
pub const APP_CONTROL_POLICY_UNAVAILABLE: &str = "unavailable";
pub const APP_CONTROL_POLICY_FAILED: &str = "failed";

pub const APP_CONTROL_IDENTITY_PUBLISHER: &str = "publisher";
pub const APP_CONTROL_IDENTITY_PATH: &str = "path";
pub const APP_CONTROL_IDENTITY_HASH: &str = "hash";
pub const APP_CONTROL_IDENTITY_PACKAGE: &str = "package";

pub const APP_CONTROL_ADMINISTRATOR_REQUIRED: &str = "administrator-required";
pub const APP_CONTROL_SERVICE_PERMISSION_REQUIRED: &str = "service-permission-required";
pub const APP_CONTROL_MANUAL_OPERATOR_REQUIRED: &str = "manual-operator-required";
pub const APP_CONTROL_ADMIN_NOT_APPLICABLE: &str = "not-applicable";

pub const APP_CONTROL_EVENT_AUDIT_VISIBLE: &str = "audit-visible";
pub const APP_CONTROL_EVENT_ROLLBACK_VISIBLE: &str = "rollback-visible";
pub const APP_CONTROL_EVENT_FAILURE_VISIBLE: &str = "failure-visible";
pub const APP_CONTROL_EVENT_MANUAL_PROOF_REQUIRED: &str = "manual-proof-required";
pub const APP_CONTROL_EVENT_UNAVAILABLE: &str = "unavailable";

pub const SOURCE_BROAD_OS_PROOF: &str = "v0-8-broad-os-adapter-proof";
pub const SOURCE_CROSS_PLATFORM_PROOF: &str = "v0-8-cross-platform-enforcement-capability-proof";
pub const SOURCE_OS_PRODUCT_PROOF: &str = "v0-8-os-adapter-product-proof";
pub const SOURCE_BROWSER_POLICY_RUNTIME: &str = "browser-policy-runtime";

pub const COMMAND_MANAGED_BROWSER_PROOF: &str =
    "node scripts/test/managed-browser-intervention-proof.mjs";
pub const COMMAND_UNMANAGED_BROWSER_PROOF: &str =
    "node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs";
pub const COMMAND_APP_TIME_LIMIT_PROOF: &str =
    "node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs";
pub const COMMAND_BROWSER_POLICY_ROLLBACK_TEST: &str = "cargo test -p ocentra-parent-agent-service browser_policy_rollback_restores_earlier_persisted_revision";

pub const ARTIFACT_MANAGED_BROWSER_PROOF: &str =
    "test-results/managed-browser-intervention-proof/proof.json";
pub const ARTIFACT_UNMANAGED_BROWSER_PROOF: &str =
    "test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json";
pub const ARTIFACT_APP_TIME_LIMIT_PROOF: &str =
    "test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json";
pub const ARTIFACT_BROWSER_POLICY_ROLLBACK_TEST: &str = "crates/agent-protocol/tests/unit/mod.rs";
pub const ARTIFACT_UNMANAGED_WARN_EVENT: &str = "unmanaged browser warning no-op service event";

pub const REQUIREMENT_ACTIVE_TAB: &str = "managed active-tab evidence";
pub const REQUIREMENT_EXACT_URL_APPLY: &str = "exact URL apply result";
pub const REQUIREMENT_ROLLBACK: &str = "rollback result";
pub const REQUIREMENT_AUDIT_CUSTODY: &str = "audit custody artifact";
pub const REQUIREMENT_WARNING_DELIVERY: &str = "parent-visible warning delivery proof";
pub const REQUIREMENT_BROWSER_INTEGRATION: &str = "browser integration proof";
pub const REQUIREMENT_MANAGED_PROFILE: &str = "managed profile integration";
pub const REQUIREMENT_BROWSER_EXTENSION: &str = "browser extension or protocol integration";
pub const REQUIREMENT_ACTIVE_TAB_CUSTODY: &str = "active tab custody evidence";
pub const REQUIREMENT_NETWORK_FILTER: &str = "host network filter adapter";
pub const REQUIREMENT_DNS_VPN_APPLY: &str = "DNS or VPN apply result";
pub const REQUIREMENT_SERVICE_UNAVAILABLE: &str = "service unavailable event";
pub const REQUIREMENT_ADAPTER_INSTALL: &str = "adapter install evidence";
pub const REQUIREMENT_OPERATOR_RETRY: &str = "operator retry path";
pub const REQUIREMENT_LINUX_SERVICE: &str = "Linux service-manager proof";
pub const REQUIREMENT_LINUX_ADAPTER: &str = "Linux browser/domain adapter proof";
pub const REQUIREMENT_MACOS_PERMISSION: &str = "macOS permission proof";
pub const REQUIREMENT_MACOS_ADAPTER: &str = "macOS browser/domain adapter proof";
pub const REQUIREMENT_ANDROID_VPN_DNS: &str = "Android VPN or DNS filtering proof";
pub const REQUIREMENT_ANDROID_DEVICE_OWNER: &str = "device-owner or managed-profile proof";
pub const REQUIREMENT_ANDROID_PACKAGE: &str = "package lifecycle proof";
pub const REQUIREMENT_IOS_NETWORK_EXTENSION: &str = "Network Extension entitlement proof";
pub const REQUIREMENT_IOS_FAMILY_DEVICE: &str = "Family Controls or DeviceActivity proof";
pub const REQUIREMENT_IOS_TESTFLIGHT: &str = "TestFlight or device artifact";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_EDITION: &str =
    "Windows edition and AppLocker or WDAC availability proof";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_ADMIN: &str = "administrator permission proof";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_POLICY: &str = "audit-mode policy artifact";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_QUERY: &str =
    "AppLocker or WDAC audit event query proof";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_ENFORCED_POLICY: &str =
    "AppLocker or WDAC enforced policy apply artifact";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_POLICY_REFRESH: &str = "policy refresh result";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_MANUAL_SETUP: &str = "parent-visible manual setup state";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_OPERATOR_CONFIRMATION: &str =
    "operator confirmation path";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_IDENTITY_REVIEW: &str = "identity target review proof";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_PROVIDER: &str =
    "unsupported Windows edition or missing policy provider proof";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_PERMISSION_DENIAL: &str =
    "service permission denial event";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_FAILURE_EVENT: &str =
    "policy create or update failure event";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_IDENTITY_FAILURE: &str =
    "policy target identity failure event";
pub const REQUIREMENT_WINDOWS_APP_CONTROL_AUDIT_FAILURE: &str = "audit failure event";

pub const CLAIM_MANAGED_INTERVENTION: &str = "Managed browser intervention state is limited to the Ocentra-owned managed-session boundary and does not prove exact active-tab URL enforcement.";
pub const CLAIM_MANAGED_EXACT_URL: &str = "Managed exact URL enforcement remains manual-required because a command target string is not foreground active-tab proof.";
pub const CLAIM_UNMANAGED_TERMINATE: &str = "Unmanaged browser terminate proof is process-only with pid/name guardrails and is not exact URL, tab, title, page, download, or intent evidence.";
pub const CLAIM_UNMANAGED_WARN: &str = "Unmanaged browser warn behavior is a degraded no-op boundary until notification delivery and browser integration exist.";
pub const CLAIM_UNMANAGED_EXACT_EVIDENCE: &str = "Unmanaged browser exact URL, active tab, title, page, download source, HTTPS content, and intent evidence remain not-claimed.";
pub const CLAIM_NETWORK_FILTER_MANUAL: &str = "Network/domain blocking remains manual-required and is not proved by domain observation or browser policy records.";
pub const CLAIM_NETWORK_ADAPTER_UNAVAILABLE: &str = "The current Windows service boundary can report network/domain adapter unavailable states but does not perform host filtering.";
pub const CLAIM_AUDIT_VISIBILITY: &str = "Audit visibility is limited to existing enforcement journal and browser policy event seams; it is not proof of broad app/domain enforcement.";
pub const CLAIM_RESTART_RECOVERY: &str = "Restart recovery visibility is limited to app time-limit state recovery and cannot upgrade browser/domain blocking support.";
pub const CLAIM_BROWSER_POLICY_ROLLBACK: &str = "Browser policy rollback visibility proves stored policy revision rollback only and does not prove host-level browser/domain enforcement rollback.";
pub const CLAIM_LINUX_ADAPTER: &str = "Linux browser/domain adapter behavior is unavailable in this proof and cannot inherit Windows managed browser behavior.";
pub const CLAIM_MACOS_ADAPTER: &str = "macOS browser/domain adapter behavior is unavailable in this proof and cannot inherit Windows managed browser behavior.";
pub const CLAIM_ANDROID_ADAPTER: &str = "Android browser/domain control is manual-required and is not implied by desktop managed-browser or network-domain proof.";
pub const CLAIM_IOS_ADAPTER: &str = "iOS browser/domain control is manual-required and cannot be inferred from desktop or Android proofs.";
pub const CLAIM_WINDOWS_APP_CONTROL_READINESS: &str = "Windows AppLocker/App Control readiness can be represented only as a detect/manual setup state until host policy artifacts exist.";
pub const CLAIM_WINDOWS_APP_CONTROL_AUDIT_ONLY: &str = "Audit-only AppLocker/App Control state is visible as a readiness/audit proof state and does not block launch.";
pub const CLAIM_WINDOWS_APP_CONTROL_ENFORCED: &str = "Enforced AppLocker/App Control mode remains manual-required until real policy create/update, refresh, audit, and rollback artifacts prove launch prevention.";
pub const CLAIM_WINDOWS_APP_CONTROL_MANUAL_REQUIRED: &str = "Manual-required AppLocker/App Control setup is represented separately from unavailable and enforced states.";
pub const CLAIM_WINDOWS_APP_CONTROL_UNAVAILABLE: &str = "Unavailable AppLocker/App Control state records when the host cannot provide a policy adapter or permission path.";
pub const CLAIM_WINDOWS_APP_CONTROL_FAILED: &str = "Failed AppLocker/App Control state is parent-visible as a policy/audit failure without claiming a blocking result.";

pub const FALLBACK_MANAGED_INTERVENTION: &str = "Return manual-required when managed browser launch, active-tab, exact URL, rollback, or audit proof is missing.";
pub const FALLBACK_MANAGED_EXACT_URL: &str = "Return manual-required until live active-tab, exact URL apply, rollback, and custody artifacts exist.";
pub const FALLBACK_UNMANAGED_TERMINATE: &str = "Reject missing pid or process-name mismatch; keep URL certainty unclaimed without browser integration.";
pub const FALLBACK_UNMANAGED_WARN: &str =
    "Return a degraded no-op instead of claiming warning delivery or URL-aware browser control.";
pub const FALLBACK_UNMANAGED_EXACT_EVIDENCE: &str =
    "Do not infer browser content from process names or command targets.";
pub const FALLBACK_NETWORK_FILTER_MANUAL: &str = "Return manual-required until host DNS/VPN/filter apply, rollback, and custody evidence exists.";
pub const FALLBACK_NETWORK_ADAPTER_UNAVAILABLE: &str =
    "Return unavailable when the host filter adapter is absent or unsupported.";
pub const FALLBACK_AUDIT_VISIBILITY: &str =
    "Return unavailable when the local audit store or event payload cannot be read.";
pub const FALLBACK_RESTART_RECOVERY: &str =
    "Return unavailable when persisted timer state is missing or incompatible.";
pub const FALLBACK_BROWSER_POLICY_ROLLBACK: &str = "Return manual-required for managed exact URL, network/domain, or unmanaged browser rollback until host artifacts exist.";
pub const FALLBACK_LINUX_ADAPTER: &str = "Report unavailable until Linux-specific browser/domain apply, rollback, and audit proof exists.";
pub const FALLBACK_MACOS_ADAPTER: &str = "Report unavailable until macOS-specific browser/domain permissions, apply, rollback, and audit proof exists.";
pub const FALLBACK_ANDROID_ADAPTER: &str = "Return manual-required until real Android package, permission, VPN/DNS, device-owner, and lifecycle artifacts exist.";
pub const FALLBACK_IOS_ADAPTER: &str = "Return manual-required until approved entitlement, signing, install, and device evidence exists.";
pub const FALLBACK_WINDOWS_APP_CONTROL_READINESS: &str = "Return manual-required when edition, permission, policy provider, or identity-target proof is missing.";
pub const FALLBACK_WINDOWS_APP_CONTROL_AUDIT_ONLY: &str = "Keep prevention false and surface audit-only status until enforce-mode apply and rollback proof exists.";
pub const FALLBACK_WINDOWS_APP_CONTROL_ENFORCED: &str = "Do not claim launch blocking from requested policy state; require host apply and rollback proof.";
pub const FALLBACK_WINDOWS_APP_CONTROL_MANUAL_REQUIRED: &str = "Show manual setup rather than silently downgrading to process termination or browser-domain blocking.";
pub const FALLBACK_WINDOWS_APP_CONTROL_UNAVAILABLE: &str = "Return unavailable and keep unmanaged fallback manual when policy provider or service permission is absent.";
pub const FALLBACK_WINDOWS_APP_CONTROL_FAILED: &str = "Record failure and rollback/manual setup requirements; do not treat failed apply as enforcement.";
