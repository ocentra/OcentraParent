use std::env;

use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, ParentAssistantApiAuthorizationState,
    ParentAssistantApiProviderAccessState, ParentAssistantApiProviderBoundary,
    ParentAssistantEvidenceContext, ParentAssistantProviderState,
};

pub(crate) fn api_provider_boundary(
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    let access_state = if !api_authorized() {
        ParentAssistantApiProviderAccessState::NotAuthorized
    } else if api_degraded() {
        ParentAssistantApiProviderAccessState::AuthorizedDegraded
    } else {
        ParentAssistantApiProviderAccessState::AuthorizedUnavailable
    };

    api_provider_boundary_for_access_state(citations, access_state)
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

fn authorized_unavailable_boundary(
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    boundary(
        citations,
        ApiProviderBoundaryParts {
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
        ApiProviderBoundaryParts {
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
        ApiProviderBoundaryParts {
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
    parts: ApiProviderBoundaryParts,
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
