use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::provider_route::ParentAssistantProviderRoutingState;
use ocentra_parent_agent_protocol::parent_assistant::provider_route::ParentAssistantProviderSelection;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantApiProviderAccessState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantApiProviderBoundary;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderState;

use super::ParentAssistantTextRef;

pub(super) fn provider_route_parts(
    local_provider_state: ParentAssistantProviderState,
    api_boundary: &ParentAssistantApiProviderBoundary,
) -> (
    ParentAssistantProviderRoutingState,
    ParentAssistantProviderSelection,
    ParentAssistantTextRef<'static>,
) {
    match local_provider_state {
        ParentAssistantProviderState::Configured => (
            ParentAssistantProviderRoutingState::LocalProviderReady,
            ParentAssistantProviderSelection::Local,
            ParentAssistantTextRef(constants::parent_assistant::PROVIDER_ROUTE_LOCAL_READY_REASON),
        ),
        ParentAssistantProviderState::Degraded => (
            ParentAssistantProviderRoutingState::LocalProviderDegraded,
            ParentAssistantProviderSelection::Local,
            ParentAssistantTextRef(
                constants::parent_assistant::PROVIDER_ROUTE_LOCAL_DEGRADED_REASON,
            ),
        ),
        ParentAssistantProviderState::Unavailable => unavailable_provider_route(api_boundary),
    }
}

fn unavailable_provider_route(
    api_boundary: &ParentAssistantApiProviderBoundary,
) -> (
    ParentAssistantProviderRoutingState,
    ParentAssistantProviderSelection,
    ParentAssistantTextRef<'static>,
) {
    match api_boundary.access_state {
        ParentAssistantApiProviderAccessState::AuthorizedUnavailable => (
            ParentAssistantProviderRoutingState::ApiProviderAuthorizedUnavailable,
            ParentAssistantProviderSelection::None,
            ParentAssistantTextRef(
                constants::parent_assistant::PROVIDER_ROUTE_API_UNAVAILABLE_REASON,
            ),
        ),
        ParentAssistantApiProviderAccessState::AuthorizedDegraded => (
            ParentAssistantProviderRoutingState::ApiProviderAuthorizedDegraded,
            ParentAssistantProviderSelection::None,
            ParentAssistantTextRef(constants::parent_assistant::PROVIDER_ROUTE_API_DEGRADED_REASON),
        ),
        ParentAssistantApiProviderAccessState::NotAuthorized => (
            ParentAssistantProviderRoutingState::NoProviderAvailable,
            ParentAssistantProviderSelection::None,
            ParentAssistantTextRef(constants::parent_assistant::PROVIDER_ROUTE_NONE_REASON),
        ),
    }
}
