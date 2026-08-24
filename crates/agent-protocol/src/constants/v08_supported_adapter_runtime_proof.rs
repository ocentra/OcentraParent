pub const READ_MODEL_ID: &str = "v0-8-supported-adapter-runtime-proof";

pub const ENTRY_ID_APP_GAME_TIMER: &str = "windows-app-game-owned-process-time-limit";
pub const ENTRY_ID_NETWORK_OBSERVE: &str = "windows-network-flow-observe-policy-handoff";
pub const ENTRY_ID_BROAD_APP_MANUAL: &str = "windows-broad-installed-app-blocking-manual-gate";
pub const ENTRY_ID_HOST_NETWORK_MANUAL: &str = "windows-host-network-domain-blocking-manual-gate";
pub const ENTRY_ID_BROAD_APP_ARTIFACT_STATUS: &str = "windows-broad-installed-app-artifact-status";
pub const ENTRY_ID_HOST_NETWORK_ARTIFACT_STATUS: &str =
    "windows-host-network-domain-artifact-status";
pub const ENTRY_ID_MANAGED_BROWSER_ARTIFACT_STATUS: &str =
    "windows-managed-browser-artifact-status";
pub const ENTRY_ID_EXACT_ACTIVE_TAB_NOT_CLAIMED: &str =
    "windows-managed-exact-active-tab-not-claimed";
pub const ENTRY_ID_PERMISSION_DEGRADED: &str = "windows-adapter-permission-dependency-degraded";
pub const ENTRY_ID_LINUX_UNAVAILABLE: &str = "linux-host-adapter-unavailable";
pub const ENTRY_ID_MACOS_UNSUPPORTED: &str = "macos-host-adapter-unsupported";
pub const ENTRY_ID_ANDROID_MANUAL: &str = "android-mobile-control-manual-gate";
pub const ENTRY_ID_IOS_MANUAL: &str = "ios-mobile-control-manual-gate";

pub const CAPABILITY_APP_GAME_TIMER: &str = "app-game-owned-process-time-limit";
pub const CAPABILITY_NETWORK_OBSERVE: &str = "network-flow-observe-policy-handoff";
pub const CAPABILITY_BROAD_APP_BLOCKING: &str = "broad-installed-app-blocking";
pub const CAPABILITY_HOST_NETWORK_BLOCKING: &str = "host-network-domain-blocking";
pub const CAPABILITY_BROAD_APP_ARTIFACT_STATUS: &str = "broad-installed-app-artifact-status";
pub const CAPABILITY_HOST_NETWORK_ARTIFACT_STATUS: &str = "host-network-domain-artifact-status";
pub const CAPABILITY_MANAGED_BROWSER_ARTIFACT_STATUS: &str = "managed-browser-artifact-status";
pub const CAPABILITY_MANAGED_EXACT_ACTIVE_TAB: &str = "managed-exact-active-tab-enforcement";
pub const CAPABILITY_PERMISSION_DEPENDENCY: &str = "adapter-permission-dependency";
pub const CAPABILITY_DESKTOP_HOST: &str = "desktop-host-platform-adapter";
pub const CAPABILITY_MOBILE_CHILD_CONTROL: &str = "mobile-child-control-adapter";

pub const STATE_IMPLEMENTED_BOUNDARY: &str = "implemented-boundary";
pub const STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const STATE_UNAVAILABLE: &str = "unavailable";
pub const STATE_NOT_CLAIMED: &str = "not-claimed";
pub const STATE_UNSUPPORTED: &str = "unsupported";
pub const STATE_DEGRADED: &str = "degraded";

pub const RESULT_SUPPORTED_BOUNDARY_PROVED: &str = "supported-boundary-proved";
pub const RESULT_MANUAL_PROOF_REQUIRED: &str = "manual-proof-required";
pub const RESULT_TARGET_UNAVAILABLE: &str = "target-unavailable";
pub const RESULT_NOT_CLAIMED: &str = "not-claimed";
pub const RESULT_UNSUPPORTED_PLATFORM: &str = "unsupported-platform";
pub const RESULT_DEGRADED_PERMISSION_OR_DEPENDENCY: &str = "degraded-permission-or-dependency";

pub const PLATFORM_SUPPORTED_WINDOWS: &str = "supported-on-windows";
pub const PLATFORM_MANUAL_REQUIRED: &str = "manual-required";
pub const PLATFORM_UNAVAILABLE_ON_TARGET: &str = "unavailable-on-target";
pub const PLATFORM_UNSUPPORTED: &str = "unsupported-platform";
pub const PLATFORM_DEGRADED: &str = "degraded";

pub const TARGET_PROCESS_SESSION_EVIDENCE: &str = "process-session-evidence-backed";
pub const TARGET_NETWORK_FLOW_EVIDENCE: &str = "network-flow-evidence-backed";
pub const TARGET_INSUFFICIENT_BROAD: &str = "insufficient-for-broad-target";
pub const TARGET_NOT_APPLICABLE: &str = "not-applicable";
pub const TARGET_UNSUPPORTED_PLATFORM: &str = "unsupported-platform-target";

pub const ROLLBACK_TIMER_RECOVERY: &str = "timer-recovery-backed";
pub const ROLLBACK_OBSERVE_ONLY_NOT_NEEDED: &str = "observe-only-not-needed";
pub const ROLLBACK_MANUAL_REQUIRED: &str = "manual-required";
pub const ROLLBACK_UNAVAILABLE: &str = "unavailable";
pub const ROLLBACK_NOT_CLAIMED: &str = "not-claimed";

pub const AUDIT_BACKED: &str = "audit-reference-backed";
pub const AUDIT_MANUAL_REQUIRED: &str = "manual-required";
pub const AUDIT_UNAVAILABLE: &str = "unavailable";
pub const AUDIT_NOT_CLAIMED: &str = "not-claimed";

pub const REFUSAL_NONE: &str = "none";
pub const REFUSAL_MANUAL_ARTIFACT_REQUIRED: &str = "manual-artifact-required";
pub const REFUSAL_TARGET_UNAVAILABLE: &str = "target-unavailable";
pub const REFUSAL_NOT_CLAIMED_BOUNDARY: &str = "not-claimed-boundary";
pub const REFUSAL_UNSUPPORTED_PLATFORM: &str = "unsupported-platform";
pub const REFUSAL_PERMISSION_OR_DEPENDENCY_DEGRADED: &str = "permission-or-dependency-degraded";

pub const SOURCE_BROAD_ADAPTER_PROOF: &str = "v0-8-broad-os-adapter-runtime-proof";
pub const SOURCE_POLICY_DISPATCH_PROOF: &str = "v0-8-enforcement-policy-dispatch-proof";
pub const SOURCE_PRODUCT_CONTROL_PROOF: &str = "v0-8-enforcement-product-control-spine";
pub const SOURCE_NETWORK_FLOW_EVIDENCE: &str = "network-flow-read-model";
pub const SOURCE_WINDOWS_ADAPTER_CAPABILITY_PROOF: &str = "v0-8-windows-adapter-capability-proof";
pub const SOURCE_WINDOWS_ADAPTER_ARTIFACT_GATE: &str = "v0-8-windows-adapter-artifact-gate";
pub const SOURCE_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF: &str =
    "v0-8-windows-adapter-artifact-ingestion-proof";

pub const COMMAND_APP_TIME_LIMIT_ADAPTER: &str =
    "node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs";
pub const COMMAND_ENFORCEMENT_TIMER_CARGO: &str =
    "cargo test -p ocentra-parent-agent-service enforcement_timer";
pub const COMMAND_NETWORK_FLOW_CARGO: &str =
    "cargo test -p ocentra-parent-agent-service network_flow_digest";
pub const COMMAND_POLICY_DISPATCH_PROOF: &str =
    "node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs";
pub const COMMAND_BROWSER_DOMAIN_ADAPTER_PROOF: &str =
    "node scripts/test/v0-8-browser-domain-adapter-proof.mjs";
pub const COMMAND_WINDOWS_ADAPTER_CAPABILITY_PROOF: &str =
    "node scripts/test/v0-8-windows-adapter-capability-proof.mjs";
pub const COMMAND_WINDOWS_ADAPTER_ARTIFACT_GATE: &str =
    "node scripts/test/v0-8-windows-adapter-artifact-gate.mjs";
pub const COMMAND_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF: &str =
    "node scripts/test/v0-8-windows-adapter-artifact-ingestion-proof.mjs";

pub const ARTIFACT_APP_TIME_LIMIT_PROOF: &str =
    "test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json";
pub const ARTIFACT_ENFORCEMENT_TIMER_STATE: &str =
    "crates/agent-service/src/enforcement_timer_state.rs";
pub const ARTIFACT_NETWORK_FLOW_DIGEST: &str = "crates/agent-service/src/network_flow_digest.rs";
pub const ARTIFACT_POLICY_DISPATCH_PROOF: &str =
    "test-results/v0-8-enforcement-policy-dispatch-proof/proof.json";
pub const ARTIFACT_BROWSER_DOMAIN_ADAPTER_PROOF: &str =
    "test-results/v0-8-browser-domain-adapter-proof/proof.json";
pub const ARTIFACT_WINDOWS_ADAPTER_CAPABILITY_PROOF: &str =
    "test-results/v0-8-windows-adapter-capability-proof/proof.json";
pub const ARTIFACT_WINDOWS_ADAPTER_ARTIFACT_GATE: &str =
    "test-results/v0-8-windows-adapter-artifact-gate/proof.json";
pub const ARTIFACT_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF: &str =
    "test-results/v0-8-windows-adapter-artifact-ingestion-proof/proof.json";

pub const REF_APP_SESSION_EVIDENCE: &str = "app-game-session-evidence-ref";
pub const REF_OWNED_PROCESS_IDENTITY: &str = "owned-process-identity-ref";
pub const REF_TIMER_STATE: &str = "timer-state-ref";
pub const REF_NETWORK_FLOW_SUMMARY: &str = "network-flow-summary-ref";
pub const REF_DOMAIN_ATTRIBUTION_STATE: &str = "domain-attribution-state-ref";
pub const REF_POLICY_PREVIEW: &str = "policy-preview-ref";
pub const REF_ADAPTER_CAPABILITY_STATE: &str = "adapter-capability-state-ref";
pub const REF_ANDROID_ADB_HOST_TOOLCHAIN: &str = "android-adb-host-toolchain-ref";
pub const REF_ANDROID_ADB_PATH_PROBE: &str = "android-adb-path-probe-ref";
pub const REF_ANDROID_ADB_SDK_PROBE: &str = "android-adb-sdk-probe-ref";
pub const REF_ANDROID_PHYSICAL_DEVICE_PROOF: &str = "android-physical-device-proof-ref";
pub const REF_ANDROID_USAGE_EVENTS_FOREGROUND: &str = "android-usage-events-foreground-ref";
pub const REF_LINUX_WSL_HOST_TOOLCHAIN: &str = "linux-wsl-host-toolchain-ref";
pub const REF_LINUX_WSL_PATH_PROBE: &str = "linux-wsl-path-probe-ref";
pub const REF_LINUX_WSLG_DISPLAY: &str = "linux-wslg-display-ref";
pub const REF_LINUX_WSLG_X11_SOCKET: &str = "linux-wslg-x11-socket-ref";
pub const REF_LINUX_WSLG_WAYLAND_SOCKET: &str = "linux-wslg-wayland-socket-ref";
pub const REF_LINUX_NATIVE_DISPLAY: &str = "linux-native-display-ref";
pub const REF_LINUX_NATIVE_X11_SOCKET: &str = "linux-native-x11-socket-ref";
pub const REF_LINUX_NATIVE_WAYLAND_SOCKET: &str = "linux-native-wayland-socket-ref";
pub const REF_LINUX_FOREGROUND_SOURCE_PREFLIGHT: &str = "linux-foreground-source-preflight-ref";
pub const REF_LINUX_XPROP_PROBE: &str = "linux-xprop-probe-ref";
pub const REF_LINUX_XDOTOOL_PROBE: &str = "linux-xdotool-probe-ref";
pub const REF_LINUX_ACTIVE_WINDOW_OBSERVED: &str = "linux-active-window-observed-ref";
pub const REF_LINUX_DOCKER_HOST_TOOLCHAIN: &str = "linux-docker-host-toolchain-ref";
pub const REF_LINUX_DOCKER_PATH_PROBE: &str = "linux-docker-path-probe-ref";
pub const REF_LINUX_DOCKER_HOST_PREFLIGHT: &str = "linux-docker-host-preflight-ref";
pub const REF_WINDOWS_HOST_LOCAL_PROBE: &str = "windows-host-local-probe-ref";
pub const ENV_PATH: &str = "PATH";
pub const ENV_PATHEXT: &str = "PATHEXT";
pub const ENV_ANDROID_HOME: &str = "ANDROID_HOME";
pub const ENV_ANDROID_SDK_ROOT: &str = "ANDROID_SDK_ROOT";
pub const EXE_ADB: &str = "adb";
pub const EXE_WSL: &str = "wsl";
pub const EXE_DOCKER: &str = "docker";
pub const DOCKER_VERSION_ARGUMENTS: [&str; 3] = ["version", "--format", "{{.Server.Version}}"];
pub const DOCKER_CONTEXT_ARGUMENTS: [&str; 4] = ["context", "ls", "--format", "1"];
pub const DOCKER_INVENTORY_ARGUMENTS: [&str; 3] =
    ["info", "--format", "{{.Images}} {{.Containers}}"];
pub const DOCKER_CONTEXT_COUNT_MARKER: &str = "1";
pub const WINDOWS_EXE_EXTENSION: &str = ".exe";
pub const ANDROID_PLATFORM_TOOLS_DIR: &str = "platform-tools";
pub const REF_WINDOWS_ADAPTER_ARTIFACT_GATE: &str = "windows-adapter-artifact-gate-ref";
pub const REF_WINDOWS_ADAPTER_ARTIFACT_INGESTION: &str = "windows-adapter-artifact-ingestion-ref";

pub const REQUIREMENT_SAME_APP_IDENTITY: &str = "same app identity proof";
pub const REQUIREMENT_HOST_BLOCK_APPLY: &str = "host block apply artifact";
pub const REQUIREMENT_ROLLBACK: &str = "rollback artifact";
pub const REQUIREMENT_AUDIT_CUSTODY: &str = "audit custody artifact";
pub const REQUIREMENT_HOST_DNS_OR_FILTER_APPLY: &str = "host DNS or filter apply artifact";
pub const REQUIREMENT_MANAGED_ACTIVE_TAB: &str = "managed active-tab evidence artifact";
pub const REQUIREMENT_EXACT_URL_APPLY: &str = "exact URL apply artifact";
pub const REQUIREMENT_SAME_IDENTITY_APP_PACKAGE_EVIDENCE: &str =
    "same-identity app package evidence";
pub const REQUIREMENT_ADAPTER_APPLY_RESULT: &str = "adapter apply result";
pub const REQUIREMENT_ADAPTER_ROLLBACK_RESULT: &str = "adapter rollback result";
pub const REQUIREMENT_NETWORK_FILTER_ROLLBACK: &str = "network/domain filter rollback result";
pub const REQUIREMENT_AUDIT_CUSTODY_EVENT: &str = "audit custody event";
pub const REQUIREMENT_MANUAL_REVIEW_AFTER_ARTIFACT_GATE: &str = "manual review after artifact gate";
pub const REQUIREMENT_MANAGED_BROWSER_EXACT_URL_EVIDENCE: &str =
    "managed-browser exact URL evidence";
pub const REQUIREMENT_PERMISSION_RESTORE: &str = "permission restoration artifact";
pub const REQUIREMENT_DEPENDENCY_REINSTALL: &str = "dependency reinstall artifact";
pub const REQUIREMENT_OPERATOR_DEGRADED_STATE: &str = "operator-visible degraded state";
pub const REQUIREMENT_LINUX_SERVICE: &str = "Linux service manager artifact";
pub const REQUIREMENT_LINUX_PERMISSION: &str = "Linux permission artifact";
pub const REQUIREMENT_LINUX_ROLLBACK: &str = "Linux rollback artifact";
pub const REQUIREMENT_MACOS_PERMISSION: &str = "macOS permission artifact";
pub const REQUIREMENT_MACOS_PACKAGE_IDENTITY: &str = "macOS package identity artifact";
pub const REQUIREMENT_MACOS_ROLLBACK: &str = "macOS rollback artifact";
pub const REQUIREMENT_ANDROID_DEVICE_OWNER: &str = "device-owner or managed-profile artifact";
pub const REQUIREMENT_ANDROID_USAGE_STATS: &str = "UsageStats artifact";
pub const REQUIREMENT_ANDROID_ACCESSIBILITY_VPN_DNS: &str = "accessibility or VPN/DNS artifact";
pub const REQUIREMENT_IOS_FAMILY_CONTROLS: &str = "Family Controls entitlement artifact";
pub const REQUIREMENT_IOS_DEVICE_ACTIVITY: &str = "DeviceActivity artifact";
pub const REQUIREMENT_IOS_NETWORK_EXTENSION: &str = "Network Extension artifact";

pub const CLAIM_APP_GAME_TIMER: &str = "App/game support is limited to owned-process identity, app-session evidence, timer state, audit refs, and recoverable expiry; it is not broad installed-app blocking.";
pub const CLAIM_NETWORK_OBSERVE: &str = "Network/domain support is observe-only policy handoff over stored flow evidence; it is not DNS, VPN, packet, or host filter enforcement.";
pub const CLAIM_BROAD_APP_MANUAL: &str = "Broad installed-app blocking remains manual-required because scoped process/timer proof does not prove package-wide blocking.";
pub const CLAIM_HOST_NETWORK_MANUAL: &str = "Host network/domain blocking remains manual-required because flow evidence and policy handoff are not filter apply proof.";
pub const CLAIM_BROAD_APP_ARTIFACT_STATUS: &str = "Windows app artifacts can make a broad-app target ready for manual review only; they do not prove broad installed-app blocking.";
pub const CLAIM_HOST_NETWORK_ARTIFACT_STATUS: &str = "Windows network/domain artifacts can make a host-filter target ready for manual review only; they do not prove DNS, VPN, packet, or domain blocking.";
pub const CLAIM_MANAGED_BROWSER_ARTIFACT_STATUS: &str = "Windows managed-browser artifacts can make exact-URL control ready for manual review only; they do not prove active-tab enforcement.";
pub const CLAIM_EXACT_ACTIVE_TAB_NOT_CLAIMED: &str = "Exact active-tab enforcement is not claimed by supported app/game or network observe-only runtime proof.";
pub const CLAIM_PERMISSION_DEGRADED: &str = "Supported-boundary adapters can degrade when permissions or dependencies are missing; degraded state is not enforcement success.";
pub const CLAIM_LINUX_UNAVAILABLE: &str =
    "Linux host adapter support is unavailable in this proof and cannot inherit Windows results.";
pub const CLAIM_MACOS_UNSUPPORTED: &str = "macOS host adapter support is unsupported in this proof and cannot reuse Windows host evidence.";
pub const CLAIM_MOBILE_MANUAL: &str = "Mobile child control remains manual-required and is not proved by Windows host supported-boundary adapters.";

pub const FALLBACK_APP_GAME_TIMER: &str = "Targets without process/session identity or timer custody return manual-required or degraded instead of escalating to broad block.";
pub const FALLBACK_NETWORK_OBSERVE: &str = "Network controls without a host filter adapter report manual-required for enforcement while preserving observe-only evidence refs.";
pub const FALLBACK_BROAD_APP_MANUAL: &str = "The runtime refuses broad app blocking claims until target host apply, rollback, and audit artifacts exist.";
pub const FALLBACK_HOST_NETWORK_MANUAL: &str = "The runtime refuses network/domain blocking claims until a host filter or DNS adapter proves apply and rollback.";
pub const FALLBACK_BROAD_APP_ARTIFACT_STATUS: &str = "Missing, mismatched, or uncustodied app artifacts stay refused and complete artifact sets remain manual-review-only.";
pub const FALLBACK_HOST_NETWORK_ARTIFACT_STATUS: &str = "Missing, mismatched, or uncustodied network artifacts stay refused and complete artifact sets remain manual-review-only.";
pub const FALLBACK_MANAGED_BROWSER_ARTIFACT_STATUS: &str = "Missing, mismatched, or uncustodied managed-browser artifacts stay refused and complete artifact sets remain manual-review-only.";
pub const FALLBACK_EXACT_ACTIVE_TAB_NOT_CLAIMED: &str = "The runtime may report managed-session or process fallback states, but exact active-tab enforcement remains not-claimed.";
pub const FALLBACK_PERMISSION_DEGRADED: &str = "The runtime emits degraded capability and keeps evidence capture or observe-only paths available where possible.";
pub const FALLBACK_LINUX_UNAVAILABLE: &str =
    "Linux targets report unavailable until a target-specific adapter proves support.";
pub const FALLBACK_MACOS_UNSUPPORTED: &str =
    "macOS targets report unsupported until a macOS-specific adapter and artifacts exist.";
pub const FALLBACK_MOBILE_MANUAL: &str = "Mobile targets keep privileged platform states manual-required until real mobile artifacts exist.";
