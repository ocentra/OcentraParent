#![forbid(unsafe_code)]

use super::super::super::{
    assert_utc_timestamp, PolicyContractScheduleTimeBudgetStatus, PolicyContractValidationResult,
};

pub(super) fn validate(
    time_budget: &PolicyContractScheduleTimeBudgetStatus,
    evaluated_at: &str,
) -> PolicyContractValidationResult {
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

        return Ok(());
    }

    if time_budget.bonus_time_remaining_minutes.is_some()
        || time_budget.bonus_time_expires_at.is_some()
    {
        return Err(
            "timeBudget.bonusTimeRemainingMinutes and bonusTimeExpiresAt require bonusTimeMinutes"
                .into(),
        );
    }

    Ok(())
}
