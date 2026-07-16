use ocentra_parent_agent_protocol::activity::{
    ActivityEventKind, ActivityObserver, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::activity_query::ACTIVITY_QUERY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity_query::{ActivityRecentSummary, ActivityStoreRow};
use rusqlite::Row;

pub fn row_from_sqlite(row: &Row<'_>) -> rusqlite::Result<ActivityStoreRow> {
    Ok(ActivityStoreRow {
        event_id: row.get(0)?,
        observed_at: row.get(1)?,
        device_id: row.get(2)?,
        platform: row.get(3)?,
        observer: row.get(4)?,
        kind: row.get(5)?,
        subject_kind: row.get(6)?,
        subject_id: row.get(7)?,
        subject_display_name: row.get(8)?,
    })
}

pub fn summary_from_rows(limit: u64, rows: &[ActivityStoreRow]) -> ActivityRecentSummary {
    let most_recent = rows.first();
    ActivityRecentSummary {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        limit,
        returned: rows.len() as u64,
        first_observed_at: rows.last().map(|row| row.observed_at.clone()),
        last_observed_at: most_recent.map(|row| row.observed_at.clone()),
        last_event_id: most_recent.map(|row| row.event_id.clone()),
        most_recent_kind: most_recent
            .and_then(|row| ActivityEventKind::from_protocol_str(&row.kind)),
        most_recent_observer: most_recent
            .and_then(|row| ActivityObserver::from_protocol_str(&row.observer)),
        most_recent_subject_kind: most_recent
            .and_then(|row| ActivitySubjectKind::from_protocol_str(&row.subject_kind)),
        most_recent_subject_id: most_recent.map(|row| row.subject_id.clone()),
        most_recent_subject_name: most_recent.and_then(|row| row.subject_display_name.clone()),
    }
}
