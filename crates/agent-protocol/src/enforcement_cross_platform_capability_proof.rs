use serde::{Deserialize, Serialize};

use crate::{constants::v08_cross_platform_enforcement_capability_proof as proof, ParentPlatform};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::WindowsOwnedProcessTerminate => proof::SURFACE_WINDOWS_OWNED_PROCESS,
            Self::WindowsAppTimeLimitLifecycle => proof::SURFACE_WINDOWS_APP_TIME_LIMIT,
            Self::WindowsManagedBrowserBoundary => proof::SURFACE_WINDOWS_MANAGED_BROWSER,
            Self::WindowsUnmanagedBrowserProcessBoundary => {
                proof::SURFACE_WINDOWS_UNMANAGED_BROWSER
            }
            Self::WindowsBroadInstalledAppBlocking => proof::SURFACE_WINDOWS_BROAD_APP,
            Self::WindowsNetworkDomainBlocking => proof::SURFACE_WINDOWS_NETWORK_DOMAIN,
            Self::LinuxEnforcementAdapterScaffold => proof::SURFACE_LINUX_ADAPTER_SCAFFOLD,
            Self::MacosEnforcementAdapterScaffold => proof::SURFACE_MACOS_ADAPTER_SCAFFOLD,
            Self::AndroidDeviceOwnerPolicy => proof::SURFACE_ANDROID_DEVICE_OWNER,
            Self::AndroidPackageLifecycle => proof::SURFACE_ANDROID_PACKAGE_LIFECYCLE,
            Self::AndroidStoreDistribution => proof::SURFACE_ANDROID_STORE,
            Self::IosFamilyControls => proof::SURFACE_IOS_FAMILY_CONTROLS,
            Self::IosSigningEntitlements => proof::SURFACE_IOS_SIGNING,
            Self::IosTestflightDistribution => proof::SURFACE_IOS_TESTFLIGHT,
            Self::IosStoreDistribution => proof::SURFACE_IOS_STORE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::HeadlessAgentService => proof::CAPABILITY_HEADLESS_AGENT_SERVICE,
            Self::OwnedProcessTerminate => proof::CAPABILITY_OWNED_PROCESS_TERMINATE,
            Self::AppTimeLimit => proof::CAPABILITY_APP_TIME_LIMIT,
            Self::AppBlocking => proof::CAPABILITY_APP_BLOCKING,
            Self::NetworkDomainBlocking => proof::CAPABILITY_NETWORK_DOMAIN_BLOCKING,
            Self::ManagedBrowserControl => proof::CAPABILITY_MANAGED_BROWSER_CONTROL,
            Self::UnmanagedBrowserDetection => proof::CAPABILITY_UNMANAGED_BROWSER_DETECTION,
            Self::DeviceOwnerPolicy => proof::CAPABILITY_DEVICE_OWNER_POLICY,
            Self::PackageLifecycle => proof::CAPABILITY_PACKAGE_LIFECYCLE,
            Self::FamilyControlsEntitlement => proof::CAPABILITY_FAMILY_CONTROLS,
            Self::SigningEntitlements => proof::CAPABILITY_SIGNING_ENTITLEMENTS,
            Self::TestflightDistribution => proof::CAPABILITY_TESTFLIGHT_DISTRIBUTION,
            Self::StoreDistribution => proof::CAPABILITY_STORE_DISTRIBUTION,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::Supported => proof::STATUS_SUPPORTED,
            Self::Implemented => proof::STATUS_IMPLEMENTED,
            Self::PreviewScaffold => proof::STATUS_PREVIEW_SCAFFOLD,
            Self::Scaffold => proof::STATUS_SCAFFOLD,
            Self::ManualRequired => proof::STATUS_MANUAL_REQUIRED,
            Self::Unavailable => proof::STATUS_UNAVAILABLE,
            Self::Planned => proof::STATUS_PLANNED,
            Self::NotImplemented => proof::STATUS_NOT_IMPLEMENTED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::ImplementedBoundary => proof::CLAIM_IMPLEMENTED_BOUNDARY,
            Self::ManualRequired => proof::CLAIM_MANUAL_REQUIRED,
            Self::Scaffold => proof::CLAIM_SCAFFOLD,
            Self::Unavailable => proof::CLAIM_UNAVAILABLE,
            Self::Planned => proof::CLAIM_PLANNED,
            Self::NotClaimed => proof::CLAIM_NOT_CLAIMED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::ExecutesRealService => proof::EXECUTES_REAL_SERVICE,
            Self::ReturnsManualRequired => proof::RETURNS_MANUAL_REQUIRED,
            Self::ReturnsUnavailable => proof::RETURNS_UNAVAILABLE,
            Self::ScaffoldOnly => proof::SCAFFOLD_ONLY,
            Self::NotInvoked => proof::NOT_INVOKED,
        }
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
