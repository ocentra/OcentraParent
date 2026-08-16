use ocentra_parent_agent_protocol::app_game::{
    AppGameSessionDailyRollup, AppGameSessionSummary, APP_GAME_SCHEMA_VERSION,
};

use super::app_game_session_time::rollup_date;

pub fn daily_rollups_from_summaries(
    summaries: &[AppGameSessionSummary],
) -> Vec<AppGameSessionDailyRollup> {
    let mut rollups = Vec::new();
    for summary in summaries {
        let Some(rollup_date) = rollup_date(&summary.started_at) else {
            continue;
        };
        match rollups
            .iter()
            .position(|rollup: &AppGameSessionDailyRollup| {
                rollup.rollup_date == rollup_date
                    && rollup.classification_state == summary.classification_state
            }) {
            Some(index) => apply_summary_to_rollup(&mut rollups[index], summary),
            None => rollups.push(rollup_from_summary(rollup_date, summary)),
        }
    }
    rollups.sort_by(|left, right| {
        right
            .rollup_date
            .cmp(&left.rollup_date)
            .then_with(|| left.classification_state.cmp(&right.classification_state))
    });
    rollups
}

fn rollup_from_summary(
    rollup_date: String,
    summary: &AppGameSessionSummary,
) -> AppGameSessionDailyRollup {
    AppGameSessionDailyRollup {
        schema_version: APP_GAME_SCHEMA_VERSION,
        rollup_date,
        classification_state: summary.classification_state.clone(),
        session_count: 1,
        running_duration_ms: summary.running_duration_ms,
        foreground_duration_ms: summary.foreground_duration_ms,
        background_duration_ms: summary.background_duration_ms,
        evidence_count: summary.evidence_count,
        session_ids: vec![summary.session_id.clone()],
        evidence: summary.evidence.clone(),
    }
}

fn apply_summary_to_rollup(
    rollup: &mut AppGameSessionDailyRollup,
    summary: &AppGameSessionSummary,
) {
    rollup.session_count += 1;
    rollup.running_duration_ms += summary.running_duration_ms;
    rollup.foreground_duration_ms += summary.foreground_duration_ms;
    rollup.background_duration_ms += summary.background_duration_ms;
    rollup.evidence_count += summary.evidence_count;
    rollup.session_ids.push(summary.session_id.clone());
    rollup.evidence.extend(summary.evidence.clone());
}
