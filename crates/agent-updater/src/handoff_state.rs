use ocentra_schema::setup_device_trust_handoff::{
    SetupDeviceTrustHandoffInstallPreconditionState, SetupDeviceTrustHandoffManualRequiredState,
    SetupDeviceTrustHandoffResponse, SetupDeviceTrustHandoffSetupState,
    SetupDeviceTrustHandoffStatus, SetupDeviceTrustHandoffTrustBootstrapState,
};

use super::ChildPackageDistributionHandoffState;

pub(super) fn project_handoff_state(
    response: &SetupDeviceTrustHandoffResponse,
    update_state: ChildPackageDistributionHandoffState,
) -> ChildPackageDistributionHandoffState {
    if response.handoff_status == SetupDeviceTrustHandoffStatus::PendingSetupCompletion {
        return ChildPackageDistributionHandoffState::AwaitingSetup;
    }
    if matches!(
        response.handoff_status,
        SetupDeviceTrustHandoffStatus::Expired
            | SetupDeviceTrustHandoffStatus::BlockedManualRequired
            | SetupDeviceTrustHandoffStatus::ConsumedByDistributionProof
    ) {
        return ChildPackageDistributionHandoffState::ManualRequired;
    }
    project_ready_state(response, update_state)
}

fn project_ready_state(
    response: &SetupDeviceTrustHandoffResponse,
    update_state: ChildPackageDistributionHandoffState,
) -> ChildPackageDistributionHandoffState {
    if response.manual_required_state != SetupDeviceTrustHandoffManualRequiredState::Not {
        return ChildPackageDistributionHandoffState::ManualRequired;
    }
    if response.setup_state != SetupDeviceTrustHandoffSetupState::TrustBootstrapIssued
        || response.trust_bootstrap_state
            != SetupDeviceTrustHandoffTrustBootstrapState::BootstrapBoundToDevice
    {
        return project_trust_state(response);
    }
    project_install_precondition_state(response, update_state)
}

fn project_trust_state(
    response: &SetupDeviceTrustHandoffResponse,
) -> ChildPackageDistributionHandoffState {
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
}

fn project_install_precondition_state(
    response: &SetupDeviceTrustHandoffResponse,
    update_state: ChildPackageDistributionHandoffState,
) -> ChildPackageDistributionHandoffState {
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
}
