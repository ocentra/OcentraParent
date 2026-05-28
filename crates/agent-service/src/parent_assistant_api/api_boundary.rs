use std::env;

use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, ParentAssistantApiAuthorizationState,
    ParentAssistantApiProviderBoundary, ParentAssistantEvidenceContext,
    ParentAssistantProviderState,
};

pub(crate) fn api_provider_boundary(
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    if api_authorized() {
        return authorized_boundary(citations);
    }
    not_authorized_boundary(citations)
}

fn authorized_boundary(
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    ParentAssistantApiProviderBoundary {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        provider_id: constants::parent_assistant::API_PROVIDER_ID_AUTHORIZED.to_string(),
        authorization_state: ParentAssistantApiAuthorizationState::Authorized,
        custody_label: constants::parent_assistant::API_PROVIDER_CUSTODY_LABEL.to_string(),
        retention_policy: constants::parent_assistant::API_PROVIDER_RETENTION_POLICY.to_string(),
        deletion_policy: constants::parent_assistant::API_PROVIDER_DELETION_POLICY.to_string(),
        citations: citations.to_vec(),
        provider_state: ParentAssistantProviderState::Unavailable,
        unavailable_reason: Some(
            constants::parent_assistant::API_PROVIDER_AUTHORIZED_UNAVAILABLE_REASON.to_string(),
        ),
        child_safety_or_enforcement_use_allowed: false,
    }
}

fn not_authorized_boundary(
    citations: &[ParentAssistantEvidenceContext],
) -> ParentAssistantApiProviderBoundary {
    ParentAssistantApiProviderBoundary {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        provider_id: constants::parent_assistant::API_PROVIDER_ID_NOT_AUTHORIZED.to_string(),
        authorization_state: ParentAssistantApiAuthorizationState::NotAuthorized,
        custody_label: constants::parent_assistant::API_PROVIDER_CUSTODY_LABEL.to_string(),
        retention_policy: constants::parent_assistant::API_PROVIDER_RETENTION_POLICY.to_string(),
        deletion_policy: constants::parent_assistant::API_PROVIDER_DELETION_POLICY.to_string(),
        citations: citations.to_vec(),
        provider_state: ParentAssistantProviderState::Unavailable,
        unavailable_reason: Some(
            constants::parent_assistant::API_PROVIDER_NOT_AUTHORIZED_REASON.to_string(),
        ),
        child_safety_or_enforcement_use_allowed: false,
    }
}

fn api_authorized() -> bool {
    env::var(constants::parent_assistant::API_PROVIDER_AUTHORIZED_ENV)
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(false)
}
