use serde::{Deserialize, Serialize};

use crate::{
    LanPairingProductionDiscoveryState, V09ProductionDiscoveryHouseholdProofState,
    V09ProductionDiscoveryHouseholdRuntimeOwner,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanProductionHouseholdProofCapability {
    SignedLanHello,
    SignedLanHeartbeat,
    PassiveNeighborDiscovery,
    RouterNeighborDiscovery,
    MdnsNameDiscovery,
    SsdpNameDiscovery,
    RouterDhcpNameDiscovery,
    TrustedRegistry,
    ParentAssignment,
    ParentRename,
    ParentIgnore,
    ParentRevocation,
    RouteCustody,
    StaleSelectedDevice,
    OfflineSelectedDevice,
    RelayRoute,
    CacheRoute,
    SecondPhysicalChildAgent,
    AndroidChildAgentParity,
    IosChildAgentParity,
    StoreSigning,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanProductionHouseholdProofStatus {
    pub schema_version: u16,
    pub capability: LanProductionHouseholdProofCapability,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub proof_state: V09ProductionDiscoveryHouseholdProofState,
    pub runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    pub evidence_label: String,
    pub required_artifact_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanProductionHouseholdProofSummary {
    pub schema_version: u16,
    pub generated_at: String,
    pub status_rows: Vec<LanProductionHouseholdProofStatus>,
    pub manual_proof_required: Vec<LanProductionHouseholdProofCapability>,
    pub not_implemented: Vec<LanProductionHouseholdProofCapability>,
    pub claims_proved: Vec<String>,
    pub claims_not_proved: Vec<String>,
}
