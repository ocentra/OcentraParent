use serde::{Deserialize, Serialize};

use crate::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingNetworkMode, LanPairingParentAuthority, LanPairingProductionDiscoveryState,
    LanPairingRejectionReason, LanPairingTrustState, LanTrustedDeviceRegistryEntry,
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
    pub schema_version: u16,
    pub discovered_at: String,
    pub child_device: LanPairingDeviceRef,
    pub agent_peer_id: String,
    pub route_id: String,
    pub network_mode: LanPairingNetworkMode,
    pub reachability: LanPairingDeviceReachability,
    pub address_ref: String,
    pub discovery_status: LanPairingDiscoveryRuntimeStatus,
    pub discovery_state: LanPairingProductionDiscoveryState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanBrowserAddDevicePairingRequest {
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
    pub schema_version: u16,
    pub source_labels: Vec<String>,
    pub scanned_device_count: u32,
    pub agent_device_count: u32,
    pub passive_device_count: u32,
    pub infrastructure_device_count: u32,
    pub unsupported_device_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanSelectedDeviceReadiness {
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
#[serde(rename_all = "camelCase")]
pub struct LanBrowserAddDeviceReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub discovery_source: LanPairingDiscoverySource,
    pub add_device_state: LanPairingProductionDiscoveryState,
    pub local_service_discovery_state: LanPairingProductionDiscoveryState,
    pub physical_household_lan_state: LanPairingProductionDiscoveryState,
    pub cloud_relay_state: LanPairingProductionDiscoveryState,
    pub scan_summary: LanBrowserAddDeviceScanSummary,
    pub discovered_devices: Vec<LanBrowserAddDeviceDiscoveryDevice>,
    pub pairing_requests: Vec<LanBrowserAddDevicePairingRequest>,
    pub trusted_device_registry: Vec<LanTrustedDeviceRegistryEntry>,
    pub trusted_device_ids: Vec<String>,
    pub revoked_device_ids: Vec<String>,
    pub selected_device_readiness: LanSelectedDeviceReadiness,
    pub controller_authority: LanPairingParentAuthority,
    pub observer_authority: LanPairingParentAuthority,
    pub route_requirement_labels: Vec<String>,
    pub audit_check_labels: Vec<String>,
    pub honest_non_claims: Vec<String>,
}
