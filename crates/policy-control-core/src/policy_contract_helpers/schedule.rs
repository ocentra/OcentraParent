#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::action::PolicyContractAction;
use super::authority::{
    PolicyContractApprovalKind, PolicyContractApprovalRequest, PolicyContractApprovalResolution,
    PolicyContractOverrideGrant, PolicyContractOverrideState, PolicyContractOverrideType,
};
use super::preview::PolicyContractPreviewBudgetBoundaryState;
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
        return Err("schedules must define at least one local window");
    }

    for window in &schedule.windows {
        if window.day_count == 0 {
            return Err("schedules must define at least one day for every window");
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
                return Err("dst-gap boundaries require dstBoundary details");
            };
            if dst_boundary.transition != PolicyContractScheduleDstTransition::SpringForward {
                return Err("dst-gap boundaries must use the spring-forward transition");
            }
            if matches!(
                dst_boundary.resolution,
                PolicyContractScheduleDstResolution::FirstOccurrence
                    | PolicyContractScheduleDstResolution::SecondOccurrence
            ) {
                return Err("dst-gap boundaries cannot use overlap-only resolutions");
            }
        }
        PolicyContractScheduleBoundaryState::DstOverlap => {
            let Some(dst_boundary) = &boundary.dst_boundary else {
                return Err("dst-overlap boundaries require dstBoundary details");
            };
            if dst_boundary.transition != PolicyContractScheduleDstTransition::FallBack {
                return Err("dst-overlap boundaries must use the fall-back transition");
            }
            if dst_boundary.resolution == PolicyContractScheduleDstResolution::SkipForward {
                return Err("dst-overlap boundaries cannot skip the repeated hour");
            }
        }
        PolicyContractScheduleBoundaryState::ClockSkew => {
            let Some(clock_skew) = &boundary.clock_skew else {
                return Err("clock-skew boundaries require clockSkew details");
            };
            if clock_skew.observed_skew_minutes.abs() <= clock_skew.allowed_skew_minutes {
                return Err("clock-skew boundaries require skew beyond the allowed tolerance");
            }
        }
        PolicyContractScheduleBoundaryState::ExceptionActive => {
            let Some(exception) = &boundary.exception else {
                return Err("exception-active boundaries require exception details");
            };
            if !(boundary.evaluated_at >= exception.starts_at
                && boundary.evaluated_at < exception.expires_at)
            {
                return Err(
                    "exception-active boundaries must be evaluated inside the exception window",
                );
            }
        }
        PolicyContractScheduleBoundaryState::Expired => {
            let Some(expiry) = &boundary.expiry else {
                return Err("expired schedule boundaries require expiry details");
            };
            if boundary.evaluated_at < expiry.expires_at {
                return Err("expired schedule boundaries must be evaluated on or after expiry");
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
            return Err("timeBudget.effectiveUntil must be after timeBudget.effectiveFrom");
        }
    }

    match time_budget.reset.kind {
        PolicyContractScheduleBudgetResetKind::Weekly => {
            if time_budget.reset.day.is_none() {
                return Err("weekly reset rules require timeBudget.reset.day");
            }
        }
        PolicyContractScheduleBudgetResetKind::Daily
        | PolicyContractScheduleBudgetResetKind::Monthly => {
            if time_budget.reset.day.is_some() {
                return Err("non-weekly reset rules cannot set timeBudget.reset.day");
            }
        }
    }

    match time_budget.carryover.mode {
        PolicyContractScheduleBudgetCarryoverMode::DiscardUnused => {
            if time_budget.carryover.max_minutes.is_some() {
                return Err("discard-unused carryover cannot set timeBudget.carryover.maxMinutes");
            }
        }
        PolicyContractScheduleBudgetCarryoverMode::CarryForward => {}
        PolicyContractScheduleBudgetCarryoverMode::CapCarryover => {
            if time_budget.carryover.max_minutes.unwrap_or(0) == 0 {
                return Err("cap-carryover requires timeBudget.carryover.maxMinutes");
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
        return Err("timeBudget.budgetWindowMinutes must be a positive number");
    }
    assert_utc_timestamp(&time_budget.reset_at, "timeBudget.resetAt")?;
    if time_budget.reset_at.as_str() <= evaluated_at {
        return Err("timeBudget.resetAt must be after evaluatedAt");
    }

    match time_budget.offline_recovery.state {
        PolicyContractScheduleOfflineRecoveryState::NotNeeded => {
            if time_budget.offline_recovery.recovered_at.is_some()
                || time_budget.offline_recovery.recovered_offline_minutes != 0
            {
                return Err("offline recovery state not-needed cannot include recovery artifacts");
            }
        }
        PolicyContractScheduleOfflineRecoveryState::RecoveredFromDevice
        | PolicyContractScheduleOfflineRecoveryState::RecomputedFromJournal => {
            let Some(recovered_at) = &time_budget.offline_recovery.recovered_at else {
                return Err("recovered offline timer states require recoveredAt");
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
            return Err("timeBudget.bonusTimeMinutes must be a positive number");
        }
        let Some(bonus_time_remaining_minutes) = time_budget.bonus_time_remaining_minutes else {
            return Err(
                "timeBudget.bonusTimeRemainingMinutes is required when bonusTimeMinutes are active",
            );
        };
        if bonus_time_remaining_minutes > bonus_time_minutes {
            return Err(
                "timeBudget.bonusTimeRemainingMinutes cannot exceed timeBudget.bonusTimeMinutes",
            );
        }
        let Some(bonus_time_expires_at) = &time_budget.bonus_time_expires_at else {
            return Err(
                "timeBudget.bonusTimeExpiresAt is required when bonusTimeMinutes are active",
            );
        };
        assert_utc_timestamp(bonus_time_expires_at, "timeBudget.bonusTimeExpiresAt")?;
        if bonus_time_expires_at.as_str() <= evaluated_at {
            return Err(
                "timeBudget.bonusTimeExpiresAt must be after evaluatedAt while bonus time is active",
            );
        }
    } else if time_budget.bonus_time_remaining_minutes.is_some()
        || time_budget.bonus_time_expires_at.is_some()
    {
        return Err(
            "timeBudget.bonusTimeRemainingMinutes and bonusTimeExpiresAt require bonusTimeMinutes",
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
            return Err("clockSkew.allowedSkewMinutes must be a non-negative number");
        }
    }
    if let Some(exception) = &boundary.exception {
        assert_utc_timestamp(&exception.starts_at, "exception.startsAt")?;
        assert_utc_timestamp(&exception.expires_at, "exception.expiresAt")?;
        if exception.expires_at <= exception.starts_at {
            return Err("schedule exceptions must expire after they start");
        }
    }
    if let Some(expiry) = &boundary.expiry {
        assert_utc_timestamp(&expiry.expires_at, "expiry.expiresAt")?;
        assert_utc_timestamp(&expiry.expired_at, "expiry.expiredAt")?;
        if expiry.expired_at < expiry.expires_at {
            return Err("expiry.expiredAt must be on or after expiry.expiresAt");
        }
        if boundary.state != PolicyContractScheduleBoundaryState::Expired
            && boundary.evaluated_at >= expiry.expires_at
        {
            return Err("non-expired schedule boundaries cannot be evaluated after expiry");
        }
    }
    Ok(())
}

fn validate_policy_approval_request(
    request: &PolicyContractApprovalRequest,
) -> PolicyContractValidationResult {
    assert_utc_timestamp(&request.requested_at, "approval.requestedAt")?;
    assert_utc_timestamp(&request.expires_at, "approval.expiresAt")?;
    if request.expires_at <= request.requested_at {
        return Err("approval.expiresAt must be after approval.requestedAt");
    }

    if let Some(schedule_boundary) = &request.schedule_boundary {
        validate_policy_schedule_boundary(schedule_boundary)?;
    }

    match request.kind {
        PolicyContractApprovalKind::BonusTime => {
            if request.requested_bonus_time_minutes.unwrap_or(0) == 0 {
                return Err(
                    "bonus-time requests must include a positive requestedBonusTimeMinutes value",
                );
            }
            let Some(schedule_boundary) = &request.schedule_boundary else {
                return Err("bonus-time requests must include scheduleBoundary details");
            };
            if schedule_boundary.time_budget.is_none() {
                return Err("bonus-time requests must include scheduleBoundary.timeBudget details");
            }
        }
        PolicyContractApprovalKind::AskParent | PolicyContractApprovalKind::TemporaryOverride => {
            if request.requested_bonus_time_minutes.is_some() {
                return Err("only bonus-time requests may include requestedBonusTimeMinutes");
            }
        }
    }

    Ok(())
}

fn validate_policy_override_grant(
    grant: &PolicyContractOverrideGrant,
    approval: &PolicyContractApprovalRequest,
    evaluated_at: &str,
) -> PolicyContractValidationResult {
    assert_utc_timestamp(&grant.effective_from, "override.effectiveFrom")?;
    assert_utc_timestamp(&grant.effective_until, "override.effectiveUntil")?;
    if grant.effective_until <= grant.effective_from {
        return Err("override.effectiveUntil must be after override.effectiveFrom");
    }

    match grant.override_type {
        PolicyContractOverrideType::TemporaryAllow => {
            if grant.action != PolicyContractAction::Allow || grant.bonus_time_minutes.is_some() {
                return Err("temporary-allow overrides must resolve to allow without bonus time");
            }
        }
        PolicyContractOverrideType::TemporaryBlock => {
            if grant.action != PolicyContractAction::Block || grant.bonus_time_minutes.is_some() {
                return Err("temporary-block overrides must resolve to block without bonus time");
            }
        }
        PolicyContractOverrideType::BonusTime => {
            if approval.kind != PolicyContractApprovalKind::BonusTime {
                return Err("bonus-time overrides require a bonus-time approval request");
            }
            if !matches!(
                grant.action,
                PolicyContractAction::Allow | PolicyContractAction::TimeLimit
            ) {
                return Err("bonus-time overrides must keep the action within allow or time-limit");
            }
            if grant.bonus_time_minutes.unwrap_or(0) == 0 {
                return Err("bonus-time overrides must include a positive bonusTimeMinutes value");
            }
        }
    }

    match grant.state {
        PolicyContractOverrideState::Active => {
            if evaluated_at >= grant.effective_until.as_str() {
                return Err("active overrides cannot already be past effectiveUntil");
            }
        }
        PolicyContractOverrideState::Expired => {
            if evaluated_at < grant.effective_until.as_str() {
                return Err("expired overrides require evaluatedAt on or after effectiveUntil");
            }
        }
        PolicyContractOverrideState::Revoked => {
            if evaluated_at < grant.effective_from.as_str() {
                return Err("revoked overrides require an effectiveFrom boundary");
            }
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
        return Err(message);
    }
    Ok(())
}

pub(crate) fn assert_resolution_has_no_review_override_or_replay_artifacts(
    resolution: &PolicyContractApprovalResolution,
    message: &'static str,
) -> PolicyContractValidationResult {
    assert_resolution_has_no_review_or_override_artifacts(resolution, message)?;
    if resolution.replay_of_approval_id.is_some() {
        return Err(message);
    }
    Ok(())
}

fn policy_action_strictness_rank(action: PolicyContractAction) -> i16 {
    match action {
        PolicyContractAction::Allow => 0,
        PolicyContractAction::Warn => 10,
        PolicyContractAction::Unknown => 20,
        PolicyContractAction::AskParent => 30,
        PolicyContractAction::TimeLimit => 40,
        PolicyContractAction::Block => 50,
    }
}

fn policy_preview_boundary_needs_manual_resolution(
    boundary: &PolicyContractScheduleBoundary,
) -> bool {
    matches!(
        boundary.state,
        PolicyContractScheduleBoundaryState::ClockSkew
    ) || matches!(
        (boundary.state, boundary.dst_boundary.as_ref()),
        (
            PolicyContractScheduleBoundaryState::DstGap
                | PolicyContractScheduleBoundaryState::DstOverlap,
            Some(PolicyContractScheduleDstBoundary {
                resolution: PolicyContractScheduleDstResolution::ManualRequired,
                ..
            })
        )
    ) || matches!(
        boundary.time_budget.as_ref(),
        Some(PolicyContractScheduleTimeBudgetStatus {
            clock_source: PolicyContractScheduleClockSource::ManualRequired,
            ..
        })
    ) || matches!(
        boundary.time_budget.as_ref(),
        Some(PolicyContractScheduleTimeBudgetStatus {
            offline_recovery: PolicyContractScheduleOfflineRecoveryStatus {
                state: PolicyContractScheduleOfflineRecoveryState::ManualRequired,
                ..
            },
            ..
        })
    )
}

fn policy_preview_boundary_bonus_time_state(
    boundary: &PolicyContractScheduleBoundary,
) -> Option<PolicyContractPreviewBudgetBoundaryState> {
    let time_budget = boundary.time_budget.as_ref()?;
    let bonus_time_minutes = time_budget.bonus_time_minutes?;
    let bonus_time_remaining_minutes = time_budget
        .bonus_time_remaining_minutes
        .unwrap_or(bonus_time_minutes);

    Some(if bonus_time_remaining_minutes < bonus_time_minutes {
        PolicyContractPreviewBudgetBoundaryState::BonusTimeExpiring
    } else {
        PolicyContractPreviewBudgetBoundaryState::BonusTimeActive
    })
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
        });
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
        });
    }

    let month = parse_time_component(&value[5..7]).unwrap_or(0);
    let day = parse_time_component(&value[8..10]).unwrap_or(0);
    let seconds = parse_time_component(&value[17..19]).unwrap_or(60);
    if month == 0 || month > 12 || day == 0 || day > 31 || seconds > 59 {
        return Err("policy contract timestamps must be ISO-8601 UTC values");
    }

    assert_local_time(&value[11..16], "policy timestamp inner local time")
}

fn parse_time_component(value: &str) -> Option<u8> {
    value.parse::<u8>().ok()
}
