use serde::{Deserialize, Serialize};

use crate::{constants::v08_cross_platform_enforcement_capability_proof as proof, ParentPlatform};

macro_rules! protocol_str_lookup {
    ($self:expr, [$($value:expr),+ $(,)?]) => {{
        const VALUES: &[&str] = &[$($value),+];
        VALUES[*$self as usize]
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08CrossPlatformEnforcementCapabilitySurface {
    #[serde(rename = "windows-owned-process-terminate")]
    WindowsOwnedProcessTerminate,
    #[serde(rename = "windows-app-time-limit-lifecycle")]
    WindowsAppTimeLimitLifecycle,
    #[serde(rename = "windows-managed-browser-boundary")]
    WindowsManagedBrowserBoundary,
    #[serde(rename = "windows-unmanaged-browser-process-boundary")]
    WindowsUnmanagedBrowserProcessBoundary,
    #[serde(rename = "windows-broad-installed-app-blocking")]
    WindowsBroadInstalledAppBlocking,
    #[serde(rename = "windows-network-domain-blocking")]
    WindowsNetworkDomainBlocking,
    #[serde(rename = "linux-enforcement-adapter-scaffold")]
    LinuxEnforcementAdapterScaffold,
    #[serde(rename = "macos-enforcement-adapter-scaffold")]
    MacosEnforcementAdapterScaffold,
    #[serde(rename = "android-device-owner-policy")]
    AndroidDeviceOwnerPolicy,
    #[serde(rename = "android-package-lifecycle")]
    AndroidPackageLifecycle,
    #[serde(rename = "android-store-distribution")]
    AndroidStoreDistribution,
    #[serde(rename = "ios-family-controls")]
    IosFamilyControls,
    #[serde(rename = "ios-signing-entitlements")]
    IosSigningEntitlements,
    #[serde(rename = "ios-testflight-distribution")]
    IosTestflightDistribution,
    #[serde(rename = "ios-store-distribution")]
    IosStoreDistribution,
}

impl V08CrossPlatformEnforcementCapabilitySurface {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::SURFACE_WINDOWS_OWNED_PROCESS,
                proof::SURFACE_WINDOWS_APP_TIME_LIMIT,
                proof::SURFACE_WINDOWS_MANAGED_BROWSER,
                proof::SURFACE_WINDOWS_UNMANAGED_BROWSER,
                proof::SURFACE_WINDOWS_BROAD_APP,
                proof::SURFACE_WINDOWS_NETWORK_DOMAIN,
                proof::SURFACE_LINUX_ADAPTER_SCAFFOLD,
                proof::SURFACE_MACOS_ADAPTER_SCAFFOLD,
                proof::SURFACE_ANDROID_DEVICE_OWNER,
                proof::SURFACE_ANDROID_PACKAGE_LIFECYCLE,
                proof::SURFACE_ANDROID_STORE,
                proof::SURFACE_IOS_FAMILY_CONTROLS,
                proof::SURFACE_IOS_SIGNING,
                proof::SURFACE_IOS_TESTFLIGHT,
                proof::SURFACE_IOS_STORE,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08CrossPlatformEnforcementCapabilityName {
    #[serde(rename = "headless-agent-service")]
    HeadlessAgentService,
    #[serde(rename = "owned-process-terminate")]
    OwnedProcessTerminate,
    #[serde(rename = "app-time-limit")]
    AppTimeLimit,
    #[serde(rename = "app-blocking")]
    AppBlocking,
    #[serde(rename = "network-domain-blocking")]
    NetworkDomainBlocking,
    #[serde(rename = "managed-browser-control")]
    ManagedBrowserControl,
    #[serde(rename = "unmanaged-browser-detection")]
    UnmanagedBrowserDetection,
    #[serde(rename = "device-owner-policy")]
    DeviceOwnerPolicy,
    #[serde(rename = "package-lifecycle")]
    PackageLifecycle,
    #[serde(rename = "family-controls-entitlement")]
    FamilyControlsEntitlement,
    #[serde(rename = "signing-entitlements")]
    SigningEntitlements,
    #[serde(rename = "testflight-distribution")]
    TestflightDistribution,
    #[serde(rename = "store-distribution")]
    StoreDistribution,
}

impl V08CrossPlatformEnforcementCapabilityName {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::CAPABILITY_HEADLESS_AGENT_SERVICE,
                proof::CAPABILITY_OWNED_PROCESS_TERMINATE,
                proof::CAPABILITY_APP_TIME_LIMIT,
                proof::CAPABILITY_APP_BLOCKING,
                proof::CAPABILITY_NETWORK_DOMAIN_BLOCKING,
                proof::CAPABILITY_MANAGED_BROWSER_CONTROL,
                proof::CAPABILITY_UNMANAGED_BROWSER_DETECTION,
                proof::CAPABILITY_DEVICE_OWNER_POLICY,
                proof::CAPABILITY_PACKAGE_LIFECYCLE,
                proof::CAPABILITY_FAMILY_CONTROLS,
                proof::CAPABILITY_SIGNING_ENTITLEMENTS,
                proof::CAPABILITY_TESTFLIGHT_DISTRIBUTION,
                proof::CAPABILITY_STORE_DISTRIBUTION,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08CrossPlatformCapabilityStatus {
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "implemented")]
    Implemented,
    #[serde(rename = "preview-scaffold")]
    PreviewScaffold,
    #[serde(rename = "scaffold")]
    Scaffold,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "not-implemented")]
    NotImplemented,
}

impl V08CrossPlatformCapabilityStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::STATUS_SUPPORTED,
                proof::STATUS_IMPLEMENTED,
                proof::STATUS_PREVIEW_SCAFFOLD,
                proof::STATUS_SCAFFOLD,
                proof::STATUS_MANUAL_REQUIRED,
                proof::STATUS_UNAVAILABLE,
                proof::STATUS_PLANNED,
                proof::STATUS_NOT_IMPLEMENTED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08CrossPlatformEnforcementCapabilityClaimState {
    #[serde(rename = "implemented-boundary")]
    ImplementedBoundary,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "scaffold")]
    Scaffold,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

impl V08CrossPlatformEnforcementCapabilityClaimState {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::CLAIM_IMPLEMENTED_BOUNDARY,
                proof::CLAIM_MANUAL_REQUIRED,
                proof::CLAIM_SCAFFOLD,
                proof::CLAIM_UNAVAILABLE,
                proof::CLAIM_PLANNED,
                proof::CLAIM_NOT_CLAIMED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08CrossPlatformAdapterExecutionState {
    #[serde(rename = "executes-real-service")]
    ExecutesRealService,
    #[serde(rename = "returns-manual-required")]
    ReturnsManualRequired,
    #[serde(rename = "returns-unavailable")]
    ReturnsUnavailable,
    #[serde(rename = "scaffold-only")]
    ScaffoldOnly,
    #[serde(rename = "not-invoked")]
    NotInvoked,
}

impl V08CrossPlatformAdapterExecutionState {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::EXECUTES_REAL_SERVICE,
                proof::RETURNS_MANUAL_REQUIRED,
                proof::RETURNS_UNAVAILABLE,
                proof::SCAFFOLD_ONLY,
                proof::NOT_INVOKED,
            ]
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08CrossPlatformEnforcementCapabilityProofEntry {
    pub schema_version: String,
    pub proof_entry_id: String,
    pub surface: V08CrossPlatformEnforcementCapabilitySurface,
    pub platform: ParentPlatform,
    pub capability: V08CrossPlatformEnforcementCapabilityName,
    pub capability_status: V08CrossPlatformCapabilityStatus,
    pub product_claim_state: V08CrossPlatformEnforcementCapabilityClaimState,
    pub adapter_execution_state: V08CrossPlatformAdapterExecutionState,
    pub linked_proof_commands: Vec<String>,
    pub linked_proof_artifacts: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub broad_blocking_claimed: bool,
    pub exact_url_claimed: bool,
    pub privileged_mobile_claimed: bool,
    pub production_distribution_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08CrossPlatformEnforcementCapabilityProofReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<V08CrossPlatformEnforcementCapabilityProofEntry>,
}
