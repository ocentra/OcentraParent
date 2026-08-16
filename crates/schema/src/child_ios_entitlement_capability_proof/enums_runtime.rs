use super::constants::*;
use serde::{Deserialize, Serialize};

const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_VARIANTS: [&str; 14] = [
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_XCODE_PROJECT,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_INFO_PLIST,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_SWIFT_SCAFFOLD,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_SIMULATOR_BUILD_SCRIPT,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_SIMULATOR_HOST,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_ENTITLEMENT,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_DEVICE_FRAMEWORK,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_NETWORK_EXTENSION,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_NOTIFICATION_PERMISSION,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_BACKGROUND_MODE,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_SIGNING,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_TESTFLIGHT,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_DEVICE_PROOF,
    CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APP_STORE_CONNECT,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementRuntimeOwner {
    #[serde(rename = "ios-xcode-project")]
    IosXcodeProject,
    #[serde(rename = "ios-info-plist")]
    IosInfoPlist,
    #[serde(rename = "ios-swift-scaffold")]
    IosSwiftScaffold,
    #[serde(rename = "ios-simulator-build-script")]
    IosSimulatorBuildScript,
    #[serde(rename = "apple-simulator-host")]
    AppleSimulatorHost,
    #[serde(rename = "apple-entitlement")]
    AppleEntitlement,
    #[serde(rename = "apple-device-framework")]
    AppleDeviceFramework,
    #[serde(rename = "apple-network-extension")]
    AppleNetworkExtension,
    #[serde(rename = "apple-notification-permission")]
    AppleNotificationPermission,
    #[serde(rename = "apple-background-mode")]
    AppleBackgroundMode,
    #[serde(rename = "apple-signing")]
    AppleSigning,
    #[serde(rename = "apple-testflight")]
    AppleTestflight,
    #[serde(rename = "apple-device-proof")]
    AppleDeviceProof,
    #[serde(rename = "app-store-connect")]
    AppStoreConnect,
}

impl ChildIosEntitlementRuntimeOwner {
    pub const VARIANTS: &'static [&'static str] = &CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_VARIANTS: [&str; 5] = [
    CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_DECLARED_IN_PROJECT,
    CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_DECLARED_IN_PLIST,
    CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_SCAFFOLD_STATUS_LABEL,
    CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_NOT_DECLARED,
    CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_NOT_APPLICABLE,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementDeclarationState {
    #[serde(rename = "declared-in-project")]
    DeclaredInProject,
    #[serde(rename = "declared-in-plist")]
    DeclaredInPlist,
    #[serde(rename = "scaffold-status-label")]
    ScaffoldStatusLabel,
    #[serde(rename = "not-declared")]
    NotDeclared,
    #[serde(rename = "not-applicable")]
    NotApplicable,
}

impl ChildIosEntitlementDeclarationState {
    pub const VARIANTS: &'static [&'static str] = &CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_VARIANTS: [&str; 13] = [
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_XCODE_PROJECT_TARGET,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_BUNDLE_IDENTIFIER,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIMULATOR_BUILD_SCRIPT,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_STATUS_VIEW,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_INFO_PLIST,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIMULATOR_BUILD,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIMULATOR_LAUNCH,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_DEVICE_INSTALL,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_DEVICE_LAUNCH,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_TESTFLIGHT_INSTALL,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIGNING_PROFILE,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_ENTITLEMENT_REVIEW,
    CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_RECOVERY_BEHAVIOR,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementPackagePhase {
    #[serde(rename = "xcode-project-target")]
    XcodeProjectTarget,
    #[serde(rename = "bundle-identifier")]
    BundleIdentifier,
    #[serde(rename = "simulator-build-script")]
    SimulatorBuildScript,
    #[serde(rename = "status-view")]
    StatusView,
    #[serde(rename = "info-plist")]
    InfoPlist,
    #[serde(rename = "simulator-build")]
    SimulatorBuild,
    #[serde(rename = "simulator-launch")]
    SimulatorLaunch,
    #[serde(rename = "device-install")]
    DeviceInstall,
    #[serde(rename = "device-launch")]
    DeviceLaunch,
    #[serde(rename = "testflight-install")]
    TestflightInstall,
    #[serde(rename = "signing-profile")]
    SigningProfile,
    #[serde(rename = "entitlement-review")]
    EntitlementReview,
    #[serde(rename = "recovery-behavior")]
    RecoveryBehavior,
}

impl ChildIosEntitlementPackagePhase {
    pub const VARIANTS: &'static [&'static str] = &CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_VARIANTS: [&str; 3] = [
    CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_CAPABILITY_SNAPSHOT_GET,
    CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_PACKAGE_PROOF_GET,
    CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_MANUAL_PROOF_GET,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementProtocolCommand {
    #[serde(rename = "child.ios.entitlement.capability.snapshot.get")]
    CapabilitySnapshotGet,
    #[serde(rename = "child.ios.entitlement.package.proof.get")]
    PackageProofGet,
    #[serde(rename = "child.ios.entitlement.manual-proof.get")]
    ManualProofGet,
}

impl ChildIosEntitlementProtocolCommand {
    pub const VARIANTS: &'static [&'static str] = &CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_VARIANTS: [&str; 3] = [
    CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_CAPABILITY_SNAPSHOT_REPORTED,
    CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_PACKAGE_PROOF_REPORTED,
    CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_MANUAL_PROOF_REPORTED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementProtocolEvent {
    #[serde(rename = "child.ios.entitlement.capability.snapshot.reported")]
    CapabilitySnapshotReported,
    #[serde(rename = "child.ios.entitlement.package.proof.reported")]
    PackageProofReported,
    #[serde(rename = "child.ios.entitlement.manual-proof.reported")]
    ManualProofReported,
}

impl ChildIosEntitlementProtocolEvent {
    pub const VARIANTS: &'static [&'static str] = &CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const CHILD_IOS_ENTITLEMENT_BRIDGE_STATE_VARIANTS: [&str; 2] = [
    CHILD_IOS_ENTITLEMENT_BRIDGE_STATE_SIMULATOR_SCAFFOLD,
    CHILD_IOS_ENTITLEMENT_BRIDGE_STATE_NOT_IMPLEMENTED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum ChildIosEntitlementBridgeState {
    #[serde(rename = "simulator-scaffold")]
    SimulatorScaffold,
    #[serde(rename = "not-implemented")]
    NotImplemented,
}

impl ChildIosEntitlementBridgeState {
    pub const VARIANTS: &'static [&'static str] = &CHILD_IOS_ENTITLEMENT_BRIDGE_STATE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}
