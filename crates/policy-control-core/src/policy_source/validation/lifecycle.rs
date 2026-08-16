#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::names::{
    policy_status_name, replacement_policy_version_must_be_newer_value,
};
use crate::policy_source::{ParentPolicySourceDocument, PolicyRollbackRef, PolicySourceStatus};

pub(crate) fn assert_status_lifecycle_refs(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    match document.status {
        PolicySourceStatus::Superseded => assert_superseded_status_lifecycle_refs(document),
        PolicySourceStatus::RolledBack => assert_rolled_back_status_lifecycle_refs(document),
        _ => assert_neutral_status_lifecycle_refs(document),
    }
}

pub(crate) fn assert_rollback_ref_matches_document(
    document: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
) -> Result<(), EventingError> {
    super::rollback::assert_rollback_ref_matches_document(document, rollback_ref)
}

fn assert_superseded_status_lifecycle_refs(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    let replacement_policy_version =
        document
            .superseded_by_policy_version
            .ok_or_else(|| EventingError::InvalidValue {
                field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                value: policy_status_name(document.status).to_string(),
            })?;

    if replacement_policy_version.value() <= document.policy_version.value() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: replacement_policy_version_must_be_newer_value(
                replacement_policy_version,
                document.policy_version,
            ),
        });
    }

    if document.rollback_ref.is_some() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ROLLED_BACK_POLICY_VERSION,
            value: policy_status_name(document.status).to_string(),
        });
    }

    Ok(())
}

fn assert_rolled_back_status_lifecycle_refs(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    let rollback_ref =
        document
            .rollback_ref
            .as_ref()
            .ok_or_else(|| EventingError::InvalidValue {
                field: policy_control::source::FIELD_ROLLED_BACK_POLICY_VERSION,
                value: policy_status_name(document.status).to_string(),
            })?;
    assert_rollback_ref_matches_document(document, rollback_ref)?;

    if document.superseded_by_policy_version.is_some() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: policy_status_name(document.status).to_string(),
        });
    }

    Ok(())
}

fn assert_neutral_status_lifecycle_refs(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    if let Some(replacement_policy_version) = document.superseded_by_policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: replacement_policy_version.value().to_string(),
        });
    }

    if let Some(rollback_ref) = &document.rollback_ref {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ROLLED_BACK_POLICY_VERSION,
            value: rollback_ref.rolled_back_policy_version.value().to_string(),
        });
    }

    Ok(())
}
