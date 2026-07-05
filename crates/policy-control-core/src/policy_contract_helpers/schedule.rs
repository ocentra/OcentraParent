#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::authority::PolicyContractApprovalResolution;
use super::PolicyContractValidationResult;

mod boundary;
mod resolution;
mod time;
mod time_budget;
mod validation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractScheduleBoundaryState {
    #[serde(rename = "within-window")]
    WithinWindow,
    #[serde(rename = "outside-window")]
    OutsideWindow,
    #[serde(rename = "dst-gap")]
    DstGap,
    #[serde(rename = "dst-overlap")]
    DstOverlap,
    #[serde(rename = "clock-skew")]
    ClockSkew,
    #[serde(rename = "exception-active")]
    ExceptionActive,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractScheduleDstTransition {
    #[serde(rename = "spring-forward")]
    SpringForward,
    #[serde(rename = "fall-back")]
    FallBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractScheduleDstResolution {
    #[serde(rename = "skip-forward")]
    SkipForward,
    #[serde(rename = "first-occurrence")]
    FirstOccurrence,
    #[serde(rename = "second-occurrence")]
    SecondOccurrence,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractScheduleClockSource {
    #[serde(rename = "child-device")]
    ChildDevice,
    #[serde(rename = "trusted-service")]
    TrustedService,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractScheduleOfflineRecoveryState {
    #[serde(rename = "not-needed")]
    NotNeeded,
    #[serde(rename = "recovered-from-device")]
    RecoveredFromDevice,
    #[serde(rename = "recomputed-from-journal")]
    RecomputedFromJournal,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractScheduleBudgetResetKind {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractScheduleBudgetCarryoverMode {
    #[serde(rename = "discard-unused")]
    DiscardUnused,
    #[serde(rename = "carry-forward")]
    CarryForward,
    #[serde(rename = "cap-carryover")]
    CapCarryover,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub struct PolicyContractScheduleWindow {
    pub day_count: usize,
    pub start_local_time: String,
    pub end_local_time: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleBudgetReset {
    pub kind: PolicyContractScheduleBudgetResetKind,
    pub local_time: String,
    pub day: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleBudgetCarryover {
    pub mode: PolicyContractScheduleBudgetCarryoverMode,
    pub max_minutes: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleTimeBudget {
    pub budget_window_minutes: u16,
    pub grace_period_minutes: u16,
    pub reset: PolicyContractScheduleBudgetReset,
    pub effective_from: String,
    pub effective_until: Option<String>,
    pub carryover: PolicyContractScheduleBudgetCarryover,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractSchedule {
    pub windows: Vec<PolicyContractScheduleWindow>,
    pub time_budget: PolicyContractScheduleTimeBudget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleDstBoundary {
    pub transition: PolicyContractScheduleDstTransition,
    pub local_time: String,
    pub offset_before_minutes: i32,
    pub offset_after_minutes: i32,
    pub resolution: PolicyContractScheduleDstResolution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleClockSkew {
    pub observed_at: String,
    pub allowed_skew_minutes: i32,
    pub observed_skew_minutes: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleException {
    pub starts_at: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleExpiry {
    pub expires_at: String,
    pub expired_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleOfflineRecoveryStatus {
    pub state: PolicyContractScheduleOfflineRecoveryState,
    pub recovered_at: Option<String>,
    pub recovered_offline_minutes: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleTimeBudgetStatus {
    pub budget_window_minutes: u16,
    pub used_minutes: u16,
    pub remaining_minutes: u16,
    pub carryover_minutes: u16,
    pub grace_period_minutes: u16,
    pub reset_at: String,
    pub clock_source: PolicyContractScheduleClockSource,
    pub offline_recovery: PolicyContractScheduleOfflineRecoveryStatus,
    pub bonus_time_minutes: Option<u16>,
    pub bonus_time_remaining_minutes: Option<u16>,
    pub bonus_time_expires_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractScheduleBoundary {
    pub evaluated_at: String,
    pub local_time: String,
    pub state: PolicyContractScheduleBoundaryState,
    pub dst_boundary: Option<PolicyContractScheduleDstBoundary>,
    pub clock_skew: Option<PolicyContractScheduleClockSkew>,
    pub exception: Option<PolicyContractScheduleException>,
    pub expiry: Option<PolicyContractScheduleExpiry>,
    pub time_budget: Option<PolicyContractScheduleTimeBudgetStatus>,
}

pub fn validate_policy_schedule(
    schedule: &PolicyContractSchedule,
) -> PolicyContractValidationResult {
    validation::validate_policy_schedule(schedule)
}

pub fn validate_policy_schedule_boundary(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    boundary::validate_policy_schedule_boundary(boundary)
}

pub(crate) fn assert_resolution_has_no_review_or_override_artifacts(
    resolution: &PolicyContractApprovalResolution,
    message: &'static str,
) -> PolicyContractValidationResult {
    resolution::assert_resolution_has_no_review_or_override_artifacts(resolution, message)
}

pub(crate) fn assert_resolution_has_no_review_override_or_replay_artifacts(
    resolution: &PolicyContractApprovalResolution,
    message: &'static str,
) -> PolicyContractValidationResult {
    resolution::assert_resolution_has_no_review_override_or_replay_artifacts(resolution, message)
}

fn assert_local_time(value: &str, field_name: &'static str) -> PolicyContractValidationResult {
    time::assert_local_time(value, field_name)
}

pub(crate) fn assert_utc_timestamp(
    value: &str,
    field_name: &'static str,
) -> PolicyContractValidationResult {
    time::assert_utc_timestamp(value, field_name)
}
