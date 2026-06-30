use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

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

macro_rules! handoff_text_identifier {
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

macro_rules! handoff_string_enum {
    ($name:ident { $($variant:ident => $value:expr),+ $(,)? }) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                $(if value == $value { return Ok(Self::$variant); })+
                Err(serde::de::Error::unknown_variant(value.as_str(), &[$($value,)+]))
            }
        }
    };
}

handoff_string_enum!(SetupDeviceTrustHandoffPlatform {
    Windows => SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_WINDOWS,
    Linux => SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_LINUX,
    Macos => SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_MACOS,
    Android => SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_ANDROID,
    Ios => SETUP_DEVICE_TRUST_HANDOFF_PLATFORM_IOS,
});

handoff_string_enum!(SetupDeviceTrustHandoffSetupState {
    SessionValidated => SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_SESSION_VALIDATED,
    TrustBootstrapIssued => SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_TRUST_BOOTSTRAP_ISSUED,
    ManualRequired => SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_MANUAL_REQUIRED,
    Expired => SETUP_DEVICE_TRUST_HANDOFF_SETUP_STATE_EXPIRED,
});

handoff_string_enum!(SetupDeviceTrustHandoffTrustBootstrapState {
    BootstrapIssued => SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_BOOTSTRAP_ISSUED,
    BootstrapBoundToDevice =>
        SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_BOOTSTRAP_BOUND_TO_DEVICE,
    ManualRequired => SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_MANUAL_REQUIRED,
    Expired => SETUP_DEVICE_TRUST_HANDOFF_TRUST_BOOTSTRAP_STATE_EXPIRED,
});

handoff_string_enum!(SetupDeviceTrustHandoffInstallPreconditionState {
    ArtifactProofRequired =>
        SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_ARTIFACT_PROOF_REQUIRED,
    ReadyForInstallHandoff =>
        SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_READY_FOR_INSTALL_HANDOFF,
    ManualRequired => SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_MANUAL_REQUIRED,
    Expired => SETUP_DEVICE_TRUST_HANDOFF_INSTALL_PRECONDITION_STATE_EXPIRED,
});

handoff_string_enum!(SetupDeviceTrustHandoffManualRequiredState {
    NotRequired => SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_NOT_REQUIRED,
    ParentActionRequired =>
        SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_PARENT_ACTION_REQUIRED,
    TargetDeviceActionRequired =>
        SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_TARGET_DEVICE_ACTION_REQUIRED,
    AdminActionRequired =>
        SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_ADMIN_ACTION_REQUIRED,
    RouteSyncRequired => SETUP_DEVICE_TRUST_HANDOFF_MANUAL_REQUIRED_STATE_ROUTE_SYNC_REQUIRED,
});

handoff_string_enum!(SetupDeviceTrustHandoffStatus {
    PendingSetupCompletion => SETUP_DEVICE_TRUST_HANDOFF_STATUS_PENDING_SETUP_COMPLETION,
    ReadyForChildPackageDistribution =>
        SETUP_DEVICE_TRUST_HANDOFF_STATUS_READY_FOR_CHILD_PACKAGE_DISTRIBUTION,
    BlockedManualRequired => SETUP_DEVICE_TRUST_HANDOFF_STATUS_BLOCKED_MANUAL_REQUIRED,
    Expired => SETUP_DEVICE_TRUST_HANDOFF_STATUS_EXPIRED,
    ConsumedByDistributionProof =>
        SETUP_DEVICE_TRUST_HANDOFF_STATUS_CONSUMED_BY_DISTRIBUTION_PROOF,
});

handoff_string_enum!(SetupDeviceTrustHandoffNoClaim {
    NotParentBootstrapProof => SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PARENT_BOOTSTRAP_PROOF,
    NotChildPairingCode => SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_CHILD_PAIRING_CODE,
    NotPackageReadiness => SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PACKAGE_READINESS,
    NotInstallRuntimeReadiness =>
        SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_INSTALL_RUNTIME_READINESS,
    NotServiceHealthProof => SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_SERVICE_HEALTH_PROOF,
    NotPackageArtifactProof => SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PACKAGE_ARTIFACT_PROOF,
    NotParentClientParity => SETUP_DEVICE_TRUST_HANDOFF_NO_CLAIM_NOT_PARENT_CLIENT_PARITY,
});

handoff_string_enum!(SetupDeviceTrustHandoffRouteSyncPlan {
    SetupInstallProvisioningPlan =>
        SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_SETUP_INSTALL_PROVISIONING_PLAN,
    DeviceTrustBootstrapPlan =>
        SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_PLAN_DEVICE_TRUST_BOOTSTRAP_PLAN,
});

handoff_string_enum!(SetupDeviceTrustHandoffRouteSyncStatus {
    NamedExternalOwner => SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_NAMED_EXTERNAL_OWNER,
    RequiredForRuntimeProof =>
        SETUP_DEVICE_TRUST_HANDOFF_ROUTE_SYNC_STATUS_REQUIRED_FOR_RUNTIME_PROOF,
});

handoff_text_identifier!(SetupDeviceTrustHandoffId);
handoff_text_identifier!(SetupDeviceTrustHandoffHouseholdRef);
handoff_text_identifier!(SetupDeviceTrustHandoffChildProfileRef);
handoff_text_identifier!(SetupDeviceTrustHandoffTargetDeviceRef);
handoff_text_identifier!(SetupDeviceTrustHandoffSetupSessionRef);
handoff_text_identifier!(SetupDeviceTrustHandoffTrustBootstrapRef);
handoff_text_identifier!(SetupDeviceTrustHandoffChildPackageTargetRef);
handoff_text_identifier!(SetupDeviceTrustHandoffArtifactRequirementRef);
handoff_text_identifier!(SetupDeviceTrustHandoffClaimBoundary);
handoff_text_identifier!(SetupDeviceTrustHandoffExternalArtifactPath);
handoff_text_identifier!(SetupDeviceTrustHandoffExpiryOrReplayGuardRef);
handoff_text_identifier!(SetupDeviceTrustHandoffTimestamp);

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
