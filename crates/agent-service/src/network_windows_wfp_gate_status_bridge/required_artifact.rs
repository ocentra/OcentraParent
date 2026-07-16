use std::fmt::{self, Display, Formatter};

use ocentra_network_evidence::windows_wfp_gate::NetworkWindowsWfpRequiredArtifact;
use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Copy)]
pub(super) struct RequiredArtifactText(&'static str);

impl Display for RequiredArtifactText {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

pub(super) fn required_artifact(
    artifact: &NetworkWindowsWfpRequiredArtifact,
) -> RequiredArtifactText {
    RequiredArtifactText(match artifact {
        NetworkWindowsWfpRequiredArtifact::AdministratorPermissionProof => {
            constants::network_flow::WFP_ARTIFACT_ADMINISTRATOR_PERMISSION_PROOF
        }
        NetworkWindowsWfpRequiredArtifact::DriverSigningProof => {
            constants::network_flow::WFP_ARTIFACT_DRIVER_SIGNING_PROOF
        }
        NetworkWindowsWfpRequiredArtifact::DriverPackageProof => {
            constants::network_flow::WFP_ARTIFACT_DRIVER_PACKAGE_PROOF
        }
        NetworkWindowsWfpRequiredArtifact::ProviderRegistrationPlan => {
            constants::network_flow::WFP_ARTIFACT_PROVIDER_REGISTRATION_PLAN
        }
        NetworkWindowsWfpRequiredArtifact::LayerCapabilityMatrix => {
            constants::network_flow::WFP_ARTIFACT_LAYER_CAPABILITY_MATRIX
        }
        NetworkWindowsWfpRequiredArtifact::RollbackPlan => {
            constants::network_flow::WFP_ARTIFACT_ROLLBACK_PLAN
        }
        NetworkWindowsWfpRequiredArtifact::LabResultArtifact => {
            constants::network_flow::WFP_ARTIFACT_LAB_RESULT_ARTIFACT
        }
        NetworkWindowsWfpRequiredArtifact::AuditEvent => {
            constants::network_flow::WFP_ARTIFACT_AUDIT_EVENT
        }
    })
}
