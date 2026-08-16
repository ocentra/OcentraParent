use super::{
    NetworkAppleNetworkExtensionArtifactRefs, NetworkAppleNetworkExtensionRequiredArtifact,
};

pub(super) fn missing_required_artifacts(
    artifacts: &NetworkAppleNetworkExtensionArtifactRefs,
    supervision_required: bool,
) -> Vec<NetworkAppleNetworkExtensionRequiredArtifact> {
    let mut missing = Vec::new();
    push_missing(
        &mut missing,
        artifacts.developer_team_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::DeveloperTeamProof,
    );
    push_missing(
        &mut missing,
        artifacts.entitlement_approval_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::EntitlementApprovalProof,
    );
    push_missing(
        &mut missing,
        artifacts.provisioning_profile_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::ProvisioningProfileProof,
    );
    push_missing(
        &mut missing,
        artifacts.signing_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::SigningProof,
    );
    push_missing(
        &mut missing,
        artifacts.device_or_testflight_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::DeviceOrTestFlightProof,
    );
    push_missing(
        &mut missing,
        artifacts.network_extension_declaration_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::NetworkExtensionDeclaration,
    );
    push_missing(
        &mut missing,
        artifacts.extension_configuration_proof_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::ExtensionConfigurationProof,
    );
    push_missing(
        &mut missing,
        artifacts.rollback_plan_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::RollbackPlan,
    );
    push_missing(
        &mut missing,
        artifacts.audit_event_ref.as_ref(),
        NetworkAppleNetworkExtensionRequiredArtifact::AuditEvent,
    );
    if supervision_required {
        push_missing(
            &mut missing,
            artifacts.supervision_or_mdm_proof_ref.as_ref(),
            NetworkAppleNetworkExtensionRequiredArtifact::SupervisionOrMdmProof,
        );
    }
    missing
}

fn push_missing(
    missing: &mut Vec<NetworkAppleNetworkExtensionRequiredArtifact>,
    value: Option<&String>,
    artifact: NetworkAppleNetworkExtensionRequiredArtifact,
) {
    if value.is_none() {
        missing.push(artifact);
    }
}
