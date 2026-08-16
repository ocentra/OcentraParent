use std::collections::BTreeSet;

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::app_game::AppGameSessionSummary;

use crate::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyRuntimeSession, AppGamePolicyRuntimeSessionRef, AppGamePolicySessionAccounting,
};
use crate::app_game_time_budget_types::AppGameTimeBudgetDurationMode;

pub(super) fn runtime_sessions(
    summaries: &[AppGameSessionSummary],
    duration_mode: AppGameTimeBudgetDurationMode,
) -> Result<Vec<AppGamePolicyRuntimeSession>, EventingError> {
    let mut seen = BTreeSet::new();
    summaries
        .iter()
        .map(|summary| runtime_session(summary, duration_mode, &mut seen))
        .collect()
}

fn runtime_session<'a>(
    summary: &'a AppGameSessionSummary,
    duration_mode: AppGameTimeBudgetDurationMode,
    seen: &mut BTreeSet<&'a str>,
) -> Result<AppGamePolicyRuntimeSession, EventingError> {
    if summary.foreground_duration_ms > summary.running_duration_ms {
        return Err(EventingError::InvalidValue {
            field: "app_game.time_budget.session_duration",
            value: String::from("foreground duration exceeds running duration"),
        });
    }
    if !seen.insert(summary.session_id.as_str()) {
        return Err(EventingError::InvalidValue {
            field: "app_game.time_budget.session_ref",
            value: String::from("duplicate stored session ref"),
        });
    }
    let duration_ms = match duration_mode {
        AppGameTimeBudgetDurationMode::Running => summary.running_duration_ms,
        AppGameTimeBudgetDurationMode::Foreground => summary.foreground_duration_ms,
    };
    Ok(AppGamePolicyRuntimeSession {
        session_ref: AppGamePolicyRuntimeSessionRef::parse(summary.session_id.clone())?,
        duration_seconds: rounded_up_seconds(duration_ms),
        accounting: AppGamePolicySessionAccounting::Counted,
    })
}

fn rounded_up_seconds(duration_ms: u64) -> u64 {
    duration_ms / 1_000 + u64::from(!duration_ms.is_multiple_of(1_000))
}
