pub mod canonical_household_device_spine;
mod history;
mod history_time;
mod production_household_proof;
mod read_model;
mod scan;
mod signed_discovery_relay_spine;

use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, LanTrustedDeviceRegistryEntry,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDevicePairingRequest,
    LanBrowserAddDeviceReadModel, LanCanonicalHouseholdDevice, LanHouseholdDeviceDecision,
    LanSelectedDeviceReadiness,
};

pub struct LanAddDeviceReadModelInput {
    pub generated_at: String,
    pub discovery_source:
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanPairingDiscoverySource,
    pub service_data_available: bool,
    pub platform_data_available: bool,
    pub add_device_state: LanPairingProductionDiscoveryState,
    pub local_service_discovery_state: LanPairingProductionDiscoveryState,
    pub physical_household_lan_state: LanPairingProductionDiscoveryState,
    pub cloud_relay_state: LanPairingProductionDiscoveryState,
    pub discovered_devices: Vec<LanBrowserAddDeviceDiscoveryDevice>,
    pub pairing_requests: Vec<LanBrowserAddDevicePairingRequest>,
    pub trusted_device_registry: Vec<LanTrustedDeviceRegistryEntry>,
    pub household_device_decisions: Vec<LanHouseholdDeviceDecision>,
    pub trusted_device_ids: Vec<String>,
    pub revoked_device_ids: Vec<String>,
    pub selected_device_readiness: LanSelectedDeviceReadiness,
    pub controller_authority: LanPairingParentAuthority,
    pub observer_authority: LanPairingParentAuthority,
}

pub fn build_lan_add_device_read_model(
    input: LanAddDeviceReadModelInput,
) -> LanBrowserAddDeviceReadModel {
    read_model::build_lan_add_device_read_model(input)
}

pub fn canonical_household_devices(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    observed_at: &str,
) -> Vec<LanCanonicalHouseholdDevice> {
    canonical_household_device_spine::canonical_household_devices(
        discovered_devices,
        trusted_registry,
        household_device_decisions,
        observed_at,
    )
}
