use super::constants::*;
use serde::{Deserialize, Serialize};

const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_VARIANTS: [&str; 11] = [
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_PACKAGE_LIFECYCLE,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_TYPED_PROTOCOL_BRIDGE,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_FAMILY_CONTROLS_ENTITLEMENT,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_DEVICE_ACTIVITY,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_SCREEN_TIME_API,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_NETWORK_EXTENSION,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_NOTIFICATIONS,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_BACKGROUND_EXECUTION,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_SIGNING_ENTITLEMENTS,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_TESTFLIGHT_DISTRIBUTION,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STORE_DISTRIBUTION,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementParentCapability {
    #[serde(rename = "package-lifecycle")]
    PackageLifecycle,
    #[serde(rename = "typed-protocol-bridge")]
    TypedProtocolBridge,
    #[serde(rename = "family-controls-entitlement")]
    FamilyControlsEntitlement,
    #[serde(rename = "device-activity")]
    DeviceActivity,
    #[serde(rename = "screen-time-api")]
    ScreenTimeApi,
    #[serde(rename = "network-extension")]
    NetworkExtension,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "background-execution")]
    BackgroundExecution,
    #[serde(rename = "signing-entitlements")]
    SigningEntitlements,
    #[serde(rename = "testflight-distribution")]
    TestflightDistribution,
    #[serde(rename = "store-distribution")]
    StoreDistribution,
}

impl ChildIosEntitlementParentCapability {
    pub const VARIANTS: &'static [&'static str] = &CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_VARIANTS: [&str; 3] = [
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_MANUAL_REQUIRED,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_SCAFFOLD,
    CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_PLANNED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementParentCapabilityStatus {
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "scaffold")]
    Scaffold,
    #[serde(rename = "planned")]
    Planned,
}

impl ChildIosEntitlementParentCapabilityStatus {
    pub const VARIANTS: &'static [&'static str] =
        &CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const CHILD_IOS_ENTITLEMENT_SURFACE_NAME_VARIANTS: [&str; 15] = [
    CHILD_IOS_ENTITLEMENT_SURFACE_SIMULATOR_APP_TARGET,
    CHILD_IOS_ENTITLEMENT_SURFACE_BUNDLE_IDENTIFIER,
    CHILD_IOS_ENTITLEMENT_SURFACE_STATUS_SURFACE,
    CHILD_IOS_ENTITLEMENT_SURFACE_FAMILY_CONTROLS_ENTITLEMENT,
    CHILD_IOS_ENTITLEMENT_SURFACE_DEVICE_ACTIVITY_FRAMEWORK,
    CHILD_IOS_ENTITLEMENT_SURFACE_SCREEN_TIME_API,
    CHILD_IOS_ENTITLEMENT_SURFACE_NETWORK_EXTENSION,
    CHILD_IOS_ENTITLEMENT_SURFACE_NOTIFICATIONS_PERMISSION,
    CHILD_IOS_ENTITLEMENT_SURFACE_BACKGROUND_EXECUTION,
    CHILD_IOS_ENTITLEMENT_SURFACE_PROVISIONING_PROFILE,
    CHILD_IOS_ENTITLEMENT_SURFACE_SUPERVISION_STATE,
    CHILD_IOS_ENTITLEMENT_SURFACE_SIGNING_ENTITLEMENTS,
    CHILD_IOS_ENTITLEMENT_SURFACE_TESTFLIGHT_DISTRIBUTION,
    CHILD_IOS_ENTITLEMENT_SURFACE_PHYSICAL_DEVICE_PROOF,
    CHILD_IOS_ENTITLEMENT_SURFACE_APP_STORE_DISTRIBUTION,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementSurfaceName {
    #[serde(rename = "simulator-app-target")]
    SimulatorAppTarget,
    #[serde(rename = "bundle-identifier")]
    BundleIdentifier,
    #[serde(rename = "status-surface")]
    StatusSurface,
    #[serde(rename = "family-controls-entitlement")]
    FamilyControlsEntitlement,
    #[serde(rename = "device-activity-framework")]
    DeviceActivityFramework,
    #[serde(rename = "screen-time-api")]
    ScreenTimeApi,
    #[serde(rename = "network-extension")]
    NetworkExtension,
    #[serde(rename = "notifications-permission")]
    NotificationsPermission,
    #[serde(rename = "background-execution")]
    BackgroundExecution,
    #[serde(rename = "provisioning-profile")]
    ProvisioningProfile,
    #[serde(rename = "supervision-state")]
    SupervisionState,
    #[serde(rename = "signing-entitlements")]
    SigningEntitlements,
    #[serde(rename = "testflight-distribution")]
    TestflightDistribution,
    #[serde(rename = "physical-device-proof")]
    PhysicalDeviceProof,
    #[serde(rename = "app-store-distribution")]
    AppStoreDistribution,
}

impl ChildIosEntitlementSurfaceName {
    pub const VARIANTS: &'static [&'static str] = &CHILD_IOS_ENTITLEMENT_SURFACE_NAME_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const CHILD_IOS_ENTITLEMENT_PROOF_STATE_VARIANTS: [&str; 9] = [
    CHILD_IOS_ENTITLEMENT_PROOF_STATE_CI_MECHANICAL_PROOF,
    CHILD_IOS_ENTITLEMENT_PROOF_STATE_SIMULATOR_SCAFFOLD,
    CHILD_IOS_ENTITLEMENT_PROOF_STATE_MANUAL_REQUIRED,
    CHILD_IOS_ENTITLEMENT_PROOF_STATE_ENTITLEMENT_REQUIRED,
    CHILD_IOS_ENTITLEMENT_PROOF_STATE_SIGNING_REQUIRED,
    CHILD_IOS_ENTITLEMENT_PROOF_STATE_DEVICE_PROOF_REQUIRED,
    CHILD_IOS_ENTITLEMENT_PROOF_STATE_NOT_DECLARED,
    CHILD_IOS_ENTITLEMENT_PROOF_STATE_NOT_IMPLEMENTED,
    CHILD_IOS_ENTITLEMENT_PROOF_STATE_PLANNED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementProofState {
    #[serde(rename = "ci-mechanical-proof")]
    CiMechanicalProof,
    #[serde(rename = "simulator-scaffold")]
    SimulatorScaffold,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "entitlement-required")]
    EntitlementRequired,
    #[serde(rename = "signing-required")]
    SigningRequired,
    #[serde(rename = "device-proof-required")]
    DeviceProofRequired,
    #[serde(rename = "not-declared")]
    NotDeclared,
    #[serde(rename = "not-implemented")]
    NotImplemented,
    #[serde(rename = "planned")]
    Planned,
}

impl ChildIosEntitlementProofState {
    pub const VARIANTS: &'static [&'static str] = &CHILD_IOS_ENTITLEMENT_PROOF_STATE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}
