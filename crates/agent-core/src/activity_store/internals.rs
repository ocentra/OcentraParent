use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::constants;
use rusqlite::{params, Connection};

use crate::ActivityStoreError;

pub(super) fn has_event_id(
    connection: &Connection,
    event_id: &str,
) -> Result<bool, ActivityStoreError> {
    let count: i64 = connection.query_row(
        constants::sqlite::COUNT_ACTIVITY_EVENT_ID,
        params![event_id],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

pub(super) fn insert_event(
    connection: &Connection,
    event: &ActivityEvent,
) -> Result<(), ActivityStoreError> {
    let fields_json = serde_json::to_string(&event.fields)?;
    let evidence_json = serde_json::to_string(&event.evidence)?;
    connection.execute(
        constants::sqlite::INSERT_ACTIVITY_EVENT,
        params![
            &event.event_id,
            &event.observed_at,
            &event.source.device_id,
            &event.source.platform,
            event.source.observer.as_protocol_str(),
            event.kind.as_protocol_str(),
            event.subject.kind.as_protocol_str(),
            &event.subject.subject_id,
            event.subject.display_name.as_deref(),
            fields_json,
            evidence_json
        ],
    )?;
    Ok(())
}

pub(super) fn event_count(connection: &Connection) -> Result<u64, ActivityStoreError> {
    let count: i64 = connection.query_row(constants::sqlite::COUNT_ACTIVITY_EVENTS, [], |row| {
        row.get(0)
    })?;
    Ok(count as u64)
}

pub(super) fn last_event_id(connection: &Connection) -> Result<Option<String>, ActivityStoreError> {
    let mut statement = connection.prepare(constants::sqlite::LAST_ACTIVITY_EVENT_ID)?;
    let mut rows = statement.query([])?;
    match rows.next()? {
        Some(row) => Ok(Some(row.get(0)?)),
        None => Ok(None),
    }
}
