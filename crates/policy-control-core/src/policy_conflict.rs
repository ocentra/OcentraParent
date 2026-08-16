#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

use crate::policy_source::{
    validate_parent_policy_source_document, ParentPolicyDocumentId, ParentPolicySourceDocument,
    PolicyAuditReferenceId, PolicyReasonCode, PolicyRollbackRef, PolicyRuleId, PolicyRuleTarget,
    PolicyScheduleClockSource, PolicyScheduleId, PolicyScheduleWindow, PolicyTargetKind,
    PolicyVersion,
};

mod detection;
mod manual_review;
mod overlap;
mod reasons;
mod schedules;

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
    detection::detect_policy_conflicts(source)
}

fn schedule_manual_review_conflict_kind(
    schedule: &PolicyScheduleWindow,
) -> Result<Option<PolicyConflictKind>, EventingError> {
    manual_review::schedule_manual_review_conflict_kind(schedule)
}

fn policy_conflict_reason(kind: PolicyConflictKind) -> &'static str {
    reasons::policy_conflict_reason(kind)
}

pub fn has_blocking_policy_conflicts(conflicts: &[PolicyConflictRecord]) -> bool {
    detection::has_blocking_policy_conflicts(conflicts)
}

fn collect_schedule_ids(
    left: Option<&PolicyScheduleId>,
    right: Option<&PolicyScheduleId>,
) -> Vec<PolicyScheduleId> {
    overlap::collect_schedule_ids(left, right)
}

fn rule_conflict_kind(
    left: &crate::policy_source::ParentPolicyRule,
    right: &crate::policy_source::ParentPolicyRule,
    schedule_map: &BTreeMap<PolicyScheduleId, &PolicyScheduleWindow>,
) -> Option<PolicyConflictKind> {
    overlap::rule_conflict_kind(left, right, schedule_map)
}

fn normalized_time_ranges(schedule: &PolicyScheduleWindow) -> Vec<(u16, u16)> {
    schedules::normalized_time_ranges(schedule)
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
