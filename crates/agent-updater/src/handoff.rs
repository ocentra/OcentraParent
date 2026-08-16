use ocentra_schema::setup_device_trust_handoff::{
    SetupDeviceTrustHandoffChildPackageTargetRef, SetupDeviceTrustHandoffExternalArtifactPath,
    SetupDeviceTrustHandoffId, SetupDeviceTrustHandoffInstallPreconditionState,
    SetupDeviceTrustHandoffManualRequiredState, SetupDeviceTrustHandoffNoClaim,
    SetupDeviceTrustHandoffPlatform, SetupDeviceTrustHandoffResponse,
    SetupDeviceTrustHandoffSetupState, SetupDeviceTrustHandoffStatus,
    SetupDeviceTrustHandoffTrustBootstrapState,
};

use crate::error::UpdaterError;
use crate::update::UpdateOutcome;

/// The package updater's typed view of a setup handoff. This is a projection,
/// not install, runtime-health, trust, or service-readiness proof.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChildPackageDistributionHandoffProjection {
    pub handoff_id: SetupDeviceTrustHandoffId,
    pub package_target: SetupDeviceTrustHandoffChildPackageTargetRef,
    pub artifact_path: SetupDeviceTrustHandoffExternalArtifactPath,
    pub platform: SetupDeviceTrustHandoffPlatform,
    pub handoff_status: SetupDeviceTrustHandoffStatus,
    pub install_precondition: SetupDeviceTrustHandoffInstallPreconditionState,
    pub manual_required: SetupDeviceTrustHandoffManualRequiredState,
    pub update_status: ChildPackageUpdateStatus,
    pub state: ChildPackageDistributionHandoffState,
    pub no_claim: Vec<SetupDeviceTrustHandoffNoClaim>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChildPackageUpdateStatus {
    Current { version: String },
    WouldInstall { current: String, latest: String },
    InstallerCompleted { current: String, latest: String },
    InstallerCompletedRebootRequired { current: String, latest: String },
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChildPackageDistributionHandoffState {
    AwaitingSetup,
    AwaitingArtifactProof,
    ManualRequired,
    RejectedInconsistent,
    UpdateCurrent,
    UpdateWouldInstall,
    UpdateInstallerCompleted,
    UpdateRebootRequired,
    UpdateFailed,
}

/// Consume the setup-owned handoff at the package/update boundary.
///
/// The returned state preserves manual-required and no-claim boundaries. A
/// successful updater outcome does not turn this projection into install,
/// service-health, trust, transport, or setup-journey proof.
pub fn consume_setup_device_trust_handoff(
    response: &SetupDeviceTrustHandoffResponse,
    update_result: Result<&UpdateOutcome, &UpdaterError>,
) -> ChildPackageDistributionHandoffProjection {
    let (update_status, update_state) = match update_result {
        Ok(outcome) => project_update_outcome(outcome),
        Err(_) => (
            ChildPackageUpdateStatus::Failed,
            ChildPackageDistributionHandoffState::UpdateFailed,
        ),
    };

    let state = if response.handoff_status
        != SetupDeviceTrustHandoffStatus::ReadyForChildPackageDistribution
    {
        match response.handoff_status {
            SetupDeviceTrustHandoffStatus::PendingSetupCompletion => {
                ChildPackageDistributionHandoffState::AwaitingSetup
            }
            SetupDeviceTrustHandoffStatus::Expired
            | SetupDeviceTrustHandoffStatus::BlockedManualRequired
            | SetupDeviceTrustHandoffStatus::ConsumedByDistributionProof => {
                ChildPackageDistributionHandoffState::ManualRequired
            }
            SetupDeviceTrustHandoffStatus::ReadyForChildPackageDistribution => update_state,
        }
    } else if response.manual_required_state != SetupDeviceTrustHandoffManualRequiredState::Not {
        ChildPackageDistributionHandoffState::ManualRequired
    } else if response.setup_state != SetupDeviceTrustHandoffSetupState::TrustBootstrapIssued
        || response.trust_bootstrap_state
            != SetupDeviceTrustHandoffTrustBootstrapState::BootstrapBoundToDevice
    {
        match (response.setup_state, response.trust_bootstrap_state) {
            (
                SetupDeviceTrustHandoffSetupState::ManualRequired
                | SetupDeviceTrustHandoffSetupState::Expired,
                _,
            )
            | (
                _,
                SetupDeviceTrustHandoffTrustBootstrapState::ManualRequired
                | SetupDeviceTrustHandoffTrustBootstrapState::Expired,
            ) => ChildPackageDistributionHandoffState::ManualRequired,
            _ => ChildPackageDistributionHandoffState::RejectedInconsistent,
        }
    } else if response.install_precondition_state
        != SetupDeviceTrustHandoffInstallPreconditionState::ReadyForInstallHandoff
    {
        match response.install_precondition_state {
            SetupDeviceTrustHandoffInstallPreconditionState::ArtifactProofRequired => {
                ChildPackageDistributionHandoffState::AwaitingArtifactProof
            }
            SetupDeviceTrustHandoffInstallPreconditionState::ManualRequired
            | SetupDeviceTrustHandoffInstallPreconditionState::Expired => {
                ChildPackageDistributionHandoffState::ManualRequired
            }
            SetupDeviceTrustHandoffInstallPreconditionState::ReadyForInstallHandoff => update_state,
        }
    } else {
        update_state
    };

    ChildPackageDistributionHandoffProjection {
        handoff_id: response.handoff_id.clone(),
        package_target: response.child_package_target_ref.clone(),
        artifact_path: response.artifact_requirement.external_artifact_path.clone(),
        platform: response.platform,
        handoff_status: response.handoff_status,
        install_precondition: response.install_precondition_state,
        manual_required: response.manual_required_state,
        update_status,
        state,
        no_claim: response.no_claim.clone(),
    }
}

fn project_update_outcome(
    outcome: &UpdateOutcome,
) -> (
    ChildPackageUpdateStatus,
    ChildPackageDistributionHandoffState,
) {
    match outcome {
        UpdateOutcome::Current { version } => (
            ChildPackageUpdateStatus::Current {
                version: version.clone(),
            },
            ChildPackageDistributionHandoffState::UpdateCurrent,
        ),
        UpdateOutcome::WouldInstall { current, latest } => (
            ChildPackageUpdateStatus::WouldInstall {
                current: current.clone(),
                latest: latest.clone(),
            },
            ChildPackageDistributionHandoffState::UpdateWouldInstall,
        ),
        UpdateOutcome::InstallerCompleted { current, latest } => (
            ChildPackageUpdateStatus::InstallerCompleted {
                current: current.clone(),
                latest: latest.clone(),
            },
            ChildPackageDistributionHandoffState::UpdateInstallerCompleted,
        ),
        UpdateOutcome::InstallerCompletedRebootRequired { current, latest } => (
            ChildPackageUpdateStatus::InstallerCompletedRebootRequired {
                current: current.clone(),
                latest: latest.clone(),
            },
            ChildPackageDistributionHandoffState::UpdateRebootRequired,
        ),
    }
}
