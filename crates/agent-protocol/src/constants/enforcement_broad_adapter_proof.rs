pub const READ_MODEL_ID: &str = "v0-8-broad-os-adapter-runtime-proof";

pub const ENTRY_ID_OWNED_PROCESS_TIMER: &str = "windows-owned-process-and-timer-runtime-boundary";
pub const ENTRY_ID_MANAGED_BROWSER_SESSION: &str =
    "windows-managed-browser-session-runtime-boundary";
pub const ENTRY_ID_BROAD_INSTALLED_APP_GATE: &str = "windows-broad-installed-app-runtime-gate";
pub const ENTRY_ID_NETWORK_DOMAIN_GATE: &str = "windows-network-domain-runtime-gate";
pub const ENTRY_ID_MANAGED_EXACT_URL_GATE: &str = "windows-managed-browser-exact-url-runtime-gate";
pub const ENTRY_ID_UNMANAGED_EXACT_EVIDENCE_GAP: &str =
    "windows-unmanaged-browser-exact-evidence-runtime-gap";
pub const ENTRY_ID_LINUX_UNAVAILABLE: &str = "linux-host-runtime-unavailable";
pub const ENTRY_ID_MACOS_MANUAL_GATE: &str = "macos-host-runtime-manual-gate";
pub const ENTRY_ID_ANDROID_MANUAL_GATE: &str = "android-mobile-runtime-manual-gate";
pub const ENTRY_ID_IOS_MANUAL_GATE: &str = "ios-mobile-runtime-manual-gate";

pub const SURFACE_OWNED_PROCESS_TIMER: &str = "windows-owned-process-and-timer-runtime-boundary";
pub const SURFACE_MANAGED_BROWSER_SESSION: &str =
    "windows-managed-browser-session-runtime-boundary";
pub const SURFACE_BROAD_INSTALLED_APP_GATE: &str = "windows-broad-installed-app-runtime-gate";
pub const SURFACE_NETWORK_DOMAIN_GATE: &str = "windows-network-domain-runtime-gate";
pub const SURFACE_MANAGED_EXACT_URL_GATE: &str = "windows-managed-browser-exact-url-runtime-gate";
pub const SURFACE_UNMANAGED_EXACT_EVIDENCE_GAP: &str =
    "windows-unmanaged-browser-exact-evidence-runtime-gap";
pub const SURFACE_LINUX_UNAVAILABLE: &str = "linux-host-runtime-unavailable";
pub const SURFACE_MACOS_MANUAL_GATE: &str = "macos-host-runtime-manual-gate";
pub const SURFACE_ANDROID_MANUAL_GATE: &str = "android-mobile-runtime-manual-gate";
pub const SURFACE_IOS_MANUAL_GATE: &str = "ios-mobile-runtime-manual-gate";

pub const CLAIM_IMPLEMENTED_BOUNDARY: &str = "implemented-boundary";
pub const CLAIM_MANUAL_REQUIRED: &str = "manual-required";
pub const CLAIM_UNAVAILABLE: &str = "unavailable";
pub const CLAIM_NOT_CLAIMED: &str = "not-claimed";

pub const EVIDENCE_COMPOSITE_RUNTIME_PROOF: &str = "composite-runtime-proof";
pub const EVIDENCE_MANUAL_ARTIFACT_REQUIRED: &str = "manual-artifact-required";
pub const EVIDENCE_TARGET_UNAVAILABLE: &str = "target-unavailable";
pub const EVIDENCE_NOT_IMPLEMENTED: &str = "not-implemented";

pub const SOURCE_BROAD_OS_ADAPTER_PROOF: &str = "v0-8-broad-os-adapter-proof";
pub const SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF: &str = "v0-8-browser-domain-adapter-proof";
pub const SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES: &str = "v0-8-os-adapter-manual-artifact-gates";
pub const SOURCE_OS_ADAPTER_PRODUCT_PROOF: &str = "v0-8-os-adapter-product-proof";

pub const COMMAND_BROAD_OS_ADAPTER_PROOF: &str =
    "node scripts/test/v0-8-broad-os-adapter-proof.mjs";
pub const COMMAND_OS_ADAPTER_PRODUCT_PROOF_CARGO: &str =
    "cargo test -p ocentra-parent-agent-service enforcement_os_adapter_product_proof_read_model";
pub const COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF: &str =
    "node scripts/test/v0-8-browser-domain-adapter-proof.mjs";
pub const COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF_CARGO: &str = "cargo test -p ocentra-parent-agent-service enforcement_browser_domain_adapter_proof_read_model";
pub const COMMAND_OS_ADAPTER_MANUAL_ARTIFACT_GATES: &str =
    "node scripts/test/v0-8-os-adapter-manual-artifact-gates.mjs";

pub const ARTIFACT_BROAD_OS_ADAPTER_PROOF: &str =
    "test-results/v0-8-broad-os-adapter-proof/proof.json";
pub const ARTIFACT_OS_ADAPTER_PRODUCT_PROOF_SERVICE: &str =
    "crates/agent-service/src/enforcement_os_adapter_product_proof_read_model.rs";
pub const ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF: &str =
    "test-results/v0-8-browser-domain-adapter-proof/proof.json";
pub const ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF_SERVICE: &str =
    "crates/agent-service/src/enforcement_browser_domain_adapter_proof_read_model.rs";
pub const ARTIFACT_OS_ADAPTER_MANUAL_ARTIFACT_GATES: &str =
    "test-results/v0-8-os-adapter-manual-artifact-gates/proof.json";

pub const REQUIREMENT_SAME_APP_IDENTITY: &str = "same app identity proof";
pub const REQUIREMENT_HOST_BLOCK_APPLY: &str = "host block apply artifact";
pub const REQUIREMENT_ROLLBACK: &str = "rollback artifact";
pub const REQUIREMENT_AUDIT_CUSTODY: &str = "audit custody artifact";
pub const REQUIREMENT_HOST_DNS_OR_FILTER_APPLY: &str = "host DNS or filter apply artifact";
pub const REQUIREMENT_ACTIVE_TAB: &str = "active tab artifact";
pub const REQUIREMENT_EXACT_URL_APPLY: &str = "exact URL apply artifact";
pub const REQUIREMENT_BROWSER_INTEGRATION: &str =
    "browser integration artifact for URL, title, page, download, HTTPS content, and intent";
pub const REQUIREMENT_LINUX_HOST: &str =
    "Linux service manager, package identity, permission, apply, rollback, and audit artifacts";
pub const REQUIREMENT_MACOS_HOST: &str =
    "macOS permission, package, service, apply, rollback, and audit artifacts";
pub const REQUIREMENT_ANDROID_DEVICE_OWNER: &str = "device-owner or managed-profile artifact";
pub const REQUIREMENT_ANDROID_USAGE_STATS: &str = "UsageStats artifact";
pub const REQUIREMENT_ANDROID_ACCESSIBILITY_VPN_DNS: &str = "accessibility or VPN/DNS artifact";
pub const REQUIREMENT_ANDROID_PACKAGE_LIFECYCLE: &str = "package lifecycle artifact";
pub const REQUIREMENT_IOS_FAMILY_CONTROLS: &str = "Family Controls entitlement artifact";
pub const REQUIREMENT_IOS_DEVICE_ACTIVITY: &str = "DeviceActivity artifact";
pub const REQUIREMENT_IOS_NETWORK_EXTENSION: &str = "Network Extension artifact";
pub const REQUIREMENT_IOS_SIGNING_TESTFLIGHT: &str = "signing and TestFlight device artifact";

pub const CLAIM_OWNED_PROCESS_TIMER: &str = "Owned-process pid/name guardrails and app timer lifecycle are runtime boundaries only; they are not broad installed-app blocking.";
pub const CLAIM_MANAGED_BROWSER_SESSION: &str = "Managed-browser runtime proof is limited to the owned managed-session intervention state and does not prove exact active-tab URL enforcement.";
pub const CLAIM_BROAD_INSTALLED_APP_GATE: &str = "Broad installed-app blocking stays manual-required even though owned-process and timer mechanics are proved.";
pub const CLAIM_NETWORK_DOMAIN_GATE: &str = "Network/domain runtime proof records manual-required and unavailable states only; domain observation is not host blocking.";
pub const CLAIM_MANAGED_EXACT_URL_GATE: &str = "Managed exact URL blocking remains manual-required and is distinct from managed-session intervention.";
pub const CLAIM_UNMANAGED_EXACT_EVIDENCE_GAP: &str = "Unmanaged browser exact evidence is not claimed; process terminate and warn boundaries do not prove URL or page certainty.";
pub const CLAIM_LINUX_UNAVAILABLE: &str = "Linux host OS adapter support is unavailable in this final pass and cannot inherit Windows proof.";
pub const CLAIM_MACOS_MANUAL_GATE: &str =
    "macOS host support stays manual-required until target-specific artifacts exist.";
pub const CLAIM_ANDROID_MANUAL_GATE: &str =
    "Android child enforcement remains manual-required and is not proved by host OS adapters.";
pub const CLAIM_IOS_MANUAL_GATE: &str = "iOS child enforcement remains manual-required and is not proved by Windows host runtime proof.";

pub const FALLBACK_OWNED_PROCESS_TIMER: &str = "Inputs outside the owned-process or timer boundary remain manual-required or unavailable instead of escalating to broad blocking.";
pub const FALLBACK_MANAGED_BROWSER_SESSION: &str = "Exact URL control and unmanaged browser evidence stay manual-required or not-claimed until browser integration artifacts exist.";
pub const FALLBACK_BROAD_INSTALLED_APP_GATE: &str = "The runtime must report manual-required for global app blocking until target host artifacts prove apply and rollback.";
pub const FALLBACK_NETWORK_DOMAIN_GATE: &str = "The runtime must return manual-required or unavailable rather than claim a host filter when no adapter artifact exists.";
pub const FALLBACK_MANAGED_EXACT_URL_GATE: &str = "The runtime exposes the managed-session boundary while leaving exact URL enforcement gated by manual artifacts.";
pub const FALLBACK_UNMANAGED_EXACT_EVIDENCE_GAP: &str = "The runtime may terminate or warn by process boundary only and must keep exact unmanaged evidence not-claimed.";
pub const FALLBACK_LINUX_UNAVAILABLE: &str = "Linux targets must report unavailable or manual-required platform states until a target adapter proves support.";
pub const FALLBACK_MACOS_MANUAL_GATE: &str =
    "macOS targets must not reuse Windows runtime proof for host enforcement claims.";
pub const FALLBACK_ANDROID_MANUAL_GATE: &str = "Android targets keep privileged mobile states manual-required until real device policy artifacts exist.";
pub const FALLBACK_IOS_MANUAL_GATE: &str = "iOS targets keep entitlement and device states manual-required until Apple-approved artifacts exist.";
