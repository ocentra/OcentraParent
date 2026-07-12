use std::fmt::{self, Display, Formatter};

use ocentra_network_evidence::windows_wfp_gate::NetworkWindowsWfpGateBoundaryReason;
use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Copy)]
pub(super) struct BoundaryReasonText(&'static str);

impl Display for BoundaryReasonText {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

pub(super) fn boundary_reason(reason: &NetworkWindowsWfpGateBoundaryReason) -> BoundaryReasonText {
    BoundaryReasonText(match reason {
        NetworkWindowsWfpGateBoundaryReason::ResearchOnlyRequested => {
            constants::network_flow::WFP_BOUNDARY_RESEARCH_ONLY_REQUESTED
        }
        NetworkWindowsWfpGateBoundaryReason::CapabilityManualRequired => {
            constants::network_flow::WFP_BOUNDARY_CAPABILITY_MANUAL_REQUIRED
        }
        NetworkWindowsWfpGateBoundaryReason::CapabilityUnavailable => {
            constants::network_flow::WFP_BOUNDARY_CAPABILITY_UNAVAILABLE
        }
        NetworkWindowsWfpGateBoundaryReason::EvidenceGradeBelowProofThreshold => {
            constants::network_flow::WFP_BOUNDARY_EVIDENCE_GRADE_BELOW_PROOF_THRESHOLD
        }
        NetworkWindowsWfpGateBoundaryReason::PolicyNotWfpApproved => {
            constants::network_flow::WFP_BOUNDARY_POLICY_NOT_WFP_APPROVED
        }
        NetworkWindowsWfpGateBoundaryReason::MissingRequiredArtifact => {
            constants::network_flow::WFP_BOUNDARY_MISSING_REQUIRED_ARTIFACT
        }
    })
}
