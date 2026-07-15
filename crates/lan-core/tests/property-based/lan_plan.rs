use crate::support::OptionTestExt as _;
use std::collections::BTreeSet;

use super::*;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceConfidence, LanDiscoveryEvidenceSource,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct LanText(String);

macro_rules! lt {
    ($value:expr) => {
        LanText(($value).to_string())
    };
}
fn assert_duplicate_source_invariants(
    network_devices: &[LanNetworkInventoryDevice],
    expected_source_sets: &BTreeSet<BTreeSet<LanText>>,
) {
    let model = lan_add_device_read_model_from_inventory(
        network_devices,
        "2026-06-27T12:00:00Z".to_string(),
    );
    assert_duplicate_source_model_shape(&model, expected_source_sets);
    assert_duplicate_source_event_history(&model);
    assert_duplicate_source_device_ids_unique(&model);
}
fn assert_duplicate_source_model_shape(
    model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
    expected_source_sets: &BTreeSet<BTreeSet<LanText>>,
) {
    assert_eq!(model.canonical_household_devices.len(), 3);
    assert_eq!(model.discovered_devices.len(), 3);
    assert_eq!(&canonical_source_sets(model), expected_source_sets);
    for device in &model.canonical_household_devices {
        assert_eq!(
            device.classification,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::UnknownLanDevice
        );
    }
    assert_eq!(
        event_count(model, &LanDiscoveryEventKind::UnknownDetected),
        3
    );
    assert_eq!(event_count(model, &LanDiscoveryEventKind::DeviceOnline), 3);
}

fn assert_duplicate_source_event_history(
    model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
) {
    assert_eq!(
        model
            .discovery_event_history
            .rows
            .first()
            .map(|row| &row.event_kind),
        Some(&LanDiscoveryEventKind::ScanStarted)
    );
    assert_eq!(
        model
            .discovery_event_history
            .rows
            .last()
            .map(|row| &row.event_kind),
        Some(&LanDiscoveryEventKind::ScanFinished)
    );
    assert!(model
        .discovery_event_history
        .rows
        .iter()
        .skip(1)
        .all(|row| row.previous_event_id.is_some()));
}

fn assert_duplicate_source_device_ids_unique(
    model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
) {
    assert_eq!(
        model
            .canonical_household_devices
            .iter()
            .map(|device| device.canonical_device_id.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        model.canonical_household_devices.len()
    );
}

#[test]
fn merge_and_evidence_invariants_hold_across_duplicate_source_permutations() {
    let expected_source_sets = BTreeSet::from([BTreeSet::from([lt!("WindowsNeighborTable")])]);
    assert_duplicate_source_invariants(
        &[
            inventory_device(
                "lan-merge-a",
                "mystery-device.local",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            ),
            inventory_device(
                "lan-merge-b",
                "mystery-device.local",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
                ],
            ),
            inventory_device(
                "lan-merge-c",
                "mystery-device.local",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
                ],
            ),
        ],
        &expected_source_sets,
    );
    assert_duplicate_source_invariants(
        &[
            inventory_device(
                "lan-merge-c",
                "mystery-device.local",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
                ],
            ),
            inventory_device(
                "lan-merge-a",
                "mystery-device.local",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            ),
            inventory_device(
                "lan-merge-b",
                "mystery-device.local",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
                ],
            ),
        ],
        &expected_source_sets,
    );
    assert_duplicate_source_invariants(
        &[
            inventory_device(
                "lan-merge-b",
                "mystery-device.local",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
                ],
            ),
            inventory_device(
                "lan-merge-c",
                "mystery-device.local",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
                ],
            ),
            inventory_device(
                "lan-merge-a",
                "mystery-device.local",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            ),
        ],
        &expected_source_sets,
    );
}

#[test]
fn locally_administered_mac_neighbors_stay_split_for_every_input_order() {
    assert_locally_administered_mac_neighbors_split(&[
        inventory_device(
            "randomized-one",
            "Printer One",
            "192.168.1.31",
            "02-aa-bb-cc-dd-ee",
            LanPairingDeviceReachability::Online,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        ),
        inventory_device(
            "randomized-two",
            "Printer Two",
            "192.168.1.32",
            "02-aa-bb-cc-dd-ee",
            LanPairingDeviceReachability::Online,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        ),
        inventory_device(
            "randomized-three",
            "Printer Three",
            "192.168.1.33",
            "02-aa-bb-cc-dd-ee",
            LanPairingDeviceReachability::Online,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        ),
    ]);
    assert_locally_administered_mac_neighbors_split(&[
        inventory_device(
            "randomized-three",
            "Printer Three",
            "192.168.1.33",
            "02-aa-bb-cc-dd-ee",
            LanPairingDeviceReachability::Online,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        ),
        inventory_device(
            "randomized-one",
            "Printer One",
            "192.168.1.31",
            "02-aa-bb-cc-dd-ee",
            LanPairingDeviceReachability::Online,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        ),
        inventory_device(
            "randomized-two",
            "Printer Two",
            "192.168.1.32",
            "02-aa-bb-cc-dd-ee",
            LanPairingDeviceReachability::Online,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        ),
    ]);
    assert_locally_administered_mac_neighbors_split(&[
        inventory_device(
            "randomized-two",
            "Printer Two",
            "192.168.1.32",
            "02-aa-bb-cc-dd-ee",
            LanPairingDeviceReachability::Online,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        ),
        inventory_device(
            "randomized-three",
            "Printer Three",
            "192.168.1.33",
            "02-aa-bb-cc-dd-ee",
            LanPairingDeviceReachability::Online,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        ),
        inventory_device(
            "randomized-one",
            "Printer One",
            "192.168.1.31",
            "02-aa-bb-cc-dd-ee",
            LanPairingDeviceReachability::Online,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        ),
    ]);
}

fn assert_locally_administered_mac_neighbors_split(network_devices: &[LanNetworkInventoryDevice]) {
    let model = lan_add_device_read_model_from_inventory(
        network_devices,
        "2026-06-27T12:00:00Z".to_string(),
    );

    assert_eq!(model.canonical_household_devices.len(), 3);
    assert_eq!(
        model.canonical_household_devices[0]
            .network_identity
            .confidence,
        LanCanonicalHouseholdDeviceConfidence::ManualRequired
    );
    assert!(!model.canonical_household_devices[0].enrollable);
    assert!(model.canonical_household_devices[0]
        .child_agent_inventory
        .is_none());
    assert_eq!(
        model.canonical_household_devices[1]
            .network_identity
            .confidence,
        LanCanonicalHouseholdDeviceConfidence::ManualRequired
    );
    assert!(!model.canonical_household_devices[1].enrollable);
    assert!(model.canonical_household_devices[1]
        .child_agent_inventory
        .is_none());
    assert_eq!(
        model.canonical_household_devices[2]
            .network_identity
            .confidence,
        LanCanonicalHouseholdDeviceConfidence::ManualRequired
    );
    assert!(!model.canonical_household_devices[2].enrollable);
    assert!(model.canonical_household_devices[2]
        .child_agent_inventory
        .is_none());
}

#[test]
fn reachability_event_ordering_stays_honest_for_online_offline_and_stale_neighbors() {
    assert_reachability_event_ordering_case(
        "online-neighbor",
        LanPairingDeviceReachability::Online,
        &LanDiscoveryEventKind::DeviceOnline,
    );
    assert_reachability_event_ordering_case(
        "offline-neighbor",
        LanPairingDeviceReachability::Offline,
        &LanDiscoveryEventKind::DeviceOffline,
    );
    assert_reachability_event_ordering_case(
        "stale-neighbor",
        LanPairingDeviceReachability::Stale,
        &LanDiscoveryEventKind::DeviceUpdated,
    );
}

fn assert_reachability_event_ordering_case(
    label: impl std::fmt::Display,
    reachability: LanPairingDeviceReachability,
    expected_event: &LanDiscoveryEventKind,
) {
    let label = label.to_string();
    let model = lan_add_device_read_model_from_inventory(
        &[inventory_device(
            label.as_str(),
            label.as_str(),
            constants::lan_pairing::TEST_LAN_IP,
            constants::lan_pairing::TEST_LAN_MAC,
            reachability,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        )],
        "2026-06-27T12:00:00Z".to_string(),
    );
    let event_kinds = model
        .discovery_event_history
        .rows
        .iter()
        .map(|row| row.event_kind.clone())
        .collect::<Vec<_>>();
    let expected_index = event_kinds
        .iter()
        .position(|event_kind| event_kind == expected_event)
        .value_or_unreachable();
    let finish_index = event_kinds
        .iter()
        .position(|event_kind| *event_kind == LanDiscoveryEventKind::ScanFinished)
        .value_or_unreachable();

    assert_eq!(
        event_kinds.first(),
        Some(&LanDiscoveryEventKind::ScanStarted)
    );
    assert!(
        expected_index > 0,
        "reachability event should not be the first row"
    );
    assert!(
        expected_index < finish_index,
        "reachability event should appear before scan finished"
    );
    assert_eq!(
        model
            .discovery_event_history
            .rows
            .last()
            .map(|row| &row.event_kind),
        Some(&LanDiscoveryEventKind::ScanFinished)
    );
    assert!(model
        .discovery_event_history
        .rows
        .iter()
        .skip(1)
        .all(|row| row.previous_event_id.is_some()));
}

#[test]
fn stronger_local_service_child_merge_wins_across_input_orders() {
    for discovered_devices in [
        vec![
            local_service_child_discovery_device("trusted-child-local", "2026-06-27T11:00:00Z"),
            ip_only_neighbor_discovery_device("neighbor-shadow-child", "2026-06-27T11:00:05Z"),
        ],
        vec![
            ip_only_neighbor_discovery_device("neighbor-shadow-child", "2026-06-27T11:00:05Z"),
            local_service_child_discovery_device("trusted-child-local", "2026-06-27T11:00:00Z"),
        ],
    ] {
        let model = local_service_read_model(discovered_devices);

        assert_eq!(model.canonical_household_devices.len(), 1);
        let canonical = &model.canonical_household_devices[0];
        assert_eq!(
            canonical.classification,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::ChildAgent
        );
        assert!(canonical.enrollable);
        let child_agent_inventory = canonical
            .child_agent_inventory
            .as_ref()
            .value_or_unreachable();
        assert_eq!(
            child_agent_inventory.device_name,
            "study-laptop.local".to_string()
        );
        assert_eq!(
            child_agent_inventory.platform,
            constants::lan_pairing::PLATFORM_WINDOWS
        );
        assert_eq!(
            child_agent_inventory.os,
            constants::lan_pairing::PLATFORM_WINDOWS
        );
        assert_eq!(
            child_agent_inventory.network_interfaces,
            vec![constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()]
        );
        assert_eq!(
            canonical.network_identity.ip_addresses,
            vec![constants::lan_pairing::TEST_LAN_IP.to_string()]
        );
        assert!(canonical.source_labels.contains(
            &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::LocalService
        ));
        assert!(canonical.source_labels.contains(
            &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::NetworkNeighbor
        ));
        assert!(canonical
            .network_identity
            .evidence_records
            .iter()
            .filter(|record| {
                record.source
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::WindowsNeighborTable
            })
            .any(|record| {
                record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::IpAddress
            }));
    }
}
#[test]
fn duplicate_local_service_evidence_state_stays_stable_across_input_orders() {
    let mut expected_signatures = None;

    for discovered_devices in [
        vec![
            local_service_child_discovery_device("trusted-child-local", "2026-06-27T11:00:00Z"),
            local_service_child_discovery_device("trusted-child-local", "2026-06-27T11:05:00Z"),
        ],
        vec![
            local_service_child_discovery_device("trusted-child-local", "2026-06-27T11:05:00Z"),
            local_service_child_discovery_device("trusted-child-local", "2026-06-27T11:00:00Z"),
        ],
    ] {
        let model = local_service_read_model(discovered_devices);

        assert_eq!(model.canonical_household_devices.len(), 1);
        let signatures = local_service_evidence_signatures(&model);
        assert_eq!(signatures.len(), 5);
        for signature in &signatures {
            let parts = signature.0.split('|').collect::<Vec<_>>();
            assert_eq!(parts.len(), 6);
            assert_eq!(parts[3], "2026-06-27T12:00:00Z");
            assert_eq!(parts[4], "2026-06-27T12:00:00Z");
        }

        let expected = expected_signatures.get_or_insert_with(|| signatures.clone());
        assert_eq!(&signatures, expected);
    }
}

#[test]
fn malformed_scan_sources_do_not_upgrade_evidence_or_scan_summary_across_input_orders() {
    for scan_sources in [
        vec![
            String::new(),
            "<script>alert(1)</script>".to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            "unknown-scan-source".to_string(),
        ],
        vec![
            "unknown-scan-source".to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            String::new(),
            "<script>alert(1)</script>".to_string(),
        ],
    ] {
        let model = lan_add_device_read_model_from_inventory(
            &[inventory_device(
                "malformed-source-neighbor",
                "malformed-source-neighbor",
                constants::lan_pairing::TEST_LAN_IP,
                constants::lan_pairing::TEST_LAN_MAC,
                LanPairingDeviceReachability::Online,
                scan_sources,
            )],
            "2026-06-27T12:00:00Z".to_string(),
        );

        assert_eq!(model.discovered_devices.len(), 1);
        assert_eq!(
            model.discovered_devices[0].evidence_sources,
            vec![
                ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::WindowsNeighborTable
            ]
        );
        assert_eq!(
            model.scan_summary.source_labels,
            vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
                constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            ]
        );
        assert!(
            !model.canonical_household_devices[0]
                .network_identity
                .evidence_records
                .iter()
                .filter(|record| {
                    record.source
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LocalService
                })
                .any(|record| record.value == "<script>alert(1)</script>")
        );
    }
}

fn inventory_device<L, H, I, M, S, T>(
    label: L,
    hostname: H,
    ip_address: I,
    mac_address: M,
    reachability: LanPairingDeviceReachability,
    scan_sources: S,
) -> LanNetworkInventoryDevice
where
    L: std::fmt::Display,
    H: std::fmt::Display,
    I: std::fmt::Display,
    M: std::fmt::Display,
    S: IntoIterator<Item = T>,
    T: std::fmt::Display,
{
    let label = label.to_string();
    let hostname = hostname.to_string();
    let ip_address = ip_address.to_string();
    let mac_address = mac_address.to_string();
    LanNetworkInventoryDevice {
        device_id: format!("network-neighbor-{label}"),
        label,
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address,
        mac_address,
        hostname: Some(hostname),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability,
        agent_status: None,
        scan_sources: scan_sources
            .into_iter()
            .map(|source| source.to_string())
            .collect(),
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }
}

fn filtered_sources<'a>(
    sources: impl Iterator<Item = &'a LanDiscoveryEvidenceSource>,
) -> BTreeSet<LanText> {
    sources
        .filter_map(|source| match source {
            LanDiscoveryEvidenceSource::WindowsNeighborTable => Some(lt!("WindowsNeighborTable")),
            LanDiscoveryEvidenceSource::LinuxProcNetArp => Some(lt!("LinuxProcNetArp")),
            LanDiscoveryEvidenceSource::LinuxIpNeigh => Some(lt!("LinuxIpNeigh")),
            _ => None,
        })
        .collect()
}

fn canonical_source_sets(
    model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
) -> BTreeSet<BTreeSet<LanText>> {
    model
        .canonical_household_devices
        .iter()
        .map(|device| {
            filtered_sources(
                device
                    .network_identity
                    .evidence_records
                    .iter()
                    .map(|record| &record.source),
            )
        })
        .collect()
}

fn event_count(
    model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
    event_kind: &LanDiscoveryEventKind,
) -> usize {
    model
        .discovery_event_history
        .rows
        .iter()
        .filter(|row| row.event_kind == *event_kind)
        .count()
}

fn local_service_evidence_signatures(
    model: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel,
) -> BTreeSet<LanText> {
    model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LocalService
        })
        .map(|record| {
            lt!(format!(
                "{:?}|{}|{:?}|{}|{}|{}",
                record.evidence_kind,
                record.normalized_value,
                record.confidence,
                record.first_seen_at,
                record.last_seen_at,
                record.note.clone().unwrap_or_default()
            ))
        })
        .collect()
}

fn local_service_read_model(
    discovered_devices: Vec<
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice,
    >,
) -> ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel
{
    build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-27T12:00:00Z".to_string(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices,
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: Vec::new(),
        trusted_device_ids: vec!["trusted-child-local".to_string()],
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

fn local_service_child_discovery_device(
    device_id: impl std::fmt::Display,
    discovered_at: impl std::fmt::Display,
) -> ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice
{
    let device_id = device_id.to_string();
    let discovered_at = discovered_at.to_string();
    let mut child_device = LanPairingDeviceRef::new(
        device_id.clone(),
        Some("child-profile-local".to_string()),
        "Study Laptop".to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    child_device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    child_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
    child_device.hostname = Some("study-laptop.local".to_string());
    child_device.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at,
        child_device,
        agent_peer_id: device_id,
        pairing_id: None,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: ocentra_parent_agent_protocol::lan_pairing::LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
        discovery_status:
            ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        evidence_sources: vec![
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LocalService,
        ],
        hint_sources: Vec::new(),
        service_identity_probe_evidence: Vec::new(),
    }
}

fn ip_only_neighbor_discovery_device(
    device_id: impl std::fmt::Display,
    discovered_at: impl std::fmt::Display,
) -> ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice
{
    let device_id = device_id.to_string();
    let discovered_at = discovered_at.to_string();
    let mut child_device = LanPairingDeviceRef::new(
        device_id.clone(),
        None,
        "Study Laptop".to_string(),
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );
    child_device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    child_device.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at,
        child_device,
        agent_peer_id: device_id,
        pairing_id: None,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: ocentra_parent_agent_protocol::lan_pairing::LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
        discovery_status:
            ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        evidence_sources: vec![
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::WindowsNeighborTable,
        ],
        hint_sources: Vec::new(),
        service_identity_probe_evidence: Vec::new(),
    }
}
