pub const READ_MODEL_ID: &str = "v0-8-cross-platform-enforcement-capability-proof";

pub const ENTRY_ID_WINDOWS_OWNED_PROCESS: &str =
    "v0-8-cross-platform-windows-owned-process-terminate";
pub const ENTRY_ID_WINDOWS_APP_TIME_LIMIT: &str = "v0-8-cross-platform-windows-app-time-limit";
pub const ENTRY_ID_WINDOWS_MANAGED_BROWSER: &str =
    "v0-8-cross-platform-windows-managed-browser-boundary";
pub const ENTRY_ID_WINDOWS_UNMANAGED_BROWSER: &str =
    "v0-8-cross-platform-windows-unmanaged-browser-boundary";
pub const ENTRY_ID_WINDOWS_BROAD_APP: &str = "v0-8-cross-platform-windows-broad-app-blocking";
pub const ENTRY_ID_WINDOWS_NETWORK_DOMAIN: &str =
    "v0-8-cross-platform-windows-network-domain-blocking";
pub const ENTRY_ID_LINUX_ADAPTER_SCAFFOLD: &str = "v0-8-cross-platform-linux-adapter-scaffold";
pub const ENTRY_ID_MACOS_ADAPTER_SCAFFOLD: &str = "v0-8-cross-platform-macos-adapter-scaffold";
pub const ENTRY_ID_ANDROID_DEVICE_OWNER: &str = "v0-8-cross-platform-android-device-owner-policy";
pub const ENTRY_ID_ANDROID_PACKAGE_LIFECYCLE: &str =
    "v0-8-cross-platform-android-package-lifecycle";
pub const ENTRY_ID_ANDROID_STORE: &str = "v0-8-cross-platform-android-store-distribution";
pub const ENTRY_ID_IOS_FAMILY_CONTROLS: &str = "v0-8-cross-platform-ios-family-controls";
pub const ENTRY_ID_IOS_SIGNING: &str = "v0-8-cross-platform-ios-signing-entitlements";
pub const ENTRY_ID_IOS_TESTFLIGHT: &str = "v0-8-cross-platform-ios-testflight-distribution";
pub const ENTRY_ID_IOS_STORE: &str = "v0-8-cross-platform-ios-store-distribution";

pub const SURFACE_WINDOWS_OWNED_PROCESS: &str = "windows-owned-process-terminate";
pub const SURFACE_WINDOWS_APP_TIME_LIMIT: &str = "windows-app-time-limit-lifecycle";
pub const SURFACE_WINDOWS_MANAGED_BROWSER: &str = "windows-managed-browser-boundary";
pub const SURFACE_WINDOWS_UNMANAGED_BROWSER: &str = "windows-unmanaged-browser-process-boundary";
pub const SURFACE_WINDOWS_BROAD_APP: &str = "windows-broad-installed-app-blocking";
pub const SURFACE_WINDOWS_NETWORK_DOMAIN: &str = "windows-network-domain-blocking";
pub const SURFACE_LINUX_ADAPTER_SCAFFOLD: &str = "linux-enforcement-adapter-scaffold";
pub const SURFACE_MACOS_ADAPTER_SCAFFOLD: &str = "macos-enforcement-adapter-scaffold";
pub const SURFACE_ANDROID_DEVICE_OWNER: &str = "android-device-owner-policy";
pub const SURFACE_ANDROID_PACKAGE_LIFECYCLE: &str = "android-package-lifecycle";
pub const SURFACE_ANDROID_STORE: &str = "android-store-distribution";
pub const SURFACE_IOS_FAMILY_CONTROLS: &str = "ios-family-controls";
pub const SURFACE_IOS_SIGNING: &str = "ios-signing-entitlements";
pub const SURFACE_IOS_TESTFLIGHT: &str = "ios-testflight-distribution";
pub const SURFACE_IOS_STORE: &str = "ios-store-distribution";

pub const CAPABILITY_HEADLESS_AGENT_SERVICE: &str = "headless-agent-service";
pub const CAPABILITY_OWNED_PROCESS_TERMINATE: &str = "owned-process-terminate";
pub const CAPABILITY_APP_TIME_LIMIT: &str = "app-time-limit";
pub const CAPABILITY_APP_BLOCKING: &str = "app-blocking";
pub const CAPABILITY_NETWORK_DOMAIN_BLOCKING: &str = "network-domain-blocking";
pub const CAPABILITY_MANAGED_BROWSER_CONTROL: &str = "managed-browser-control";
pub const CAPABILITY_UNMANAGED_BROWSER_DETECTION: &str = "unmanaged-browser-detection";
pub const CAPABILITY_DEVICE_OWNER_POLICY: &str = "device-owner-policy";
pub const CAPABILITY_PACKAGE_LIFECYCLE: &str = "package-lifecycle";
pub const CAPABILITY_FAMILY_CONTROLS: &str = "family-controls-entitlement";
pub const CAPABILITY_SIGNING_ENTITLEMENTS: &str = "signing-entitlements";
pub const CAPABILITY_TESTFLIGHT_DISTRIBUTION: &str = "testflight-distribution";
pub const CAPABILITY_STORE_DISTRIBUTION: &str = "store-distribution";

pub const STATUS_IMPLEMENTED: &str = "implemented";
pub const STATUS_SUPPORTED: &str = "supported";
pub const STATUS_PREVIEW_SCAFFOLD: &str = "preview-scaffold";
pub const STATUS_SCAFFOLD: &str = "scaffold";
pub const STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const STATUS_UNAVAILABLE: &str = "unavailable";
pub const STATUS_PLANNED: &str = "planned";
pub const STATUS_NOT_IMPLEMENTED: &str = "not-implemented";

pub const CLAIM_IMPLEMENTED_BOUNDARY: &str = "implemented-boundary";
pub const CLAIM_MANUAL_REQUIRED: &str = "manual-required";
pub const CLAIM_SCAFFOLD: &str = "scaffold";
pub const CLAIM_UNAVAILABLE: &str = "unavailable";
pub const CLAIM_PLANNED: &str = "planned";
pub const CLAIM_NOT_CLAIMED: &str = "not-claimed";

pub const EXECUTES_REAL_SERVICE: &str = "executes-real-service";
pub const RETURNS_MANUAL_REQUIRED: &str = "returns-manual-required";
pub const RETURNS_UNAVAILABLE: &str = "returns-unavailable";
pub const SCAFFOLD_ONLY: &str = "scaffold-only";
pub const NOT_INVOKED: &str = "not-invoked";

pub const SOURCE_BROAD_PROOF: &str = "v0-8-broad-os-adapter-proof";
pub const SOURCE_PRODUCT_PROOF: &str = "v0-8-os-adapter-product-proof";
pub const SOURCE_PRODUCT_AGGREGATE: &str = "enforcement-lan-mobile-product-proof";
pub const SOURCE_PLATFORM_CAPABILITIES: &str = "parent-control-platform-capabilities";

pub const COMMAND_WINDOWS_UNMANAGED_PROOF: &str =
    "node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs";
pub const COMMAND_WINDOWS_TIMER_PROOF: &str =
    "node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs";
pub const COMMAND_MANAGED_BROWSER_PROOF: &str =
    "node scripts/test/managed-browser-intervention-proof.mjs";
pub const ARTIFACT_WINDOWS_UNMANAGED_PROOF: &str =
    "test-results/windows-managed-unmanaged-browser-enforcement-proof/proof.json";
pub const ARTIFACT_WINDOWS_TIMER_PROOF: &str =
    "test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json";
pub const ARTIFACT_MANAGED_BROWSER_PROOF: &str =
    "test-results/managed-browser-intervention-proof/proof.json";

pub const REQUIREMENT_OS_APP_IDENTITY: &str = "OS-approved installed-app identity";
pub const REQUIREMENT_BLOCK_APPLY: &str = "block apply result";
pub const REQUIREMENT_ROLLBACK: &str = "rollback result";
pub const REQUIREMENT_AUDIT_CUSTODY: &str = "audit custody artifact";
pub const REQUIREMENT_NETWORK_FILTER: &str = "host network filter adapter";
pub const REQUIREMENT_DOMAIN_APPLY: &str = "domain filter apply result";
pub const REQUIREMENT_LINUX_SERVICE: &str = "Linux service-manager install proof";
pub const REQUIREMENT_LINUX_ADAPTER: &str = "Linux adapter apply/rollback proof";
pub const REQUIREMENT_MACOS_PERMISSIONS: &str = "macOS permissions proof";
pub const REQUIREMENT_MACOS_PACKAGE: &str = "launchd/package proof";
pub const REQUIREMENT_MACOS_ADAPTER: &str = "macOS adapter apply/rollback proof";
pub const REQUIREMENT_ANDROID_DEVICE_OWNER: &str = "device-owner enrollment artifact";
pub const REQUIREMENT_ANDROID_POLICY_APPLY: &str = "policy apply result";
pub const REQUIREMENT_ANDROID_PROFILE: &str = "managed-profile compatibility proof";
pub const REQUIREMENT_ANDROID_PACKAGE: &str = "debug/release package install artifact";
pub const REQUIREMENT_ANDROID_LIFECYCLE: &str = "background/reboot lifecycle proof";
pub const REQUIREMENT_ANDROID_UNINSTALL: &str = "uninstall/update proof";
pub const REQUIREMENT_GOOGLE_PLAY: &str = "Google Play signing proof";
pub const REQUIREMENT_RELEASE_TRACK: &str = "release track artifact";
pub const REQUIREMENT_POLICY_REVIEW: &str = "policy compliance review";
pub const REQUIREMENT_IOS_FAMILY: &str = "Family Controls entitlement approval";
pub const REQUIREMENT_IOS_DEVICE_ACTIVITY: &str = "DeviceActivity proof";
pub const REQUIREMENT_IOS_DEVICE: &str = "real device or TestFlight artifact";
pub const REQUIREMENT_APPLE_SIGNING: &str = "Apple signing credentials";
pub const REQUIREMENT_IOS_ENTITLEMENTS: &str = "approved entitlements";
pub const REQUIREMENT_IOS_INSTALL: &str = "device or TestFlight install proof";
pub const REQUIREMENT_TESTFLIGHT: &str = "TestFlight build artifact";
pub const REQUIREMENT_APP_STORE_CONNECT: &str = "App Store Connect evidence";
pub const REQUIREMENT_APP_STORE_REVIEW: &str = "App Store review path";
pub const REQUIREMENT_APPLE_RELEASE: &str = "release artifact";

pub const CLAIM_WINDOWS_OWNED_PROCESS: &str = "Windows owned-process terminate is limited to pid/name guarded process control and is not broad app blocking.";
pub const CLAIM_WINDOWS_APP_TIME_LIMIT: &str = "Windows app time-limit proof covers timer lifecycle, restart recovery, parent cancel, expiry, and audit only.";
pub const CLAIM_WINDOWS_MANAGED_BROWSER: &str = "Managed-browser control is limited to the Ocentra-owned managed browser boundary and is not unmanaged exact URL proof.";
pub const CLAIM_WINDOWS_UNMANAGED_BROWSER: &str = "Unmanaged browser detection is process-only and cannot prove URL, active tab, title, page, HTTPS content, or intent.";
pub const CLAIM_WINDOWS_BROAD_APP: &str = "Broad installed-app blocking remains manual-required beyond owned-process terminate and app timer proof.";
pub const CLAIM_WINDOWS_NETWORK_DOMAIN: &str = "Network/domain blocking remains manual-required and is not proved by network observation metadata.";
pub const CLAIM_LINUX_SCAFFOLD: &str = "Linux package preview is scaffold evidence only and cannot inherit Windows enforcement behavior.";
pub const CLAIM_MACOS_SCAFFOLD: &str = "macOS package preview is scaffold evidence only and cannot inherit Windows enforcement behavior.";
pub const CLAIM_ANDROID_DEVICE_OWNER: &str = "Android device-owner enforcement is manual-required and not implied by parent mobile or protocol scaffold.";
pub const CLAIM_ANDROID_PACKAGE: &str = "Android package lifecycle remains manual-required before any child enforcement support upgrade.";
pub const CLAIM_ANDROID_STORE: &str =
    "Android store distribution is planned and cannot be used as enforcement support evidence.";
pub const CLAIM_IOS_FAMILY: &str = "iOS Family Controls support is manual-required and cannot be inferred from simulator/package scaffolds.";
pub const CLAIM_IOS_SIGNING: &str = "iOS signing and entitlements are manual-required before any privileged child enforcement claim.";
pub const CLAIM_IOS_TESTFLIGHT: &str = "iOS TestFlight distribution is manual-required before mobile enforcement support can be claimed.";
pub const CLAIM_IOS_STORE: &str =
    "iOS store distribution is planned and is not privileged enforcement proof.";

pub const FALLBACK_WINDOWS_OWNED_PROCESS: &str =
    "Reject missing pid or process-name mismatch; return unavailable on unsupported hosts.";
pub const FALLBACK_WINDOWS_APP_TIME_LIMIT: &str =
    "Return unavailable when timer state, persisted state, or adapter support is missing.";
pub const FALLBACK_WINDOWS_MANAGED_BROWSER: &str =
    "Return manual-required when active-tab or exact URL apply/rollback proof is missing.";
pub const FALLBACK_WINDOWS_UNMANAGED_BROWSER: &str = "Keep exact unmanaged browser evidence not-claimed unless explicit browser integration proof exists.";
pub const FALLBACK_WINDOWS_BROAD_APP: &str = "Return manual-required until package identity, apply, rollback, and audit custody artifacts exist.";
pub const FALLBACK_WINDOWS_NETWORK_DOMAIN: &str =
    "Return manual-required until DNS/VPN/filter apply, rollback, and custody evidence exists.";
pub const FALLBACK_LINUX_SCAFFOLD: &str =
    "Report scaffold-only until Linux-specific enforcement adapter proof exists.";
pub const FALLBACK_MACOS_SCAFFOLD: &str =
    "Report scaffold-only until macOS-specific enforcement adapter and permission proof exists.";
pub const FALLBACK_ANDROID_DEVICE_OWNER: &str =
    "Return manual-required until real device-owner or managed-profile proof exists.";
pub const FALLBACK_ANDROID_PACKAGE: &str =
    "Return manual-required until emulator or physical-device package lifecycle artifacts exist.";
pub const FALLBACK_ANDROID_STORE: &str = "Do not invoke privileged mobile enforcement until store/signing proof and device capability proof exist.";
pub const FALLBACK_IOS_FAMILY: &str =
    "Return manual-required until approved entitlement and device proof exist.";
pub const FALLBACK_IOS_SIGNING: &str =
    "Return manual-required until signing, entitlement, and install artifacts exist.";
pub const FALLBACK_IOS_TESTFLIGHT: &str =
    "Return manual-required until TestFlight and device proof exist.";
pub const FALLBACK_IOS_STORE: &str =
    "Do not claim mobile child enforcement from planned store distribution work.";
