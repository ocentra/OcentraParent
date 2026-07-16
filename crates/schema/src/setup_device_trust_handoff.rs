use serde::{Deserialize, Serialize};

mod enum_types;
mod text_types_core;
mod text_types_refs;

pub const SETUP_DEVICE_TRUST_HANDOFF_SCHEMA_VERSION: &str =
    "setup-device-trust-handoff-contract-proof";

const SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_WINDOWS: &str = "windows";
const SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_LINUX: &str = "linux";
const SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_MACOS: &str = "macos";
const SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_ANDROID: &str = "android";
const SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_IOS: &str = "ios";

const SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_SESSION_VALIDATED: &str = "session-validated";
const SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_TRUST_BOOTSTRAP_ISSUED: &str =
    "trust-bootstrap-issued";
const SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_MANUAL_REQUIRED: &str = "manual-required";
const SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_EXPIRED: &str = "expired";

const SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_BOOTSTRAP_ISSUED: &str = "bootstrap-issued";
const SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_BOOTSTRAP_BOUND_TO_DEVICE: &str =
    "bootstrap-bound-to-device";
const SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_MANUAL_REQUIRED: &str = "manual-required";
const SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_EXPIRED: &str = "expired";

const SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_ARTIFACT_PROOF_REQUIRED: &str =
    "artifact-proof-required";
const SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_READY_FOR_INSTALL_HANDOFF: &str =
    "ready-for-install-handoff";
const SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_MANUAL_REQUIRED: &str =
    "manual-required";
const SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_EXPIRED: &str = "expired";

const SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_NOT_REQUIRED: &str = "not-required";
const SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_PARENT_ACTION_REQUIRED: &str =
    "parent-action-required";
const SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_TARGET_DEVICE_ACTION_REQUIRED: &str =
    "target-device-action-required";
const SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_ADMIN_ACTION_REQUIRED: &str =
    "admin-action-required";
const SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_ROUTE_SYNC_REQUIRED: &str =
    "route-sync-required";

const SETUP_DEVICE_TRUST_HANDOFF_STATUS_PENDING_SETUP_COMPLETION: &str = "pending-setup-completion";
const SETUP_DEVICE_TRUST_HANDOFF_STATUS_READY_FOR_CHILD_PACKAGE_DISTRIBUTION: &str =
    "ready-for-child-package-distribution";
const SETUP_DEVICE_TRUST_HANDOFF_STATUS_BLOCKED_MANUAL_REQUIRED: &str = "blocked-manual-required";
const SETUP_DEVICE_TRUST_HANDOFF_STATUS_EXPIRED: &str = "expired";
const SETUP_DEVICE_TRUST_HANDOFF_STATUS_CONSUMED_BY_DISTRIBUTION_PROOF: &str =
    "consumed-by-distribution-proof";

const SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PARENT_BOOTSTRAP_PROOF: &str =
    "not-parent-bootstrap-proof";
const SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_CHILD_PAIRING_CODE: &str = "not-child-pairing-code";
const SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PACKAGE_READINESS: &str = "not-package-readiness";
const SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_INSTALL_RUNTIME_READINESS: &str =
    "not-install-runtime-readiness";
const SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_SERVICE_HEALTH_PROOF: &str =
    "not-service-health-proof";
const SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PACKAGE_ARTIFACT_PROOF: &str =
    "not-package-artifact-proof";
const SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PARENT_CLIENT_PARITY: &str =
    "not-parent-client-parity";

const SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_SETUP_INSTALL_PROVISIONING_PLAN: &str =
    "setup-install-provisioning-plan";
const SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_DEVICE_TRUST_BOOTSTRAP_PLAN: &str =
    "device-trust-bootstrap-plan";

const SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_NAMED_EXTERNAL_OWNER: &str =
    "named-external-owner";
const SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_REQUIRED_FOR_RUNTIME_PROOF: &str =
    "required-for-runtime-proof";

pub type SetupDeviceTrustHandoffPlatform = enum_types::SetupDeviceTrustHandoffPlatform;
pub type SetupDeviceTrustHandoffSetupState = enum_types::SetupDeviceTrustHandoffSetupState;
pub type SetupDeviceTrustHandoffTrustBootstrapState =
    enum_types::SetupDeviceTrustHandoffTrustBootstrapState;
pub type SetupDeviceTrustHandoffInstallPreconditionState =
    enum_types::SetupDeviceTrustHandoffInstallPreconditionState;
pub type SetupDeviceTrustHandoffManualRequiredState =
    enum_types::SetupDeviceTrustHandoffManualRequiredState;
pub type SetupDeviceTrustHandoffStatus = enum_types::SetupDeviceTrustHandoffStatus;
pub type SetupDeviceTrustHandoffNoClaim = enum_types::SetupDeviceTrustHandoffNoClaim;
pub type SetupDeviceTrustHandoffRouteSyncPlan = enum_types::SetupDeviceTrustHandoffRouteSyncPlan;
pub type SetupDeviceTrustHandoffRouteSyncStatus =
    enum_types::SetupDeviceTrustHandoffRouteSyncStatus;

pub type SetupDeviceTrustHandoffId = text_types_core::SetupDeviceTrustHandoffId;
pub type SetupDeviceTrustHandoffHouseholdRef = text_types_core::SetupDeviceTrustHandoffHouseholdRef;
pub type SetupDeviceTrustHandoffChildProfileRef =
    text_types_core::SetupDeviceTrustHandoffChildProfileRef;
pub type SetupDeviceTrustHandoffTargetDeviceRef =
    text_types_core::SetupDeviceTrustHandoffTargetDeviceRef;
pub type SetupDeviceTrustHandoffSetupSessionRef =
    text_types_core::SetupDeviceTrustHandoffSetupSessionRef;
pub type SetupDeviceTrustHandoffTrustBootstrapRef =
    text_types_core::SetupDeviceTrustHandoffTrustBootstrapRef;
pub type SetupDeviceTrustHandoffChildPackageTargetRef =
    text_types_refs::SetupDeviceTrustHandoffChildPackageTargetRef;
pub type SetupDeviceTrustHandoffArtifactRequirementRef =
    text_types_refs::SetupDeviceTrustHandoffArtifactRequirementRef;
pub type SetupDeviceTrustHandoffClaimBoundary =
    text_types_refs::SetupDeviceTrustHandoffClaimBoundary;
pub type SetupDeviceTrustHandoffExternalArtifactPath =
    text_types_refs::SetupDeviceTrustHandoffExternalArtifactPath;
pub type SetupDeviceTrustHandoffExpiryOrReplayGuardRef =
    text_types_refs::SetupDeviceTrustHandoffExpiryOrReplayGuardRef;
pub type SetupDeviceTrustHandoffTimestamp = text_types_refs::SetupDeviceTrustHandoffTimestamp;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupDeviceTrustHandoffArtifactRequirement {
    pub requirement_ref: SetupDeviceTrustHandoffArtifactRequirementRef,
    pub external_artifact_path: SetupDeviceTrustHandoffExternalArtifactPath,
    pub claim_boundary: SetupDeviceTrustHandoffClaimBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupDeviceTrustHandoffRouteSyncRequirement {
    pub plan: SetupDeviceTrustHandoffRouteSyncPlan,
    pub status: SetupDeviceTrustHandoffRouteSyncStatus,
    pub claim_boundary: SetupDeviceTrustHandoffClaimBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupDeviceTrustHandoffRequest {
    pub schema_version: String,
    pub handoff_id: SetupDeviceTrustHandoffId,
    pub household_ref: SetupDeviceTrustHandoffHouseholdRef,
    pub child_profile_ref: SetupDeviceTrustHandoffChildProfileRef,
    pub target_device_ref: SetupDeviceTrustHandoffTargetDeviceRef,
    pub setup_session_ref: SetupDeviceTrustHandoffSetupSessionRef,
    pub trust_bootstrap_ref: SetupDeviceTrustHandoffTrustBootstrapRef,
    pub child_package_target_ref: SetupDeviceTrustHandoffChildPackageTargetRef,
    pub platform: SetupDeviceTrustHandoffPlatform,
    pub setup_state: SetupDeviceTrustHandoffSetupState,
    pub trust_bootstrap_state: SetupDeviceTrustHandoffTrustBootstrapState,
    pub artifact_requirement: SetupDeviceTrustHandoffArtifactRequirement,
    pub expiry_or_replay_guard_ref: SetupDeviceTrustHandoffExpiryOrReplayGuardRef,
    pub requested_at: SetupDeviceTrustHandoffTimestamp,
    pub no_claim: Vec<SetupDeviceTrustHandoffNoClaim>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupDeviceTrustHandoffResponse {
    pub schema_version: String,
    pub handoff_id: SetupDeviceTrustHandoffId,
    pub household_ref: SetupDeviceTrustHandoffHouseholdRef,
    pub child_profile_ref: SetupDeviceTrustHandoffChildProfileRef,
    pub target_device_ref: SetupDeviceTrustHandoffTargetDeviceRef,
    pub setup_session_ref: SetupDeviceTrustHandoffSetupSessionRef,
    pub trust_bootstrap_ref: SetupDeviceTrustHandoffTrustBootstrapRef,
    pub child_package_target_ref: SetupDeviceTrustHandoffChildPackageTargetRef,
    pub platform: SetupDeviceTrustHandoffPlatform,
    pub setup_state: SetupDeviceTrustHandoffSetupState,
    pub trust_bootstrap_state: SetupDeviceTrustHandoffTrustBootstrapState,
    pub artifact_requirement: SetupDeviceTrustHandoffArtifactRequirement,
    pub install_precondition_state: SetupDeviceTrustHandoffInstallPreconditionState,
    pub manual_required_state: SetupDeviceTrustHandoffManualRequiredState,
    pub expiry_or_replay_guard_ref: SetupDeviceTrustHandoffExpiryOrReplayGuardRef,
    pub handoff_status: SetupDeviceTrustHandoffStatus,
    pub no_claim: Vec<SetupDeviceTrustHandoffNoClaim>,
    pub updated_at: SetupDeviceTrustHandoffTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupDeviceTrustHandoffContractProof {
    pub schema_version: String,
    pub request: SetupDeviceTrustHandoffRequest,
    pub response: SetupDeviceTrustHandoffResponse,
    pub route_sync: Vec<SetupDeviceTrustHandoffRouteSyncRequirement>,
    pub updated_at: SetupDeviceTrustHandoffTimestamp,
}
