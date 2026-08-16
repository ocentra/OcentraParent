#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::{ParentPolicyRule, ParentPolicySourceDocument};

use super::super::{
    collect_schedule_ids, rule_conflict_kind, PolicyConflictKind, PolicyConflictPrecedenceState,
    PolicyConflictRecord, PolicyConflictSeverity, PolicyReasonCode, PolicyRuleId,
};
use super::ConflictScheduleMap;

pub(super) fn push_pairwise_conflicts(
    conflicts: &mut Vec<PolicyConflictRecord>,
    source: &ParentPolicySourceDocument,
    schedule_map: &ConflictScheduleMap<'_>,
) -> Result<(), EventingError> {
    let enabled_rules = source
        .rules
        .iter()
        .filter(|rule| rule.enabled)
        .collect::<Vec<_>>();

    for left_index in 0..enabled_rules.len() {
        let left = enabled_rules[left_index];
        for right in enabled_rules.iter().skip(left_index + 1) {
            let Some(conflict) = pairwise_conflict_record(source, left, right, schedule_map)?
            else {
                continue;
            };
            conflicts.push(conflict);
        }
    }

    Ok(())
}

fn pairwise_conflict_record(
    source: &ParentPolicySourceDocument,
    left: &ParentPolicyRule,
    right: &ParentPolicyRule,
    schedule_map: &ConflictScheduleMap<'_>,
) -> Result<Option<PolicyConflictRecord>, EventingError> {
    if !same_target_with_different_actions(left, right) {
        return Ok(None);
    }

    let Some(kind) = rule_conflict_kind(left, right, schedule_map) else {
        return Ok(None);
    };

    let outcome = pairwise_conflict_outcome(kind, left, right)?;
    Ok(Some(PolicyConflictRecord {
        kind,
        severity: outcome.severity,
        precedence_state: outcome.precedence_state,
        source_document_id: source.document_id.clone(),
        source_policy_version: source.policy_version,
        target: left.target.clone(),
        winning_rule_id: outcome.winning_rule_id,
        losing_rule_id: outcome.losing_rule_id,
        schedule_ids: collect_schedule_ids(left.schedule_id.as_ref(), right.schedule_id.as_ref()),
        reason_code: outcome.reason_code,
        audit_reference_ids: source.audit_reference_ids.clone(),
        superseded_by_policy_version: source.superseded_by_policy_version,
        rollback_ref: source.rollback_ref.clone(),
    }))
}

fn same_target_with_different_actions(left: &ParentPolicyRule, right: &ParentPolicyRule) -> bool {
    left.target == right.target && left.action != right.action
}

fn pairwise_conflict_outcome(
    kind: PolicyConflictKind,
    left: &ParentPolicyRule,
    right: &ParentPolicyRule,
) -> Result<PairwiseConflictOutcome, EventingError> {
    match kind {
        PolicyConflictKind::EqualPriority => {
            PairwiseConflictOutcome::blocking(policy_control::conflict::REASON_EQUAL_PRIORITY)
        }
        PolicyConflictKind::TimezoneBoundary => {
            PairwiseConflictOutcome::blocking(policy_control::conflict::REASON_TIMEZONE_BOUNDARY)
        }
        _ => Ok(PairwiseConflictOutcome::resolved_visible(
            winning_rule_ids(left, right),
        )?),
    }
}

fn winning_rule_ids(
    left: &ParentPolicyRule,
    right: &ParentPolicyRule,
) -> (Option<PolicyRuleId>, Option<PolicyRuleId>) {
    if left.priority > right.priority {
        (Some(left.rule_id.clone()), Some(right.rule_id.clone()))
    } else {
        (Some(right.rule_id.clone()), Some(left.rule_id.clone()))
    }
}

struct PairwiseConflictOutcome {
    severity: PolicyConflictSeverity,
    precedence_state: PolicyConflictPrecedenceState,
    winning_rule_id: Option<PolicyRuleId>,
    losing_rule_id: Option<PolicyRuleId>,
    reason_code: PolicyReasonCode,
}

impl PairwiseConflictOutcome {
    fn blocking(reason: &'static str) -> Result<Self, EventingError> {
        Ok(Self {
            severity: PolicyConflictSeverity::Blocking,
            precedence_state: PolicyConflictPrecedenceState::ManualRequired,
            winning_rule_id: None,
            losing_rule_id: None,
            reason_code: PolicyReasonCode::parse(reason)?,
        })
    }

    fn resolved_visible(
        rule_ids: (Option<PolicyRuleId>, Option<PolicyRuleId>),
    ) -> Result<Self, EventingError> {
        Ok(Self {
            severity: PolicyConflictSeverity::ResolvedVisible,
            precedence_state: PolicyConflictPrecedenceState::HigherPriorityWins,
            winning_rule_id: rule_ids.0,
            losing_rule_id: rule_ids.1,
            reason_code: PolicyReasonCode::parse(
                policy_control::conflict::REASON_OVERLAPPING_ACTIONS,
            )?,
        })
    }
}
