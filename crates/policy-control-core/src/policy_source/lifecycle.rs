#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::constants::policy_control;

mod compatibility;
mod delivery;

use super::names::{
    duplicate_source_truth_value, missing_audit_reference_for_status_value,
    replacement_policy_version_must_be_newer_value, stale_policy_version_value,
};
use super::validation::{
    assert_actor_authority_matches_document, assert_rollback_ref_matches_document,
    assert_source_status_can_compile, validate_parent_policy_source_document,
};
use super::{
    CompiledDomainPolicyArtifact, ParentPolicySourceDocument, PolicyAuditEvent,
    PolicyAuditReferenceId, PolicyConsumerDomain, PolicyEnforcementResultArtifact,
    PolicyEnforcementResultState, PolicyRollbackRef, PolicySourceActorAuthority,
    PolicySourceCompatibilityReport, PolicySourceStatus, PolicyVersion,
};

pub(crate) fn register_parent_policy_source_document(
    existing: Option<&ParentPolicySourceDocument>,
    candidate: ParentPolicySourceDocument,
) -> Result<ParentPolicySourceDocument, EventingError> {
    validate_parent_policy_source_document(&candidate)?;

    if let Some(current) = existing {
        if current.household_id == candidate.household_id {
            assert_not_stale_source_version(current, &candidate)?;
            assert_not_duplicate_source_truth(current, &candidate)?;
        }
    }

    Ok(candidate)
}

pub(crate) fn register_parent_policy_source_document_with_authority(
    existing: Option<&ParentPolicySourceDocument>,
    candidate: ParentPolicySourceDocument,
    authority: &PolicySourceActorAuthority,
) -> Result<ParentPolicySourceDocument, EventingError> {
    assert_actor_authority_matches_document(
        &candidate,
        &authority.household_id,
        &authority.actor_id,
        authority.actor_role,
        authority.actor_state,
    )?;
    register_parent_policy_source_document(existing, candidate)
}

pub(crate) fn mark_parent_policy_source_document_active(
    document: &ParentPolicySourceDocument,
    delivery_results: &[PolicyEnforcementResultArtifact],
) -> Result<ParentPolicySourceDocument, EventingError> {
    validate_parent_policy_source_document(document)?;

    delivery::assert_delivery_results_match_document(document, delivery_results)?;

    let mut activated = document.clone();
    activated.status = PolicySourceStatus::Active;
    validate_parent_policy_source_document(&activated)?;
    Ok(activated)
}

pub(crate) fn supersede_parent_policy_source_document(
    current: &ParentPolicySourceDocument,
    replacement_policy_version: PolicyVersion,
    supersede_audit_reference_id: PolicyAuditReferenceId,
) -> Result<ParentPolicySourceDocument, EventingError> {
    validate_parent_policy_source_document(current)?;

    assert_newer_supersede_version(current, replacement_policy_version)?;
    assert_supersede_audit_reference_is_unique(current, &supersede_audit_reference_id)?;

    let mut superseded = current.clone();
    superseded.status = PolicySourceStatus::Superseded;
    superseded.superseded_by_policy_version = Some(replacement_policy_version);
    superseded.rollback_ref = None;
    superseded
        .audit_reference_ids
        .push(supersede_audit_reference_id);
    validate_parent_policy_source_document(&superseded)?;
    Ok(superseded)
}

pub(crate) fn rollback_parent_policy_source_document(
    current: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
    rollback_audit_reference_id: PolicyAuditReferenceId,
) -> Result<ParentPolicySourceDocument, EventingError> {
    validate_parent_policy_source_document(current)?;
    assert_rollback_ref_matches_document(current, rollback_ref)?;
    assert_rollback_audit_reference_is_unique(current, &rollback_audit_reference_id)?;

    let mut rolled_back = current.clone();
    rolled_back.status = PolicySourceStatus::RolledBack;
    rolled_back.superseded_by_policy_version = None;
    rolled_back.rollback_ref = Some(rollback_ref.clone());
    rolled_back
        .audit_reference_ids
        .push(rollback_audit_reference_id);
    validate_parent_policy_source_document(&rolled_back)?;
    Ok(rolled_back)
}

pub(crate) fn compile_domain_policy_artifact(
    source: &ParentPolicySourceDocument,
    domain: PolicyConsumerDomain,
) -> Result<CompiledDomainPolicyArtifact, EventingError> {
    validate_parent_policy_source_document(source)?;
    assert_source_status_can_compile(source.status)?;
    Ok(CompiledDomainPolicyArtifact {
        household_id: source.household_id.clone(),
        policy_version: source.policy_version,
        source_document_id: source.document_id.clone(),
        domain,
        rule_count: source.rules.len(),
        schedules: source.schedules.clone(),
        audit_reference_ids: source.audit_reference_ids.clone(),
        superseded_by_policy_version: source.superseded_by_policy_version,
        rollback_ref: source.rollback_ref.clone(),
    })
}

pub(crate) fn policy_enforcement_result_artifact(
    source: &ParentPolicySourceDocument,
    state: PolicyEnforcementResultState,
) -> Result<PolicyEnforcementResultArtifact, EventingError> {
    validate_parent_policy_source_document(source)?;
    Ok(PolicyEnforcementResultArtifact {
        household_id: source.household_id.clone(),
        policy_version: source.policy_version,
        source_document_id: source.document_id.clone(),
        state,
        audit_reference_ids: source.audit_reference_ids.clone(),
    })
}

pub(crate) fn latest_policy_audit_event(
    source: &ParentPolicySourceDocument,
) -> Result<PolicyAuditEvent, EventingError> {
    validate_parent_policy_source_document(source)?;
    let audit_reference_id =
        source
            .audit_reference_ids
            .last()
            .cloned()
            .ok_or_else(|| EventingError::InvalidValue {
                field: policy_control::source::FIELD_AUDIT_REFERENCE_IDS,
                value: missing_audit_reference_for_status_value(source.status),
            })?;

    Ok(PolicyAuditEvent {
        audit_reference_id,
        household_id: source.household_id.clone(),
        policy_version: source.policy_version,
        actor_id: source.actor_id.clone(),
        actor_role: source.actor_role,
        status: source.status,
    })
}

pub(crate) fn assess_policy_source_compatibility(
    source: &ParentPolicySourceDocument,
    supported_schema_version: SchemaVersion,
    minimum_supported_policy_version: PolicyVersion,
) -> Result<PolicySourceCompatibilityReport, EventingError> {
    validate_parent_policy_source_document(source)?;
    compatibility::assess_policy_source_compatibility(
        source,
        supported_schema_version,
        minimum_supported_policy_version,
    )
}

fn assert_not_stale_source_version(
    current: &ParentPolicySourceDocument,
    candidate: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    if candidate.policy_version.value() < current.policy_version.value() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_POLICY_VERSION,
            value: stale_policy_version_value(candidate.policy_version, current.policy_version),
        });
    }

    Ok(())
}

fn assert_not_duplicate_source_truth(
    current: &ParentPolicySourceDocument,
    candidate: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    if candidate.policy_version.value() == current.policy_version.value()
        && candidate.document_id != current.document_id
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_DOCUMENT_ID,
            value: duplicate_source_truth_value(&candidate.household_id, candidate.policy_version),
        });
    }

    Ok(())
}

fn assert_newer_supersede_version(
    current: &ParentPolicySourceDocument,
    replacement_policy_version: PolicyVersion,
) -> Result<(), EventingError> {
    if replacement_policy_version.value() <= current.policy_version.value() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: replacement_policy_version_must_be_newer_value(
                replacement_policy_version,
                current.policy_version,
            ),
        });
    }

    Ok(())
}

fn assert_supersede_audit_reference_is_unique(
    current: &ParentPolicySourceDocument,
    supersede_audit_reference_id: &PolicyAuditReferenceId,
) -> Result<(), EventingError> {
    if current
        .audit_reference_ids
        .contains(supersede_audit_reference_id)
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_AUDIT_REFERENCE_ID,
            value: supersede_audit_reference_id.as_str().to_string(),
        });
    }

    Ok(())
}

fn assert_rollback_audit_reference_is_unique(
    current: &ParentPolicySourceDocument,
    rollback_audit_reference_id: &PolicyAuditReferenceId,
) -> Result<(), EventingError> {
    if current
        .audit_reference_ids
        .contains(rollback_audit_reference_id)
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_AUDIT_REFERENCE_ID,
            value: rollback_audit_reference_id.as_str().to_string(),
        });
    }

    Ok(())
}
