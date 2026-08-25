use serde::{Deserialize, Serialize};

use super::{
    deserialize_lan_schema_version, LanBrowserAddDeviceDiscoveryDevice,
    LanBrowserAddDevicePairingRequest, LanBrowserAddDeviceScanSummary,
    LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdDeviceConfidence,
    LanCanonicalHouseholdDeviceRole, LanCanonicalHouseholdDeviceSource, LanDiscoveryEventHistory,
    LanDiscoveryEvidenceRecord, LanPairingDiscoverySource, LanPairingNetworkMode,
    LanPairingProductionDiscoveryState, LanPairingTrustState, LanSelectedDeviceReadiness,
};
use crate::{LanPairingParentAuthority, LanTrustedDeviceRegistryEntry};

use super::production_household_proof::LanProductionHouseholdProofSummary;
use super::signed_discovery_relay_spine::LanSignedDiscoveryRelaySpineSummary;
use super::source_matrix::LanDiscoverySourceMatrix;

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
    pub reachability: super::LanPairingDeviceReachability,
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
