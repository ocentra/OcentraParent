#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::names::{
    missing_audit_references_for_status_value, policy_actor_role_name, policy_actor_state_name,
    policy_status_name, policy_surface_name, replacement_policy_version_must_be_newer_value,
    restored_policy_version_must_be_older_value,
};
use super::{
    ParentPolicyActorRole, ParentPolicyRule, ParentPolicySourceDocument, PolicyRollbackRef,
    PolicyScheduleBudgetCarryoverMode, PolicyScheduleBudgetResetKind, PolicyScheduleTimeBudget,
    PolicyScheduleWindow, PolicySourceActorState, PolicySourceStatus, PolicySourceSurface,
};

pub(crate) fn validate_parent_policy_source_document(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    assert_write_surface_can_author_source_truth(document.source_surface)?;
    assert_actor_role_can_author_source_truth(document.actor_role)?;
    assert_audit_refs_match_status(document)?;
    assert_status_lifecycle_refs(document)?;
    assert_schedule_windows(document)?;
    assert_unique_schedule_ids(document)?;
    assert_unique_rule_ids(document)?;
    assert_rule_schedule_refs(document)?;
    assert_active_policy_has_rules(document)?;
    Ok(())
}

pub(crate) fn assert_actor_authority_matches_document(
    document: &ParentPolicySourceDocument,
    authority_household_id: &super::PolicyHouseholdId,
    authority_actor_id: &super::PolicyActorId,
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

pub(crate) fn assert_rollback_ref_matches_document(
    document: &ParentPolicySourceDocument,
    rollback_ref: &PolicyRollbackRef,
) -> Result<(), EventingError> {
    assert_rollback_ref_identity(document, rollback_ref)?;
    assert_rollback_ref_restoration(document, rollback_ref)?;
    Ok(())
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

fn assert_write_surface_can_author_source_truth(
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

fn assert_actor_role_can_author_source_truth(
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

fn assert_audit_refs_match_status(
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

fn assert_status_lifecycle_refs(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    match document.status {
        PolicySourceStatus::Superseded => assert_superseded_status_lifecycle_refs(document),
        PolicySourceStatus::RolledBack => assert_rolled_back_status_lifecycle_refs(document),
        _ => assert_neutral_status_lifecycle_refs(document),
    }
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

fn assert_rollback_ref_identity(
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

fn assert_rollback_ref_restoration(
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

fn assert_unique_schedule_ids(document: &ParentPolicySourceDocument) -> Result<(), EventingError> {
    let mut seen = BTreeSet::new();
    for schedule in &document.schedules {
        if !seen.insert(schedule.schedule_id.clone()) {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_SCHEDULE_ID,
                value: schedule.schedule_id.as_str().to_string(),
            });
        }
    }
    Ok(())
}

fn assert_schedule_windows(document: &ParentPolicySourceDocument) -> Result<(), EventingError> {
    for schedule in &document.schedules {
        assert_schedule_window(schedule)?;
    }

    Ok(())
}

fn assert_schedule_window(schedule: &PolicyScheduleWindow) -> Result<(), EventingError> {
    assert_local_time(
        policy_control::source::FIELD_SCHEDULE_STARTS_AT,
        &schedule.starts_at,
    )?;
    assert_local_time(
        policy_control::source::FIELD_SCHEDULE_ENDS_AT,
        &schedule.ends_at,
    )?;
    assert_schedule_time_budget(&schedule.time_budget)?;
    Ok(())
}

fn assert_unique_rule_ids(document: &ParentPolicySourceDocument) -> Result<(), EventingError> {
    let mut seen = BTreeSet::new();
    for rule in &document.rules {
        if !seen.insert(rule.rule_id.clone()) {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_RULE_ID,
                value: rule.rule_id.as_str().to_string(),
            });
        }
    }
    Ok(())
}

fn assert_rule_schedule_refs(document: &ParentPolicySourceDocument) -> Result<(), EventingError> {
    let schedule_ids = document
        .schedules
        .iter()
        .map(|schedule| schedule.schedule_id.clone())
        .collect::<BTreeSet<_>>();

    for rule in &document.rules {
        assert_rule_schedule_ref(rule, &schedule_ids)?;
    }

    Ok(())
}

fn assert_rule_schedule_ref(
    rule: &ParentPolicyRule,
    schedule_ids: &BTreeSet<super::PolicyScheduleId>,
) -> Result<(), EventingError> {
    if rule.action == super::PolicyRuleAction::TimeLimit && rule.schedule_id.is_none() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_RULE_SCHEDULE_ID,
            value: rule.rule_id.as_str().to_string(),
        });
    }

    if let Some(schedule_id) = &rule.schedule_id {
        if !schedule_ids.contains(schedule_id) {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_RULE_SCHEDULE_ID,
                value: schedule_id.as_str().to_string(),
            });
        }
    }

    Ok(())
}

fn assert_schedule_time_budget(budget: &PolicyScheduleTimeBudget) -> Result<(), EventingError> {
    assert_schedule_time_budget_basics(budget)?;
    assert_schedule_time_budget_effective_until(budget)?;
    assert_schedule_time_budget_reset(budget)?;
    assert_schedule_time_budget_carryover(budget)?;
    Ok(())
}

fn assert_schedule_time_budget_basics(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    if budget.budget_window_minutes == 0 {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SCHEDULE_BUDGET_WINDOW_MINUTES,
            value: budget.budget_window_minutes.to_string(),
        });
    }

    if budget.bonus_expiry_minutes == 0 {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_SCHEDULE_BONUS_EXPIRY_MINUTES,
            value: budget.bonus_expiry_minutes.to_string(),
        });
    }

    assert_local_time(
        policy_control::source::FIELD_SCHEDULE_RESET_LOCAL_TIME,
        &budget.reset.local_time,
    )?;
    assert_utc_timestamp(
        policy_control::source::FIELD_SCHEDULE_EFFECTIVE_FROM,
        &budget.effective_from,
    )?;
    Ok(())
}

fn assert_schedule_time_budget_effective_until(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    if let Some(effective_until) = &budget.effective_until {
        assert_utc_timestamp(
            policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
            effective_until,
        )?;
        if effective_until <= &budget.effective_from {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
                value: effective_until.clone(),
            });
        }
    }

    Ok(())
}

fn assert_schedule_time_budget_reset(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    match budget.reset.kind {
        PolicyScheduleBudgetResetKind::Weekly => {
            if budget.reset.day.is_none() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_RESET_DAY,
                    value: "missing-weekly-reset-day".to_string(),
                });
            }
        }
        PolicyScheduleBudgetResetKind::Daily | PolicyScheduleBudgetResetKind::Monthly => {
            if budget.reset.day.is_some() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_RESET_DAY,
                    value: "unexpected-reset-day".to_string(),
                });
            }
        }
    }

    Ok(())
}

fn assert_schedule_time_budget_carryover(
    budget: &PolicyScheduleTimeBudget,
) -> Result<(), EventingError> {
    match budget.carryover.mode {
        PolicyScheduleBudgetCarryoverMode::DiscardUnused => {
            if budget.carryover.max_minutes.is_some() {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
                    value: "discard-unused".to_string(),
                });
            }
        }
        PolicyScheduleBudgetCarryoverMode::CapCarryover => {
            if budget.carryover.max_minutes.unwrap_or(0) == 0 {
                return Err(EventingError::InvalidValue {
                    field: policy_control::source::FIELD_SCHEDULE_CARRYOVER_MAX_MINUTES,
                    value: "cap-carryover".to_string(),
                });
            }
        }
        PolicyScheduleBudgetCarryoverMode::CarryForward => {}
    }

    Ok(())
}

fn assert_local_time(field: &'static str, value: &str) -> Result<(), EventingError> {
    if value.len() != 5 || !value.is_ascii() || value.as_bytes()[2] != b':' {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    let hour = parse_time_component(field, &value[0..2])?;
    let minute = parse_time_component(field, &value[3..5])?;
    if hour > 23 || minute > 59 {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    Ok(())
}

fn assert_utc_timestamp(field: &'static str, value: &str) -> Result<(), EventingError> {
    if value.len() != 20
        || !value.is_ascii()
        || value.as_bytes()[4] != b'-'
        || value.as_bytes()[7] != b'-'
        || value.as_bytes()[10] != b'T'
        || value.as_bytes()[13] != b':'
        || value.as_bytes()[16] != b':'
        || value.as_bytes()[19] != b'Z'
    {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    let month = parse_time_component(field, &value[5..7])?;
    let day = parse_time_component(field, &value[8..10])?;
    if month == 0 || month > 12 || day == 0 || day > 31 {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    assert_local_time(field, &value[11..16])?;
    let seconds = parse_time_component(field, &value[17..19])?;
    if seconds > 59 {
        return Err(EventingError::InvalidValue {
            field,
            value: value.to_string(),
        });
    }

    Ok(())
}

fn parse_time_component(field: &'static str, value: &str) -> Result<u8, EventingError> {
    value
        .parse::<u8>()
        .map_err(|_error| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })
}

fn assert_active_policy_has_rules(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    if document.status == PolicySourceStatus::Active && document.rules.is_empty() {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_RULES,
            value: policy_control::source::VALUE_ACTIVE_POLICY_HAS_NO_RULES.to_string(),
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
