#![forbid(unsafe_code)]
#![allow(clippy::expect_used)]

use std::collections::BTreeMap;

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::SchemaVersion;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use crate::policy_authority::PolicyManualReviewState;
use crate::policy_source::{
    policy_status_name, validate_parent_policy_source_document, ParentPolicyDocumentId,
    ParentPolicyRule, ParentPolicySourceDocument, PolicyConsumerDomain, PolicyRuleId,
    PolicyRuleTarget, PolicyScheduleClockSource, PolicyScheduleId, PolicyScheduleWindow,
    PolicySourceDocumentStatus, PolicyTargetReferenceId, PolicyVersion,
};

const POLICY_PREVIEW_SCHEMA_VERSION_VALUE: u16 = 1;

pub type PolicyPreviewTargetState = ocentra_parent_agent_protocol::PolicyPreviewTargetState;
pub type PolicyPreviewFindingKind = ocentra_parent_agent_protocol::PolicyPreviewFindingKind;

macro_rules! policy_preview_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue { field: $field });
                }
                Ok(Self(value))
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

    let manual_review_state = if findings
        .iter()
        .any(|finding| finding_requires_manual_review(finding.kind))
    {
        PolicyManualReviewState::Required
    } else {
        PolicyManualReviewState::NotRequired
    };
    let save_state = if findings.is_empty() {
        if request.preview_acknowledged {
            PolicyPreviewSaveState::ReadyToSave
        } else {
            PolicyPreviewSaveState::PreviewRequired
        }
    } else {
        PolicyPreviewSaveState::Blocked
    };

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

fn assert_preview_candidate_status(
    status: PolicySourceDocumentStatus,
) -> Result<(), EventingError> {
    if matches!(
        status,
        PolicySourceDocumentStatus::Draft | PolicySourceDocumentStatus::Preview
    ) {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::preview::FIELD_CANDIDATE_STATUS,
        value: policy_status_name(status).to_string(),
    })
}

fn assert_current_document_matches_household(
    candidate: &ParentPolicySourceDocument,
    current: Option<&ParentPolicySourceDocument>,
) -> Result<(), EventingError> {
    if let Some(current) = current {
        if current.household_id != candidate.household_id {
            return Err(EventingError::InvalidValue {
                field: policy_control::preview::FIELD_CURRENT_DOCUMENT_HOUSEHOLD_ID,
                value: current.household_id.as_str().to_string(),
            });
        }
    }

    Ok(())
}

fn collect_source_version_findings(
    candidate: &ParentPolicySourceDocument,
    current: Option<&ParentPolicySourceDocument>,
    findings: &mut Vec<PolicyPreviewFinding>,
) {
    if let Some(current) = current {
        if current.policy_version.value() > candidate.policy_version.value() {
            findings.push(PolicyPreviewFinding {
                kind: PolicyPreviewFindingKind::StaleSourceDocument,
                target_reference_id: None,
                rule_ids: Vec::new(),
                schedule_ids: Vec::new(),
                explanation_code: preview_explanation_code(
                    policy_control::preview::EXPLANATION_STALE_POLICY_VERSION,
                ),
            });
        }
    }
}

fn collect_target_findings(
    target_inputs: &[PolicyPreviewTargetInput],
    findings: &mut Vec<PolicyPreviewFinding>,
) {
    for target in target_inputs {
        let Some(kind) = target_state_finding_kind(target.state) else {
            continue;
        };

        findings.push(PolicyPreviewFinding {
            kind,
            target_reference_id: Some(target.target.reference_id.clone()),
            rule_ids: Vec::new(),
            schedule_ids: Vec::new(),
            explanation_code: target.explanation_code.clone(),
        });
    }
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
    let mut findings = Vec::new();

    for rule in &enabled_rules {
        let Some(schedule_id) = &rule.schedule_id else {
            continue;
        };
        let schedule = schedule_lookup.get(schedule_id).copied().ok_or_else(|| {
            EventingError::InvalidValue {
                field: policy_control::preview::FIELD_SCHEDULE_ID,
                value: schedule_id.as_str().to_string(),
            }
        })?;
        let Some(kind) = schedule_manual_review_finding_kind(schedule)? else {
            continue;
        };

        findings.push(PolicyPreviewFinding {
            kind,
            target_reference_id: Some(rule.target.reference_id.clone()),
            rule_ids: vec![rule.rule_id.clone()],
            schedule_ids: vec![schedule_id.clone()],
            explanation_code: preview_explanation_code(preview_conflict_explanation_code(kind)),
        });
    }

    for (left_index, left_rule) in enabled_rules.iter().enumerate() {
        for right_rule in enabled_rules.iter().skip(left_index + 1) {
            if left_rule.target != right_rule.target || left_rule.action == right_rule.action {
                continue;
            }

            let Some(kind) = rule_conflict_kind(left_rule, right_rule, &schedule_lookup)? else {
                continue;
            };

            findings.push(PolicyPreviewFinding {
                kind,
                target_reference_id: Some(left_rule.target.reference_id.clone()),
                rule_ids: vec![left_rule.rule_id.clone(), right_rule.rule_id.clone()],
                schedule_ids: conflict_schedule_ids(left_rule, right_rule),
                explanation_code: preview_explanation_code(preview_conflict_explanation_code(kind)),
            });
        }
    }

    Ok(findings)
}

fn schedule_manual_review_finding_kind(
    schedule: &PolicyScheduleWindow,
) -> Result<Option<PolicyPreviewFindingKind>, EventingError> {
    if schedule_has_nonexistent_local_time(schedule)? {
        return Ok(Some(PolicyPreviewFindingKind::NonexistentLocalTime));
    }
    if schedule_has_ambiguous_local_time(schedule)? {
        return Ok(Some(PolicyPreviewFindingKind::AmbiguousLocalTime));
    }
    if matches!(
        schedule.time_budget.clock_source,
        PolicyScheduleClockSource::ManualRequired
    ) {
        return Ok(Some(PolicyPreviewFindingKind::ClockSkew));
    }
    Ok(None)
}

fn rule_conflict_kind(
    left_rule: &ParentPolicyRule,
    right_rule: &ParentPolicyRule,
    schedule_lookup: &BTreeMap<PolicyScheduleId, &PolicyScheduleWindow>,
) -> Result<Option<PolicyPreviewFindingKind>, EventingError> {
    match (&left_rule.schedule_id, &right_rule.schedule_id) {
        (None, None) | (None, Some(_)) | (Some(_), None) => {
            Ok(Some(PolicyPreviewFindingKind::OverlappingSchedule))
        }
        (Some(left_schedule_id), Some(right_schedule_id)) => {
            let left_schedule =
                schedule_lookup
                    .get(left_schedule_id)
                    .copied()
                    .ok_or_else(|| EventingError::InvalidValue {
                        field: policy_control::preview::FIELD_SCHEDULE_ID,
                        value: left_schedule_id.as_str().to_string(),
                    })?;
            let right_schedule =
                schedule_lookup
                    .get(right_schedule_id)
                    .copied()
                    .ok_or_else(|| EventingError::InvalidValue {
                        field: policy_control::preview::FIELD_SCHEDULE_ID,
                        value: right_schedule_id.as_str().to_string(),
                    })?;

            if left_schedule.timezone_name != right_schedule.timezone_name {
                return Ok(Some(PolicyPreviewFindingKind::TimezoneBoundary));
            }

            if schedule_windows_overlap(left_schedule, right_schedule)? {
                Ok(Some(PolicyPreviewFindingKind::OverlappingSchedule))
            } else {
                Ok(None)
            }
        }
    }
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
    [
        &schedule.starts_at,
        &schedule.ends_at,
        &schedule.time_budget.reset.local_time,
    ]
    .into_iter()
    .try_fold(false, |found, value| {
        if found {
            return Ok(true);
        }
        let minutes = parse_clock_time(value)?;
        Ok(match transition {
            DstTransitionKind::SpringForward => (120..180).contains(&minutes),
            DstTransitionKind::FallBack => (60..120).contains(&minutes),
        })
    })
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

fn expand_schedule_ranges(
    schedule: &PolicyScheduleWindow,
) -> Result<Vec<(u16, u16)>, EventingError> {
    let start_minutes = parse_clock_time(&schedule.starts_at)?;
    let end_minutes = parse_clock_time(&schedule.ends_at)?;

    if start_minutes == end_minutes {
        return Ok(vec![(0, 24 * 60)]);
    }

    if start_minutes < end_minutes {
        return Ok(vec![(start_minutes, end_minutes)]);
    }

    Ok(vec![(start_minutes, 24 * 60), (0, end_minutes)])
}

fn parse_clock_time(value: &str) -> Result<u16, EventingError> {
    let Some((hours, minutes)) = value.split_once(':') else {
        return Err(EventingError::InvalidValue {
            field: policy_control::preview::FIELD_SCHEDULE_TIME,
            value: value.to_string(),
        });
    };

    let hours = hours
        .parse::<u16>()
        .map_err(|_error| EventingError::InvalidValue {
            field: policy_control::preview::FIELD_SCHEDULE_TIME,
            value: value.to_string(),
        })?;
    let minutes = minutes
        .parse::<u16>()
        .map_err(|_error| EventingError::InvalidValue {
            field: policy_control::preview::FIELD_SCHEDULE_TIME,
            value: value.to_string(),
        })?;

    if hours > 23 || minutes > 59 {
        return Err(EventingError::InvalidValue {
            field: policy_control::preview::FIELD_SCHEDULE_TIME,
            value: value.to_string(),
        });
    }

    Ok((hours * 60) + minutes)
}

fn target_state_finding_kind(state: PolicyPreviewTargetState) -> Option<PolicyPreviewFindingKind> {
    match state {
        PolicyPreviewTargetState::Supported => None,
        PolicyPreviewTargetState::Unsupported => Some(PolicyPreviewFindingKind::UnsupportedTarget),
        PolicyPreviewTargetState::ManualRequired => {
            Some(PolicyPreviewFindingKind::ManualRequiredTarget)
        }
        PolicyPreviewTargetState::Offline => Some(PolicyPreviewFindingKind::OfflineTarget),
        PolicyPreviewTargetState::Stale => Some(PolicyPreviewFindingKind::StaleTarget),
    }
}

fn finding_requires_manual_review(kind: PolicyPreviewFindingKind) -> bool {
    matches!(
        kind,
        PolicyPreviewFindingKind::OverlappingSchedule
            | PolicyPreviewFindingKind::TimezoneBoundary
            | PolicyPreviewFindingKind::AmbiguousLocalTime
            | PolicyPreviewFindingKind::NonexistentLocalTime
            | PolicyPreviewFindingKind::ClockSkew
            | PolicyPreviewFindingKind::ManualRequiredTarget
            | PolicyPreviewFindingKind::OfflineTarget
            | PolicyPreviewFindingKind::StaleTarget
            | PolicyPreviewFindingKind::StaleSourceDocument
    )
}

fn conflict_schedule_ids(
    left_rule: &ParentPolicyRule,
    right_rule: &ParentPolicyRule,
) -> Vec<PolicyScheduleId> {
    let mut schedule_ids = Vec::new();

    if let Some(schedule_id) = &left_rule.schedule_id {
        schedule_ids.push(schedule_id.clone());
    }
    if let Some(schedule_id) = &right_rule.schedule_id {
        if !schedule_ids.contains(schedule_id) {
            schedule_ids.push(schedule_id.clone());
        }
    }

    schedule_ids
}

fn preview_explanation_code(value: &str) -> PolicyPreviewExplanationCode {
    PolicyPreviewExplanationCode::parse(value)
        .expect(policy_control::preview::ERROR_STATIC_EXPLANATION_CODE)
}

fn preview_conflict_explanation_code(kind: PolicyPreviewFindingKind) -> &'static str {
    match kind {
        PolicyPreviewFindingKind::AmbiguousLocalTime => {
            policy_control::preview::EXPLANATION_AMBIGUOUS_LOCAL_TIME
        }
        PolicyPreviewFindingKind::NonexistentLocalTime => {
            policy_control::preview::EXPLANATION_NONEXISTENT_LOCAL_TIME
        }
        PolicyPreviewFindingKind::ClockSkew => policy_control::preview::EXPLANATION_CLOCK_SKEW,
        PolicyPreviewFindingKind::TimezoneBoundary => {
            policy_control::preview::EXPLANATION_SCHEDULE_TIMEZONE_BOUNDARY
        }
        PolicyPreviewFindingKind::OverlappingSchedule => {
            policy_control::preview::EXPLANATION_OVERLAPPING_SCHEDULE
        }
        _ => policy_control::preview::EXPLANATION_UNSUPPORTED_TARGET,
    }
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
