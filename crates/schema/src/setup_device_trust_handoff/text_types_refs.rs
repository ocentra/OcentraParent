#[path = "artifact_refs.rs"]
mod artifact_refs;
#[path = "timing_refs.rs"]
mod timing_refs;

pub type SetupDeviceTrustHandoffChildPackageTargetRef =
    artifact_refs::SetupDeviceTrustHandoffChildPackageTargetRef;
pub type SetupDeviceTrustHandoffArtifactRequirementRef =
    artifact_refs::SetupDeviceTrustHandoffArtifactRequirementRef;
pub type SetupDeviceTrustHandoffClaimBoundary = artifact_refs::SetupDeviceTrustHandoffClaimBoundary;
pub type SetupDeviceTrustHandoffExternalArtifactPath =
    timing_refs::SetupDeviceTrustHandoffExternalArtifactPath;
pub type SetupDeviceTrustHandoffExpiryOrReplayGuardRef =
    timing_refs::SetupDeviceTrustHandoffExpiryOrReplayGuardRef;
pub type SetupDeviceTrustHandoffTimestamp = timing_refs::SetupDeviceTrustHandoffTimestamp;
