use serde::{Deserialize, Serialize};

use super::{
    SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_ARTIFACT_PROOF_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_EXPIRED,
    SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_MANUAL_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_READY_FOR_INSTALL_HANDOFF,
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_ADMIN_ACTION_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_NOT_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_PARENT_ACTION_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_ROUTE_SYNC_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_TARGET_DEVICE_ACTION_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_CHILD_PAIRING_CODE,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_INSTALL_RUNTIME_READINESS,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PACKAGE_ARTIFACT_PROOF,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PACKAGE_READINESS,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PARENT_BOOTSTRAP_PROOF,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PARENT_CLIENT_PARITY,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_SERVICE_HEALTH_PROOF,
    SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_ANDROID, SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_IOS,
    SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_LINUX, SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_MACOS,
    SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_WINDOWS,
    SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_DEVICE_TRUST_BOOTSTRAP_PLAN,
    SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_SETUP_INSTALL_PROVISIONING_PLAN,
    SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_NAMED_EXTERNAL_OWNER,
    SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_REQUIRED_FOR_RUNTIME_PROOF,
    SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_EXPIRED,
    SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_MANUAL_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_SESSION_VALIDATED,
    SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_TRUST_BOOTSTRAP_ISSUED,
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_BLOCKED_MANUAL_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_CONSUMED_BY_DISTRIBUTION_PROOF,
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_EXPIRED,
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_PENDING_SETUP_COMPLETION,
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_READY_FOR_CHILD_PACKAGE_DISTRIBUTION,
    SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_BOOTSTRAP_BOUND_TO_DEVICE,
    SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_BOOTSTRAP_ISSUED,
    SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_EXPIRED,
    SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_MANUAL_REQUIRED,
};

const SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_VARIANTS: [&str; 5] = [
    SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_WINDOWS,
    SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_LINUX,
    SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_MACOS,
    SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_ANDROID,
    SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_IOS,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum SetupDeviceTrustHandoffPlatform {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "macos")]
    Macos,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "ios")]
    Ios,
}

impl SetupDeviceTrustHandoffPlatform {
    pub const VARIANTS: &'static [&'static str] = &SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_VARIANTS: [&str; 4] = [
    SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_SESSION_VALIDATED,
    SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_TRUST_BOOTSTRAP_ISSUED,
    SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_MANUAL_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_EXPIRED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum SetupDeviceTrustHandoffSetupState {
    #[serde(rename = "session-validated")]
    SessionValidated,
    #[serde(rename = "trust-bootstrap-issued")]
    TrustBootstrapIssued,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "expired")]
    Expired,
}

impl SetupDeviceTrustHandoffSetupState {
    pub const VARIANTS: &'static [&'static str] = &SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_VARIANTS: [&str; 4] = [
    SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_BOOTSTRAP_ISSUED,
    SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_BOOTSTRAP_BOUND_TO_DEVICE,
    SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_MANUAL_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_EXPIRED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum SetupDeviceTrustHandoffTrustBootstrapState {
    #[serde(rename = "bootstrap-issued")]
    BootstrapIssued,
    #[serde(rename = "bootstrap-bound-to-device")]
    BootstrapBoundToDevice,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "expired")]
    Expired,
}

impl SetupDeviceTrustHandoffTrustBootstrapState {
    pub const VARIANTS: &'static [&'static str] =
        &SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_VARIANTS: [&str; 4] = [
    SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_ARTIFACT_PROOF_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_READY_FOR_INSTALL_HANDOFF,
    SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_MANUAL_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_EXPIRED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum SetupDeviceTrustHandoffInstallPreconditionState {
    #[serde(rename = "artifact-proof-required")]
    ArtifactProofRequired,
    #[serde(rename = "ready-for-install-handoff")]
    ReadyForInstallHandoff,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "expired")]
    Expired,
}

impl SetupDeviceTrustHandoffInstallPreconditionState {
    pub const VARIANTS: &'static [&'static str] =
        &SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_VARIANTS: [&str; 5] = [
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_NOT_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_PARENT_ACTION_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_TARGET_DEVICE_ACTION_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_ADMIN_ACTION_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_ROUTE_SYNC_REQUIRED,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum SetupDeviceTrustHandoffManualRequiredState {
    #[serde(rename = "not-required")]
    Not,
    #[serde(rename = "parent-action-required")]
    ParentAction,
    #[serde(rename = "target-device-action-required")]
    TargetDeviceAction,
    #[serde(rename = "admin-action-required")]
    AdminAction,
    #[serde(rename = "route-sync-required")]
    RouteSync,
}

impl SetupDeviceTrustHandoffManualRequiredState {
    pub const VARIANTS: &'static [&'static str] =
        &SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const SETUP_DEVICE_TRUST_HANDOFF_STATUS_VARIANTS: [&str; 5] = [
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_PENDING_SETUP_COMPLETION,
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_READY_FOR_CHILD_PACKAGE_DISTRIBUTION,
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_BLOCKED_MANUAL_REQUIRED,
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_EXPIRED,
    SETUP_DEVICE_TRUST_HANDOFF_STATUS_CONSUMED_BY_DISTRIBUTION_PROOF,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum SetupDeviceTrustHandoffStatus {
    #[serde(rename = "pending-setup-completion")]
    PendingSetupCompletion,
    #[serde(rename = "ready-for-child-package-distribution")]
    ReadyForChildPackageDistribution,
    #[serde(rename = "blocked-manual-required")]
    BlockedManualRequired,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "consumed-by-distribution-proof")]
    ConsumedByDistributionProof,
}

impl SetupDeviceTrustHandoffStatus {
    pub const VARIANTS: &'static [&'static str] = &SETUP_DEVICE_TRUST_HANDOFF_STATUS_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_VARIANTS: [&str; 7] = [
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PARENT_BOOTSTRAP_PROOF,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_CHILD_PAIRING_CODE,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PACKAGE_READINESS,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_INSTALL_RUNTIME_READINESS,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_SERVICE_HEALTH_PROOF,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PACKAGE_ARTIFACT_PROOF,
    SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PARENT_CLIENT_PARITY,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum SetupDeviceTrustHandoffNoClaim {
    #[serde(rename = "not-parent-bootstrap-proof")]
    ParentBootstrapProof,
    #[serde(rename = "not-child-pairing-code")]
    ChildPairingCode,
    #[serde(rename = "not-package-readiness")]
    PackageReadiness,
    #[serde(rename = "not-install-runtime-readiness")]
    InstallRuntimeReadiness,
    #[serde(rename = "not-service-health-proof")]
    ServiceHealthProof,
    #[serde(rename = "not-package-artifact-proof")]
    PackageArtifactProof,
    #[serde(rename = "not-parent-client-parity")]
    ParentClientParity,
}

impl SetupDeviceTrustHandoffNoClaim {
    pub const VARIANTS: &'static [&'static str] = &SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_VARIANTS: [&str; 2] = [
    SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_SETUP_INSTALL_PROVISIONING_PLAN,
    SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_DEVICE_TRUST_BOOTSTRAP_PLAN,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum SetupDeviceTrustHandoffRouteSyncPlan {
    #[serde(rename = "setup-install-provisioning-plan")]
    SetupInstallProvisioningPlan,
    #[serde(rename = "device-trust-bootstrap-plan")]
    DeviceTrustBootstrapPlan,
}

impl SetupDeviceTrustHandoffRouteSyncPlan {
    pub const VARIANTS: &'static [&'static str] =
        &SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}

const SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_VARIANTS: [&str; 2] = [
    SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_NAMED_EXTERNAL_OWNER,
    SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_REQUIRED_FOR_RUNTIME_PROOF,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(usize)]
pub enum SetupDeviceTrustHandoffRouteSyncStatus {
    #[serde(rename = "named-external-owner")]
    NamedExternalOwner,
    #[serde(rename = "required-for-runtime-proof")]
    RequiredForRuntimeProof,
}

impl SetupDeviceTrustHandoffRouteSyncStatus {
    pub const VARIANTS: &'static [&'static str] =
        &SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_VARIANTS;

    pub fn as_str(&self) -> &'static str {
        Self::VARIANTS[*self as usize]
    }
}
