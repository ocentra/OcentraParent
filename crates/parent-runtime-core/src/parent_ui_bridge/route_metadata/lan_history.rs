use super::common::{parse_identifier, parse_optional_identifier};
use super::*;

pub(super) fn parent_lan_discovery_event_history_snapshot(
    history: &LanDiscoveryEventHistory,
    read_model: &LanBrowserAddDeviceReadModel,
) -> ParentLanDiscoveryEventHistorySnapshot {
    let latest_event_row = history.rows.last();
    ParentLanDiscoveryEventHistorySnapshot {
        schema_version: history.schema_version,
        generated_at: history.generated_at.clone(),
        state: parent_lan_discovery_event_history_state_label(history, read_model),
        latest_event_id: parse_optional_identifier(
            history
                .latest_event_id
                .clone()
                .or_else(|| latest_event_row.map(|row| row.event_id.clone())),
            ParentLanDiscoveryEventId::parse,
        ),
        latest_observed_at: history
            .latest_observed_at
            .clone()
            .or_else(|| latest_event_row.map(|row| row.occurred_at.clone())),
        rows: history
            .rows
            .iter()
            .map(parent_lan_discovery_event_row_snapshot)
            .collect(),
    }
}

pub(super) fn parent_lan_discovery_event_history_state_label(
    history: &LanDiscoveryEventHistory,
    read_model: &LanBrowserAddDeviceReadModel,
) -> String {
    if matches!(
        history.state,
        LanDiscoveryEventHistoryState::Ready | LanDiscoveryEventHistoryState::Empty
    ) && parent_lan_read_model_has_stale_history_metadata(read_model)
    {
        return "stale".to_string();
    }
    serialized_enum_label(&history.state)
}

pub(super) fn parent_lan_read_model_has_stale_history_metadata(
    read_model: &LanBrowserAddDeviceReadModel,
) -> bool {
    read_model.selected_device_readiness.reachability == LanPairingDeviceReachability::Stale
        || read_model.selected_device_readiness.stale_at.is_some()
        || read_model
            .canonical_household_devices
            .iter()
            .any(parent_lan_canonical_device_has_stale_history_metadata)
}

pub(super) fn parent_lan_canonical_device_has_stale_history_metadata(
    device: &LanCanonicalHouseholdDevice,
) -> bool {
    device.network_identity.reachability == LanPairingDeviceReachability::Stale
        || device.network_identity.stale_at.is_some()
}

pub(super) fn parent_lan_discovery_event_row_snapshot(
    row: &LanDiscoveryEventRow,
) -> ParentLanDiscoveryEventRowSnapshot {
    ParentLanDiscoveryEventRowSnapshot {
        schema_version: row.schema_version,
        event_id: parse_identifier(row.event_id.clone(), "lan discovery event_id", |value| {
            ParentLanDiscoveryEventId::parse(value)
        }),
        event_kind: serialized_enum_label(&row.event_kind),
        occurred_at: row.occurred_at.clone(),
        previous_event_id: parse_optional_identifier(row.previous_event_id.clone(), |value| {
            ParentLanDiscoveryEventId::parse(value)
        }),
        scan_session_id: parse_optional_identifier(row.scan_session_id.clone(), |value| {
            ParentLanScanSessionId::parse(value)
        }),
        affected_device_id: parse_optional_identifier(row.affected_device_id.clone(), |value| {
            ParentLanDeviceId::parse(value)
        }),
        evidence_id: parse_optional_identifier(row.evidence_id.clone(), |value| {
            ParentEvidenceId::parse(value)
        }),
        summary: row.summary.clone(),
    }
}

pub(super) fn parent_lan_pairing_device_ref_snapshot(
    device: &LanPairingDeviceRef,
) -> ParentLanPairingDeviceRefSnapshot {
    ParentLanPairingDeviceRefSnapshot {
        device_id: parse_identifier(device.device_id.clone(), "lan device_id", |value| {
            ParentLanDeviceId::parse(value)
        }),
        child_profile_id: parse_optional_identifier(device.child_profile_id.clone(), |value| {
            ParentContractReferenceId::parse(value)
        }),
        label: device.label.clone(),
        platform: device.platform.clone(),
        ip_address: device.ip_address.clone(),
        mac_address: device.mac_address.clone(),
        hostname: device.hostname.clone(),
        network_interface: device.network_interface.clone(),
        agent_status: device.agent_status.clone(),
    }
}

pub(super) fn parent_lan_browser_add_device_discovery_device_snapshot(
    device: &LanBrowserAddDeviceDiscoveryDevice,
) -> ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot {
    ParentLanBrowserAddDeviceDiscoveryDeviceSnapshot {
        schema_version: device.schema_version,
        discovered_at: device.discovered_at.clone(),
        child_device: parent_lan_pairing_device_ref_snapshot(&device.child_device),
        agent_peer_id: parse_identifier(
            device.agent_peer_id.clone(),
            "lan agent_peer_id",
            ParentRoutePeerId::parse,
        ),
        route_id: parse_identifier(device.route_id.clone(), "lan route_id", |value| {
            ParentLanRouteId::parse(value)
        }),
        network_mode: serialized_enum_label(&device.network_mode),
        reachability: serialized_enum_label(&device.reachability),
        address_ref: parse_identifier(device.address_ref.clone(), "lan address_ref", |value| {
            ParentLanAddressRef::parse(value)
        }),
        discovery_status: serialized_enum_label(&device.discovery_status),
        discovery_state: serialized_enum_label(&device.discovery_state),
        evidence_sources: device
            .evidence_sources
            .iter()
            .map(serialized_enum_label)
            .collect(),
        service_identity_probe_evidence: device
            .service_identity_probe_evidence
            .iter()
            .map(|evidence| ParentLanServiceIdentityProbeEvidenceSnapshot {
                evidence_kind: serialized_enum_label(&evidence.evidence_kind),
                value: evidence.value.clone(),
            })
            .collect(),
        hint_sources: device
            .hint_sources
            .iter()
            .map(serialized_enum_label)
            .collect(),
    }
}

pub(super) fn parent_lan_browser_add_device_pairing_request_snapshot(
    request: &LanBrowserAddDevicePairingRequest,
) -> ParentLanBrowserAddDevicePairingRequestSnapshot {
    ParentLanBrowserAddDevicePairingRequestSnapshot {
        schema_version: request.schema_version,
        challenge_id: parse_identifier(request.challenge_id.clone(), "lan challenge_id", |value| {
            ParentLanChallengeId::parse(value)
        }),
        child_device_id: parse_identifier(
            request.child_device_id.clone(),
            "lan child_device_id",
            ParentLanDeviceId::parse,
        ),
        parent_device_id: parse_identifier(
            request.parent_device_id.clone(),
            "lan parent_device_id",
            ParentLanDeviceId::parse,
        ),
        route_id: parse_identifier(request.route_id.clone(), "lan route_id", |value| {
            ParentLanRouteId::parse(value)
        }),
        origin: request.origin.clone(),
        pairing_state: serialized_enum_label(&request.pairing_state),
        rejection_reason: request.rejection_reason.as_ref().map(serialized_enum_label),
        issued_at: request.issued_at.clone(),
        expires_at: request.expires_at.clone(),
    }
}

pub(super) fn parent_lan_discovery_evidence_record_snapshot(
    record: &LanDiscoveryEvidenceRecord,
) -> ParentLanDiscoveryEvidenceRecordSnapshot {
    ParentLanDiscoveryEvidenceRecordSnapshot {
        schema_version: record.schema_version,
        evidence_id: parse_identifier(record.evidence_id.clone(), "lan evidence_id", |value| {
            ParentEvidenceId::parse(value)
        }),
        source: serialized_enum_label(&record.source),
        evidence_kind: serialized_enum_label(&record.evidence_kind),
        device_id: parse_identifier(record.device_id.clone(), "lan device_id", |value| {
            ParentLanDeviceId::parse(value)
        }),
        value: record.value.clone(),
        normalized_value: record.normalized_value.clone(),
        first_seen_at: record.first_seen_at.clone(),
        last_seen_at: record.last_seen_at.clone(),
        expires_at: record.expires_at.clone(),
        confidence: serialized_enum_label(&record.confidence),
        merge_key: record.merge_key.clone(),
        note: record.note.clone(),
    }
}

pub(super) fn parent_lan_canonical_household_network_identity_snapshot(
    identity: &LanCanonicalHouseholdNetworkIdentity,
) -> ParentLanCanonicalHouseholdNetworkIdentitySnapshot {
    ParentLanCanonicalHouseholdNetworkIdentitySnapshot {
        hostname: identity.hostname.clone(),
        ip_addresses: identity.ip_addresses.clone(),
        mac_address: identity.mac_address.clone(),
        mac_vendor: identity.mac_vendor.clone(),
        network_interfaces: identity.network_interfaces.clone(),
        reachability: serialized_enum_label(&identity.reachability),
        confidence: serialized_enum_label(&identity.confidence),
        stale_at: identity.stale_at.clone(),
        offline_at: identity.offline_at.clone(),
        evidence_records: identity
            .evidence_records
            .iter()
            .map(parent_lan_discovery_evidence_record_snapshot)
            .collect(),
    }
}

pub(super) fn parent_lan_child_agent_inventory_packet_snapshot(
    inventory: &LanChildAgentInventoryPacket,
) -> ParentLanChildAgentInventoryPacketSnapshot {
    ParentLanChildAgentInventoryPacketSnapshot {
        device_name: inventory.device_name.clone(),
        platform: inventory.platform.clone(),
        os: inventory.os.clone(),
        cpu_model: inventory.cpu_model.clone(),
        cpu_cores: inventory.cpu_cores.clone(),
        memory_total: inventory.memory_total.clone(),
        gpu_model: inventory.gpu_model.clone(),
        gpu_driver: inventory.gpu_driver.clone(),
        gpu_memory: inventory.gpu_memory.clone(),
        nvidia_smi: inventory.nvidia_smi.clone(),
        network_interfaces: inventory.network_interfaces.clone(),
        capabilities: inventory.capabilities.clone(),
        role_state: serialized_enum_label(&inventory.role_state),
        route_state: serialized_enum_label(&inventory.route_state),
        pairing_trust_state: serialized_enum_label(&inventory.pairing_trust_state),
    }
}
