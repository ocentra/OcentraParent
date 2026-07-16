#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::{
    ParentPolicySourceDocument, PolicyEnforcementResultArtifact, PolicyEnforcementResultState,
};

pub(super) fn assert_delivery_results_match_document(
    document: &ParentPolicySourceDocument,
    delivery_results: &[PolicyEnforcementResultArtifact],
) -> Result<(), EventingError> {
    if delivery_results.is_empty() {
        return Err(acknowledged_delivery_required_error());
    }

    for delivery_result in delivery_results {
        assert_delivery_result_matches_document(document, delivery_result)?;
    }

    Ok(())
}

fn assert_delivery_result_matches_document(
    document: &ParentPolicySourceDocument,
    delivery_result: &PolicyEnforcementResultArtifact,
) -> Result<(), EventingError> {
    if delivery_result.household_id != document.household_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_HOUSEHOLD_ID,
            value: delivery_result.household_id.as_str().to_string(),
        });
    }
    if delivery_result.source_document_id != document.document_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_DOCUMENT_ID,
            value: delivery_result.source_document_id.as_str().to_string(),
        });
    }
    if delivery_result.policy_version != document.policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_POLICY_VERSION,
            value: delivery_result.policy_version.value().to_string(),
        });
    }
    if delivery_result.state != PolicyEnforcementResultState::Acknowledged {
        return Err(acknowledged_delivery_required_error());
    }

    Ok(())
}

fn acknowledged_delivery_required_error() -> EventingError {
    EventingError::InvalidValue {
        field: policy_control::source::FIELD_STATUS,
        value: policy_control::source::VALUE_ACTIVE_POLICY_REQUIRES_ACKNOWLEDGED_DELIVERY
            .to_string(),
    }
}
