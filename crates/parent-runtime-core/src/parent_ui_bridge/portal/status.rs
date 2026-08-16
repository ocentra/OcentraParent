use super::*;

pub(super) fn parent_portal_shell_status(
    route: &ParentRouteId,
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    connection_state: &ParentBridgeConnectionState,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentPortalShellStatusSnapshot {
    let route_label = route_title(route).to_string();
    let global_connection_state =
        global_connection_state_for_connection(connection_state).to_string();
    let data_source_label = data_source_label(data_source).to_string();

    ParentPortalShellStatusSnapshot {
        route_label: route_label.clone(),
        parent_access_state: parent_access_state_for_read_model(lan_add_device_read_model),
        global_connection_state: global_connection_state.clone(),
        route_capability_state: summary.route_capability.clone(),
        data_source_label: data_source_label.clone(),
        cards: parent_portal_shell_status_cards(
            summary,
            data_source,
            connection_state,
            lan_add_device_read_model,
            &route_label,
            &global_connection_state,
            &data_source_label,
        ),
    }
}

fn parent_portal_shell_status_cards(
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    connection_state: &ParentBridgeConnectionState,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
    route_label: &str,
    global_connection_state: &str,
    data_source_label: &str,
) -> Vec<ParentPortalShellStatusCardSnapshot> {
    vec![
        ParentPortalShellStatusCardSnapshot {
            id: parent_portal_shell_status_card_id("parent-access"),
            label: "Parent access".to_string(),
            value: summary.parent_access.clone(),
            detail: parent_access_detail(lan_add_device_read_model),
            tone: ParentPortalTone::Muted,
        },
        ParentPortalShellStatusCardSnapshot {
            id: parent_portal_shell_status_card_id("connection"),
            label: "Connection".to_string(),
            value: global_connection_state.to_string(),
            detail: format!("route: {route_label}"),
            tone: connection_tone(connection_state),
        },
        ParentPortalShellStatusCardSnapshot {
            id: parent_portal_shell_status_card_id("household"),
            label: "Household".to_string(),
            value: summary.household.clone(),
            detail: household_detail(lan_add_device_read_model),
            tone: ParentPortalTone::Muted,
        },
        ParentPortalShellStatusCardSnapshot {
            id: parent_portal_shell_status_card_id("child-device"),
            label: "Child device".to_string(),
            value: summary.child_device.clone(),
            detail: child_device_detail(lan_add_device_read_model, data_source_label),
            tone: ParentPortalTone::Muted,
        },
        ParentPortalShellStatusCardSnapshot {
            id: parent_portal_shell_status_card_id("route-capability"),
            label: "Route capability".to_string(),
            value: summary.route_capability.clone(),
            detail: format!("bridge: {data_source_label}"),
            tone: route_capability_tone(data_source),
        },
        ParentPortalShellStatusCardSnapshot {
            id: parent_portal_shell_status_card_id("data-source"),
            label: "Data source".to_string(),
            value: data_source_label.to_string(),
            detail: "product UI now reads this route through the host bridge facade".to_string(),
            tone: data_source_tone(data_source),
        },
    ]
}

fn household_detail(read_model: Option<&LanBrowserAddDeviceReadModel>) -> String {
    if let Some(read_model) = read_model {
        return format!(
            "{} canonical / {} discovered / {} trusted",
            read_model.canonical_household_devices.len(),
            read_model.discovered_devices.len(),
            read_model.trusted_device_registry.len()
        );
    }
    "No runtime-backed household summary is attached to this route snapshot.".to_string()
}

fn child_device_detail(
    read_model: Option<&LanBrowserAddDeviceReadModel>,
    data_source_label: &str,
) -> String {
    if let Some(read_model) = read_model {
        if let Some(selected_child_device_id) = read_model
            .selected_device_readiness
            .selected_child_device_id
            .as_deref()
        {
            return format!("selected child device: {selected_child_device_id}");
        }
        return "No child device is selected for control.".to_string();
    }
    format!("data source: {data_source_label}")
}
