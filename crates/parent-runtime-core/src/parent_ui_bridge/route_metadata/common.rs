use super::*;

const DATA_SOURCE_LABELS: &[(&str, &str)] = &[
    ("rust-read-model", "available"),
    ("host-bridge", "available"),
    ("dev-diagnostics", "diagnostic-only"),
    ("unavailable", "unavailable"),
];
const DATA_SOURCE_TONES: &[(&str, ParentPortalTone)] = &[
    ("rust-read-model", ParentPortalTone::Cyan),
    ("host-bridge", ParentPortalTone::Gold),
    ("dev-diagnostics", ParentPortalTone::Muted),
    ("unavailable", ParentPortalTone::Muted),
];
const CONNECTION_SEASON_LABELS: &[(&str, &str)] = &[
    ("connected", "live"),
    ("connecting", "checking"),
    ("disconnected", "offline"),
    ("error", "offline"),
];
const CONNECTION_GLOBAL_LABELS: &[(&str, &str)] = &[
    ("connected", "connected"),
    ("connecting", "connecting"),
    ("disconnected", "offline"),
    ("error", "offline"),
];
const CONNECTION_TONES: &[(&str, ParentPortalTone)] = &[
    ("connected", ParentPortalTone::Cyan),
    ("connecting", ParentPortalTone::Gold),
    ("disconnected", ParentPortalTone::Red),
    ("error", ParentPortalTone::Red),
];

pub(super) fn portal_row_snapshot(
    label: &str,
    order: u16,
    primary_area: &str,
    trend: String,
    tone: ParentPortalTone,
) -> ParentPortalRowSnapshot {
    let available = !matches!(
        trend.as_str(),
        "manual-required" | "unavailable" | "offline" | "proof-missing" | "unauthenticated"
    );
    ParentPortalRowSnapshot {
        label: label.to_string(),
        order,
        signal_score: if available { 100 } else { 0 },
        ready_count: if available { 1 } else { 0 },
        gap_count: if available { 0 } else { 1 },
        primary_area: primary_area.to_string(),
        trend,
        tone,
    }
}

pub(super) fn serialized_enum_label<T: Serialize>(value: &T) -> String {
    match serde_json::to_value(value) {
        Ok(Value::String(label)) => label,
        Ok(_) | Err(_) => "unknown".to_string(),
    }
}

pub(super) fn parse_identifier<T>(
    value: impl Into<String>,
    field_name: &str,
    parse: impl FnOnce(String) -> Option<T>,
) -> T {
    let value = value.into();
    let _ = field_name;
    parse(value).unwrap_or_else(|| std::process::abort())
}

pub(super) fn parse_optional_identifier<T>(
    value: Option<String>,
    parse: impl Fn(String) -> Option<T>,
) -> Option<T> {
    value.and_then(parse)
}

pub(super) fn parse_identifier_list<T>(
    values: &[String],
    field_name: &str,
    parse: impl Fn(String) -> Option<T> + Copy,
) -> Vec<T> {
    values
        .iter()
        .cloned()
        .map(|value| parse_identifier(value, field_name, parse))
        .collect()
}

pub(super) fn data_source_label(data_source: &ParentRouteDataSource) -> &'static str {
    let value = serialized_enum_label(data_source);
    DATA_SOURCE_LABELS
        .iter()
        .find(|(raw, _)| *raw == value)
        .map(|(_, label)| *label)
        .unwrap_or("unavailable")
}

pub(super) fn route_capability_state_for_data_source(
    data_source: &ParentRouteDataSource,
) -> &'static str {
    data_source_label(data_source)
}

pub(super) fn season_label_for_connection(
    connection_state: &ParentBridgeConnectionState,
) -> &'static str {
    let value = serialized_enum_label(connection_state);
    CONNECTION_SEASON_LABELS
        .iter()
        .find(|(raw, _)| *raw == value)
        .map(|(_, label)| *label)
        .unwrap_or("offline")
}

pub(super) fn global_connection_state_for_connection(
    connection_state: &ParentBridgeConnectionState,
) -> &'static str {
    let value = serialized_enum_label(connection_state);
    CONNECTION_GLOBAL_LABELS
        .iter()
        .find(|(raw, _)| *raw == value)
        .map(|(_, label)| *label)
        .unwrap_or("offline")
}

pub(super) fn connection_tone(connection_state: &ParentBridgeConnectionState) -> ParentPortalTone {
    let value = serialized_enum_label(connection_state);
    CONNECTION_TONES
        .iter()
        .find(|(raw, _)| *raw == value)
        .map(|(_, tone)| tone.clone())
        .unwrap_or(ParentPortalTone::Red)
}

pub(super) fn route_capability_tone(data_source: &ParentRouteDataSource) -> ParentPortalTone {
    match data_source {
        ParentRouteDataSource::RustReadModel | ParentRouteDataSource::HostBridge => {
            ParentPortalTone::Gold
        }
        ParentRouteDataSource::DevDiagnostics | ParentRouteDataSource::Unavailable => {
            ParentPortalTone::Muted
        }
    }
}

pub(super) fn data_source_tone(data_source: &ParentRouteDataSource) -> ParentPortalTone {
    let value = serialized_enum_label(data_source);
    DATA_SOURCE_TONES
        .iter()
        .find(|(raw, _)| *raw == value)
        .map(|(_, tone)| tone.clone())
        .unwrap_or(ParentPortalTone::Muted)
}

pub(super) fn parent_portal_shell_status_card_id(
    value: &'static str,
) -> ParentPortalShellStatusCardId {
    ParentPortalShellStatusCardId::parse(value).unwrap_or_else(|| std::process::abort())
}

pub(super) fn current_lan_add_device_read_model_value(
    read_model: &LanBrowserAddDeviceReadModel,
) -> ParentLanAddDeviceReadModelSnapshot {
    ParentLanAddDeviceReadModelSnapshot {
        schema_version: read_model.schema_version,
        generated_at: read_model.generated_at.clone(),
        discovery_source: serialized_enum_label(&read_model.discovery_source),
        add_device_state: serialized_enum_label(&read_model.add_device_state),
        local_service_discovery_state: serialized_enum_label(
            &read_model.local_service_discovery_state,
        ),
        physical_household_lan_state: serialized_enum_label(
            &read_model.physical_household_lan_state,
        ),
        cloud_relay_state: serialized_enum_label(&read_model.cloud_relay_state),
        scan_summary: current_lan_add_device_scan_summary_snapshot(read_model),
        discovered_devices: read_model
            .discovered_devices
            .iter()
            .map(parent_lan_browser_add_device_discovery_device_snapshot)
            .collect(),
        discovery_event_history: parent_lan_discovery_event_history_snapshot(
            &read_model.discovery_event_history,
            read_model,
        ),
        canonical_household_devices: read_model
            .canonical_household_devices
            .iter()
            .map(parent_lan_canonical_household_device_snapshot)
            .collect(),
        pairing_requests: read_model
            .pairing_requests
            .iter()
            .map(parent_lan_browser_add_device_pairing_request_snapshot)
            .collect(),
        trusted_device_registry: read_model
            .trusted_device_registry
            .iter()
            .map(parent_lan_trusted_device_registry_entry_snapshot)
            .collect(),
        household_device_decisions: read_model
            .household_device_decisions
            .iter()
            .map(parent_lan_household_device_decision_snapshot)
            .collect(),
        signed_discovery_relay_spine: read_model
            .signed_discovery_relay_spine
            .as_ref()
            .map(parent_lan_signed_discovery_relay_spine_summary_snapshot),
        lan_discovery_source_matrix: read_model
            .lan_discovery_source_matrix
            .as_ref()
            .map(parent_lan_discovery_source_matrix_snapshot),
        trusted_device_ids: parse_identifier_list(
            &read_model.trusted_device_ids,
            "trusted_device_ids",
            ParentLanDeviceId::parse,
        ),
        revoked_device_ids: parse_identifier_list(
            &read_model.revoked_device_ids,
            "revoked_device_ids",
            ParentLanDeviceId::parse,
        ),
        selected_device_readiness: parent_lan_selected_device_readiness_snapshot(
            &read_model.selected_device_readiness,
        ),
        controller_authority: serialized_enum_label(&read_model.controller_authority),
        observer_authority: serialized_enum_label(&read_model.observer_authority),
        route_requirement_labels: read_model.route_requirement_labels.clone(),
        audit_check_labels: read_model.audit_check_labels.clone(),
        honest_non_claims: read_model.honest_non_claims.clone(),
    }
}

fn current_lan_add_device_scan_summary_snapshot(
    read_model: &LanBrowserAddDeviceReadModel,
) -> ParentLanAddDeviceScanSummarySnapshot {
    ParentLanAddDeviceScanSummarySnapshot {
        schema_version: read_model.scan_summary.schema_version,
        source_labels: read_model.scan_summary.source_labels.clone(),
        scanned_device_count: read_model.scan_summary.scanned_device_count,
        agent_device_count: read_model.scan_summary.agent_device_count,
        passive_device_count: read_model.scan_summary.passive_device_count,
        infrastructure_device_count: read_model.scan_summary.infrastructure_device_count,
        unsupported_device_count: read_model.scan_summary.unsupported_device_count,
    }
}
