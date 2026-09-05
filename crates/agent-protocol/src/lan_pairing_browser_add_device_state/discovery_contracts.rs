use serde::{Deserialize, Serialize};

use super::{
    deserialize_lan_schema_version, LanPairingDeviceReachability, LanPairingDeviceRef,
    LanPairingDiscoveryRuntimeStatus, LanPairingNetworkMode, LanPairingProductionDiscoveryState,
    LanPairingRejectionReason, LanPairingTrustState,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanPairingDiscoverySource {
    LocalService,
    PhysicalHouseholdLan,
    CloudRelay,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanBrowserAddDeviceDiscoveryDevice {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub discovered_at: String,
    pub child_device: LanPairingDeviceRef,
    pub agent_peer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pairing_id: Option<String>,
    pub route_id: String,
    pub network_mode: LanPairingNetworkMode,
    pub reachability: LanPairingDeviceReachability,
    pub address_ref: String,
    pub discovery_status: LanPairingDiscoveryRuntimeStatus,
    pub discovery_state: LanPairingProductionDiscoveryState,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_sources: Vec<LanDiscoveryEvidenceSource>,
    #[serde(default)]
    pub hint_sources: Vec<LanDiscoveryEvidenceSource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_identity_probe_evidence: Vec<LanServiceIdentityProbeEvidence>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanServiceIdentityProbeEvidenceKind {
    HttpStatus,
    HtmlTitle,
    ServerHeader,
    Banner,
    RedirectLocation,
    CertificateSubject,
    DescriptorLink,
    WsdEndpointAddress,
    WsdTypes,
    SnmpSysDescr,
    SnmpSysName,
    MdnsServiceType,
    MdnsInstanceName,
    SsdpUdn,
    SsdpDeviceType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanServiceIdentityProbeEvidence {
    pub evidence_kind: LanServiceIdentityProbeEvidenceKind,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_interface: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanBrowserAddDevicePairingRequest {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub challenge_id: String,
    pub child_device_id: String,
    pub parent_device_id: String,
    pub route_id: String,
    pub origin: String,
    pub pairing_state: LanPairingProductionDiscoveryState,
    pub rejection_reason: Option<LanPairingRejectionReason>,
    pub issued_at: String,
    pub expires_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanBrowserAddDeviceScanSummary {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub source_labels: Vec<String>,
    pub scanned_device_count: u32,
    pub agent_device_count: u32,
    pub passive_device_count: u32,
    pub infrastructure_device_count: u32,
    pub unsupported_device_count: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passive_local_neighbor_collection_summaries:
        Vec<LanPassiveDiscoveryLocalNeighborCollectionSummary>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPassiveDiscoveryLocalNeighborCollectionSummary {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub source_label: String,
    pub observed_count: u32,
    pub recorded_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanSelectedDeviceReadiness {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub selected_child_device_id: Option<String>,
    pub route_id: Option<String>,
    pub pairing_id: Option<String>,
    pub trust_state: LanPairingTrustState,
    pub reachability: LanPairingDeviceReachability,
    pub ready_for_control: bool,
    pub stale_at: Option<String>,
    pub offline_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoveryEventHistoryState {
    Ready,
    Empty,
    AgentOffline,
    ManualRequired,
    Unavailable,
    Degraded,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoveryEventKind {
    InterfaceChanged,
    ScanStarted,
    ScanFinished,
    EvidenceFound,
    DeviceFound,
    DeviceUpdated,
    DeviceOnline,
    DeviceOffline,
    AgentDiscovered,
    AgentConfirmed,
    UnknownDetected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanDiscoveryEventRow {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub event_id: String,
    pub event_kind: LanDiscoveryEventKind,
    pub occurred_at: String,
    pub previous_event_id: Option<String>,
    pub scan_session_id: Option<String>,
    pub affected_device_id: Option<String>,
    pub evidence_id: Option<String>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanDiscoveryEventHistory {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub generated_at: String,
    pub state: LanDiscoveryEventHistoryState,
    pub latest_event_id: Option<String>,
    pub latest_observed_at: Option<String>,
    pub rows: Vec<LanDiscoveryEventRow>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdDeviceRole {
    ParentController,
    ParentObserver,
    ChildAgent,
    Portal,
    AiProvider,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdDeviceSource {
    LocalService,
    NetworkNeighbor,
    TrustedRegistry,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdDeviceConfidence {
    AgentConfirmed,
    MacIpMatch,
    NetworkNeighbor,
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoveryEvidenceSource {
    LocalService,
    ServiceIdentityProbe,
    WindowsNeighborTable,
    MdnsDnsSdQuery,
    SsdpUpnpQuery,
    LinuxProcNetArp,
    LinuxIpNeigh,
    MacosArp,
    PreviousScanSnapshot,
    DnsCache,
    Netbios,
    Llmnr,
    TrustedRegistry,
    ParentAssignment,
    ChildAgentHello,
    ChildAgentHeartbeat,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoveryEvidenceKind {
    Interface,
    IpAddress,
    MacAddress,
    Hostname,
    InstallId,
    PairingId,
    Vendor,
    RouterClassification,
    ChildAgentPresence,
    HistoricalIdentityHint,
    TrustedRegistry,
    ParentDecision,
    Route,
    ServiceProbeHint,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoveryEvidenceConfidence {
    Confirmed,
    Strong,
    Weak,
    ManualRequired,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanDiscoveryEvidenceRecord {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub evidence_id: String,
    pub source: LanDiscoveryEvidenceSource,
    pub evidence_kind: LanDiscoveryEvidenceKind,
    pub device_id: String,
    pub value: String,
    pub normalized_value: String,
    pub first_seen_at: String,
    pub last_seen_at: String,
    pub expires_at: Option<String>,
    pub confidence: LanDiscoveryEvidenceConfidence,
    pub merge_key: String,
    pub note: Option<String>,
}
