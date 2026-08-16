use std::time::{Duration, Instant};

use ocentra_lan_core::read_model_builder::{
    build_lan_add_device_read_model, LanAddDeviceReadModelInput,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingNetworkMode, LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanDiscoveryEvidenceSource, LanPairingDiscoverySource,
    LanSelectedDeviceReadiness,
};
use ocentra_parent_agent_protocol::LanTrustedDeviceRegistryEntry;

const READ_MODEL_256_DEVICE_BUDGET: Duration = Duration::from_secs(2);
const TRUSTED_REGISTRY_1000_DEVICE_BUDGET: Duration = Duration::from_secs(8);

#[test]
fn lan_read_model_builds_256_discovered_devices_inside_budget() {
    let devices = (0..256).map(discovered_device).collect::<Vec<_>>();

    let started = Instant::now();
    let model = build_lan_add_device_read_model(input_with_discovered_devices(devices));
    let elapsed = started.elapsed();

    assert_eq!(model.discovered_devices.len(), 256);
    assert_eq!(model.canonical_household_devices.len(), 256);
    assert!(
        elapsed <= READ_MODEL_256_DEVICE_BUDGET,
        "256-device LAN read model elapsed {elapsed:?}, budget {READ_MODEL_256_DEVICE_BUDGET:?}"
    );
}

#[test]
fn lan_read_model_restores_1000_trusted_devices_inside_budget() {
    let registry = (0..1000).map(trusted_registry_entry).collect::<Vec<_>>();

    let started = Instant::now();
    let model = build_lan_add_device_read_model(input_with_trusted_registry(registry));
    let elapsed = started.elapsed();

    assert_eq!(model.trusted_device_registry.len(), 1000);
    assert_eq!(model.canonical_household_devices.len(), 1000);
    assert!(
        elapsed <= TRUSTED_REGISTRY_1000_DEVICE_BUDGET,
        "1000-device LAN registry restore elapsed {elapsed:?}, budget {TRUSTED_REGISTRY_1000_DEVICE_BUDGET:?}"
    );
}

fn input_with_discovered_devices(
    discovered_devices: Vec<LanBrowserAddDeviceDiscoveryDevice>,
) -> LanAddDeviceReadModelInput {
    LanAddDeviceReadModelInput {
        discovered_devices,
        ..base_input()
    }
}

fn input_with_trusted_registry(
    trusted_device_registry: Vec<LanTrustedDeviceRegistryEntry>,
) -> LanAddDeviceReadModelInput {
    LanAddDeviceReadModelInput {
        trusted_device_ids: trusted_device_registry
            .iter()
            .map(|entry| entry.child_device.device_id.clone())
            .collect(),
        trusted_device_registry,
        ..base_input()
    }
}

fn base_input() -> LanAddDeviceReadModelInput {
    LanAddDeviceReadModelInput {
        generated_at: "2026-06-28T10:00:00Z".to_string(),
        discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: Vec::new(),
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
    }
}

fn discovered_device(index: usize) -> LanBrowserAddDeviceDiscoveryDevice {
    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: format!("2026-06-28T10:{:02}:00Z", index % 60),
        child_device: child_device(index),
        agent_peer_id: format!("perf-peer-{index}"),
        pairing_id: None,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        evidence_sources: vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        hint_sources: Vec::new(),
        service_identity_probe_evidence: Vec::new(),
    }
}

fn trusted_registry_entry(index: usize) -> LanTrustedDeviceRegistryEntry {
    LanTrustedDeviceRegistryEntry {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: format!("perf-pairing-{index}"),
        child_device: child_device(index),
        parent_device: LanPairingDeviceRef::new(
            constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
            None,
            "Parent".to_string(),
            constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        ),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: "trusted-registry-performance-test".to_string(),
        proof_digest: format!("sha256:perf-proof-{index}"),
        trust_state: LanPairingTrustState::Paired,
        trusted_at: "2026-06-28T09:59:00Z".to_string(),
        expires_at: "2026-06-29T09:59:00Z".to_string(),
        revoked_at: None,
    }
}

fn child_device(index: usize) -> LanPairingDeviceRef {
    let mut child_device = LanPairingDeviceRef::new(
        format!("perf-child-{index}"),
        Some(format!("perf-profile-{index}")),
        format!("Perf LAN Device {index}"),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    child_device.hostname = Some(format!("perf-child-{index}.local"));
    child_device.ip_address = Some(format!("192.168.2.{}", (index % 253) + 1));
    child_device.mac_address = Some(format!(
        "02-00-00-{:02x}-{:02x}-{:02x}",
        (index >> 16) & 0xff,
        (index >> 8) & 0xff,
        index & 0xff
    ));
    child_device.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());
    child_device
}
