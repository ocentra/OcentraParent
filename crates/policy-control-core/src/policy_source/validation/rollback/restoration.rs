#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::names::restored_policy_version_must_be_older_value;
use crate::policy_source::{ParentPolicySourceDocument, PolicyRollbackRef};

pub(crate) fn assert_rollback_ref_restoration(
    document: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
) -> Result<(), EventingError> {
    if rollback_ref.restored_document_id == document.document_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_RESTORED_DOCUMENT_ID,
            value: rollback_ref.restored_document_id.as_str().to_string(),
        });
    }

    if rollback_ref.restored_policy_version.value() >= document.policy_version.value() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_RESTORED_POLICY_VERSION,
            value: restored_policy_version_must_be_older_value(
                rollback_ref.restored_policy_version,
                document.policy_version,
            ),
        });
    }

    Ok(())
}
