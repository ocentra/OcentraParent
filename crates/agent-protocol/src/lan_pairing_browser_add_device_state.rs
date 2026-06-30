use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};

use crate::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingNetworkMode, LanPairingParentAuthority, LanPairingProductionDiscoveryState,
    LanPairingRejectionReason, LanPairingTrustState, LanTrustedDeviceRegistryEntry,
};

pub mod production_household_proof;
pub mod signed_discovery_relay_spine;
pub mod source_matrix;
use self::{
    production_household_proof::LanProductionHouseholdProofSummary,
    signed_discovery_relay_spine::LanSignedDiscoveryRelaySpineSummary,
    source_matrix::LanDiscoverySourceMatrix,
};

fn deserialize_lan_schema_version<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let version = u16::deserialize(deserializer)?;
    if version == crate::constants::lan_pairing::SCHEMA_VERSION {
        Ok(version)
    } else {
        Err(de::Error::custom(format!(
            "unsupported LAN schema version {version}; expected {}",
            crate::constants::lan_pairing::SCHEMA_VERSION
        )))
    }
}

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
pub enum LanCanonicalHouseholdDeviceClassification {
    ChildAgent,
    Phone,
    Tablet,
    Laptop,
    Desktop,
    Printer,
    Television,
    GameConsole,
    Camera,
    NetworkAttachedStorage,
    InternetOfThings,
    NetworkInfrastructure,
    UnsupportedLanDevice,
    UnknownLanDevice,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanHouseholdDeviceActionKind {
    Assign,
    Rename,
    Ignore,
    Revoke,
    Restore,
    Trust,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanHouseholdDeviceDecision {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub action_id: String,
    pub action_kind: LanHouseholdDeviceActionKind,
    pub canonical_device_id: String,
    pub child_profile_id: Option<String>,
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_kind: Option<String>,
    pub parent_actor_id: String,
    pub decided_at: String,
    pub revoked_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdRouteState {
    Localhost,
    LocalNetwork,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdRoleState {
    Implemented,
    Scaffold,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanCanonicalHouseholdSurface {
    Devices,
    Policy,
    Browser,
    App,
    Screen,
    Network,
    Activity,
    Tracking,
    Ai,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanCanonicalHouseholdNetworkIdentity {
    pub hostname: Option<String>,
    pub ip_addresses: Vec<String>,
    pub mac_address: Option<String>,
    pub mac_vendor: Option<String>,
    pub network_interfaces: Vec<String>,
    pub reachability: LanPairingDeviceReachability,
    pub confidence: LanCanonicalHouseholdDeviceConfidence,
    pub stale_at: Option<String>,
    pub offline_at: Option<String>,
    pub evidence_records: Vec<LanDiscoveryEvidenceRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanChildAgentInventoryPacket {
    pub device_name: String,
    pub platform: String,
    pub os: String,
    pub cpu_model: Option<String>,
    pub cpu_cores: Option<String>,
    pub memory_total: Option<String>,
    pub gpu_model: Option<String>,
    pub gpu_driver: Option<String>,
    pub gpu_memory: Option<String>,
    pub nvidia_smi: Option<String>,
    pub network_interfaces: Vec<String>,
    pub capabilities: Vec<String>,
    pub role_state: LanCanonicalHouseholdRoleState,
    pub route_state: LanCanonicalHouseholdRouteState,
    pub pairing_trust_state: LanPairingTrustState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanCanonicalHouseholdDevice {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub canonical_device_id: String,
    pub display_name: String,
    pub classification: LanCanonicalHouseholdDeviceClassification,
    pub role_badges: Vec<LanCanonicalHouseholdDeviceRole>,
    pub enrollable: bool,
    pub discovery_state: LanPairingProductionDiscoveryState,
    pub trust_state: LanPairingTrustState,
    pub route_id: Option<String>,
    pub route_state: LanCanonicalHouseholdRouteState,
    pub network_mode: LanPairingNetworkMode,
    pub source_labels: Vec<LanCanonicalHouseholdDeviceSource>,
    pub network_identity: LanCanonicalHouseholdNetworkIdentity,
    pub child_agent_inventory: Option<LanChildAgentInventoryPacket>,
    pub policy_target_surfaces: Vec<LanCanonicalHouseholdSurface>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanBrowserAddDeviceReadModel {
    #[serde(deserialize_with = "deserialize_lan_schema_version")]
    pub schema_version: u16,
    pub generated_at: String,
    pub discovery_source: LanPairingDiscoverySource,
    pub add_device_state: LanPairingProductionDiscoveryState,
    pub local_service_discovery_state: LanPairingProductionDiscoveryState,
    pub physical_household_lan_state: LanPairingProductionDiscoveryState,
    pub cloud_relay_state: LanPairingProductionDiscoveryState,
    pub scan_summary: LanBrowserAddDeviceScanSummary,
    pub discovered_devices: Vec<LanBrowserAddDeviceDiscoveryDevice>,
    pub discovery_event_history: LanDiscoveryEventHistory,
    pub canonical_household_devices: Vec<LanCanonicalHouseholdDevice>,
    pub pairing_requests: Vec<LanBrowserAddDevicePairingRequest>,
    pub trusted_device_registry: Vec<LanTrustedDeviceRegistryEntry>,
    pub household_device_decisions: Vec<LanHouseholdDeviceDecision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_household_proof: Option<LanProductionHouseholdProofSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_discovery_relay_spine: Option<LanSignedDiscoveryRelaySpineSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lan_discovery_source_matrix: Option<LanDiscoverySourceMatrix>,
    pub trusted_device_ids: Vec<String>,
    pub revoked_device_ids: Vec<String>,
    pub selected_device_readiness: LanSelectedDeviceReadiness,
    pub controller_authority: LanPairingParentAuthority,
    pub observer_authority: LanPairingParentAuthority,
    pub route_requirement_labels: Vec<String>,
    pub audit_check_labels: Vec<String>,
    pub honest_non_claims: Vec<String>,
}
