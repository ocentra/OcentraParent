#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use ocentra_eventing::error::EventingError;

use crate::policy_source::ParentPolicySourceDocument;

use super::{validate_parent_policy_source_document, PolicyConflictRecord, PolicyConflictSeverity};

mod pairwise;
mod rule_level;

pub(super) type ConflictScheduleMap<'a> =
    BTreeMap<super::PolicyScheduleId, &'a super::PolicyScheduleWindow>;

pub(super) fn detect_policy_conflicts(
    source: &ParentPolicySourceDocument,
) -> Result<Vec<PolicyConflictRecord>, EventingError> {
    validate_parent_policy_source_document(source)?;

    let mut conflicts = Vec::new();
    let schedule_map = source
        .schedules
        .iter()
        .map(|schedule| (schedule.schedule_id.clone(), schedule))
        .collect::<ConflictScheduleMap<'_>>();

    rule_level::push_rule_level_conflicts(&mut conflicts, source, &schedule_map)?;
    pairwise::push_pairwise_conflicts(&mut conflicts, source, &schedule_map)?;
    Ok(conflicts)
}

pub(super) fn has_blocking_policy_conflicts(conflicts: &[PolicyConflictRecord]) -> bool {
    conflicts
        .iter()
        .any(|conflict| conflict.severity == PolicyConflictSeverity::Blocking)
}
