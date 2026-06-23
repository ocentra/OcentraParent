use std::env;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::parent_assistant::provider_route::ParentAssistantProviderRoute;
use ocentra_parent_agent_protocol::parent_assistant::provider_route::ParentAssistantProviderRoutingState;
use ocentra_parent_agent_protocol::parent_assistant::provider_route::ParentAssistantProviderSelection;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantApiAuthorizationState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantApiProviderAccessState;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantApiProviderBoundary;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantEvidenceContext;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderState;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

pub(crate) fn api_provider_boundary(
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    api_provider_boundary_for_access_state(
        citations,
        ParentAssistantApiProviderAccessState::NotAuthorized,
    )
}

pub(crate) fn api_provider_boundary_for_command(
    command: &AgentCommandEnvelope,
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    let access_state = if !api_authorized() || !api_authorization_context_is_complete(command) {
        ParentAssistantApiProviderAccessState::NotAuthorized
    } else if api_degraded() {
        ParentAssistantApiProviderAccessState::AuthorizedDegraded
    } else {
        ParentAssistantApiProviderAccessState::AuthorizedUnavailable
    };

    api_provider_boundary_for_access_state(citations, access_state)
}

pub(crate) fn api_authorization_context_is_complete(command: &AgentCommandEnvelope) -> bool {
    command_field_matches(
        command,
        constants::field::PARENT_ASSISTANT_API_AUTHORIZATION_STATE,
        constants::parent_assistant::API_PROVIDER_AUTHORIZATION_AUTHORIZED,
    ) && command_field_matches(
        command,
        constants::field::PARENT_ASSISTANT_API_CUSTODY_LABEL,
        constants::parent_assistant::API_PROVIDER_CUSTODY_LABEL,
    ) && command_field_matches(
        command,
        constants::field::PARENT_ASSISTANT_API_RETENTION_STATE,
        constants::parent_assistant::API_PROVIDER_RETENTION_PARENT_AUTHORIZED,
    ) && command_field_matches(
        command,
        constants::field::PARENT_ASSISTANT_API_DELETION_STATE,
        constants::parent_assistant::API_PROVIDER_DELETION_STATE,
    )
}

pub(crate) fn api_provider_boundary_for_access_state(
    citations: &[ParentAssistantEvidenceContext],
    access_state: ParentAssistantApiProviderAccessState,
) -> ParentAssistantApiProviderBoundary {
    match access_state {
        ParentAssistantApiProviderAccessState::NotAuthorized => not_authorized_boundary(citations),
        ParentAssistantApiProviderAccessState::AuthorizedUnavailable => {
            authorized_unavailable_boundary(citations)
        }
        ParentAssistantApiProviderAccessState::AuthorizedDegraded => {
            authorized_degraded_boundary(citations)
        }
    }
}

pub(crate) fn provider_route(
    local_provider_state: ParentAssistantProviderState,
    api_boundary: &ParentAssistantApiProviderBoundary,
) -> ParentAssistantProviderRoute {
    let (routing_state, selected_provider, reason) =
        provider_route_parts(local_provider_state, api_boundary);
    ParentAssistantProviderRoute {
        routing_state,
        selected_provider,
        local_provider_state,
        api_provider_state: api_boundary.provider_state,
        api_access_state: api_boundary.access_state,
        evidence_citation_required: true,
        remote_ai_optional: true,
        child_safety_or_enforcement_use_allowed: false,
        reason: reason.to_string(),
    }
}

fn provider_route_parts(
    local_provider_state: ParentAssistantProviderState,
    api_boundary: &ParentAssistantApiProviderBoundary,
) -> (
    ParentAssistantProviderRoutingState,
    ParentAssistantProviderSelection,
    &'static str,
) {
    match local_provider_state {
        ParentAssistantProviderState::Configured => (
            ParentAssistantProviderRoutingState::LocalProviderReady,
            ParentAssistantProviderSelection::Local,
            constants::parent_assistant::PROVIDER_ROUTE_LOCAL_READY_REASON,
        ),
        ParentAssistantProviderState::Degraded => (
            ParentAssistantProviderRoutingState::LocalProviderDegraded,
            ParentAssistantProviderSelection::Local,
            constants::parent_assistant::PROVIDER_ROUTE_LOCAL_DEGRADED_REASON,
        ),
        ParentAssistantProviderState::Unavailable => unavailable_provider_route(api_boundary),
    }
}

fn unavailable_provider_route(
    api_boundary: &ParentAssistantApiProviderBoundary,
) -> (
    ParentAssistantProviderRoutingState,
    ParentAssistantProviderSelection,
    &'static str,
) {
    match api_boundary.access_state {
        ParentAssistantApiProviderAccessState::AuthorizedUnavailable => (
            ParentAssistantProviderRoutingState::ApiProviderAuthorizedUnavailable,
            ParentAssistantProviderSelection::None,
            constants::parent_assistant::PROVIDER_ROUTE_API_UNAVAILABLE_REASON,
        ),
        ParentAssistantApiProviderAccessState::AuthorizedDegraded => (
            ParentAssistantProviderRoutingState::ApiProviderAuthorizedDegraded,
            ParentAssistantProviderSelection::None,
            constants::parent_assistant::PROVIDER_ROUTE_API_DEGRADED_REASON,
        ),
        ParentAssistantApiProviderAccessState::NotAuthorized => (
            ParentAssistantProviderRoutingState::NoProviderAvailable,
            ParentAssistantProviderSelection::None,
            constants::parent_assistant::PROVIDER_ROUTE_NONE_REASON,
        ),
    }
}

fn authorized_unavailable_boundary(
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    boundary(
        citations,
        &ApiProviderBoundaryParts {
            provider_id: constants::parent_assistant::API_PROVIDER_ID_AUTHORIZED,
            authorization_state: ParentAssistantApiAuthorizationState::Authorized,
            access_state: ParentAssistantApiProviderAccessState::AuthorizedUnavailable,
            retention_state: constants::parent_assistant::API_PROVIDER_RETENTION_PARENT_AUTHORIZED,
            provider_state: ParentAssistantProviderState::Unavailable,
            unavailable_reason:
                constants::parent_assistant::API_PROVIDER_AUTHORIZED_UNAVAILABLE_REASON,
        },
    )
}

fn authorized_degraded_boundary(
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    boundary(
        citations,
        &ApiProviderBoundaryParts {
            provider_id: constants::parent_assistant::API_PROVIDER_ID_AUTHORIZED,
            authorization_state: ParentAssistantApiAuthorizationState::Authorized,
            access_state: ParentAssistantApiProviderAccessState::AuthorizedDegraded,
            retention_state: constants::parent_assistant::API_PROVIDER_RETENTION_PARENT_AUTHORIZED,
            provider_state: ParentAssistantProviderState::Degraded,
            unavailable_reason:
                constants::parent_assistant::API_PROVIDER_AUTHORIZED_DEGRADED_REASON,
        },
    )
}

fn not_authorized_boundary(
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    boundary(
        citations,
        &ApiProviderBoundaryParts {
            provider_id: constants::parent_assistant::API_PROVIDER_ID_NOT_AUTHORIZED,
            authorization_state: ParentAssistantApiAuthorizationState::NotAuthorized,
            access_state: ParentAssistantApiProviderAccessState::NotAuthorized,
            retention_state: constants::parent_assistant::API_PROVIDER_RETENTION_NO_AUTHORIZATION,
            provider_state: ParentAssistantProviderState::Unavailable,
            unavailable_reason: constants::parent_assistant::API_PROVIDER_NOT_AUTHORIZED_REASON,
        },
    )
}

struct ApiProviderBoundaryParts {
    provider_id: &'static str,
    authorization_state: ParentAssistantApiAuthorizationState,
    access_state: ParentAssistantApiProviderAccessState,
    retention_state: &'static str,
    provider_state: ParentAssistantProviderState,
    unavailable_reason: &'static str,
}

fn boundary(
    citations: &[ParentAssistantEvidenceContext],
    parts: &ApiProviderBoundaryParts,
) -> ParentAssistantApiProviderBoundary {
    ParentAssistantApiProviderBoundary {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        provider_id: parts.provider_id.to_string(),
        authorization_state: parts.authorization_state,
        access_state: parts.access_state,
        parent_authorization_required: true,
        evidence_citation_required: true,
        custody_label: constants::parent_assistant::API_PROVIDER_CUSTODY_LABEL.to_string(),
        custody_state: constants::parent_assistant::API_PROVIDER_CUSTODY_STATE.to_string(),
        retention_policy: constants::parent_assistant::API_PROVIDER_RETENTION_POLICY.to_string(),
        retention_state: parts.retention_state.to_string(),
        deletion_policy: constants::parent_assistant::API_PROVIDER_DELETION_POLICY.to_string(),
        deletion_state: constants::parent_assistant::API_PROVIDER_DELETION_STATE.to_string(),
        citations: citations.to_vec(),
        provider_state: parts.provider_state,
        unavailable_reason: Some(parts.unavailable_reason.to_string()),
        child_safety_or_enforcement_use_allowed: false,
    }
}

fn api_authorized() -> bool {
    env::var(constants::parent_assistant::API_PROVIDER_AUTHORIZED_ENV)
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(false)
}

fn api_degraded() -> bool {
    env::var(constants::parent_assistant::API_PROVIDER_DEGRADED_ENV)
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(false)
}

fn command_field_matches(
    command: &AgentCommandEnvelope,
    key: &'static str,
    expected: &'static str,
) -> bool {
    matches!(
        command.payload.get(key),
        Some(LogFieldValue::String(value)) if value == expected
    )
}
