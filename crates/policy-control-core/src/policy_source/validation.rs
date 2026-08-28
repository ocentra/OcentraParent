#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::names::policy_status_name;
use super::{
    ParentPolicyActorRole, ParentPolicySourceDocument, PolicyRollbackRef, PolicySourceActorState,
    PolicySourceStatus,
};

mod authority;
mod lifecycle;
mod rollback;
mod schedule;
pub(crate) mod time;

pub(crate) fn validate_parent_policy_source_document(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    authority::assert_write_surface_can_author_source_truth(document.source_surface)?;
    authority::assert_actor_role_can_author_source_truth(document.actor_role)?;
    authority::assert_audit_refs_match_status(document)?;
    lifecycle::assert_status_lifecycle_refs(document)?;
    schedule::assert_schedule_windows(document)?;
    schedule::assert_unique_schedule_ids(document)?;
    schedule::assert_unique_rule_ids(document)?;
    schedule::assert_rule_schedule_refs(document)?;
    schedule::assert_active_policy_has_rules(document)?;
    Ok(())
}

pub(crate) fn assert_actor_authority_matches_document(
    document: &ParentPolicySourceDocument,
    authority_household_id: &super::PolicyHouseholdId,
    authority_actor_id: &super::PolicyActorId,
    authority_actor_role: ParentPolicyActorRole,
    authority_actor_state: PolicySourceActorState,
) -> Result<(), EventingError> {
    authority::assert_actor_authority_matches_document(
        document,
        authority_household_id,
        authority_actor_id,
        authority_actor_role,
        authority_actor_state,
    )
}

pub(crate) fn assert_rollback_ref_matches_document(
    document: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
) -> Result<(), EventingError> {
    lifecycle::assert_rollback_ref_matches_document(document, rollback_ref)
}

pub(crate) fn assert_source_status_can_compile(
    status: PolicySourceStatus,
) -> Result<(), EventingError> {
    if matches!(
        status,
        PolicySourceStatus::Draft | PolicySourceStatus::Preview
    ) {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_STATUS,
            value: policy_status_name(status).to_string(),
        });
    }

    Ok(())
}
