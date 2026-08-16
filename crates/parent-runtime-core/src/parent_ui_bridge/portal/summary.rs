use super::*;
#[path = "visibility.rs"]
mod visibility;

pub(super) fn summary_for_route(
    route: &ParentRouteId,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> ParentRouteSummary {
    if let Some(read_model) = lan_add_device_read_model {
        return ParentRouteSummary {
            title: route_title(route).to_string(),
            route_capability: route_capability_state_for_data_source(data_source).to_string(),
            parent_access: parent_access_summary(read_model),
            household: lan_household_summary(read_model),
            child_device: lan_child_device_summary(read_model),
        };
    }

    ParentRouteSummary {
        title: route_title(route).to_string(),
        route_capability: route_capability_state_for_data_source(data_source).to_string(),
        parent_access: if *data_source == ParentRouteDataSource::DevDiagnostics {
            "unavailable".to_string()
        } else {
            "proof-missing".to_string()
        },
        household: "unavailable".to_string(),
        child_device: "unavailable".to_string(),
    }
}

fn lan_household_summary(read_model: &LanBrowserAddDeviceReadModel) -> String {
    let device_count = visibility::lan_visible_device_count(read_model);
    if device_count == 0 {
        serialized_enum_label(&read_model.physical_household_lan_state)
    } else if device_count == 1 {
        "1 device visible".to_string()
    } else {
        format!("{device_count} devices visible")
    }
}

fn lan_child_device_summary(read_model: &LanBrowserAddDeviceReadModel) -> String {
    if read_model.selected_device_readiness.ready_for_control {
        return "ready-for-control".to_string();
    }
    if read_model
        .selected_device_readiness
        .selected_child_device_id
        .is_some()
    {
        return "selected".to_string();
    }
    let device_count = visibility::lan_visible_device_count(read_model);
    if device_count == 0 {
        serialized_enum_label(&read_model.add_device_state)
    } else if device_count == 1 {
        "1 discoverable".to_string()
    } else {
        format!("{device_count} discoverable")
    }
}

fn parent_portal_rows(
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Vec<ParentPortalRowSnapshot> {
    let local_agent_trend = lan_add_device_read_model
        .map(|read_model| serialized_enum_label(&read_model.local_service_discovery_state))
        .unwrap_or_else(|| data_source_label(data_source).to_string());
    let household_trend = summary.household.clone();
    let child_device_trend = summary.child_device.clone();

    vec![
        portal_row_snapshot(
            "Local agent",
            1,
            "Runtime",
            local_agent_trend,
            ParentPortalTone::Cyan,
        ),
        portal_row_snapshot(
            "Route capability",
            2,
            "Route",
            summary.route_capability.clone(),
            ParentPortalTone::Gold,
        ),
        portal_row_snapshot(
            "Parent access",
            3,
            "Authority",
            summary.parent_access.clone(),
            ParentPortalTone::Purple,
        ),
        portal_row_snapshot(
            "Household",
            4,
            "Custody",
            household_trend,
            ParentPortalTone::Red,
        ),
        portal_row_snapshot(
            "Child device",
            5,
            "Transport",
            child_device_trend,
            ParentPortalTone::Cyan,
        ),
        portal_row_snapshot(
            "Data source",
            6,
            "Bridge",
            data_source_label(data_source).to_string(),
            ParentPortalTone::Muted,
        ),
    ]
}

pub(super) fn parent_portal_rows_for_route(
    route: &ParentRouteId,
    summary: &ParentRouteSummary,
    data_source: &ParentRouteDataSource,
    lan_add_device_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> Option<Vec<ParentPortalRowSnapshot>> {
    if is_lan_surface_route(route) {
        return Some(parent_portal_rows(
            summary,
            data_source,
            lan_add_device_read_model,
        ));
    }
    None
}
