use chrono::{SecondsFormat, Utc};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceMatrix;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDeviceReadModel,
    LanBrowserAddDeviceScanSummary, LanPairingDiscoverySource, LanSelectedDeviceReadiness,
};

use crate::network_inventory::api::discover_lan_network_devices;
use crate::network_inventory::LanNetworkInventoryDevice;
use crate::network_inventory_hardware::local_network_identity;
use crate::read_model_builder::{build_lan_add_device_read_model, LanAddDeviceReadModelInput};

mod source_rows;
mod workpack_rows;
use self::source_rows::source_rows;
use self::workpack_rows::workpack_rows;

pub fn current_lan_add_device_read_model() -> LanBrowserAddDeviceReadModel {
    let platform_data_available =
        platform_data_available_for_identity(local_network_identity().is_some());
    let network_devices = discover_lan_network_devices();
    lan_add_device_read_model_from_inventory_with_platform_data(
        &network_devices,
        generated_at(),
        platform_data_available,
    )
}

fn platform_data_available_for_identity(has_local_network_identity: bool) -> bool {
    platform_data_available_for_identity_with_manual_required_override(
        has_local_network_identity,
        apple_lan_discovery_is_manual_required(),
    )
}

pub fn platform_data_available_for_identity_with_manual_required_override(
    has_local_network_identity: bool,
    manual_required_platform: bool,
) -> bool {
    has_local_network_identity || manual_required_platform
}

fn apple_lan_discovery_is_manual_required() -> bool {
    cfg!(any(target_os = "macos", target_os = "ios"))
}

pub fn discovered_devices_from_network_inventory(
    network_devices: &[LanNetworkInventoryDevice],
    generated_at: &str,
) -> Vec<LanBrowserAddDeviceDiscoveryDevice> {
    crate::network_inventory::api::discovered_devices_from_network_inventory(
        network_devices,
        generated_at,
    )
}

pub fn lan_add_device_read_model_from_inventory(
    network_devices: &[LanNetworkInventoryDevice],
    generated_at: String,
) -> LanBrowserAddDeviceReadModel {
    lan_add_device_read_model_from_inventory_with_platform_data(network_devices, generated_at, true)
}

pub fn lan_add_device_read_model_from_inventory_with_platform_data(
    network_devices: &[LanNetworkInventoryDevice],
    generated_at: String,
    platform_data_available: bool,
) -> LanBrowserAddDeviceReadModel {
    let discovered_devices =
        discovered_devices_from_network_inventory(network_devices, &generated_at);
    let physical_household_lan_state = if discovered_devices.is_empty() {
        LanPairingProductionDiscoveryState::ManualRequired
    } else {
        LanPairingProductionDiscoveryState::Discovered
    };

    build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at,
        discovery_source: if discovered_devices.is_empty() {
            LanPairingDiscoverySource::LocalService
        } else {
            LanPairingDiscoverySource::PhysicalHouseholdLan
        },
        service_data_available: platform_data_available,
        platform_data_available,
        add_device_state: physical_household_lan_state.clone(),
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices,
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: Vec::new(),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    })
}

pub(crate) fn audit_check_labels() -> Vec<String> {
    [
        constants::value::LAN_REASON_ANONYMOUS,
        constants::value::LAN_REASON_WRONG_ORIGIN,
        constants::value::LAN_REASON_WRONG_DEVICE,
        constants::value::LAN_REASON_REPLAYED,
        constants::value::LAN_REASON_STALE,
        constants::value::LAN_REASON_REVOKED,
        constants::value::LAN_REASON_OFFLINE,
        constants::value::LAN_REASON_EXPIRED,
    ]
    .iter()
    .map(|label| (*label).to_string())
    .collect()
}

pub(crate) fn honest_non_claims() -> Vec<String> {
    [
        constants::value::LAN_NON_CLAIM_PHYSICAL_HOUSEHOLD_MANUAL_REQUIRED,
        constants::value::LAN_NON_CLAIM_CLOUD_RELAY_NOT_IMPLEMENTED,
        constants::value::LAN_NON_CLAIM_REMOTE_DESKTOP_NOT_IMPLEMENTED,
    ]
    .iter()
    .map(|claim| (*claim).to_string())
    .collect()
}

fn generated_at() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

pub(crate) fn lan_discovery_source_matrix(
    generated_at: &str,
    scan_summary: &LanBrowserAddDeviceScanSummary,
) -> LanDiscoverySourceMatrix {
    LanDiscoverySourceMatrix {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        workpack_rows: workpack_rows(scan_summary),
        source_rows: source_rows(),
        claims_proved: vec![
            constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_READ_MODEL.to_string(),
            constants::lan_pairing::LAN_SOURCE_MATRIX_CLAIM_WEAK_SOURCES.to_string(),
        ],
        claims_not_proved: vec![
            constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PACKET_MODE.to_string(),
            constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_PHYSICAL.to_string(),
            constants::lan_pairing::LAN_SOURCE_MATRIX_NON_CLAIM_MDNS_SSDP.to_string(),
        ],
    }
}
