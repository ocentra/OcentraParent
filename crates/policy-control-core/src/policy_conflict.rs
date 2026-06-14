#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use ocentra_eventing::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use crate::policy_source::{
    validate_parent_policy_source_document, ParentPolicyDocumentId, ParentPolicySourceDocument,
    PolicyAuditReferenceId, PolicyReasonCode, PolicyRollbackRef, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleClockSource, PolicyScheduleId, PolicyScheduleWindow, PolicyTargetKind,
    PolicyVersion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyConflictKind {
    #[serde(rename = "overlapping-actions")]
    OverlappingActions,
    #[serde(rename = "equal-priority")]
    EqualPriority,
    #[serde(rename = "unknown-device-target")]
    UnknownDeviceTarget,
    #[serde(rename = "timezone-boundary")]
    TimezoneBoundary,
    #[serde(rename = "ambiguous-local-time")]
    AmbiguousLocalTime,
    #[serde(rename = "nonexistent-local-time")]
    NonexistentLocalTime,
    #[serde(rename = "clock-skew")]
    ClockSkew,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyConflictSeverity {
    #[serde(rename = "resolved-visible")]
    ResolvedVisible,
    #[serde(rename = "blocking")]
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyConflictPrecedenceState {
    #[serde(rename = "higher-priority-wins")]
    HigherPriorityWins,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyConflictRecord {
    pub kind: PolicyConflictKind,
    pub severity: PolicyConflictSeverity,
    pub precedence_state: PolicyConflictPrecedenceState,
    pub source_document_id: ParentPolicyDocumentId,
    pub source_policy_version: PolicyVersion,
    pub target: PolicyRuleTarget,
    pub winning_rule_id: Option<PolicyRuleId>,
    pub losing_rule_id: Option<PolicyRuleId>,
    pub schedule_ids: Vec<PolicyScheduleId>,
    pub reason_code: PolicyReasonCode,
    pub audit_reference_ids: Vec<PolicyAuditReferenceId>,
    pub superseded_by_policy_version: Option<PolicyVersion>,
    pub rollback_ref: Option<PolicyRollbackRef>,
}

pub fn detect_policy_conflicts(
    source: &ParentPolicySourceDocument,
) -> Result<Vec<PolicyConflictRecord>, EventingError> {
    validate_parent_policy_source_document(source)?;

    let mut conflicts = Vec::new();
    let schedule_map = source
        .schedules
        .iter()
        .map(|schedule| (schedule.schedule_id.clone(), schedule))
        .collect::<BTreeMap<_, _>>();

    for rule in source.rules.iter().filter(|rule| rule.enabled) {
        if rule.target.kind == PolicyTargetKind::Device
            && !source
                .device_ids
                .iter()
                .any(|device_id| device_id.as_str() == rule.target.reference_id.as_str())
        {
            conflicts.push(PolicyConflictRecord {
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
            });
        }

        let Some(schedule_id) = &rule.schedule_id else {
            continue;
        };
        let Some(schedule) = schedule_map.get(schedule_id).copied() else {
            continue;
        };
        let Some(kind) = schedule_manual_review_conflict_kind(schedule)? else {
            continue;
        };

        conflicts.push(PolicyConflictRecord {
            kind,
            severity: PolicyConflictSeverity::Blocking,
            precedence_state: PolicyConflictPrecedenceState::ManualRequired,
            source_document_id: source.document_id.clone(),
            source_policy_version: source.policy_version,
            target: rule.target.clone(),
            winning_rule_id: None,
            losing_rule_id: None,
            schedule_ids: vec![schedule_id.clone()],
            reason_code: PolicyReasonCode::parse(policy_conflict_reason(kind))?,
            audit_reference_ids: source.audit_reference_ids.clone(),
            superseded_by_policy_version: source.superseded_by_policy_version,
            rollback_ref: source.rollback_ref.clone(),
        });
    }

    let enabled_rules = source
        .rules
        .iter()
        .filter(|rule| rule.enabled)
        .collect::<Vec<_>>();

    for left_index in 0..enabled_rules.len() {
        let left = enabled_rules[left_index];
        for right in enabled_rules.iter().skip(left_index + 1) {
            if left.target != right.target || left.action == right.action {
                continue;
            }

            let Some(kind) = rule_conflict_kind(left, right, &schedule_map) else {
                continue;
            };

            let schedule_ids =
                collect_schedule_ids(left.schedule_id.as_ref(), right.schedule_id.as_ref());

            let (severity, precedence_state, winning_rule_id, losing_rule_id, reason_code) = if kind
                == PolicyConflictKind::EqualPriority
            {
                (
                    PolicyConflictSeverity::Blocking,
                    PolicyConflictPrecedenceState::ManualRequired,
                    None,
                    None,
                    PolicyReasonCode::parse(policy_control::conflict::REASON_EQUAL_PRIORITY)?,
                )
            } else if kind == PolicyConflictKind::TimezoneBoundary {
                (
                    PolicyConflictSeverity::Blocking,
                    PolicyConflictPrecedenceState::ManualRequired,
                    None,
                    None,
                    PolicyReasonCode::parse(policy_control::conflict::REASON_TIMEZONE_BOUNDARY)?,
                )
            } else {
                let (winning_rule_id, losing_rule_id) = if left.priority > right.priority {
                    (Some(left.rule_id.clone()), Some(right.rule_id.clone()))
                } else {
                    (Some(right.rule_id.clone()), Some(left.rule_id.clone()))
                };
                (
                    PolicyConflictSeverity::ResolvedVisible,
                    PolicyConflictPrecedenceState::HigherPriorityWins,
                    winning_rule_id,
                    losing_rule_id,
                    PolicyReasonCode::parse(policy_control::conflict::REASON_OVERLAPPING_ACTIONS)?,
                )
            };

            conflicts.push(PolicyConflictRecord {
                kind,
                severity,
                precedence_state,
                source_document_id: source.document_id.clone(),
                source_policy_version: source.policy_version,
                target: left.target.clone(),
                winning_rule_id,
                losing_rule_id,
                schedule_ids,
                reason_code,
                audit_reference_ids: source.audit_reference_ids.clone(),
                superseded_by_policy_version: source.superseded_by_policy_version,
                rollback_ref: source.rollback_ref.clone(),
            });
        }
    }

    Ok(conflicts)
}

fn schedule_manual_review_conflict_kind(
    schedule: &PolicyScheduleWindow,
) -> Result<Option<PolicyConflictKind>, EventingError> {
    if schedule_has_nonexistent_local_time(schedule)? {
        return Ok(Some(PolicyConflictKind::NonexistentLocalTime));
    }
    if schedule_has_ambiguous_local_time(schedule)? {
        return Ok(Some(PolicyConflictKind::AmbiguousLocalTime));
    }
    if matches!(
        schedule.time_budget.clock_source,
        PolicyScheduleClockSource::ManualRequired
    ) {
        return Ok(Some(PolicyConflictKind::ClockSkew));
    }
    Ok(None)
}

fn policy_conflict_reason(kind: PolicyConflictKind) -> &'static str {
    match kind {
        PolicyConflictKind::AmbiguousLocalTime => {
            policy_control::conflict::REASON_AMBIGUOUS_LOCAL_TIME
        }
        PolicyConflictKind::NonexistentLocalTime => {
            policy_control::conflict::REASON_NONEXISTENT_LOCAL_TIME
        }
        PolicyConflictKind::ClockSkew => policy_control::conflict::REASON_CLOCK_SKEW,
        PolicyConflictKind::TimezoneBoundary => policy_control::conflict::REASON_TIMEZONE_BOUNDARY,
        PolicyConflictKind::EqualPriority => policy_control::conflict::REASON_EQUAL_PRIORITY,
        PolicyConflictKind::UnknownDeviceTarget => {
            policy_control::conflict::REASON_UNKNOWN_DEVICE_TARGET
        }
        PolicyConflictKind::OverlappingActions => {
            policy_control::conflict::REASON_OVERLAPPING_ACTIONS
        }
    }
}

pub fn has_blocking_policy_conflicts(conflicts: &[PolicyConflictRecord]) -> bool {
    conflicts
        .iter()
        .any(|conflict| conflict.severity == PolicyConflictSeverity::Blocking)
}

fn collect_schedule_ids(
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

fn rule_conflict_kind(
    left: &crate::policy_source::ParentPolicyRule,
    right: &crate::policy_source::ParentPolicyRule,
    schedule_map: &BTreeMap<PolicyScheduleId, &PolicyScheduleWindow>,
) -> Option<PolicyConflictKind> {
    match (&left.schedule_id, &right.schedule_id) {
        (None, None) | (None, Some(_)) | (Some(_), None) => {
            if left.priority == right.priority {
                Some(PolicyConflictKind::EqualPriority)
            } else {
                Some(PolicyConflictKind::OverlappingActions)
            }
        }
        (Some(left_schedule_id), Some(right_schedule_id)) => {
            let Some(left_schedule) = schedule_map.get(left_schedule_id).copied() else {
                return None;
            };
            let Some(right_schedule) = schedule_map.get(right_schedule_id).copied() else {
                return None;
            };

            if left_schedule.timezone_name != right_schedule.timezone_name {
                return Some(PolicyConflictKind::TimezoneBoundary);
            }

            if !intervals_overlap(left_schedule, right_schedule) {
                return None;
            }

            if left.priority == right.priority {
                Some(PolicyConflictKind::EqualPriority)
            } else {
                Some(PolicyConflictKind::OverlappingActions)
            }
        }
    }
}

fn intervals_overlap(left: &PolicyScheduleWindow, right: &PolicyScheduleWindow) -> bool {
    let left_ranges = normalized_time_ranges(left);
    let right_ranges = normalized_time_ranges(right);

    left_ranges.iter().any(|(left_start, left_end)| {
        right_ranges
            .iter()
            .any(|(right_start, right_end)| left_start < right_end && right_start < left_end)
    })
}

fn schedule_has_nonexistent_local_time(
    schedule: &PolicyScheduleWindow,
) -> Result<bool, EventingError> {
    if !schedule_uses_supported_wp07_dst_timezone(schedule) {
        return Ok(false);
    }
    if !schedule_on_single_transition_day(schedule, DstTransitionKind::SpringForward)? {
        return Ok(false);
    }
    schedule_has_transition_local_time(schedule, DstTransitionKind::SpringForward)
}

fn schedule_has_ambiguous_local_time(
    schedule: &PolicyScheduleWindow,
) -> Result<bool, EventingError> {
    if !schedule_uses_supported_wp07_dst_timezone(schedule) {
        return Ok(false);
    }
    if !schedule_on_single_transition_day(schedule, DstTransitionKind::FallBack)? {
        return Ok(false);
    }
    schedule_has_transition_local_time(schedule, DstTransitionKind::FallBack)
}

fn schedule_has_transition_local_time(
    schedule: &PolicyScheduleWindow,
    transition: DstTransitionKind,
) -> Result<bool, EventingError> {
    Ok([
        &schedule.starts_at,
        &schedule.ends_at,
        &schedule.time_budget.reset.local_time,
    ]
    .into_iter()
    .try_fold(false, |found, value| {
        if found {
            return Ok(true);
        }
        let minutes = parse_clock_minutes(value).ok_or_else(|| EventingError::InvalidValue {
            field: policy_control::source::FIELD_SCHEDULE_STARTS_AT,
            value: value.to_string(),
        })?;
        Ok(match transition {
            DstTransitionKind::SpringForward => (120..180).contains(&minutes),
            DstTransitionKind::FallBack => (60..120).contains(&minutes),
        })
    })?)
}

fn schedule_on_single_transition_day(
    schedule: &PolicyScheduleWindow,
    transition: DstTransitionKind,
) -> Result<bool, EventingError> {
    let Some(effective_until) = &schedule.time_budget.effective_until else {
        return Ok(false);
    };

    let effective_from = parse_utc_date(
        policy_control::source::FIELD_SCHEDULE_EFFECTIVE_FROM,
        &schedule.time_budget.effective_from,
    )?;
    let effective_until = parse_utc_date(
        policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
        effective_until,
    )?;

    if effective_from != effective_until {
        return Ok(false);
    }

    Ok(match transition {
        DstTransitionKind::SpringForward => {
            effective_from.month == 3
                && effective_from.day >= 8
                && effective_from.day <= 14
                && day_of_week(
                    effective_from.year,
                    effective_from.month,
                    effective_from.day,
                ) == 0
        }
        DstTransitionKind::FallBack => {
            effective_from.month == 11
                && effective_from.day >= 1
                && effective_from.day <= 7
                && day_of_week(
                    effective_from.year,
                    effective_from.month,
                    effective_from.day,
                ) == 0
        }
    })
}

fn schedule_uses_supported_wp07_dst_timezone(schedule: &PolicyScheduleWindow) -> bool {
    matches!(
        schedule.timezone_name.as_str(),
        "America/Toronto" | "America/Vancouver" | "America/Los_Angeles" | "America/Winnipeg"
    )
}

fn normalized_time_ranges(schedule: &PolicyScheduleWindow) -> Vec<(u16, u16)> {
    let start = parse_clock_minutes(&schedule.starts_at).unwrap_or(0);
    let end = parse_clock_minutes(&schedule.ends_at).unwrap_or(start);

    if start == end {
        return vec![(0, 24 * 60)];
    }

    if start < end {
        return vec![(start, end)];
    }

    vec![(start, 24 * 60), (0, end)]
}

fn parse_clock_minutes(value: &str) -> Option<u16> {
    let (hours, minutes) = value.split_once(':')?;
    let hours = hours.parse::<u16>().ok()?;
    let minutes = minutes.parse::<u16>().ok()?;
    if hours > 23 || minutes > 59 {
        return None;
    }
    Some(hours * 60 + minutes)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DstTransitionKind {
    SpringForward,
    FallBack,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct UtcDate {
    year: i32,
    month: u8,
    day: u8,
}

fn parse_utc_date(field: &'static str, value: &str) -> Result<UtcDate, EventingError> {
    let year = value[0..4]
        .parse::<i32>()
        .map_err(|_| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })?;
    let month = value[5..7]
        .parse::<u8>()
        .map_err(|_| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })?;
    let day = value[8..10]
        .parse::<u8>()
        .map_err(|_| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })?;

    Ok(UtcDate { year, month, day })
}

fn day_of_week(year: i32, month: u8, day: u8) -> u8 {
    let offsets = [0_i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut year = year;
    if month < 3 {
        year -= 1;
    }
    ((year + (year / 4) - (year / 100)
        + (year / 400)
        + offsets[(month - 1) as usize]
        + i32::from(day))
        % 7) as u8
}
