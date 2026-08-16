use serde::{Deserialize, Serialize};

use super::{deserialize_lan_schema_version_text, LanPairingText};
use crate::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingRejectionReason,
    LanPairingTrustState,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum V09ProductionDiscoveryHouseholdProofBoundary {
    LocalRealServiceNotPhysicalHouseholdLan,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum V09ProductionDiscoveryHouseholdRuntimeOwner {
    ParentDomainContract,
    AgentProtocol,
    RustServiceReadModel,
    ProofHarness,
    ManualProof,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum V09ProductionDiscoveryHouseholdCheck {
    ProductionDiscoveryStates,
    PairedRouteAccepted,
    FailedUnpairedRejected,
    ReplayRejected,
    RestartSelectedRouteRecovered,
    RestartRegistryStateRecovered,
    StaleSourceRejected,
    OfflineDeviceRejected,
    RevokedPairingRejected,
    UnavailableRouteRejected,
    WrongOriginRejected,
    WrongDeviceRejected,
    ManualPhysicalHouseholdChecklist,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum V09ProductionDiscoveryHouseholdSourceState {
    Discovered,
    Pending,
    Paired,
    FailedUnpaired,
    RestartRecovered,
    Stale,
    Offline,
    Revoked,
    Unavailable,
    WrongOrigin,
    WrongDevice,
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum V09ProductionDiscoveryHouseholdRouteRecoveryState {
    SelectedRoutePersisted,
    RegistryRestoredAfterRestart,
    FailClosedUnpaired,
    ManualRequiredPhysicalRouteRecovery,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum V09ProductionDiscoveryHouseholdProofState {
    CiMechanicalProof,
    ManualRequired,
    NotImplemented,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum V09ProductionDiscoveryHouseholdReadinessDecision {
    NotReadyForProductReadyHouseholdLanClaim,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum V09ProductionDiscoveryHouseholdManualProofGate {
    TwoPhysicalHosts,
    HouseholdRouterReachability,
    OsFirewallOrLocalNetworkPermission,
    AllowedOriginOnPhysicalController,
    PhysicalRouteSelectionAndTakeover,
    PhysicalRevocationAndRejection,
    PhysicalStaleOfflineSelectedDevice,
    RealMobileControllerPackage,
    RealMobileObserverPackage,
    RealLanAiProviderHost,
    CloudRelaySeparateProof,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V09ProductionDiscoveryHouseholdStateEvidence {
    #[serde(deserialize_with = "deserialize_lan_schema_version_text")]
    pub schema_version: LanPairingText,
    pub check: V09ProductionDiscoveryHouseholdCheck,
    pub source_state: V09ProductionDiscoveryHouseholdSourceState,
    pub route_id: String,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub trust_state: LanPairingTrustState,
    pub reachability: LanPairingDeviceReachability,
    pub rejection_reason: Option<LanPairingRejectionReason>,
    pub route_recovery_state: V09ProductionDiscoveryHouseholdRouteRecoveryState,
    pub proof_state: V09ProductionDiscoveryHouseholdProofState,
    pub runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    pub evidence_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V09ProductionDiscoveryHouseholdManualChecklistItem {
    #[serde(deserialize_with = "deserialize_lan_schema_version_text")]
    pub schema_version: LanPairingText,
    pub gate: V09ProductionDiscoveryHouseholdManualProofGate,
    pub state: V09ProductionDiscoveryHouseholdProofState,
    pub required_artifact_summary: String,
    pub runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V09ProductionDiscoveryHouseholdProofReadModel {
    #[serde(deserialize_with = "deserialize_lan_schema_version_text")]
    pub schema_version: LanPairingText,
    pub checked_at: String,
    pub proof_boundary: V09ProductionDiscoveryHouseholdProofBoundary,
    pub product_readiness_decision: V09ProductionDiscoveryHouseholdReadinessDecision,
    pub production_discovery_states: Vec<V09ProductionDiscoveryHouseholdStateEvidence>,
    pub route_checks: Vec<V09ProductionDiscoveryHouseholdStateEvidence>,
    pub restart_recovery: Vec<V09ProductionDiscoveryHouseholdStateEvidence>,
    pub source_device_states: Vec<V09ProductionDiscoveryHouseholdStateEvidence>,
    pub manual_household_proof_checklist: Vec<V09ProductionDiscoveryHouseholdManualChecklistItem>,
    pub claims_proved: Vec<String>,
    pub claims_not_proved: Vec<String>,
}
