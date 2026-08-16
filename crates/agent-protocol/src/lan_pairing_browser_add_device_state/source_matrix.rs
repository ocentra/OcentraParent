use serde::{Deserialize, Serialize};

use super::deserialize_lan_schema_version;
use crate::{
    LanPairingProductionDiscoveryState, V09ProductionDiscoveryHouseholdProofState,
    V09ProductionDiscoveryHouseholdRuntimeOwner,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPlanWorkpackId {
    #[serde(rename = "01")]
    W01,
    #[serde(rename = "02")]
    W02,
    #[serde(rename = "03")]
    W03,
    #[serde(rename = "04")]
    W04,
    #[serde(rename = "05")]
    W05,
    #[serde(rename = "06")]
    W06,
    #[serde(rename = "07")]
    W07,
    #[serde(rename = "08")]
    W08,
    #[serde(rename = "09")]
    W09,
    #[serde(rename = "10")]
    W10,
    #[serde(rename = "11")]
    W11,
    #[serde(rename = "12")]
    W12,
    #[serde(rename = "13")]
    W13,
    #[serde(rename = "14")]
    W14,
    #[serde(rename = "15")]
    W15,
    #[serde(rename = "16")]
    W16,
    #[serde(rename = "17")]
    W17,
    #[serde(rename = "18")]
    W18,
    #[serde(rename = "19")]
    W19,
    #[serde(rename = "20")]
    W20,
    #[serde(rename = "21")]
    W21,
    #[serde(rename = "22")]
    W22,
    #[serde(rename = "23")]
    W23,
    #[serde(rename = "24")]
    W24,
    #[serde(rename = "25")]
    W25,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoverySourceKind {
    ContractBoundary,
    EvidenceModel,
    InterfaceSelection,
    WindowsNeighborTable,
    LinuxProcNetArp,
    LinuxIpNeigh,
    MacosArp,
    TargetedArpRefresh,
    BoundedArpSweep,
    PassiveArpListener,
    PassiveDhcpListener,
    PassiveMdnsListener,
    PassiveSsdpListener,
    PassiveWsDiscoveryListener,
    PassiveLlmnrListener,
    PassiveNetbiosListener,
    PassiveSnmpResponseListener,
    MdnsDnsSdQuery,
    SsdpUpnpQuery,
    NetbiosNameCache,
    LlmnrNameQuery,
    ReverseDnsQuery,
    ServiceIdentityProbe,
    PreviousScanSnapshot,
    OuiVendorLookup,
    MergeDeduplication,
    ExplainableClassification,
    HouseholdDeviceStore,
    ReadModelEventStream,
    ParentMdnsAdvertisement,
    ChildMdnsAdvertisement,
    SignedChildAgentHello,
    SignedChildAgentHeartbeat,
    AssignmentRevocationAudit,
    ProofGateRollout,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoverySourceStatus {
    Implemented,
    Partial,
    ParserProof,
    ManualRequired,
    NotImplemented,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoverySourceAuthority {
    StrongIdentity,
    WeakIdentity,
    NameOnly,
    ClassificationOnly,
    PresenceOnly,
    ManualParentDecision,
    RouteCustody,
    ProofGate,
    NoChildConfirmation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoverySourceRuntimePath {
    TypescriptContract,
    AgentProtocol,
    RustServiceReadModel,
    PortalReadModel,
    ProofHarness,
    ManualArtifact,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoverySourceUiSurface {
    DevicesLan,
    ActivityNetwork,
    PolicyNetwork,
    SetupFlow,
    ProofReport,
    NotVisible,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPlanWorkpackStatusRow {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub workpack_id: LanPlanWorkpackId,
    pub title: String,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub proof_state: V09ProductionDiscoveryHouseholdProofState,
    pub runtime_owner: V09ProductionDiscoveryHouseholdRuntimeOwner,
    pub status: LanDiscoverySourceStatus,
    pub read_model_visible: bool,
    pub required_artifact_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanDiscoverySourceRow {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub source: LanDiscoverySourceKind,
    pub workpack_id: LanPlanWorkpackId,
    pub status: LanDiscoverySourceStatus,
    pub authority: LanDiscoverySourceAuthority,
    pub runtime_path: LanDiscoverySourceRuntimePath,
    pub ui_surface: LanDiscoverySourceUiSurface,
    pub can_confirm_child_agent: bool,
    pub can_assign_child_profile: bool,
    pub can_control_route: bool,
    pub requires_selected_interface: bool,
    pub persists_across_restart: bool,
    pub evidence_label: String,
    pub required_artifact_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanDiscoverySourceMatrix {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub generated_at: String,
    pub workpack_rows: Vec<LanPlanWorkpackStatusRow>,
    pub source_rows: Vec<LanDiscoverySourceRow>,
    pub claims_proved: Vec<String>,
    pub claims_not_proved: Vec<String>,
}
