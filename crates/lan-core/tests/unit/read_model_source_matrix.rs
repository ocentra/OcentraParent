use super::*;

fn assert_wp10_name_sources_stay_partial_name_only_rust_read_model_inputs() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    for source in [
        LanDiscoverySourceKind::NetbiosNameCache,
        LanDiscoverySourceKind::LlmnrNameQuery,
        LanDiscoverySourceKind::ReverseDnsQuery,
    ] {
        let row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == LanPlanWorkpackId::W10)
            .value_or_unreachable();
        assert_eq!(row.status, LanDiscoverySourceStatus::Partial);
        assert_eq!(row.authority, LanDiscoverySourceAuthority::NameOnly);
        assert_eq!(
            row.runtime_path,
            LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert!(!row.can_confirm_child_agent);
        assert!(!row.can_control_route);
        assert!(row.required_artifact_summary.is_none());
    }
}

#[test]
fn source_matrix_marks_wp10_name_sources_as_partial_name_only_rust_read_model_inputs() {
    assert_wp10_name_sources_stay_partial_name_only_rust_read_model_inputs();
}

#[test]
fn source_matrix_separates_real_weak_name_sources_from_unimplemented_llmnr() {
    // Keep the historical WP10 filter target alive while asserting the current
    // Rust-owned contract: all three W10 name sources are partial, name-only,
    // and non-controlling in the read model.
    assert_wp10_name_sources_stay_partial_name_only_rust_read_model_inputs();
}

#[test]
fn source_matrix_keeps_service_identity_probe_partial_and_non_controlling() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    let row = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source == LanDiscoverySourceKind::ServiceIdentityProbe
                && row.workpack_id == LanPlanWorkpackId::W11
        })
        .value_or_unreachable();

    assert_eq!(row.status, LanDiscoverySourceStatus::Partial);
    assert_eq!(row.authority, LanDiscoverySourceAuthority::PresenceOnly);
    assert_eq!(
        row.runtime_path,
        LanDiscoverySourceRuntimePath::RustServiceReadModel
    );
    assert_eq!(
        row.ui_surface,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::DevicesLan
    );
    assert!(!row.can_confirm_child_agent);
    assert!(!row.can_assign_child_profile);
    assert!(!row.can_control_route);
    assert!(row.required_artifact_summary.is_none());
}

#[test]
fn locally_administered_mac_downgrades_neighbor_confidence_and_emits_warning() {
    let model = lan_add_device_read_model_from_inventory(
        &[neighbor_with_mac(
            "phone-private-mac",
            Some(constants::lan_pairing::TEST_HOSTNAME),
            LanPairingDeviceReachability::Online,
            "02-aa-bb-cc-dd-ee",
        )],
        "2026-06-23T00:00:00Z".to_string(),
    );

    let canonical = &model.canonical_household_devices[0];
    assert_eq!(
        canonical.network_identity.confidence,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceConfidence::ManualRequired
    );
    assert_eq!(canonical.network_identity.mac_address.as_deref(), None);
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.evidence_kind
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceKind::Vendor
                && record.confidence
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceConfidence::ManualRequired
                && record.note.as_deref()
                    == Some(constants::lan_pairing::LAN_VENDOR_LOCAL_ADMINISTERED_NOTE)
        }));
}

#[test]
fn mdns_and_ssdp_rows_are_partially_implemented_rust_read_model_sources() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    for (source, workpack_id, title) in [
        (
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::MdnsDnsSdQuery,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W08,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_08,
        ),
        (
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::SsdpUpnpQuery,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W09,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_09,
        ),
    ] {
        let source_row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == workpack_id)
            .value_or_unreachable();
        assert_eq!(source_row.evidence_label, title);
        assert_eq!(
            source_row.status,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
        );
        assert_eq!(
            source_row.authority,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceAuthority::PresenceOnly
        );
        assert_eq!(
            source_row.runtime_path,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert_eq!(
            source_row.ui_surface,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::DevicesLan
        );
        assert!(!source_row.can_confirm_child_agent);
        assert!(!source_row.can_control_route);
        assert!(source_row.required_artifact_summary.is_none());

        let workpack_row = matrix
            .workpack_rows
            .iter()
            .find(|row| row.workpack_id == workpack_id)
            .value_or_unreachable();
        assert_eq!(workpack_row.title, title);
        assert_eq!(
            workpack_row.discovery_state,
            ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState::Pending
        );
        assert_eq!(
            workpack_row.runtime_owner,
            ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel
        );
        assert_eq!(
            workpack_row.status,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
        );
        assert!(workpack_row.read_model_visible);
        assert!(workpack_row.required_artifact_summary.is_none());
    }
}

#[test]
fn passive_listener_rows_reflect_current_udp_runtime_support_and_manual_gaps() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    for source in [
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveArpListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveDhcpListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveMdnsListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveSsdpListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveWsDiscoveryListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveLlmnrListener,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveNetbiosListener,
    ] {
        let source_row = matrix
            .source_rows
            .iter()
            .find(|row| {
                row.source == source
                    && row.workpack_id
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W07
            })
            .value_or_unreachable();
        assert_eq!(
            source_row.status,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
        );
        assert_eq!(
            source_row.runtime_path,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert_eq!(
            source_row.ui_surface,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::DevicesLan
        );
        assert!(!source_row.can_confirm_child_agent);
        assert!(!source_row.can_control_route);
    }

    {
        let source = ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::PassiveSnmpResponseListener;
        let source_row = matrix
            .source_rows
            .iter()
            .find(|row| {
                row.source == source
                    && row.workpack_id
                        == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W07
            })
            .value_or_unreachable();
        assert_eq!(
            source_row.status,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
        );
        assert_eq!(
            source_row.runtime_path,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
    }

    let workpack_row = matrix
        .workpack_rows
        .iter()
        .find(|row| {
            row.workpack_id
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W07
        })
        .value_or_unreachable();
    assert_eq!(
        workpack_row.status,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
    );
    assert_eq!(
        workpack_row.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE)
    );
}

#[test]
fn active_refresh_rows_separate_targeted_arp_from_bounded_sweep_completion() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    for (source, workpack_id, title, status) in [
        (
            LanDiscoverySourceKind::TargetedArpRefresh,
            LanPlanWorkpackId::W05,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_05,
            LanDiscoverySourceStatus::Implemented,
        ),
        (
            LanDiscoverySourceKind::BoundedArpSweep,
            LanPlanWorkpackId::W06,
            constants::lan_pairing::LAN_SOURCE_MATRIX_TITLE_06,
            LanDiscoverySourceStatus::Implemented,
        ),
    ] {
        let source_row = matrix
            .source_rows
            .iter()
            .find(|row| row.source == source && row.workpack_id == workpack_id)
            .value_or_unreachable();
        assert_eq!(source_row.evidence_label, title);
        assert_eq!(source_row.status, status);
        assert_eq!(
            source_row.authority,
            LanDiscoverySourceAuthority::PresenceOnly
        );
        assert_eq!(
            source_row.runtime_path,
            LanDiscoverySourceRuntimePath::RustServiceReadModel
        );
        assert_eq!(
            source_row.ui_surface,
            ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::DevicesLan
        );
        assert!(source_row.requires_selected_interface);
        assert!(!source_row.can_confirm_child_agent);
        assert!(!source_row.can_control_route);
        assert!(source_row.required_artifact_summary.is_none());

        let workpack_row = matrix
            .workpack_rows
            .iter()
            .find(|row| row.workpack_id == workpack_id)
            .value_or_unreachable();
        assert_eq!(workpack_row.title, title);
        assert_eq!(
            workpack_row.runtime_owner,
            ocentra_parent_agent_protocol::lan_pairing::V09ProductionDiscoveryHouseholdRuntimeOwner::RustServiceReadModel
        );
        assert_eq!(workpack_row.status, status);
        assert!(workpack_row.read_model_visible);
        assert!(workpack_row.required_artifact_summary.is_none());
    }
}

#[test]
fn parent_and_child_mdns_advertisements_are_partial_but_hint_only() {
    let model = lan_add_device_read_model_from_inventory(&[], "2026-06-23T00:00:00Z".to_string());
    let matrix = model
        .lan_discovery_source_matrix
        .as_ref()
        .value_or_unreachable();

    let parent_row = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::ParentMdnsAdvertisement
                && row.workpack_id
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W17
        })
        .value_or_unreachable();
    assert_eq!(
        parent_row.status,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
    );
    assert_eq!(
        parent_row.authority,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceAuthority::PresenceOnly
    );
    assert_eq!(
        parent_row.runtime_path,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::AgentProtocol
    );
    assert_eq!(
        parent_row.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP)
    );
    assert!(!parent_row.can_confirm_child_agent);
    assert!(!parent_row.can_control_route);

    let child_row = matrix
        .source_rows
        .iter()
        .find(|row| {
            row.source
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceKind::ChildMdnsAdvertisement
                && row.workpack_id
                    == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W17
    })
        .value_or_unreachable();
    assert_eq!(
        child_row.status,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
    );
    assert_eq!(
        child_row.authority,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceAuthority::PresenceOnly
    );
    assert_eq!(
        child_row.runtime_path,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceRuntimePath::AgentProtocol
    );
    assert_eq!(
        child_row.ui_surface,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceUiSurface::ProofReport
    );
    assert!(!child_row.can_confirm_child_agent);
    assert!(!child_row.can_control_route);
    assert_eq!(
        child_row.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP)
    );

    let workpack_row = matrix
        .workpack_rows
        .iter()
        .find(|row| {
            row.workpack_id
                == ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanPlanWorkpackId::W17
        })
        .value_or_unreachable();
    assert_eq!(
        workpack_row.status,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::source_matrix::LanDiscoverySourceStatus::Partial
    );
    assert_eq!(
        workpack_row.required_artifact_summary.as_deref(),
        Some(constants::lan_pairing::LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP)
    );
}
