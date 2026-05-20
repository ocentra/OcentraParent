use std::path::Path;

use duckdb::{params, Connection};
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityIngestStatus, ActivityRecentSummary, ActivityStoreRow,
    ACTIVITY_QUERY_SCHEMA_VERSION,
};

use crate::{
    activity_store_rows::{row_from_duckdb, summary_from_rows},
    ActivityJournal, ActivityStoreError,
};

pub struct ActivityStore {
    connection: Connection,
}

impl ActivityStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ActivityStoreError> {
        let connection = Connection::open(path)?;
        initialize_connection(&connection)?;
        Ok(Self { connection })
    }

    pub fn open_in_memory() -> Result<Self, ActivityStoreError> {
        let connection = Connection::open_in_memory()?;
        initialize_connection(&connection)?;
        Ok(Self { connection })
    }

    pub fn ingest_events(
        &self,
        events: &[ActivityEvent],
    ) -> Result<ActivityIngestStatus, ActivityStoreError> {
        let mut ingested = 0;
        let mut duplicate_events = 0;
        for event in events {
            if self.has_event_id(&event.event_id)? {
                duplicate_events += 1;
                continue;
            }
            self.insert_event(event)?;
            ingested += 1;
        }

        self.status_with_counts(ingested, duplicate_events)
    }

    pub fn ingest_journal(
        &self,
        journal: &ActivityJournal,
    ) -> Result<ActivityIngestStatus, ActivityStoreError> {
        let mut events = Vec::new();
        for line in journal.lines()? {
            events.push(journal.decrypt_line(&line)?);
        }
        self.ingest_events(&events)
    }

    pub fn status(&self) -> Result<ActivityIngestStatus, ActivityStoreError> {
        self.status_with_counts(0, 0)
    }

    pub fn recent_summary(&self, limit: u64) -> Result<ActivityRecentSummary, ActivityStoreError> {
        let rows = self.recent_rows(limit)?;
        Ok(summary_from_rows(limit, &rows))
    }

    fn status_with_counts(
        &self,
        events_ingested: u64,
        duplicate_events: u64,
    ) -> Result<ActivityIngestStatus, ActivityStoreError> {
        Ok(ActivityIngestStatus {
            schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
            database_ready: true,
            events_ingested,
            events_stored: self.event_count()?,
            duplicate_events,
            last_event_id: self.last_event_id()?,
        })
    }

    fn has_event_id(&self, event_id: &str) -> Result<bool, ActivityStoreError> {
        let count: i64 = self.connection.query_row(
            constants::duckdb::COUNT_ACTIVITY_EVENT_ID,
            params![event_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    fn insert_event(&self, event: &ActivityEvent) -> Result<(), ActivityStoreError> {
        let fields_json = serde_json::to_string(&event.fields)?;
        let evidence_json = serde_json::to_string(&event.evidence)?;
        self.connection.execute(
            constants::duckdb::INSERT_ACTIVITY_EVENT,
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

    fn event_count(&self) -> Result<u64, ActivityStoreError> {
        let count: i64 =
            self.connection
                .query_row(constants::duckdb::COUNT_ACTIVITY_EVENTS, [], |row| {
                    row.get(0)
                })?;
        Ok(count as u64)
    }

    fn last_event_id(&self) -> Result<Option<String>, ActivityStoreError> {
        let mut statement = self
            .connection
            .prepare(constants::duckdb::LAST_ACTIVITY_EVENT_ID)?;
        let mut rows = statement.query([])?;
        match rows.next()? {
            Some(row) => Ok(Some(row.get(0)?)),
            None => Ok(None),
        }
    }

    fn recent_rows(&self, limit: u64) -> Result<Vec<ActivityStoreRow>, ActivityStoreError> {
        let mut statement = self
            .connection
            .prepare(constants::duckdb::SELECT_RECENT_ACTIVITY)?;
        let rows = statement.query_map(params![limit as i64], row_from_duckdb)?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
}

fn initialize_connection(connection: &Connection) -> Result<(), ActivityStoreError> {
    connection.execute_batch(constants::duckdb::CREATE_ACTIVITY_EVENTS_TABLE)?;
    Ok(())
}
