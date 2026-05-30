use serde::{Deserialize, Serialize};

use crate::{
    LanAiProviderRoutingState, LanPairingDeviceReachability, LanPairingProductionDiscoveryState,
    LanPairingRejectionReason, LanPairingTrustState,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanProviderSelectionLifecycleState {
    CandidateDiscovered,
    CandidateEligible,
    CandidateSelected,
    CandidateRejected,
    CandidateDegraded,
    CandidateUnavailable,
    ManualRequired,
    NotImplemented,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanProviderSelectionPolicyDecision {
    SelectAuthorizedProvider,
    RefuseUnpairedProvider,
    RefuseRouteBlockedProvider,
    RefuseUnsupportedCapability,
    DegradeBusyProvider,
    DegradeProviderUnavailable,
    RequirePhysicalHouseholdProof,
    RequireCloudRelayDecision,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanProviderSelectionProofState {
    CiMechanicalProof,
    ManualRequired,
    NotImplemented,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanProviderSelectionManualRequirement {
    PhysicalHouseholdProviderHost,
    ProviderRouteOriginAllowlist,
    ProviderRouteStaleOfflineArtifact,
    ProviderRevocationArtifact,
    CloudRelayProviderDecision,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanProviderSelectionCloudRelayImplementationState {
    NotImplemented,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanProviderSelectionCloudRelayDecisionState {
    ManualDecisionRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanProviderSelectionCandidateEvidence {
    pub schema_version: u16,
    pub provider_peer_id: String,
    pub route_id: String,
    pub lifecycle_state: LanProviderSelectionLifecycleState,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub trust_state: LanPairingTrustState,
    pub reachability: LanPairingDeviceReachability,
    pub routing_state: LanAiProviderRoutingState,
    pub rejection_reason: Option<LanPairingRejectionReason>,
    pub policy_decision: LanProviderSelectionPolicyDecision,
    pub proof_state: LanProviderSelectionProofState,
    pub evidence_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanProviderSelectionManualRequirementEvidence {
    pub schema_version: u16,
    pub requirement: LanProviderSelectionManualRequirement,
    pub state: LanProviderSelectionProofState,
    pub required_artifact_summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanProviderSelectionReadModel {
    pub schema_version: u16,
    pub checked_at: String,
    pub selected_provider_route_id: Option<String>,
    pub authorized_provider_selection_state: LanProviderSelectionProofState,
    pub physical_household_provider_proof_state: LanProviderSelectionProofState,
    pub cloud_relay_implementation_state: LanProviderSelectionCloudRelayImplementationState,
    pub cloud_relay_decision_state: LanProviderSelectionCloudRelayDecisionState,
    pub candidates: Vec<LanProviderSelectionCandidateEvidence>,
    pub manual_requirements: Vec<LanProviderSelectionManualRequirementEvidence>,
}
