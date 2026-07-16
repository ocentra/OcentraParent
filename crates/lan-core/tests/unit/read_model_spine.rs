use super::*;

#[macro_use]
#[path = "read_model_spine_platform_support.rs"]
mod platform_support;

#[test]
fn service_probe_snmp_evidence_adds_allowed_snmp_response_scan_label() {
    let mut discovered = neighbor(
        constants::lan_pairing::TEST_HOSTNAME,
        Some(constants::lan_pairing::TEST_HOSTNAME),
        LanPairingDeviceReachability::Online,
    );
    discovered.agent_status =
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS.to_string());
    discovered.service_identity_probe_evidence = vec![
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidence {
            evidence_kind:
                ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::SnmpSysDescr,
            value: "Printer".to_string(),
            selected_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        },
    ];

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE.to_string(),
        ]
    );
}

#[test]
fn previous_scan_hint_stays_weak_but_visible_in_scan_summary_and_evidence() {
    let mut discovered = neighbor(
        constants::lan_pairing::TEST_HOSTNAME,
        Some(constants::lan_pairing::TEST_HOSTNAME),
        LanPairingDeviceReachability::Online,
    );
    discovered.used_previous_scan_hint = true;

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_PREVIOUS_SCAN_SNAPSHOT.to_string(),
        ]
    );
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::PreviousScanSnapshot
                && record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::HistoricalIdentityHint
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Weak
                && record.note.as_deref()
                    == Some(constants::lan_pairing::LAN_PREVIOUS_SCAN_CONTINUITY_NOTE)
        }));
    assert!(model
        .lan_discovery_source_matrix
        .as_ref()
        .is_some_and(|matrix| matrix.source_rows.iter().any(|row| {
            row.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PreviousScanSnapshot
                && row.workpack_id
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W15
                && row.status
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Implemented
                && row.persists_across_restart
        })));
}

#[test]
fn inventory_backed_read_model_keeps_linux_neighbor_sources_truthful() {
    let mut discovered = neighbor(
        constants::lan_pairing::TEST_HOSTNAME,
        Some(constants::lan_pairing::TEST_HOSTNAME),
        LanPairingDeviceReachability::Online,
    );
    discovered.scan_sources = vec![
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
    ];

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
        ]
    );
    assert!(model.canonical_household_devices[0]
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::LinuxIpNeigh
                && record.evidence_kind
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::IpAddress
        }));
}

#[test]
fn inventory_backed_read_model_preserves_neighbor_observed_at_timestamp() {
    let model = lan_add_device_read_model_from_inventory(
        &[LanNetworkInventoryDevice {
            device_id: "network-neighbor-study-laptop".to_string(),
            label: "study-laptop.local".to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: "192.168.2.90".to_string(),
            mac_address: "00-11-22-33-44-90".to_string(),
            hostname: Some("study-laptop.local".to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            observed_at: "2026-06-28T10:15:30Z".to_string(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        }],
        "2026-06-28T12:00:00Z".to_string(),
    );

    assert_eq!(model.discovered_devices.len(), 1);
    assert_eq!(
        model.discovered_devices[0].discovered_at,
        "2026-06-28T10:15:30Z"
    );
}

#[test]
fn mdns_only_inventory_rows_remain_visible_as_weak_agentless_devices() {
    let discovered = LanNetworkInventoryDevice {
        device_id: "network-neighbor-mdns-officeprinter".to_string(),
        label: "Office Printer".to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "192.168.2.88".to_string(),
        mac_address: String::new(),
        hostname: Some("office-printer.local".to_string()),
        network_interface: None,
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(model.scan_summary.scanned_device_count, 1);
    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD.to_string(),
        ]
    );
    assert_eq!(model.scan_summary.agent_device_count, 0);
    let canonical = &model.canonical_household_devices[0];
    assert_eq!(canonical.display_name, "office-printer.local");
    assert_eq!(
        canonical.classification,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Printer
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert!(canonical.network_identity.mac_address.is_none());
    assert_eq!(
        canonical.network_identity.ip_addresses,
        vec!["192.168.2.88".to_string()]
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::MdnsDnsSdQuery
                && record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::IpAddress
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Strong
        }));
}

#[test]
fn ssdp_only_inventory_rows_remain_visible_as_agentless_protocol_hints() {
    let discovered = LanNetworkInventoryDevice {
        device_id: "tv-1".to_string(),
        label: "Living Room TV".to_string(),
        platform: "MediaRenderer".to_string(),
        ip_address: "192.168.2.89".to_string(),
        mac_address: String::new(),
        hostname: None,
        network_interface: None,
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    let model =
        lan_add_device_read_model_from_inventory(&[discovered], "2026-06-23T00:00:00Z".to_string());

    assert_eq!(model.scan_summary.scanned_device_count, 1);
    assert_eq!(
        model.scan_summary.source_labels,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP.to_string(),
        ]
    );
    assert_eq!(model.scan_summary.agent_device_count, 0);
    let canonical = &model.canonical_household_devices[0];
    assert_eq!(canonical.display_name, "Living Room TV");
    assert_eq!(
        canonical.classification,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Television
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert_eq!(
        canonical.source_labels,
        vec![
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource::NetworkNeighbor
        ]
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource::SsdpUpnpQuery
                && record.evidence_kind
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::IpAddress
        }));
}

#[test]
fn passive_local_neighbor_collection_summaries_are_persisted_in_scan_summary() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let summaries = &model
        .scan_summary
        .passive_local_neighbor_collection_summaries;

    let expected_sources = expected_passive_summary_sources!();
    assert_eq!(summaries.len(), expected_sources.len());
    for summary in summaries {
        assert_eq!(
            summary.schema_version,
            constants::lan_pairing::SCHEMA_VERSION
        );
        assert!(!summary.source_label.trim().is_empty());
        assert!(summary.observed_count >= summary.recorded_count);
    }

    for expected_source in expected_sources {
        assert!(summaries
            .iter()
            .any(|summary| summary.source_label == *expected_source));
    }
}

#[test]
fn oui_vendor_lookup_is_visible_in_read_model_truth() {
    let model = lan_add_device_read_model_from_inventory(
        &[neighbor(
            constants::lan_pairing::TEST_HOSTNAME,
            Some(constants::lan_pairing::TEST_HOSTNAME),
            LanPairingDeviceReachability::Online,
        )],
        "2026-06-23T00:00:00Z".to_string(),
    );

    let canonical = &model.canonical_household_devices[0];
    assert_eq!(
        canonical.network_identity.mac_vendor.as_deref(),
        Some("AzureWave Technology Inc.")
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.evidence_kind
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::Vendor
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::Strong
                && record.value == "AzureWave Technology Inc."
        }));
    assert!(model
        .lan_discovery_source_matrix
        .as_ref()
        .is_some_and(|matrix| matrix.source_rows.iter().any(|row| {
            row.source == LanDiscoverySourceKind::OuiVendorLookup
                && row.status == LanDiscoverySourceStatus::Implemented
                && row.authority == LanDiscoverySourceAuthority::ClassificationOnly
                && row.runtime_path == LanDiscoverySourceRuntimePath::RustServiceReadModel
        })));
}

#[test]
fn source_matrix_has_runtime_rows_for_all_lan_workpacks() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    let workpack_ids = matrix
        .workpack_rows
        .iter()
        .map(|row| &row.workpack_id)
        .collect::<Vec<_>>();
    assert_eq!(workpack_ids.len(), 25);
    for workpack_id in [
        LanPlanWorkpackId::W01,
        LanPlanWorkpackId::W02,
        LanPlanWorkpackId::W03,
        LanPlanWorkpackId::W04,
        LanPlanWorkpackId::W05,
        LanPlanWorkpackId::W06,
        LanPlanWorkpackId::W07,
        LanPlanWorkpackId::W08,
        LanPlanWorkpackId::W09,
        LanPlanWorkpackId::W10,
        LanPlanWorkpackId::W11,
        LanPlanWorkpackId::W12,
        LanPlanWorkpackId::W13,
        LanPlanWorkpackId::W14,
        LanPlanWorkpackId::W15,
        LanPlanWorkpackId::W16,
        LanPlanWorkpackId::W17,
        LanPlanWorkpackId::W18,
        LanPlanWorkpackId::W19,
        LanPlanWorkpackId::W20,
        LanPlanWorkpackId::W21,
        LanPlanWorkpackId::W22,
        LanPlanWorkpackId::W23,
        LanPlanWorkpackId::W24,
        LanPlanWorkpackId::W25,
    ] {
        assert!(
            workpack_ids.contains(&&workpack_id),
            "missing workpack row for {workpack_id:?}"
        );
    }

    for source in [
        LanDiscoverySourceKind::EvidenceModel,
        LanDiscoverySourceKind::InterfaceSelection,
        LanDiscoverySourceKind::MacosArp,
        LanDiscoverySourceKind::MergeDeduplication,
        LanDiscoverySourceKind::ExplainableClassification,
        LanDiscoverySourceKind::HouseholdDeviceStore,
        LanDiscoverySourceKind::ReadModelEventStream,
        LanDiscoverySourceKind::AssignmentRevocationAudit,
        LanDiscoverySourceKind::ProofGateRollout,
    ] {
        assert!(
            matrix.source_rows.iter().any(|row| row.source == source),
            "missing source row for {source:?}"
        );
    }
}

#[test]
fn source_matrix_reports_cross_platform_neighbor_sources_without_platform_overclaiming() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    for source in [
        LanDiscoverySourceKind::WindowsNeighborTable,
        LanDiscoverySourceKind::LinuxProcNetArp,
        LanDiscoverySourceKind::LinuxIpNeigh,
    ] {
        let row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == LanPlanWorkpackId::W04)
            .value_or_unreachable();
        assert_eq!(row.status, LanDiscoverySourceStatus::Implemented);
        assert_eq!(row.authority, LanDiscoverySourceAuthority::WeakIdentity);
        assert_eq!(
            row.runtime_path,
            LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert!(row.requires_selected_interface);
        assert!(!row.can_confirm_child_agent);
        assert!(!row.can_control_route);
        assert!(row.required_artifact_summary.is_none());
    }

    let macos = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::MacosArp
                && row.workpack_id == LanPlanWorkpackId::W04
        })
        .value_or_unreachable();
    assert_eq!(macos.status, LanDiscoverySourceStatus::Partial);
    assert_eq!(macos.authority, LanDiscoverySourceAuthority::WeakIdentity);
    assert_eq!(
        macos.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert!(macos.requires_selected_interface);
    assert!(!macos.can_confirm_child_agent);
    assert!(!macos.can_control_route);
    assert_eq!(
        macos.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PHYSICAL)
    );
}

#[test]
fn source_matrix_marks_core_read_model_spine_without_manual_proof_churn() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    for (source, workpack_id, authority) in [
        (
            LanDiscoverySourceKind::ContractBoundary,
            LanPlanWorkpackId::W01,
            LanDiscoverySourceAuthority::ProofGate,
        ),
        (
            LanDiscoverySourceKind::EvidenceModel,
            LanPlanWorkpackId::W02,
            LanDiscoverySourceAuthority::ProofGate,
        ),
        (
            LanDiscoverySourceKind::InterfaceSelection,
            LanPlanWorkpackId::W03,
            LanDiscoverySourceAuthority::WeakIdentity,
        ),
        (
            LanDiscoverySourceKind::TargetedArpRefresh,
            LanPlanWorkpackId::W05,
            LanDiscoverySourceAuthority::PresenceOnly,
        ),
        (
            LanDiscoverySourceKind::OuiVendorLookup,
            LanPlanWorkpackId::W12,
            LanDiscoverySourceAuthority::ClassificationOnly,
        ),
        (
            LanDiscoverySourceKind::MergeDeduplication,
            LanPlanWorkpackId::W13,
            LanDiscoverySourceAuthority::ProofGate,
        ),
        (
            LanDiscoverySourceKind::ExplainableClassification,
            LanPlanWorkpackId::W14,
            LanDiscoverySourceAuthority::ClassificationOnly,
        ),
        (
            LanDiscoverySourceKind::HouseholdDeviceStore,
            LanPlanWorkpackId::W15,
            LanDiscoverySourceAuthority::ManualParentDecision,
        ),
        (
            LanDiscoverySourceKind::ReadModelEventStream,
            LanPlanWorkpackId::W16,
            LanDiscoverySourceAuthority::ProofGate,
        ),
        (
            LanDiscoverySourceKind::AssignmentRevocationAudit,
            LanPlanWorkpackId::W19,
            LanDiscoverySourceAuthority::ManualParentDecision,
        ),
    ] {
        let row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == workpack_id)
            .value_or_unreachable();
        assert_eq!(row.status, LanDiscoverySourceStatus::Implemented);
        assert_eq!(row.authority, authority);
        if source == LanDiscoverySourceKind::ContractBoundary {
            assert_eq!(
                row.runtime_path,
                LanDiscoverySourceRuntimePath::AgentProtocol
            );
        } else {
            assert_eq!(
                row.runtime_path,
                LanDiscoverySourceRuntimePath::RustServiceReadModel
            );
        }
        assert!(row.required_artifact_summary.is_none());
    }
}

#[test]
fn source_matrix_spine_rows_cover_merge_classification_and_durable_store_details() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    let merge = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::MergeDeduplication
                && row.workpack_id == LanPlanWorkpackId::W13
        })
        .value_or_unreachable();
    assert_eq!(merge.status, LanDiscoverySourceStatus::Implemented);
    assert_eq!(merge.authority, LanDiscoverySourceAuthority::ProofGate);
    assert_eq!(
        merge.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert!(!merge.persists_across_restart);
    assert!(merge.required_artifact_summary.is_none());

    let classification = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::ExplainableClassification
                && row.workpack_id == LanPlanWorkpackId::W14
        })
        .value_or_unreachable();
    assert_eq!(classification.status, LanDiscoverySourceStatus::Implemented);
    assert_eq!(
        classification.authority,
        LanDiscoverySourceAuthority::ClassificationOnly
    );
    assert_eq!(
        classification.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert!(!classification.can_confirm_child_agent);
    assert!(!classification.can_assign_child_profile);
    assert!(!classification.can_control_route);
    assert!(!classification.requires_selected_interface);
    assert!(!classification.persists_across_restart);
    assert!(classification.required_artifact_summary.is_none());

    let durable_store = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::HouseholdDeviceStore
                && row.workpack_id == LanPlanWorkpackId::W15
        })
        .value_or_unreachable();
    assert_eq!(durable_store.status, LanDiscoverySourceStatus::Implemented);
    assert_eq!(
        durable_store.authority,
        LanDiscoverySourceAuthority::ManualParentDecision
    );
    assert_eq!(
        durable_store.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert!(durable_store.persists_across_restart);
    assert!(durable_store.required_artifact_summary.is_none());
}
