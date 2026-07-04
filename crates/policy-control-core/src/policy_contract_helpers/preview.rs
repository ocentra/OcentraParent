#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::{
    action::PolicyContractAction,
    schedule::{
        PolicyContractScheduleBoundary, PolicyContractScheduleBoundaryState,
        PolicyContractScheduleClockSource, PolicyContractScheduleDstBoundary,
        PolicyContractScheduleDstResolution, PolicyContractScheduleOfflineRecoveryState,
        PolicyContractScheduleOfflineRecoveryStatus, PolicyContractScheduleTimeBudgetStatus,
    },
    PolicyContractValidationResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractDecisionHandoffState {
    #[serde(rename = "not-requested")]
    NotRequested,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "handed-off")]
    HandedOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractPreviewConfirmationState {
    #[serde(rename = "confirmation-required")]
    ConfirmationRequired,
    #[serde(rename = "confirmed")]
    Confirmed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyContractPreviewBudgetBoundaryState {
    #[serde(rename = "within-budget")]
    WithinBudget,
    #[serde(rename = "bonus-time-active")]
    BonusTimeActive,
    #[serde(rename = "bonus-time-expiring")]
    BonusTimeExpiring,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractDecision {
    pub action: PolicyContractAction,
    pub dry_run: bool,
    pub enforcement_handoff_state: PolicyContractDecisionHandoffState,
    pub local_ai_result_id: Option<String>,
    pub evidence_reference_count: usize,
    pub rule_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContractPreview {
    pub confirmation_state: PolicyContractPreviewConfirmationState,
    pub confirmed_by_present: bool,
    pub confirmed_at: Option<String>,
    pub decision: PolicyContractDecision,
}

pub fn validate_policy_preview(preview: &PolicyContractPreview) -> PolicyContractValidationResult {
    if preview.confirmed_by_present != preview.confirmed_at.is_some() {
        return Err(
            "preview confirmation requires both confirmedBy and confirmedAt together".into(),
        );
    }
    if !preview.decision.dry_run {
        return Err("preview decisions must remain dry-run".into());
    }
    if preview.decision.enforcement_handoff_state != PolicyContractDecisionHandoffState::Disabled {
        return Err("preview decisions must keep enforcement handoff disabled".into());
    }

    match preview.confirmation_state {
        PolicyContractPreviewConfirmationState::ConfirmationRequired => {
            if preview.confirmed_by_present || preview.confirmed_at.is_some() {
                return Err(
                    "confirmation-required previews cannot include confirmedBy or confirmedAt"
                        .into(),
                );
            }
        }
        PolicyContractPreviewConfirmationState::Confirmed => {
            if !preview.confirmed_by_present || preview.confirmed_at.is_none() {
                return Err("confirmed previews require confirmedBy and confirmedAt".into());
            }
        }
    }

    Ok(())
}

pub fn resolve_policy_preview_budget_boundary_state(
    boundary: Option<&PolicyContractScheduleBoundary>,
) -> PolicyContractPreviewBudgetBoundaryState {
    let Some(boundary) = boundary else {
        return PolicyContractPreviewBudgetBoundaryState::WithinBudget;
    };

    if policy_preview_boundary_needs_manual_resolution(boundary) {
        return PolicyContractPreviewBudgetBoundaryState::ManualRequired;
    }

    if boundary.state == PolicyContractScheduleBoundaryState::Expired {
        return PolicyContractPreviewBudgetBoundaryState::Expired;
    }

    if let Some(bonus_time_state) = policy_preview_boundary_bonus_time_state(boundary) {
        return bonus_time_state;
    }

    PolicyContractPreviewBudgetBoundaryState::WithinBudget
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
