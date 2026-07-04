#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::authority::PolicyContractApprovalResolution;
use super::PolicyContractValidationResult;

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
    if schedule.windows.is_empty() {
        return Err("schedules must define at least one local window".into());
    }

    for window in &schedule.windows {
        if window.day_count == 0 {
            return Err("schedules must define at least one day for every window".into());
        }
        assert_local_time(&window.start_local_time, "windows.startLocalTime")?;
        assert_local_time(&window.end_local_time, "windows.endLocalTime")?;
    }

    validate_policy_schedule_time_budget(&schedule.time_budget)
}

pub fn validate_policy_schedule_boundary(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    assert_utc_timestamp(&boundary.evaluated_at, "evaluatedAt")?;
    assert_local_time(&boundary.local_time, "localTime")?;
    validate_policy_schedule_boundary_optional_sections(boundary)?;

    if let Some(time_budget) = &boundary.time_budget {
        validate_policy_schedule_time_budget_status(time_budget, &boundary.evaluated_at)?;
    }

    match boundary.state {
        PolicyContractScheduleBoundaryState::DstGap => {
            let Some(dst_boundary) = &boundary.dst_boundary else {
                return Err("dst-gap boundaries require dstBoundary details".into());
            };
            if dst_boundary.transition != PolicyContractScheduleDstTransition::SpringForward {
                return Err("dst-gap boundaries must use the spring-forward transition".into());
            }
            if matches!(
                dst_boundary.resolution,
                PolicyContractScheduleDstResolution::FirstOccurrence
                    | PolicyContractScheduleDstResolution::SecondOccurrence
            ) {
                return Err("dst-gap boundaries cannot use overlap-only resolutions".into());
            }
        }
        PolicyContractScheduleBoundaryState::DstOverlap => {
            let Some(dst_boundary) = &boundary.dst_boundary else {
                return Err("dst-overlap boundaries require dstBoundary details".into());
            };
            if dst_boundary.transition != PolicyContractScheduleDstTransition::FallBack {
                return Err("dst-overlap boundaries must use the fall-back transition".into());
            }
            if dst_boundary.resolution == PolicyContractScheduleDstResolution::SkipForward {
                return Err("dst-overlap boundaries cannot skip the repeated hour".into());
            }
        }
        PolicyContractScheduleBoundaryState::ClockSkew => {
            let Some(clock_skew) = &boundary.clock_skew else {
                return Err("clock-skew boundaries require clockSkew details".into());
            };
            if clock_skew.observed_skew_minutes.abs() <= clock_skew.allowed_skew_minutes {
                return Err(
                    "clock-skew boundaries require skew beyond the allowed tolerance".into(),
                );
            }
        }
        PolicyContractScheduleBoundaryState::ExceptionActive => {
            let Some(exception) = &boundary.exception else {
                return Err("exception-active boundaries require exception details".into());
            };
            if !(boundary.evaluated_at >= exception.starts_at
                && boundary.evaluated_at < exception.expires_at)
            {
                return Err(
                    "exception-active boundaries must be evaluated inside the exception window"
                        .into(),
                );
            }
        }
        PolicyContractScheduleBoundaryState::Expired => {
            let Some(expiry) = &boundary.expiry else {
                return Err("expired schedule boundaries require expiry details".into());
            };
            if boundary.evaluated_at < expiry.expires_at {
                return Err(
                    "expired schedule boundaries must be evaluated on or after expiry".into(),
                );
            }
        }
        PolicyContractScheduleBoundaryState::WithinWindow
        | PolicyContractScheduleBoundaryState::OutsideWindow => {}
    }

    Ok(())
}

fn validate_policy_schedule_time_budget(
    time_budget: &PolicyContractScheduleTimeBudget,
) -> PolicyContractValidationResult {
    assert_local_time(&time_budget.reset.local_time, "timeBudget.reset.localTime")?;
    assert_utc_timestamp(&time_budget.effective_from, "timeBudget.effectiveFrom")?;

    if let Some(effective_until) = &time_budget.effective_until {
        assert_utc_timestamp(effective_until, "timeBudget.effectiveUntil")?;
        if effective_until <= &time_budget.effective_from {
            return Err("timeBudget.effectiveUntil must be after timeBudget.effectiveFrom".into());
        }
    }

    match time_budget.reset.kind {
        PolicyContractScheduleBudgetResetKind::Weekly => {
            if time_budget.reset.day.is_none() {
                return Err("weekly reset rules require timeBudget.reset.day".into());
            }
        }
        PolicyContractScheduleBudgetResetKind::Daily
        | PolicyContractScheduleBudgetResetKind::Monthly => {
            if time_budget.reset.day.is_some() {
                return Err("non-weekly reset rules cannot set timeBudget.reset.day".into());
            }
        }
    }

    match time_budget.carryover.mode {
        PolicyContractScheduleBudgetCarryoverMode::DiscardUnused => {
            if time_budget.carryover.max_minutes.is_some() {
                return Err(
                    "discard-unused carryover cannot set timeBudget.carryover.maxMinutes".into(),
                );
            }
        }
        PolicyContractScheduleBudgetCarryoverMode::CarryForward => {}
        PolicyContractScheduleBudgetCarryoverMode::CapCarryover => {
            if time_budget.carryover.max_minutes.unwrap_or(0) == 0 {
                return Err("cap-carryover requires timeBudget.carryover.maxMinutes".into());
            }
        }
    }

    Ok(())
}

fn validate_policy_schedule_time_budget_status(
    time_budget: &PolicyContractScheduleTimeBudgetStatus,
    evaluated_at: &str,
) -> PolicyContractValidationResult {
    if time_budget.budget_window_minutes == 0 {
        return Err("timeBudget.budgetWindowMinutes must be a positive number".into());
    }
    assert_utc_timestamp(&time_budget.reset_at, "timeBudget.resetAt")?;
    if time_budget.reset_at.as_str() <= evaluated_at {
        return Err("timeBudget.resetAt must be after evaluatedAt".into());
    }

    match time_budget.offline_recovery.state {
        PolicyContractScheduleOfflineRecoveryState::NotNeeded => {
            if time_budget.offline_recovery.recovered_at.is_some()
                || time_budget.offline_recovery.recovered_offline_minutes != 0
            {
                return Err(
                    "offline recovery state not-needed cannot include recovery artifacts".into(),
                );
            }
        }
        PolicyContractScheduleOfflineRecoveryState::RecoveredFromDevice
        | PolicyContractScheduleOfflineRecoveryState::RecomputedFromJournal => {
            let Some(recovered_at) = &time_budget.offline_recovery.recovered_at else {
                return Err("recovered offline timer states require recoveredAt".into());
            };
            assert_utc_timestamp(recovered_at, "offlineRecovery.recoveredAt")?;
        }
        PolicyContractScheduleOfflineRecoveryState::ManualRequired => {
            if let Some(recovered_at) = &time_budget.offline_recovery.recovered_at {
                assert_utc_timestamp(recovered_at, "offlineRecovery.recoveredAt")?;
            }
        }
    }

    if let Some(bonus_time_minutes) = time_budget.bonus_time_minutes {
        if bonus_time_minutes == 0 {
            return Err("timeBudget.bonusTimeMinutes must be a positive number".into());
        }
        let Some(bonus_time_remaining_minutes) = time_budget.bonus_time_remaining_minutes else {
            return Err(
                "timeBudget.bonusTimeRemainingMinutes is required when bonusTimeMinutes are active"
                    .into(),
            );
        };
        if bonus_time_remaining_minutes > bonus_time_minutes {
            return Err(
                "timeBudget.bonusTimeRemainingMinutes cannot exceed timeBudget.bonusTimeMinutes"
                    .into(),
            );
        }
        let Some(bonus_time_expires_at) = &time_budget.bonus_time_expires_at else {
            return Err(
                "timeBudget.bonusTimeExpiresAt is required when bonusTimeMinutes are active".into(),
            );
        };
        assert_utc_timestamp(bonus_time_expires_at, "timeBudget.bonusTimeExpiresAt")?;
        if bonus_time_expires_at.as_str() <= evaluated_at {
            return Err(
                "timeBudget.bonusTimeExpiresAt must be after evaluatedAt while bonus time is active"
                    .into(),
            );
        }
    } else if time_budget.bonus_time_remaining_minutes.is_some()
        || time_budget.bonus_time_expires_at.is_some()
    {
        return Err(
            "timeBudget.bonusTimeRemainingMinutes and bonusTimeExpiresAt require bonusTimeMinutes"
                .into(),
        );
    }

    Ok(())
}

fn validate_policy_schedule_boundary_optional_sections(
    boundary: &PolicyContractScheduleBoundary,
) -> PolicyContractValidationResult {
    if let Some(dst_boundary) = &boundary.dst_boundary {
        assert_local_time(&dst_boundary.local_time, "dstBoundary.localTime")?;
    }
    if let Some(clock_skew) = &boundary.clock_skew {
        assert_utc_timestamp(&clock_skew.observed_at, "clockSkew.observedAt")?;
        if clock_skew.allowed_skew_minutes < 0 {
            return Err("clockSkew.allowedSkewMinutes must be a non-negative number".into());
        }
    }
    if let Some(exception) = &boundary.exception {
        assert_utc_timestamp(&exception.starts_at, "exception.startsAt")?;
        assert_utc_timestamp(&exception.expires_at, "exception.expiresAt")?;
        if exception.expires_at <= exception.starts_at {
            return Err("schedule exceptions must expire after they start".into());
        }
    }
    if let Some(expiry) = &boundary.expiry {
        assert_utc_timestamp(&expiry.expires_at, "expiry.expiresAt")?;
        assert_utc_timestamp(&expiry.expired_at, "expiry.expiredAt")?;
        if expiry.expired_at < expiry.expires_at {
            return Err("expiry.expiredAt must be on or after expiry.expiresAt".into());
        }
        if boundary.state != PolicyContractScheduleBoundaryState::Expired
            && boundary.evaluated_at >= expiry.expires_at
        {
            return Err("non-expired schedule boundaries cannot be evaluated after expiry".into());
        }
    }
    Ok(())
}

pub(crate) fn assert_resolution_has_no_review_or_override_artifacts(
    resolution: &PolicyContractApprovalResolution,
    message: &'static str,
) -> PolicyContractValidationResult {
    if resolution.reviewed_by_actor_id.is_some()
        || resolution.reviewed_at.is_some()
        || resolution.audit_reference_id.is_some()
        || resolution.override_grant.is_some()
    {
        return Err(message.into());
    }
    Ok(())
}

pub(crate) fn assert_resolution_has_no_review_override_or_replay_artifacts(
    resolution: &PolicyContractApprovalResolution,
    message: &'static str,
) -> PolicyContractValidationResult {
    assert_resolution_has_no_review_or_override_artifacts(resolution, message)?;
    if resolution.replay_of_approval_id.is_some() {
        return Err(message.into());
    }
    Ok(())
}

fn assert_local_time(value: &str, field_name: &'static str) -> PolicyContractValidationResult {
    if value.len() != 5
        || !value.is_ascii()
        || value.as_bytes()[2] != b':'
        || parse_time_component(&value[0..2]).is_none_or(|hour| hour > 23)
        || parse_time_component(&value[3..5]).is_none_or(|minute| minute > 59)
    {
        return Err(match field_name {
            "localTime" => "localTime must use HH:MM 24-hour local time",
            _ => "policy contract local time must use HH:MM 24-hour local time",
        }
        .into());
    }

    Ok(())
}

pub(crate) fn assert_utc_timestamp(
    value: &str,
    field_name: &'static str,
) -> PolicyContractValidationResult {
    let bytes = value.as_bytes();
    if value.len() != 20
        || !value.is_ascii()
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return Err(match field_name {
            "evaluatedAt" => "evaluatedAt must be an ISO-8601 timestamp",
            "reviewedAt" => "reviewedAt must be an ISO-8601 timestamp",
            _ => "policy contract timestamps must be ISO-8601 UTC values",
        }
        .into());
    }

    let month = parse_time_component(&value[5..7]).unwrap_or(0);
    let day = parse_time_component(&value[8..10]).unwrap_or(0);
    let seconds = parse_time_component(&value[17..19]).unwrap_or(60);
    if month == 0 || month > 12 || day == 0 || day > 31 || seconds > 59 {
        return Err("policy contract timestamps must be ISO-8601 UTC values".into());
    }

    assert_local_time(&value[11..16], "policy timestamp inner local time")
}

fn parse_time_component(value: &str) -> Option<u8> {
    value.parse::<u8>().ok()
}
