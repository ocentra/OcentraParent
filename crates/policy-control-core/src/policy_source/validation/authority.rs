#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::names::{
    missing_audit_references_for_status_value, policy_actor_role_name, policy_actor_state_name,
    policy_surface_name,
};
use crate::policy_source::{
    ParentPolicyActorRole, ParentPolicySourceDocument, PolicyActorId, PolicyHouseholdId,
    PolicySourceActorState, PolicySourceStatus, PolicySourceSurface,
};

pub(crate) fn assert_write_surface_can_author_source_truth(
    surface: PolicySourceSurface,
) -> Result<(), EventingError> {
    if matches!(
        surface,
        PolicySourceSurface::ParentPortal | PolicySourceSurface::ParentCompanion
    ) {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::source::FIELD_SOURCE_SURFACE,
        value: policy_surface_name(surface).to_string(),
    })
}

pub(crate) fn assert_actor_role_can_author_source_truth(
    role: ParentPolicyActorRole,
) -> Result<(), EventingError> {
    if matches!(
        role,
        ParentPolicyActorRole::Parent | ParentPolicyActorRole::CoParent
    ) {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::source::FIELD_ACTOR_ROLE,
        value: policy_actor_role_name(role).to_string(),
    })
}

pub(crate) fn assert_actor_authority_matches_document(
    document: &ParentPolicySourceDocument,
    authority_household_id: &PolicyHouseholdId,
    authority_actor_id: &PolicyActorId,
    authority_actor_role: ParentPolicyActorRole,
    authority_actor_state: PolicySourceActorState,
) -> Result<(), EventingError> {
    if authority_household_id != &document.household_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_HOUSEHOLD_ID,
            value: authority_household_id.as_str().to_string(),
        });
    }

    if authority_actor_id != &document.actor_id {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ACTOR_ID,
            value: authority_actor_id.as_str().to_string(),
        });
    }

    if authority_actor_role != document.actor_role {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ACTOR_ROLE,
            value: policy_actor_role_name(authority_actor_role).to_string(),
        });
    }

    if authority_actor_state != PolicySourceActorState::Active {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_ACTOR_STATE,
            value: policy_actor_state_name(authority_actor_state).to_string(),
        });
    }

    Ok(())
}

pub(crate) fn assert_audit_refs_match_status(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    if policy_status_requires_audit_refs(document.status) && document.audit_reference_ids.is_empty()
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_AUDIT_REFERENCE_IDS,
            value: missing_audit_references_for_status_value(document.status),
        });
    }

    Ok(())
}

fn policy_status_requires_audit_refs(status: PolicySourceStatus) -> bool {
    !matches!(
        status,
        PolicySourceStatus::Draft | PolicySourceStatus::Preview
    )
}
