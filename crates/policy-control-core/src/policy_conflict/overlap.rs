#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use super::{normalized_time_ranges, PolicyConflictKind, PolicyScheduleId, PolicyScheduleWindow};

pub(super) fn collect_schedule_ids(
    left: Option<&PolicyScheduleId>,
    right: Option<&PolicyScheduleId>,
) -> Vec<PolicyScheduleId> {
    match (left, right) {
        (Some(left), Some(right)) if left == right => vec![left.clone()],
        (Some(left), Some(right)) => vec![left.clone(), right.clone()],
        (Some(left), None) => vec![left.clone()],
        (None, Some(right)) => vec![right.clone()],
        (None, None) => Vec::new(),
    }
}

pub(super) fn rule_conflict_kind(
    left: &crate::policy_source::ParentPolicyRule,
    right: &crate::policy_source::ParentPolicyRule,
    schedule_map: &BTreeMap<PolicyScheduleId, &PolicyScheduleWindow>,
) -> Option<PolicyConflictKind> {
    match (&left.schedule_id, &right.schedule_id) {
        (None, None) | (None, Some(_)) | (Some(_), None) => {
            Some(priority_conflict_kind(left.priority, right.priority))
        }
        (Some(left_schedule_id), Some(right_schedule_id)) => {
            let left_schedule = schedule_map.get(left_schedule_id).copied()?;
            let right_schedule = schedule_map.get(right_schedule_id).copied()?;

            if left_schedule.timezone_name != right_schedule.timezone_name {
                return Some(PolicyConflictKind::TimezoneBoundary);
            }

            if !intervals_overlap(left_schedule, right_schedule) {
                return None;
            }

            Some(priority_conflict_kind(left.priority, right.priority))
        }
    }
}

pub(super) fn intervals_overlap(left: &PolicyScheduleWindow, right: &PolicyScheduleWindow) -> bool {
    let left_ranges = normalized_time_ranges(left);
    let right_ranges = normalized_time_ranges(right);

    left_ranges.iter().any(|(left_start, left_end)| {
        right_ranges
            .iter()
            .any(|(right_start, right_end)| left_start < right_end && right_start < left_end)
    })
}

fn priority_conflict_kind(left_priority: u16, right_priority: u16) -> PolicyConflictKind {
    if left_priority == right_priority {
        PolicyConflictKind::EqualPriority
    } else {
        PolicyConflictKind::OverlappingActions
    }
}
