use super::*;

pub(super) fn lan_route_query_for_load_impl(
    route: &ParentRouteId,
    context: Option<&ParentRouteContext>,
) -> LanRouteQuery {
    if !requires_lan_read_model(route) {
        return LanRouteQuery::NotRequired;
    }
    match load_lan_status_snapshot(context) {
        Ok(snapshot) => LanRouteQuery::Available(Box::new(snapshot)),
        Err(error) => LanRouteQuery::Unavailable(error.to_string()),
    }
}

pub(super) fn lan_route_query_for_action_impl(action: &ParentUiAction) -> LanRouteQuery {
    if !requires_lan_read_model(&action.route) {
        return LanRouteQuery::NotRequired;
    }

    let context = action.context.as_ref();
    let response = match action.action {
        ParentUiActionKind::AgentCommandRequested => action
            .command
            .as_deref()
            .ok_or_else(missing_lan_agent_command_error)
            .and_then(|command_name| {
                dispatch_lan_agent_command(AgentCommandText(command_name), &action.payload, context)
            }),
        ParentUiActionKind::LanPairingBrowserDiscoveryScanRequested => {
            request_lan_browser_discovery_scan(context)
        }
        _ => load_lan_status_snapshot(context),
    };
    match response {
        Ok(snapshot) => LanRouteQuery::Available(Box::new(snapshot)),
        Err(error) => LanRouteQuery::Unavailable(error.to_string()),
    }
}

fn missing_lan_agent_command_error() -> AgentServiceError {
    AgentServiceError::from_display(
        "parent Rust facade rejected LAN agent command request without a command name",
    )
}
