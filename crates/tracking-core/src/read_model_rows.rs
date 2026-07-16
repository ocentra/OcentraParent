use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use rusqlite::{params, Connection, Row};

pub(crate) struct TrackingStoreRow {
    pub event_id: String,
    pub observed_at: String,
    pub device_id: String,
    pub platform: String,
    pub observer: String,
    pub kind: String,
    pub subject_kind: String,
    pub subject_id: String,
    pub subject_display_name: Option<String>,
    pub fields: LogFields,
    pub evidence: Vec<ActivityEvidenceRef>,
}

pub(crate) fn tracking_rows(
    connection: &Connection,
    limit: u64,
) -> rusqlite::Result<Vec<TrackingStoreRow>> {
    let mut statement = connection.prepare(constants::sqlite::SELECT_RECENT_TRACKING_ACTIVITY)?;
    let rows = statement.query_map(
        params![
            constants::activity_event_kind::LOCATION_OBSERVED,
            constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
            constants::activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED,
            constants::activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED,
            constants::activity_event_kind::TRACKING_CHILD_CHECK_IN_RESPONDED,
            constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
            constants::activity_event_kind::TRACKING_RETENTION_DELETED,
            limit as i64
        ],
        row_from_sqlite,
    )?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

fn row_from_sqlite(row: &Row<'_>) -> rusqlite::Result<TrackingStoreRow> {
    let fields_json: String = row.get(9)?;
    let evidence_json: String = row.get(10)?;
    let fields = serde_json::from_str::<LogFields>(&fields_json).map_err(json_to_sqlite_error)?;
    let evidence = serde_json::from_str::<Vec<ActivityEvidenceRef>>(&evidence_json)
        .map_err(json_to_sqlite_error)?;

    Ok(TrackingStoreRow {
        event_id: row.get(0)?,
        observed_at: row.get(1)?,
        device_id: row.get(2)?,
        platform: row.get(3)?,
        observer: row.get(4)?,
        kind: row.get(5)?,
        subject_kind: row.get(6)?,
        subject_id: row.get(7)?,
        subject_display_name: row.get(8)?,
        fields,
        evidence,
    })
}

fn json_to_sqlite_error(error: serde_json::Error) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(error))
}

pub(crate) fn row_lifecycle_state(row: &TrackingStoreRow) -> TrackingReadModelRowLifecycleState {
    if row.kind == constants::activity_event_kind::TRACKING_RETENTION_DELETED {
        TrackingReadModelRowLifecycleState::Tombstone
    } else {
        TrackingReadModelRowLifecycleState::Active
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TrackingReadModelRowLifecycleState {
    Active,
    Tombstone,
}
