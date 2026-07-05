#![forbid(unsafe_code)]

use super::super::super::{
    assert_utc_timestamp, PolicyContractScheduleOfflineRecoveryState,
    PolicyContractScheduleTimeBudgetStatus, PolicyContractValidationResult,
};

pub(super) fn validate(
    time_budget: &PolicyContractScheduleTimeBudgetStatus,
) -> PolicyContractValidationResult {
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

    Ok(())
}
