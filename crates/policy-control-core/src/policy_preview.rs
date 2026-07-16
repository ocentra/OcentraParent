#![forbid(unsafe_code)]
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyPreviewFindingKind, PolicyPreviewTargetState, PolicySourceStatus,
};
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use crate::policy_authority::PolicyManualReviewState;
use crate::policy_source::{
    policy_status_name, validate_parent_policy_source_document, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyConsumerDomain, PolicyRuleId,
    PolicyRuleTarget, PolicyScheduleId, PolicyScheduleWindow, PolicyTargetReferenceId,
    PolicyVersion,
};

const POLICY_PREVIEW_SCHEMA_VERSION_VALUE: u16 = 1;
const SUPPORTED_WP07_DST_TIMEZONES: [&str; 4] = [
    "America/Toronto",
    "America/Vancouver",
    "America/Los_Angeles",
    "America/Winnipeg",
];
const POLICY_PREVIEW_SAVE_STATE_MATRIX: [[PolicyPreviewSaveState; 2]; 2] = [
    [
        PolicyPreviewSaveState::Blocked,
        PolicyPreviewSaveState::Blocked,
    ],
    [
        PolicyPreviewSaveState::PreviewRequired,
        PolicyPreviewSaveState::ReadyToSave,
    ],
];
const MANUAL_REVIEW_FINDING_KINDS: [PolicyPreviewFindingKind; 9] = [
    PolicyPreviewFindingKind::OverlappingSchedule,
    PolicyPreviewFindingKind::TimezoneBoundary,
    PolicyPreviewFindingKind::AmbiguousLocalTime,
    PolicyPreviewFindingKind::NonexistentLocalTime,
    PolicyPreviewFindingKind::ClockSkew,
    PolicyPreviewFindingKind::ManualRequiredTarget,
    PolicyPreviewFindingKind::OfflineTarget,
    PolicyPreviewFindingKind::StaleTarget,
    PolicyPreviewFindingKind::StaleSourceDocument,
];
const ALLOWED_PREVIEW_SOURCE_STATUSES: [PolicySourceStatus; 2] =
    [PolicySourceStatus::Draft, PolicySourceStatus::Preview];
const TARGET_STATE_FINDINGS: [Option<PolicyPreviewFindingKind>; 5] = [
    None,
    Some(PolicyPreviewFindingKind::UnsupportedTarget),
    Some(PolicyPreviewFindingKind::ManualRequiredTarget),
    Some(PolicyPreviewFindingKind::OfflineTarget),
    Some(PolicyPreviewFindingKind::StaleTarget),
];
const CLOCK_SOURCE_FINDINGS: [Option<PolicyPreviewFindingKind>; 3] =
    [None, None, Some(PolicyPreviewFindingKind::ClockSkew)];
const FINDING_EXPLANATION_CODES: [&str; 10] = [
    policy_control::preview::EXPLANATION_OVERLAPPING_SCHEDULE,
    policy_control::preview::EXPLANATION_SCHEDULE_TIMEZONE_BOUNDARY,
    policy_control::preview::EXPLANATION_AMBIGUOUS_LOCAL_TIME,
    policy_control::preview::EXPLANATION_NONEXISTENT_LOCAL_TIME,
    policy_control::preview::EXPLANATION_CLOCK_SKEW,
    policy_control::preview::EXPLANATION_UNSUPPORTED_TARGET,
    policy_control::preview::EXPLANATION_UNSUPPORTED_TARGET,
    policy_control::preview::EXPLANATION_UNSUPPORTED_TARGET,
    policy_control::preview::EXPLANATION_UNSUPPORTED_TARGET,
    policy_control::preview::EXPLANATION_UNSUPPORTED_TARGET,
];
const DST_TRANSITION_MINUTE_RANGES: [(u16, u16); 2] = [(120, 180), (60, 120)];
const DST_TRANSITION_MONTHS: [u8; 2] = [3, 11];
const DST_TRANSITION_DAY_RANGES: [(u8, u8); 2] = [(8, 14), (1, 7)];

macro_rules! policy_preview_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                parse_non_empty_text_id(value, $field).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

policy_preview_text_id!(
    PolicyPreviewRequestId,
    policy_control::preview::FIELD_REQUEST_ID
);
policy_preview_text_id!(
    PolicyPreviewExplanationCode,
    policy_control::preview::FIELD_EXPLANATION_CODE
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyPreviewSaveState {
    #[serde(rename = "preview-required")]
    PreviewRequired,
    #[serde(rename = "ready-to-save")]
    ReadyToSave,
    #[serde(rename = "blocked")]
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPreviewTargetInput {
    pub target: PolicyRuleTarget,
    pub domain: PolicyConsumerDomain,
    pub state: PolicyPreviewTargetState,
    pub explanation_code: PolicyPreviewExplanationCode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPreviewTargetResult {
    pub target: PolicyRuleTarget,
    pub domain: PolicyConsumerDomain,
    pub state: PolicyPreviewTargetState,
    pub explanation_code: PolicyPreviewExplanationCode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPreviewFinding {
    pub kind: PolicyPreviewFindingKind,
    pub target_reference_id: Option<PolicyTargetReferenceId>,
    pub rule_ids: Vec<PolicyRuleId>,
    pub schedule_ids: Vec<PolicyScheduleId>,
    pub explanation_code: PolicyPreviewExplanationCode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPreviewRequest {
    pub schema_version: SchemaVersion,
    pub request_id: PolicyPreviewRequestId,
    pub candidate_document: ParentPolicySourceDocument,
    pub current_document: Option<ParentPolicySourceDocument>,
    pub preview_acknowledged: bool,
    pub target_inputs: Vec<PolicyPreviewTargetInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyPreviewResult {
    pub schema_version: SchemaVersion,
    pub request_id: PolicyPreviewRequestId,
    pub source_document_id: ParentPolicyDocumentId,
    pub policy_version: PolicyVersion,
    pub save_state: PolicyPreviewSaveState,
    pub manual_review_state: PolicyManualReviewState,
    pub findings: Vec<PolicyPreviewFinding>,
    pub target_results: Vec<PolicyPreviewTargetResult>,
}

pub fn policy_preview_schema_version() -> Result<SchemaVersion, EventingError> {
    SchemaVersion::new(POLICY_PREVIEW_SCHEMA_VERSION_VALUE)
}

pub fn preview_parent_policy_before_save(
    request: &PolicyPreviewRequest,
) -> Result<PolicyPreviewResult, EventingError> {
    validate_parent_policy_source_document(&request.candidate_document)?;
    assert_preview_candidate_status(request.candidate_document.status)?;
    assert_current_document_matches_household(
        &request.candidate_document,
        request.current_document.as_ref(),
    )?;

    let mut findings = collect_schedule_findings(&request.candidate_document)?;
    collect_source_version_findings(
        &request.candidate_document,
        request.current_document.as_ref(),
        &mut findings,
    );
    collect_target_findings(&request.target_inputs, &mut findings);

    let has_findings = !findings.is_empty();
    let manual_review_state = [
        PolicyManualReviewState::NotRequired,
        PolicyManualReviewState::Required,
    ][usize::from(
        findings
            .iter()
            .any(|finding| finding_requires_manual_review(finding.kind)),
    )];
    let save_state = POLICY_PREVIEW_SAVE_STATE_MATRIX[usize::from(!has_findings)]
        [usize::from(request.preview_acknowledged)];

    Ok(PolicyPreviewResult {
        schema_version: policy_preview_schema_version()?,
        request_id: request.request_id.clone(),
        source_document_id: request.candidate_document.document_id.clone(),
        policy_version: request.candidate_document.policy_version,
        save_state,
        manual_review_state,
        findings,
        target_results: request
            .target_inputs
            .iter()
            .cloned()
            .map(PolicyPreviewTargetResult::from)
            .collect(),
    })
}

impl From<PolicyPreviewTargetInput> for PolicyPreviewTargetResult {
    fn from(value: PolicyPreviewTargetInput) -> Self {
        Self {
            target: value.target,
            domain: value.domain,
            state: value.state,
            explanation_code: value.explanation_code,
        }
    }
}

fn assert_preview_candidate_status(status: PolicySourceStatus) -> Result<(), EventingError> {
    ALLOWED_PREVIEW_SOURCE_STATUSES
        .contains(&status)
        .then_some(())
        .ok_or_else(|| EventingError::InvalidValue {
            field: policy_control::preview::FIELD_CANDIDATE_STATUS,
            value: policy_status_name(status).to_string(),
        })
}

fn assert_current_document_matches_household(
    candidate: &ParentPolicySourceDocument,
    current: Option<&ParentPolicySourceDocument>,
) -> Result<(), EventingError> {
    current
        .filter(|current| current.household_id != candidate.household_id)
        .map_or(Ok(()), |current| {
            Err(EventingError::InvalidValue {
                field: policy_control::preview::FIELD_CURRENT_DOCUMENT_HOUSEHOLD_ID,
                value: current.household_id.as_str().to_string(),
            })
        })
}

fn collect_source_version_findings(
    candidate: &ParentPolicySourceDocument,
    current: Option<&ParentPolicySourceDocument>,
    findings: &mut Vec<PolicyPreviewFinding>,
) {
    findings.extend(
        current
            .filter(|current| current.policy_version.value() > candidate.policy_version.value())
            .map(|_| PolicyPreviewFinding {
                kind: PolicyPreviewFindingKind::StaleSourceDocument,
                target_reference_id: None,
                rule_ids: Vec::new(),
                schedule_ids: Vec::new(),
                explanation_code: preview_explanation_code(
                    policy_control::preview::EXPLANATION_STALE_POLICY_VERSION,
                ),
            }),
    );
}

fn collect_target_findings(
    target_inputs: &[PolicyPreviewTargetInput],
    findings: &mut Vec<PolicyPreviewFinding>,
) {
    findings.extend(target_inputs.iter().filter_map(|target| {
        target_state_finding_kind(target.state).map(|kind| PolicyPreviewFinding {
            kind,
            target_reference_id: Some(target.target.reference_id.clone()),
            rule_ids: Vec::new(),
            schedule_ids: Vec::new(),
            explanation_code: target.explanation_code.clone(),
        })
    }));
}

fn collect_schedule_findings(
    document: &ParentPolicySourceDocument,
) -> Result<Vec<PolicyPreviewFinding>, EventingError> {
    let schedule_lookup = document
        .schedules
        .iter()
        .map(|schedule| (schedule.schedule_id.clone(), schedule))
        .collect::<BTreeMap<_, _>>();
    let enabled_rules = document
        .rules
        .iter()
        .filter(|rule| rule.enabled)
        .collect::<Vec<_>>();
    let mut findings = schedule_rule_findings(&enabled_rules, &schedule_lookup)?;
    findings.extend(schedule_conflict_findings(
        &enabled_rules,
        &schedule_lookup,
    )?);
    Ok(findings)
}

fn schedule_rule_findings(
    enabled_rules: &[&ParentPolicyRule],
    schedule_lookup: &BTreeMap<PolicyScheduleId, &PolicyScheduleWindow>,
) -> Result<Vec<PolicyPreviewFinding>, EventingError> {
    enabled_rules
        .iter()
        .filter_map(|rule| {
            rule.schedule_id
                .as_ref()
                .map(|schedule_id| (rule, schedule_id))
        })
        .map(|(rule, schedule_id)| {
            schedule_lookup
                .get(schedule_id)
                .copied()
                .ok_or_else(|| EventingError::InvalidValue {
                    field: policy_control::preview::FIELD_SCHEDULE_ID,
                    value: schedule_id.as_str().to_string(),
                })
                .and_then(schedule_manual_review_finding_kind)
                .map(|kind| {
                    kind.map(|kind| PolicyPreviewFinding {
                        kind,
                        target_reference_id: Some(rule.target.reference_id.clone()),
                        rule_ids: vec![rule.rule_id.clone()],
                        schedule_ids: vec![schedule_id.clone()],
                        explanation_code: preview_explanation_code(
                            preview_conflict_explanation_code(kind),
                        ),
                    })
                })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|findings| findings.into_iter().flatten().collect())
}

fn schedule_conflict_findings(
    enabled_rules: &[&ParentPolicyRule],
    schedule_lookup: &BTreeMap<PolicyScheduleId, &PolicyScheduleWindow>,
) -> Result<Vec<PolicyPreviewFinding>, EventingError> {
    enabled_rules
        .iter()
        .enumerate()
        .flat_map(|(left_index, left_rule)| {
            enabled_rules
                .iter()
                .skip(left_index + 1)
                .map(move |right_rule| (left_rule, right_rule))
        })
        .filter(|(left_rule, right_rule)| {
            left_rule.target == right_rule.target && left_rule.action != right_rule.action
        })
        .map(|(left_rule, right_rule)| {
            rule_conflict_kind(left_rule, right_rule, schedule_lookup).map(|kind| {
                kind.map(|kind| PolicyPreviewFinding {
                    kind,
                    target_reference_id: Some(left_rule.target.reference_id.clone()),
                    rule_ids: vec![left_rule.rule_id.clone(), right_rule.rule_id.clone()],
                    schedule_ids: conflict_schedule_ids(left_rule, right_rule),
                    explanation_code: preview_explanation_code(preview_conflict_explanation_code(
                        kind,
                    )),
                })
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|findings| findings.into_iter().flatten().collect())
}

fn schedule_manual_review_finding_kind(
    schedule: &PolicyScheduleWindow,
) -> Result<Option<PolicyPreviewFindingKind>, EventingError> {
    [
        schedule_has_nonexistent_local_time(schedule)
            .map(|applies| applies.then_some(PolicyPreviewFindingKind::NonexistentLocalTime)),
        schedule_has_ambiguous_local_time(schedule)
            .map(|applies| applies.then_some(PolicyPreviewFindingKind::AmbiguousLocalTime)),
        Ok(CLOCK_SOURCE_FINDINGS[schedule.time_budget.clock_source as usize]),
    ]
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .map(|kinds| kinds.into_iter().flatten().next())
}

fn rule_conflict_kind(
    left_rule: &ParentPolicyRule,
    right_rule: &ParentPolicyRule,
    schedule_lookup: &BTreeMap<PolicyScheduleId, &PolicyScheduleWindow>,
) -> Result<Option<PolicyPreviewFindingKind>, EventingError> {
    let paired_schedule_ids = left_rule
        .schedule_id
        .as_ref()
        .zip(right_rule.schedule_id.as_ref());

    paired_schedule_ids
        .map(|(left_schedule_id, right_schedule_id)| {
            schedule_lookup
                .get(left_schedule_id)
                .copied()
                .ok_or_else(|| EventingError::InvalidValue {
                    field: policy_control::preview::FIELD_SCHEDULE_ID,
                    value: left_schedule_id.as_str().to_string(),
                })
                .and_then(|left_schedule| {
                    schedule_lookup
                        .get(right_schedule_id)
                        .copied()
                        .ok_or_else(|| EventingError::InvalidValue {
                            field: policy_control::preview::FIELD_SCHEDULE_ID,
                            value: right_schedule_id.as_str().to_string(),
                        })
                        .and_then(|right_schedule| {
                            Ok(
                                (left_schedule.timezone_name != right_schedule.timezone_name)
                                    .then_some(PolicyPreviewFindingKind::TimezoneBoundary)
                                    .or(schedule_windows_overlap(left_schedule, right_schedule)?
                                        .then_some(PolicyPreviewFindingKind::OverlappingSchedule)),
                            )
                        })
                })
        })
        .transpose()
        .map(|kind| {
            kind.flatten().or_else(|| {
                paired_schedule_ids
                    .is_none()
                    .then_some(PolicyPreviewFindingKind::OverlappingSchedule)
            })
        })
}

fn schedule_windows_overlap(
    left_schedule: &PolicyScheduleWindow,
    right_schedule: &PolicyScheduleWindow,
) -> Result<bool, EventingError> {
    let left_ranges = expand_schedule_ranges(left_schedule)?;
    let right_ranges = expand_schedule_ranges(right_schedule)?;

    Ok(left_ranges.iter().any(|(left_start, left_end)| {
        right_ranges
            .iter()
            .any(|(right_start, right_end)| left_start < right_end && right_start < left_end)
    }))
}

fn schedule_has_nonexistent_local_time(
    schedule: &PolicyScheduleWindow,
) -> Result<bool, EventingError> {
    Ok(schedule_uses_supported_wp07_dst_timezone(schedule)
        && schedule_on_single_transition_day(schedule, DstTransitionKind::SpringForward)?
        && schedule_has_transition_local_time(schedule, DstTransitionKind::SpringForward)?)
}

fn schedule_has_ambiguous_local_time(
    schedule: &PolicyScheduleWindow,
) -> Result<bool, EventingError> {
    Ok(schedule_uses_supported_wp07_dst_timezone(schedule)
        && schedule_on_single_transition_day(schedule, DstTransitionKind::FallBack)?
        && schedule_has_transition_local_time(schedule, DstTransitionKind::FallBack)?)
}

fn schedule_has_transition_local_time(
    schedule: &PolicyScheduleWindow,
    transition: DstTransitionKind,
) -> Result<bool, EventingError> {
    let (range_start, range_end) = DST_TRANSITION_MINUTE_RANGES[transition as usize];
    [
        &schedule.starts_at,
        &schedule.ends_at,
        &schedule.time_budget.reset.local_time,
    ]
    .into_iter()
    .map(|value| parse_clock_time(value))
    .try_fold(false, |found, minutes| {
        let minutes = minutes?;
        Ok(found || (range_start..range_end).contains(&minutes))
    })
}

fn schedule_on_single_transition_day(
    schedule: &PolicyScheduleWindow,
    transition: DstTransitionKind,
) -> Result<bool, EventingError> {
    let (transition_start_day, transition_end_day) = DST_TRANSITION_DAY_RANGES[transition as usize];
    let transition_month = DST_TRANSITION_MONTHS[transition as usize];

    schedule
        .time_budget
        .effective_until
        .as_deref()
        .map(|effective_until| {
            let effective_from = parse_utc_date(
                policy_control::source::FIELD_SCHEDULE_EFFECTIVE_FROM,
                &schedule.time_budget.effective_from,
            )?;
            let effective_until = parse_utc_date(
                policy_control::source::FIELD_SCHEDULE_EFFECTIVE_UNTIL,
                effective_until,
            )?;

            Ok(effective_from == effective_until
                && effective_from.month == transition_month
                && (transition_start_day..=transition_end_day).contains(&effective_from.day)
                && day_of_week(
                    effective_from.year,
                    effective_from.month,
                    effective_from.day,
                ) == 0)
        })
        .transpose()
        .map(|result| result.unwrap_or(false))
}

fn schedule_uses_supported_wp07_dst_timezone(schedule: &PolicyScheduleWindow) -> bool {
    SUPPORTED_WP07_DST_TIMEZONES.contains(&schedule.timezone_name.as_str())
}

fn expand_schedule_ranges(
    schedule: &PolicyScheduleWindow,
) -> Result<Vec<(u16, u16)>, EventingError> {
    let start_minutes = parse_clock_time(&schedule.starts_at)?;
    let end_minutes = parse_clock_time(&schedule.ends_at)?;
    let ordering = start_minutes.cmp(&end_minutes);

    Ok([
        (ordering == Ordering::Equal).then_some(vec![(0, 24 * 60)]),
        (ordering == Ordering::Less).then_some(vec![(start_minutes, end_minutes)]),
        (ordering == Ordering::Greater).then_some(vec![(start_minutes, 24 * 60), (0, end_minutes)]),
    ]
    .into_iter()
    .flatten()
    .next()
    .unwrap_or_default())
}

fn parse_clock_time(value: &str) -> Result<u16, EventingError> {
    let invalid_value = || EventingError::InvalidValue {
        field: policy_control::preview::FIELD_SCHEDULE_TIME,
        value: value.to_string(),
    };
    let (hours, minutes) = value.split_once(':').ok_or_else(invalid_value)?;
    let hours = hours.parse::<u16>().map_err(|_error| invalid_value())?;
    let minutes = minutes.parse::<u16>().map_err(|_error| invalid_value())?;

    (hours < 24 && minutes < 60)
        .then_some((hours * 60) + minutes)
        .ok_or_else(invalid_value)
}

fn target_state_finding_kind(state: PolicyPreviewTargetState) -> Option<PolicyPreviewFindingKind> {
    TARGET_STATE_FINDINGS[state as usize]
}

fn finding_requires_manual_review(kind: PolicyPreviewFindingKind) -> bool {
    MANUAL_REVIEW_FINDING_KINDS.contains(&kind)
}

fn conflict_schedule_ids(
    left_rule: &ParentPolicyRule,
    right_rule: &ParentPolicyRule,
) -> Vec<PolicyScheduleId> {
    [
        left_rule.schedule_id.as_ref(),
        right_rule.schedule_id.as_ref(),
    ]
    .into_iter()
    .flatten()
    .cloned()
    .collect::<BTreeSet<_>>()
    .into_iter()
    .collect()
}

fn preview_explanation_code(value: &str) -> PolicyPreviewExplanationCode {
    PolicyPreviewExplanationCode(value.to_string())
}

fn preview_conflict_explanation_code(kind: PolicyPreviewFindingKind) -> &'static str {
    FINDING_EXPLANATION_CODES[kind as usize]
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
        .map_err(|_error| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })?;
    let month = value[5..7]
        .parse::<u8>()
        .map_err(|_error| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })?;
    let day = value[8..10]
        .parse::<u8>()
        .map_err(|_error| EventingError::InvalidValue {
            field,
            value: value.to_string(),
        })?;

    Ok(UtcDate { year, month, day })
}

fn day_of_week(year: i32, month: u8, day: u8) -> u8 {
    let offsets = [0_i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = year - i32::from(month < 3);
    ((year + (year / 4) - (year / 100)
        + (year / 400)
        + offsets[(month - 1) as usize]
        + i32::from(day))
        % 7) as u8
}

fn parse_non_empty_text_id(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, EventingError> {
    let value = value.into();
    (!value.trim().is_empty())
        .then_some(value)
        .ok_or(EventingError::EmptyValue { field })
}
