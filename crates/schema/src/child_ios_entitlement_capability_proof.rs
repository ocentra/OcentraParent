use std::fmt::{Display, Formatter};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

macro_rules! ios_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! ios_string_enum {
    ($name:ident { $($variant:ident => $const_ident:ident),+ $(,)? }) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum $name {
            $(
                $variant,
            )+
        }

        impl $name {
            pub const TYPE_NAME: &'static str = stringify!($name);
            pub const VARIANTS: &'static [&'static str] = &[
                $(
                    $const_ident,
                )+
            ];

            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $const_ident,)+
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct EnumVisitor;

                impl<'de> Visitor<'de> for EnumVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str($name::TYPE_NAME)
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            $($const_ident => Ok($name::$variant),)+
                            _ => Err(E::unknown_variant(value, $name::VARIANTS)),
                        }
                    }
                }

                deserializer.deserialize_str(EnumVisitor)
            }
        }
    };
}

pub const CHILD_IOS_ENTITLEMENT_CAPABILITY_PROOF_SCHEMA_VERSION: &str =
    "child-ios-entitlement-capability-proof";
pub const CHILD_IOS_ENTITLEMENT_BUNDLE_ID: &str = "ca.ocentra.parent.agent";
pub const CHILD_IOS_ENTITLEMENT_CLASS_NAME: &str = "AgentStatusViewController";
pub const CHILD_IOS_ENTITLEMENT_UPDATED_AT: &str = "2026-05-31T00:00:00.000Z";

pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_PACKAGE_LIFECYCLE: &str = "package-lifecycle";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_TYPED_PROTOCOL_BRIDGE: &str =
    "typed-protocol-bridge";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_FAMILY_CONTROLS_ENTITLEMENT: &str =
    "family-controls-entitlement";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_DEVICE_ACTIVITY: &str = "device-activity";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_SCREEN_TIME_API: &str = "screen-time-api";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_NETWORK_EXTENSION: &str = "network-extension";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_NOTIFICATIONS: &str = "notifications";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_BACKGROUND_EXECUTION: &str =
    "background-execution";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_SIGNING_ENTITLEMENTS: &str =
    "signing-entitlements";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_TESTFLIGHT_DISTRIBUTION: &str =
    "testflight-distribution";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STORE_DISTRIBUTION: &str = "store-distribution";

pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_SCAFFOLD: &str = "scaffold";
pub const CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_PLANNED: &str = "planned";

pub const CHILD_IOS_ENTITLEMENT_SURFACE_SIMULATOR_APP_TARGET: &str = "simulator-app-target";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_BUNDLE_IDENTIFIER: &str = "bundle-identifier";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_STATUS_SURFACE: &str = "status-surface";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_FAMILY_CONTROLS_ENTITLEMENT: &str =
    "family-controls-entitlement";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_DEVICE_ACTIVITY_FRAMEWORK: &str =
    "device-activity-framework";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_SCREEN_TIME_API: &str = "screen-time-api";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_NETWORK_EXTENSION: &str = "network-extension";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_NOTIFICATIONS_PERMISSION: &str = "notifications-permission";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_BACKGROUND_EXECUTION: &str = "background-execution";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_PROVISIONING_PROFILE: &str = "provisioning-profile";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_SUPERVISION_STATE: &str = "supervision-state";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_SIGNING_ENTITLEMENTS: &str = "signing-entitlements";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_TESTFLIGHT_DISTRIBUTION: &str = "testflight-distribution";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_PHYSICAL_DEVICE_PROOF: &str = "physical-device-proof";
pub const CHILD_IOS_ENTITLEMENT_SURFACE_APP_STORE_DISTRIBUTION: &str = "app-store-distribution";

pub const CHILD_IOS_ENTITLEMENT_PROOF_STATE_CI_MECHANICAL_PROOF: &str = "ci-mechanical-proof";
pub const CHILD_IOS_ENTITLEMENT_PROOF_STATE_SIMULATOR_SCAFFOLD: &str = "simulator-scaffold";
pub const CHILD_IOS_ENTITLEMENT_PROOF_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const CHILD_IOS_ENTITLEMENT_PROOF_STATE_ENTITLEMENT_REQUIRED: &str = "entitlement-required";
pub const CHILD_IOS_ENTITLEMENT_PROOF_STATE_SIGNING_REQUIRED: &str = "signing-required";
pub const CHILD_IOS_ENTITLEMENT_PROOF_STATE_DEVICE_PROOF_REQUIRED: &str = "device-proof-required";
pub const CHILD_IOS_ENTITLEMENT_PROOF_STATE_NOT_DECLARED: &str = "not-declared";
pub const CHILD_IOS_ENTITLEMENT_PROOF_STATE_NOT_IMPLEMENTED: &str = "not-implemented";
pub const CHILD_IOS_ENTITLEMENT_PROOF_STATE_PLANNED: &str = "planned";

pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_XCODE_PROJECT: &str = "ios-xcode-project";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_INFO_PLIST: &str = "ios-info-plist";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_SWIFT_SCAFFOLD: &str = "ios-swift-scaffold";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_SIMULATOR_BUILD_SCRIPT: &str =
    "ios-simulator-build-script";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_SIMULATOR_HOST: &str = "apple-simulator-host";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_ENTITLEMENT: &str = "apple-entitlement";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_DEVICE_FRAMEWORK: &str =
    "apple-device-framework";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_NETWORK_EXTENSION: &str =
    "apple-network-extension";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_NOTIFICATION_PERMISSION: &str =
    "apple-notification-permission";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_BACKGROUND_MODE: &str = "apple-background-mode";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_SIGNING: &str = "apple-signing";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_TESTFLIGHT: &str = "apple-testflight";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_DEVICE_PROOF: &str = "apple-device-proof";
pub const CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APP_STORE_CONNECT: &str = "app-store-connect";

pub const CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_DECLARED_IN_PROJECT: &str = "declared-in-project";
pub const CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_DECLARED_IN_PLIST: &str = "declared-in-plist";
pub const CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_SCAFFOLD_STATUS_LABEL: &str =
    "scaffold-status-label";
pub const CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_NOT_DECLARED: &str = "not-declared";
pub const CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_NOT_APPLICABLE: &str = "not-applicable";

pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_XCODE_PROJECT_TARGET: &str = "xcode-project-target";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_BUNDLE_IDENTIFIER: &str = "bundle-identifier";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIMULATOR_BUILD_SCRIPT: &str =
    "simulator-build-script";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_STATUS_VIEW: &str = "status-view";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_INFO_PLIST: &str = "info-plist";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIMULATOR_BUILD: &str = "simulator-build";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIMULATOR_LAUNCH: &str = "simulator-launch";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_DEVICE_INSTALL: &str = "device-install";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_DEVICE_LAUNCH: &str = "device-launch";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_TESTFLIGHT_INSTALL: &str = "testflight-install";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIGNING_PROFILE: &str = "signing-profile";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_ENTITLEMENT_REVIEW: &str = "entitlement-review";
pub const CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_RECOVERY_BEHAVIOR: &str = "recovery-behavior";

pub const CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_CAPABILITY_SNAPSHOT_GET: &str =
    "child.ios.entitlement.capability.snapshot.get";
pub const CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_PACKAGE_PROOF_GET: &str =
    "child.ios.entitlement.package.proof.get";
pub const CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_MANUAL_PROOF_GET: &str =
    "child.ios.entitlement.manual-proof.get";

pub const CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_CAPABILITY_SNAPSHOT_REPORTED: &str =
    "child.ios.entitlement.capability.snapshot.reported";
pub const CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_PACKAGE_PROOF_REPORTED: &str =
    "child.ios.entitlement.package.proof.reported";
pub const CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_MANUAL_PROOF_REPORTED: &str =
    "child.ios.entitlement.manual-proof.reported";

pub const CHILD_IOS_ENTITLEMENT_BRIDGE_STATE_SIMULATOR_SCAFFOLD: &str = "simulator-scaffold";
pub const CHILD_IOS_ENTITLEMENT_BRIDGE_STATE_NOT_IMPLEMENTED: &str = "not-implemented";

ios_string_enum!(ChildIosEntitlementParentCapability {
    PackageLifecycle => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_PACKAGE_LIFECYCLE,
    TypedProtocolBridge => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_TYPED_PROTOCOL_BRIDGE,
    FamilyControlsEntitlement => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_FAMILY_CONTROLS_ENTITLEMENT,
    DeviceActivity => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_DEVICE_ACTIVITY,
    ScreenTimeApi => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_SCREEN_TIME_API,
    NetworkExtension => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_NETWORK_EXTENSION,
    Notifications => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_NOTIFICATIONS,
    BackgroundExecution => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_BACKGROUND_EXECUTION,
    SigningEntitlements => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_SIGNING_ENTITLEMENTS,
    TestflightDistribution => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_TESTFLIGHT_DISTRIBUTION,
    StoreDistribution => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STORE_DISTRIBUTION,
});

ios_string_enum!(ChildIosEntitlementParentCapabilityStatus {
    ManualRequired => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_MANUAL_REQUIRED,
    Scaffold => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_SCAFFOLD,
    Planned => CHILD_IOS_ENTITLEMENT_PARENT_CAPABILITY_STATUS_PLANNED,
});

ios_string_enum!(ChildIosEntitlementSurfaceName {
    SimulatorAppTarget => CHILD_IOS_ENTITLEMENT_SURFACE_SIMULATOR_APP_TARGET,
    BundleIdentifier => CHILD_IOS_ENTITLEMENT_SURFACE_BUNDLE_IDENTIFIER,
    StatusSurface => CHILD_IOS_ENTITLEMENT_SURFACE_STATUS_SURFACE,
    FamilyControlsEntitlement => CHILD_IOS_ENTITLEMENT_SURFACE_FAMILY_CONTROLS_ENTITLEMENT,
    DeviceActivityFramework => CHILD_IOS_ENTITLEMENT_SURFACE_DEVICE_ACTIVITY_FRAMEWORK,
    ScreenTimeApi => CHILD_IOS_ENTITLEMENT_SURFACE_SCREEN_TIME_API,
    NetworkExtension => CHILD_IOS_ENTITLEMENT_SURFACE_NETWORK_EXTENSION,
    NotificationsPermission => CHILD_IOS_ENTITLEMENT_SURFACE_NOTIFICATIONS_PERMISSION,
    BackgroundExecution => CHILD_IOS_ENTITLEMENT_SURFACE_BACKGROUND_EXECUTION,
    ProvisioningProfile => CHILD_IOS_ENTITLEMENT_SURFACE_PROVISIONING_PROFILE,
    SupervisionState => CHILD_IOS_ENTITLEMENT_SURFACE_SUPERVISION_STATE,
    SigningEntitlements => CHILD_IOS_ENTITLEMENT_SURFACE_SIGNING_ENTITLEMENTS,
    TestflightDistribution => CHILD_IOS_ENTITLEMENT_SURFACE_TESTFLIGHT_DISTRIBUTION,
    PhysicalDeviceProof => CHILD_IOS_ENTITLEMENT_SURFACE_PHYSICAL_DEVICE_PROOF,
    AppStoreDistribution => CHILD_IOS_ENTITLEMENT_SURFACE_APP_STORE_DISTRIBUTION,
});

ios_string_enum!(ChildIosEntitlementProofState {
    CiMechanicalProof => CHILD_IOS_ENTITLEMENT_PROOF_STATE_CI_MECHANICAL_PROOF,
    SimulatorScaffold => CHILD_IOS_ENTITLEMENT_PROOF_STATE_SIMULATOR_SCAFFOLD,
    ManualRequired => CHILD_IOS_ENTITLEMENT_PROOF_STATE_MANUAL_REQUIRED,
    EntitlementRequired => CHILD_IOS_ENTITLEMENT_PROOF_STATE_ENTITLEMENT_REQUIRED,
    SigningRequired => CHILD_IOS_ENTITLEMENT_PROOF_STATE_SIGNING_REQUIRED,
    DeviceProofRequired => CHILD_IOS_ENTITLEMENT_PROOF_STATE_DEVICE_PROOF_REQUIRED,
    NotDeclared => CHILD_IOS_ENTITLEMENT_PROOF_STATE_NOT_DECLARED,
    NotImplemented => CHILD_IOS_ENTITLEMENT_PROOF_STATE_NOT_IMPLEMENTED,
    Planned => CHILD_IOS_ENTITLEMENT_PROOF_STATE_PLANNED,
});

ios_string_enum!(ChildIosEntitlementRuntimeOwner {
    IosXcodeProject => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_XCODE_PROJECT,
    IosInfoPlist => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_INFO_PLIST,
    IosSwiftScaffold => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_SWIFT_SCAFFOLD,
    IosSimulatorBuildScript => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_IOS_SIMULATOR_BUILD_SCRIPT,
    AppleSimulatorHost => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_SIMULATOR_HOST,
    AppleEntitlement => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_ENTITLEMENT,
    AppleDeviceFramework => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_DEVICE_FRAMEWORK,
    AppleNetworkExtension => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_NETWORK_EXTENSION,
    AppleNotificationPermission => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_NOTIFICATION_PERMISSION,
    AppleBackgroundMode => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_BACKGROUND_MODE,
    AppleSigning => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_SIGNING,
    AppleTestflight => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_TESTFLIGHT,
    AppleDeviceProof => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APPLE_DEVICE_PROOF,
    AppStoreConnect => CHILD_IOS_ENTITLEMENT_RUNTIME_OWNER_APP_STORE_CONNECT,
});

ios_string_enum!(ChildIosEntitlementDeclarationState {
    DeclaredInProject => CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_DECLARED_IN_PROJECT,
    DeclaredInPlist => CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_DECLARED_IN_PLIST,
    ScaffoldStatusLabel => CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_SCAFFOLD_STATUS_LABEL,
    NotDeclared => CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_NOT_DECLARED,
    NotApplicable => CHILD_IOS_ENTITLEMENT_DECLARATION_STATE_NOT_APPLICABLE,
});

ios_string_enum!(ChildIosEntitlementPackagePhase {
    XcodeProjectTarget => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_XCODE_PROJECT_TARGET,
    BundleIdentifier => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_BUNDLE_IDENTIFIER,
    SimulatorBuildScript => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIMULATOR_BUILD_SCRIPT,
    StatusView => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_STATUS_VIEW,
    InfoPlist => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_INFO_PLIST,
    SimulatorBuild => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIMULATOR_BUILD,
    SimulatorLaunch => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIMULATOR_LAUNCH,
    DeviceInstall => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_DEVICE_INSTALL,
    DeviceLaunch => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_DEVICE_LAUNCH,
    TestflightInstall => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_TESTFLIGHT_INSTALL,
    SigningProfile => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_SIGNING_PROFILE,
    EntitlementReview => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_ENTITLEMENT_REVIEW,
    RecoveryBehavior => CHILD_IOS_ENTITLEMENT_PACKAGE_PHASE_RECOVERY_BEHAVIOR,
});

ios_string_enum!(ChildIosEntitlementProtocolCommand {
    CapabilitySnapshotGet => CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_CAPABILITY_SNAPSHOT_GET,
    PackageProofGet => CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_PACKAGE_PROOF_GET,
    ManualProofGet => CHILD_IOS_ENTITLEMENT_PROTOCOL_COMMAND_MANUAL_PROOF_GET,
});

ios_string_enum!(ChildIosEntitlementProtocolEvent {
    CapabilitySnapshotReported => CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_CAPABILITY_SNAPSHOT_REPORTED,
    PackageProofReported => CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_PACKAGE_PROOF_REPORTED,
    ManualProofReported => CHILD_IOS_ENTITLEMENT_PROTOCOL_EVENT_MANUAL_PROOF_REPORTED,
});

ios_string_enum!(ChildIosEntitlementBridgeState {
    SimulatorScaffold => CHILD_IOS_ENTITLEMENT_BRIDGE_STATE_SIMULATOR_SCAFFOLD,
    NotImplemented => CHILD_IOS_ENTITLEMENT_BRIDGE_STATE_NOT_IMPLEMENTED,
});

ios_text_identifier!(ChildIosEntitlementBundleId);
ios_text_identifier!(ChildIosEntitlementClassName);
ios_text_identifier!(ChildIosEntitlementRequirement);
ios_text_identifier!(ChildIosEntitlementBoundary);
ios_text_identifier!(ChildIosEntitlementTimestamp);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementSurfaceProof {
    pub surface: ChildIosEntitlementSurfaceName,
    pub parent_capability: ChildIosEntitlementParentCapability,
    pub parent_capability_status: ChildIosEntitlementParentCapabilityStatus,
    pub declaration_state: ChildIosEntitlementDeclarationState,
    pub proof_state: ChildIosEntitlementProofState,
    pub runtime_owner: ChildIosEntitlementRuntimeOwner,
    pub proof_requirement: ChildIosEntitlementRequirement,
    pub claim_boundary: ChildIosEntitlementBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementPackageLifecycleProof {
    pub phase: ChildIosEntitlementPackagePhase,
    pub proof_state: ChildIosEntitlementProofState,
    pub runtime_owner: ChildIosEntitlementRuntimeOwner,
    pub proof_requirement: ChildIosEntitlementRequirement,
    pub claim_boundary: ChildIosEntitlementBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementProtocolBridgeProof {
    pub bundle_id: ChildIosEntitlementBundleId,
    pub status_surface_class: ChildIosEntitlementClassName,
    pub bridge_state: ChildIosEntitlementBridgeState,
    pub external_transport_state: ChildIosEntitlementBridgeState,
    pub commands: Vec<ChildIosEntitlementProtocolCommand>,
    pub events: Vec<ChildIosEntitlementProtocolEvent>,
    pub runtime_owner: ChildIosEntitlementRuntimeOwner,
    pub proof_requirement: ChildIosEntitlementRequirement,
    pub claim_boundary: ChildIosEntitlementBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementClaimBoundaries {
    pub simulator_package: ChildIosEntitlementBoundary,
    pub launch_availability: ChildIosEntitlementBoundary,
    pub family_controls: ChildIosEntitlementBoundary,
    pub device_activity: ChildIosEntitlementBoundary,
    pub screen_time: ChildIosEntitlementBoundary,
    pub network_extension: ChildIosEntitlementBoundary,
    pub notifications: ChildIosEntitlementBoundary,
    pub background_execution: ChildIosEntitlementBoundary,
    pub recovery_behavior: ChildIosEntitlementBoundary,
    pub provisioning_profile: ChildIosEntitlementBoundary,
    pub supervision: ChildIosEntitlementBoundary,
    pub signing_entitlements: ChildIosEntitlementBoundary,
    pub testflight: ChildIosEntitlementBoundary,
    pub device_proof: ChildIosEntitlementBoundary,
    pub capability_only_state: ChildIosEntitlementBoundary,
    pub external_transport: ChildIosEntitlementBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildIosEntitlementCapabilityReadModel {
    pub schema_version: String,
    pub bundle_id: ChildIosEntitlementBundleId,
    pub status_surface_class: ChildIosEntitlementClassName,
    pub protocol_bridge_proof: ChildIosEntitlementProtocolBridgeProof,
    pub surface_proofs: Vec<ChildIosEntitlementSurfaceProof>,
    pub package_lifecycle_proofs: Vec<ChildIosEntitlementPackageLifecycleProof>,
    pub claim_boundaries: ChildIosEntitlementClaimBoundaries,
    pub updated_at: ChildIosEntitlementTimestamp,
}

fn bundle_id(value: &str) -> ChildIosEntitlementBundleId {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementBundleId::parse(value),
        "child iOS entitlement bundle id",
    )
}

fn class_name(value: &str) -> ChildIosEntitlementClassName {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementClassName::parse(value),
        "child iOS entitlement class name",
    )
}

fn requirement(value: &str) -> ChildIosEntitlementRequirement {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementRequirement::parse(value),
        "child iOS entitlement requirement",
    )
}

fn boundary(value: &str) -> ChildIosEntitlementBoundary {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementBoundary::parse(value),
        "child iOS entitlement boundary",
    )
}

fn timestamp(value: &str) -> ChildIosEntitlementTimestamp {
    crate::schema_option_or_unreachable(
        ChildIosEntitlementTimestamp::parse(value),
        "child iOS entitlement timestamp",
    )
}

fn surface_proof(
    surface: ChildIosEntitlementSurfaceName,
    parent_capability: ChildIosEntitlementParentCapability,
    parent_capability_status: ChildIosEntitlementParentCapabilityStatus,
    declaration_state: ChildIosEntitlementDeclarationState,
    proof_state: ChildIosEntitlementProofState,
    runtime_owner: ChildIosEntitlementRuntimeOwner,
) -> ChildIosEntitlementSurfaceProof {
    let proof_requirement = requirement(&format!(
        "{} remains {} until Apple artifacts change it",
        surface.as_str(),
        proof_state.as_str()
    ));
    let claim_boundary = boundary(proof_requirement.as_str());

    ChildIosEntitlementSurfaceProof {
        surface,
        parent_capability,
        parent_capability_status,
        declaration_state,
        proof_state,
        runtime_owner,
        proof_requirement,
        claim_boundary,
    }
}

fn lifecycle_proof(
    phase: ChildIosEntitlementPackagePhase,
    proof_state: ChildIosEntitlementProofState,
    runtime_owner: ChildIosEntitlementRuntimeOwner,
) -> ChildIosEntitlementPackageLifecycleProof {
    let proof_requirement = requirement(&format!(
        "{} proof state is {}",
        phase.as_str(),
        proof_state.as_str()
    ));
    let claim_boundary = boundary(&format!(
        "{} does not upgrade iOS child capability without entitlement signing or device evidence",
        phase.as_str()
    ));

    ChildIosEntitlementPackageLifecycleProof {
        phase,
        proof_state,
        runtime_owner,
        proof_requirement,
        claim_boundary,
    }
}

fn sample_surface_proofs() -> Vec<ChildIosEntitlementSurfaceProof> {
    let mut proofs = sample_surface_proofs_identity_and_declaration();
    proofs.extend(sample_surface_proofs_runtime_requirements());
    proofs.extend(sample_surface_proofs_distribution_and_proof());
    proofs
}

fn sample_surface_proofs_identity_and_declaration() -> Vec<ChildIosEntitlementSurfaceProof> {
    vec![
        surface_proof(
            ChildIosEntitlementSurfaceName::SimulatorAppTarget,
            ChildIosEntitlementParentCapability::PackageLifecycle,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::DeclaredInProject,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosXcodeProject,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::BundleIdentifier,
            ChildIosEntitlementParentCapability::PackageLifecycle,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::DeclaredInProject,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosXcodeProject,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::StatusSurface,
            ChildIosEntitlementParentCapability::TypedProtocolBridge,
            ChildIosEntitlementParentCapabilityStatus::Scaffold,
            ChildIosEntitlementDeclarationState::ScaffoldStatusLabel,
            ChildIosEntitlementProofState::SimulatorScaffold,
            ChildIosEntitlementRuntimeOwner::IosSwiftScaffold,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::FamilyControlsEntitlement,
            ChildIosEntitlementParentCapability::FamilyControlsEntitlement,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleEntitlement,
        ),
    ]
}

fn sample_surface_proofs_runtime_requirements() -> Vec<ChildIosEntitlementSurfaceProof> {
    vec![
        surface_proof(
            ChildIosEntitlementSurfaceName::DeviceActivityFramework,
            ChildIosEntitlementParentCapability::DeviceActivity,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceFramework,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::ScreenTimeApi,
            ChildIosEntitlementParentCapability::ScreenTimeApi,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceFramework,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::NetworkExtension,
            ChildIosEntitlementParentCapability::NetworkExtension,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleNetworkExtension,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::NotificationsPermission,
            ChildIosEntitlementParentCapability::Notifications,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::ManualRequired,
            ChildIosEntitlementRuntimeOwner::AppleNotificationPermission,
        ),
    ]
}

fn sample_surface_proofs_distribution_and_proof() -> Vec<ChildIosEntitlementSurfaceProof> {
    vec![
        surface_proof(
            ChildIosEntitlementSurfaceName::BackgroundExecution,
            ChildIosEntitlementParentCapability::BackgroundExecution,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotDeclared,
            ChildIosEntitlementProofState::ManualRequired,
            ChildIosEntitlementRuntimeOwner::AppleBackgroundMode,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::ProvisioningProfile,
            ChildIosEntitlementParentCapability::SigningEntitlements,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::SigningRequired,
            ChildIosEntitlementRuntimeOwner::AppleSigning,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::SupervisionState,
            ChildIosEntitlementParentCapability::PackageLifecycle,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceProof,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::SigningEntitlements,
            ChildIosEntitlementParentCapability::SigningEntitlements,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::SigningRequired,
            ChildIosEntitlementRuntimeOwner::AppleSigning,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::TestflightDistribution,
            ChildIosEntitlementParentCapability::TestflightDistribution,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleTestflight,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::PhysicalDeviceProof,
            ChildIosEntitlementParentCapability::PackageLifecycle,
            ChildIosEntitlementParentCapabilityStatus::ManualRequired,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceProof,
        ),
        surface_proof(
            ChildIosEntitlementSurfaceName::AppStoreDistribution,
            ChildIosEntitlementParentCapability::StoreDistribution,
            ChildIosEntitlementParentCapabilityStatus::Planned,
            ChildIosEntitlementDeclarationState::NotApplicable,
            ChildIosEntitlementProofState::Planned,
            ChildIosEntitlementRuntimeOwner::AppStoreConnect,
        ),
    ]
}

fn sample_package_lifecycle_proofs() -> Vec<ChildIosEntitlementPackageLifecycleProof> {
    let mut proofs = sample_package_lifecycle_proofs_project_and_build();
    proofs.extend(sample_package_lifecycle_proofs_runtime_and_install());
    proofs.extend(sample_package_lifecycle_proofs_signing_and_recovery());
    proofs
}

fn sample_package_lifecycle_proofs_project_and_build(
) -> Vec<ChildIosEntitlementPackageLifecycleProof> {
    vec![
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::XcodeProjectTarget,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosXcodeProject,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::BundleIdentifier,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosXcodeProject,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::SimulatorBuildScript,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosSimulatorBuildScript,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::StatusView,
            ChildIosEntitlementProofState::SimulatorScaffold,
            ChildIosEntitlementRuntimeOwner::IosSwiftScaffold,
        ),
    ]
}

fn sample_package_lifecycle_proofs_runtime_and_install(
) -> Vec<ChildIosEntitlementPackageLifecycleProof> {
    vec![
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::InfoPlist,
            ChildIosEntitlementProofState::CiMechanicalProof,
            ChildIosEntitlementRuntimeOwner::IosInfoPlist,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::SimulatorBuild,
            ChildIosEntitlementProofState::ManualRequired,
            ChildIosEntitlementRuntimeOwner::IosSimulatorBuildScript,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::SimulatorLaunch,
            ChildIosEntitlementProofState::ManualRequired,
            ChildIosEntitlementRuntimeOwner::AppleSimulatorHost,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::DeviceInstall,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceProof,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::DeviceLaunch,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleDeviceProof,
        ),
    ]
}

fn sample_package_lifecycle_proofs_signing_and_recovery(
) -> Vec<ChildIosEntitlementPackageLifecycleProof> {
    vec![
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::TestflightInstall,
            ChildIosEntitlementProofState::DeviceProofRequired,
            ChildIosEntitlementRuntimeOwner::AppleTestflight,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::SigningProfile,
            ChildIosEntitlementProofState::SigningRequired,
            ChildIosEntitlementRuntimeOwner::AppleSigning,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::EntitlementReview,
            ChildIosEntitlementProofState::EntitlementRequired,
            ChildIosEntitlementRuntimeOwner::AppleEntitlement,
        ),
        lifecycle_proof(
            ChildIosEntitlementPackagePhase::RecoveryBehavior,
            ChildIosEntitlementProofState::NotImplemented,
            ChildIosEntitlementRuntimeOwner::AppleBackgroundMode,
        ),
    ]
}

pub fn sample_child_ios_entitlement_capability_read_model() -> ChildIosEntitlementCapabilityReadModel
{
    ChildIosEntitlementCapabilityReadModel {
        schema_version: CHILD_IOS_ENTITLEMENT_CAPABILITY_PROOF_SCHEMA_VERSION.to_string(),
        bundle_id: bundle_id(CHILD_IOS_ENTITLEMENT_BUNDLE_ID),
        status_surface_class: class_name(CHILD_IOS_ENTITLEMENT_CLASS_NAME),
        protocol_bridge_proof: ChildIosEntitlementProtocolBridgeProof {
            bundle_id: bundle_id(CHILD_IOS_ENTITLEMENT_BUNDLE_ID),
            status_surface_class: class_name(CHILD_IOS_ENTITLEMENT_CLASS_NAME),
            bridge_state: ChildIosEntitlementBridgeState::SimulatorScaffold,
            external_transport_state: ChildIosEntitlementBridgeState::NotImplemented,
            commands: vec![
                ChildIosEntitlementProtocolCommand::CapabilitySnapshotGet,
                ChildIosEntitlementProtocolCommand::PackageProofGet,
                ChildIosEntitlementProtocolCommand::ManualProofGet,
            ],
            events: vec![
                ChildIosEntitlementProtocolEvent::CapabilitySnapshotReported,
                ChildIosEntitlementProtocolEvent::PackageProofReported,
                ChildIosEntitlementProtocolEvent::ManualProofReported,
            ],
            runtime_owner: ChildIosEntitlementRuntimeOwner::IosSwiftScaffold,
            proof_requirement: requirement("iOS simulator scaffold status surface names capability-only launch, recovery, entitlement, provisioning, and supervision states".to_string().as_str()),
            claim_boundary: boundary("status surface is capability-only; no hidden daemon, persistent background service, launch recovery, external child-agent transport, Apple entitlement, provisioning, or device proof is claimed".to_string().as_str()),
        },
        surface_proofs: sample_surface_proofs(),
        package_lifecycle_proofs: sample_package_lifecycle_proofs(),
        claim_boundaries: ChildIosEntitlementClaimBoundaries {
            simulator_package: boundary("Xcode project target, bundle id, plist, status view, and package script are source proof only".to_string().as_str()),
            launch_availability: boundary("simulator and physical-device launch availability remain manual-required or device-proof-required without Apple host or device artifacts".to_string().as_str()),
            family_controls: boundary("Family Controls remains entitlement-required without Apple approval and device artifacts".to_string().as_str()),
            device_activity: boundary("DeviceActivity remains entitlement-required without schedule and event artifacts".to_string().as_str()),
            screen_time: boundary("Screen Time API remains entitlement-required without authorization and behavior artifacts".to_string().as_str()),
            network_extension: boundary("Network Extension remains entitlement-required without filtering artifacts".to_string().as_str()),
            notifications: boundary("notification authorization and delivery remain manual-required".to_string().as_str()),
            background_execution: boundary("background execution remains manual-required without UIBackgroundModes and device proof".to_string().as_str()),
            recovery_behavior: boundary("launch recovery remains not-implemented; no iOS daemon, relaunch, or persistent background recovery is claimed".to_string().as_str()),
            provisioning_profile: boundary("provisioning remains manual-required without Apple signing credentials, provisioning profile artifacts, and install evidence".to_string().as_str()),
            supervision: boundary("supervision remains manual-required without supervised-device enrollment and device artifacts".to_string().as_str()),
            signing_entitlements: boundary("signing and entitlements remain signing-required; simulator script disables signing".to_string().as_str()),
            testflight: boundary("TestFlight and App Store distribution remain device-proof-required or planned".to_string().as_str()),
            device_proof: boundary("physical-device install and runtime behavior remain device-proof-required".to_string().as_str()),
            capability_only_state: boundary("iOS child runtime remains capability-only; no hidden daemon or persistent background service is claimed".to_string().as_str()),
            external_transport: boundary("no external LAN or WebSocket iOS child-agent transport is claimed".to_string().as_str()),
        },
        updated_at: timestamp(CHILD_IOS_ENTITLEMENT_UPDATED_AT),
    }
}
