#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::time::{self, format};
use crate::policy_source::{
    ParentPolicyRule, ParentPolicySourceDocument, PolicyRuleAction, PolicyScheduleId,
    PolicyScheduleWindow, PolicySourceStatus,
};

pub(crate) fn assert_unique_schedule_ids(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
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

pub(crate) fn assert_schedule_windows(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
    for schedule in &document.schedules {
        assert_schedule_window(schedule)?;
    }

    Ok(())
}

pub(crate) fn assert_unique_rule_ids(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
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

pub(crate) fn assert_rule_schedule_refs(
    document: &ParentPolicySourceDocument,
) -> Result<(), EventingError> {
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

pub(crate) fn assert_active_policy_has_rules(
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

fn assert_schedule_window(schedule: &PolicyScheduleWindow) -> Result<(), EventingError> {
    time::assert_schedule_time_budget(&schedule.time_budget)?;
    format::assert_local_time(
        policy_control::source::FIELD_SCHEDULE_STARTS_AT,
        &schedule.starts_at,
    )?;
    format::assert_local_time(
        policy_control::source::FIELD_SCHEDULE_ENDS_AT,
        &schedule.ends_at,
    )?;
    Ok(())
}

fn assert_rule_schedule_ref(
    rule: &ParentPolicyRule,
    schedule_ids: &BTreeSet<PolicyScheduleId>,
) -> Result<(), EventingError> {
    if rule.action == PolicyRuleAction::TimeLimit && rule.schedule_id.is_none() {
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
