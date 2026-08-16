#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_source::{ParentPolicyRule, ParentPolicySourceDocument};

use super::super::{
    policy_conflict_reason, schedule_manual_review_conflict_kind, PolicyConflictKind,
    PolicyConflictPrecedenceState, PolicyConflictRecord, PolicyConflictSeverity, PolicyReasonCode,
    PolicyRuleTarget, PolicyTargetKind,
};
use super::ConflictScheduleMap;

pub(super) fn push_rule_level_conflicts(
    conflicts: &mut Vec<PolicyConflictRecord>,
    source: &ParentPolicySourceDocument,
    schedule_map: &ConflictScheduleMap<'_>,
) -> Result<(), EventingError> {
    for rule in source.rules.iter().filter(|rule| rule.enabled) {
        push_optional_conflict(conflicts, unknown_device_target_conflict(source, rule)?);
        push_optional_conflict(
            conflicts,
            manual_review_conflict(source, rule, schedule_map)?,
        );
    }

    Ok(())
}

fn push_optional_conflict(
    conflicts: &mut Vec<PolicyConflictRecord>,
    conflict: Option<PolicyConflictRecord>,
) {
    if let Some(conflict) = conflict {
        conflicts.push(conflict);
    }
}

fn unknown_device_target_conflict(
    source: &ParentPolicySourceDocument,
    rule: &ParentPolicyRule,
) -> Result<Option<PolicyConflictRecord>, EventingError> {
    if !is_unknown_device_target(source, rule) {
        return Ok(None);
    }

    Ok(Some(PolicyConflictRecord {
        kind: PolicyConflictKind::UnknownDeviceTarget,
        severity: PolicyConflictSeverity::Blocking,
        precedence_state: PolicyConflictPrecedenceState::ManualRequired,
        source_document_id: source.document_id.clone(),
        source_policy_version: source.policy_version,
        target: rule.target.clone(),
        winning_rule_id: None,
        losing_rule_id: Some(rule.rule_id.clone()),
        schedule_ids: rule.schedule_id.clone().into_iter().collect(),
        reason_code: PolicyReasonCode::parse(
            policy_control::conflict::REASON_UNKNOWN_DEVICE_TARGET,
        )?,
        audit_reference_ids: source.audit_reference_ids.clone(),
        superseded_by_policy_version: source.superseded_by_policy_version,
        rollback_ref: source.rollback_ref.clone(),
    }))
}

fn is_unknown_device_target(source: &ParentPolicySourceDocument, rule: &ParentPolicyRule) -> bool {
    rule.target.kind == PolicyTargetKind::Device
        && !source
            .device_ids
            .iter()
            .any(|device_id| device_id.as_str() == rule.target.reference_id.as_str())
}

fn manual_review_conflict(
    source: &ParentPolicySourceDocument,
    rule: &ParentPolicyRule,
    schedule_map: &ConflictScheduleMap<'_>,
) -> Result<Option<PolicyConflictRecord>, EventingError> {
    let Some(schedule_id) = &rule.schedule_id else {
        return Ok(None);
    };
    let Some(schedule) = schedule_map.get(schedule_id).copied() else {
        return Ok(None);
    };
    let Some(kind) = schedule_manual_review_conflict_kind(schedule)? else {
        return Ok(None);
    };

    Ok(Some(blocking_schedule_conflict(
        source,
        kind,
        &rule.target,
        schedule_id,
    )?))
}

fn blocking_schedule_conflict(
    source: &ParentPolicySourceDocument,
    kind: PolicyConflictKind,
    target: &PolicyRuleTarget,
    schedule_id: &super::super::PolicyScheduleId,
) -> Result<PolicyConflictRecord, EventingError> {
    Ok(PolicyConflictRecord {
        kind,
        severity: PolicyConflictSeverity::Blocking,
        precedence_state: PolicyConflictPrecedenceState::ManualRequired,
        source_document_id: source.document_id.clone(),
        source_policy_version: source.policy_version,
        target: target.clone(),
        winning_rule_id: None,
        losing_rule_id: None,
        schedule_ids: vec![schedule_id.clone()],
        reason_code: PolicyReasonCode::parse(policy_conflict_reason(kind))?,
        audit_reference_ids: source.audit_reference_ids.clone(),
        superseded_by_policy_version: source.superseded_by_policy_version,
        rollback_ref: source.rollback_ref.clone(),
    })
}
