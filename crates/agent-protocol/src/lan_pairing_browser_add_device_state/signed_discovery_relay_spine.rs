use serde::{Deserialize, Serialize};

use super::deserialize_lan_schema_version;
use crate::{
    LanPairingProductionDiscoveryState, LanPairingRejectionReason, LanPairingResponseState,
    V09ProductionDiscoveryHouseholdProofState, V09ProductionDiscoveryHouseholdRuntimeOwner,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanSignedDiscoveryRelayAdapterKind {
    PassiveLanNeighbor,
    RouterInfrastructure,
    MdnsName,
    SsdpName,
    RouterDhcpName,
    ManualDirectAddress,
    SignedChildAgentHello,
    SignedChildAgentHeartbeat,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanSignedDiscoveryRelaySourceConfidence {
    Confirmed,
    Strong,
    Weak,
    ManualRequired,
    Unavailable,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanSignedDiscoveryRelayCustodyLabel {
    ParentLocalService,
    PassiveLanObservation,
    RouterInfrastructureObservation,
    ManualParentEntry,
    SignedChildAgentArtifact,
    NoOcentraChildDataCustody,
    ParentOwnedStorageUnavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanSignedDiscoveryRelaySignedProofCheck {
    SignedHelloManualRequired,
    SignedHeartbeatManualRequired,
    AcceptedSignedChildAgentManualRequired,
    UnauthenticatedCallerRejected,
    ExpiredSignedProofRejected,
    ReplayedSignedProofRejected,
    WrongOriginSignedProofRejected,
    WrongDeviceSignedProofRejected,
    RevokedSignedProofRejected,
    StaleSignedProofRejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanSignedDiscoveryRelayRouteSafetyCheck {
    TrustedRegistryRestartRecovery,
    SelectedRouteCustody,
    StaleSelectedDeviceRejected,
    OfflineSelectedDeviceRejected,
    WrongRouteRejected,
    RevokedRouteRejected,
    ParentAssignDecisionAudited,
    ParentRenameDecisionAudited,
    ParentIgnoreDecisionAudited,
    ParentRestoreDecisionAudited,
    ParentTrustDecisionAudited,
    ParentRevokeDecisionAudited,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanSignedDiscoveryRelayCacheCheck {
    RelayRouteUnavailable,
    RelayRouteQueuedNotConfigured,
    CacheRouteUnavailable,
    ParentOwnedStorageUnavailable,
    OcentraChildDataCustodyNotClaimed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanSignedDiscoveryRelayDecisionState {
    LocalFirst,
    Unavailable,
    QueuedNotConfigured,
    NotImplemented,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanSignedDiscoveryRelayAdapterRow {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub adapter: LanSignedDiscoveryRelayAdapterKind,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub proof_state: V09ProductionDiscoveryHouseholdProofState,
    pub source_confidence: LanSignedDiscoveryRelaySourceConfidence,
    pub custody_label: LanSignedDiscoveryRelayCustodyLabel,
    pub runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    pub evidence_label: String,
    pub required_artifact_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanSignedDiscoveryRelaySignedProofRow {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub check: LanSignedDiscoveryRelaySignedProofCheck,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub response_state: LanPairingResponseState,
    pub rejection_reason: Option<LanPairingRejectionReason>,
    pub proof_state: V09ProductionDiscoveryHouseholdProofState,
    pub runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    pub evidence_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanSignedDiscoveryRelayRouteSafetyRow {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub check: LanSignedDiscoveryRelayRouteSafetyCheck,
    pub route_id: Option<String>,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub response_state: LanPairingResponseState,
    pub rejection_reason: Option<LanPairingRejectionReason>,
    pub proof_state: V09ProductionDiscoveryHouseholdProofState,
    pub runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    pub custody_label: LanSignedDiscoveryRelayCustodyLabel,
    pub evidence_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanSignedDiscoveryRelayCacheRow {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub check: LanSignedDiscoveryRelayCacheCheck,
    pub decision_state: LanSignedDiscoveryRelayDecisionState,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub proof_state: V09ProductionDiscoveryHouseholdProofState,
    pub runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    pub custody_label: LanSignedDiscoveryRelayCustodyLabel,
    pub evidence_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanSignedDiscoveryRelaySpineSummary {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub generated_at: String,
    pub adapter_rows: Vec<LanSignedDiscoveryRelayAdapterRow>,
    pub signed_proof_rows: Vec<LanSignedDiscoveryRelaySignedProofRow>,
    pub route_safety_rows: Vec<LanSignedDiscoveryRelayRouteSafetyRow>,
    pub relay_cache_rows: Vec<LanSignedDiscoveryRelayCacheRow>,
    pub manual_proof_required: Vec<LanSignedDiscoveryRelayAdapterKind>,
    pub not_implemented: Vec<LanSignedDiscoveryRelayCacheCheck>,
    pub claims_proved: Vec<String>,
    pub claims_not_proved: Vec<String>,
}
