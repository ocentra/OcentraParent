use std::collections::HashSet;

use ocentra_parent_agent_protocol::constants::v08_enforcement_policy_dispatch as dispatch;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchApprovalState, EnforcementPolicyDispatchOutcomeState,
    EnforcementPolicyDispatchReadModelEntry, EnforcementPolicyDispatchTimerState,
};

pub(super) fn matching_policy_decision_references(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> bool {
    let Some(policy_suffix) = entry
        .intent
        .policy_decision_id
        .strip_prefix(dispatch::PREFIX_POLICY)
    else {
        return false;
    };
    let Some(decision_suffix) = entry
        .intent
        .policy_decision_ref
        .strip_prefix(dispatch::PREFIX_DECISION)
    else {
        return false;
    };

    !entry.intent.intent_id.trim().is_empty()
        && !policy_suffix.trim().is_empty()
        && policy_suffix == decision_suffix
        && policy_suffix == entry.intent.intent_id.as_str()
        && has_dispatch_reference_prefix(&entry.matrix_row.matrix_id, dispatch::PREFIX_MATRIX)
}

pub(super) fn has_dispatch_reference_prefix(value: &str, prefix: &str) -> bool {
    value
        .strip_prefix(prefix)
        .is_some_and(|suffix| !suffix.trim().is_empty())
}

pub(super) fn valid_target_reference(entry: &EnforcementPolicyDispatchReadModelEntry) -> bool {
    has_dispatch_reference_prefix(&entry.intent.target.target_id, dispatch::PREFIX_TARGET)
        && !entry.intent.target.target_value.trim().is_empty()
}

pub(super) fn valid_evidence_references(entry: &EnforcementPolicyDispatchReadModelEntry) -> bool {
    if entry.intent.evidence_references.is_empty() {
        return false;
    }

    let mut evidence_ids = HashSet::new();
    entry.intent.evidence_references.iter().all(|reference| {
        has_dispatch_reference_prefix(&reference.evidence_reference_id, dispatch::PREFIX_EVIDENCE)
            && !reference.observed_at.trim().is_empty()
            && evidence_ids.insert(reference.evidence_reference_id.as_str())
    })
}

pub(super) fn valid_related_references(entry: &EnforcementPolicyDispatchReadModelEntry) -> bool {
    valid_approval_reference(entry)
        && valid_audit_references(entry)
        && valid_timer_references(entry)
        && valid_timestamps(entry)
}

fn valid_approval_reference(entry: &EnforcementPolicyDispatchReadModelEntry) -> bool {
    let approval_required =
        entry.approval_state != EnforcementPolicyDispatchApprovalState::NotRequired;
    let Some(reference) = entry.intent.approval_ref.as_ref() else {
        return !approval_required;
    };

    approval_required
        && has_dispatch_reference_prefix(&reference.action_reference_id, dispatch::PREFIX_APPROVAL)
        && reference.actor == entry.intent.actor
        && reference.policy_version == entry.intent.policy_version
        && !reference.created_at.trim().is_empty()
}

fn valid_audit_references(entry: &EnforcementPolicyDispatchReadModelEntry) -> bool {
    if entry.audit_refs.is_empty() {
        return false;
    }

    let mut audit_ids = HashSet::new();
    entry.audit_refs.iter().all(|reference| {
        has_dispatch_reference_prefix(reference, dispatch::PREFIX_AUDIT)
            && audit_ids.insert(reference.as_str())
    })
}

fn valid_timer_references(entry: &EnforcementPolicyDispatchReadModelEntry) -> bool {
    let timer_required = entry.timer_state != EnforcementPolicyDispatchTimerState::NotRequired;
    if timer_required == entry.timer_refs.is_empty() {
        return false;
    }

    let mut timer_ids = HashSet::new();
    entry.timer_refs.iter().all(|reference| {
        has_dispatch_reference_prefix(reference, dispatch::PREFIX_TIMER)
            && timer_ids.insert(reference.as_str())
    })
}

fn valid_timestamps(entry: &EnforcementPolicyDispatchReadModelEntry) -> bool {
    let dispatched =
        entry.matrix_row.outcome_state == EnforcementPolicyDispatchOutcomeState::DispatchReady;
    if dispatched != entry.dispatched_at.is_some() {
        return false;
    }
    if entry
        .dispatched_at
        .as_deref()
        .is_some_and(|timestamp| timestamp.trim().is_empty())
        || entry
            .next_check_at
            .as_deref()
            .is_some_and(|timestamp| timestamp.trim().is_empty())
    {
        return false;
    }

    let next_check_required = matches!(
        entry.timer_state,
        EnforcementPolicyDispatchTimerState::Active
            | EnforcementPolicyDispatchTimerState::RestartRecovered
            | EnforcementPolicyDispatchTimerState::RecoveryNeeded
    );
    next_check_required == entry.next_check_at.is_some()
}
