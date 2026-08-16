#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::{ParentPolicySourceDocument, PolicyRollbackRef};

pub(crate) fn assert_rollback_ref_identity(
    document: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
) -> Result<(), EventingError> {
    if rollback_ref.household_id != document.household_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_HOUSEHOLD_ID,
            value: rollback_ref.household_id.as_str().to_string(),
        });
    }

    if rollback_ref.rolled_back_document_id != document.document_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_DOCUMENT_ID,
            value: rollback_ref.rolled_back_document_id.as_str().to_string(),
        });
    }

    if rollback_ref.rolled_back_policy_version != document.policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ROLLED_BACK_POLICY_VERSION,
            value: rollback_ref.rolled_back_policy_version.value().to_string(),
        });
    }

    Ok(())
}
